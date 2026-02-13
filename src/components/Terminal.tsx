import { useEffect, useMemo, useRef, useState } from 'react';
import type { FileSystem } from '@/filesystem';
import {
  getCurrentPath,
  isDirectory,
  listDirectory,
} from '@/filesystem';
import { executeCommand, getCommandNames } from '@/terminal/commands';
import { parseCommand } from '@/terminal/parser';

type TerminalProps = {
  fs: FileSystem;
  onFsChange: (next: FileSystem) => void;
  theme: string;
  onThemeChange: (next: string) => void;
};

type HistoryLine = {
  id: number;
  kind: 'input' | 'output' | 'error';
  text: string;
};


function createOpenUrlHandler(): (url: string) => boolean {
  return (url: string) => {
    try {
      window.open(url, '_blank', 'noopener,noreferrer');
      return true;
    } catch {
      return false;
    }
  };
}

export function Terminal({ fs, onFsChange, theme, onThemeChange }: TerminalProps) {
  const [input, setInput] = useState('');
  const [cursor, setCursor] = useState(0);
  const [history, setHistory] = useState<HistoryLine[]>([]);
  const [commandHistory, setCommandHistory] = useState<string[]>([]);
  const [historyIndex, setHistoryIndex] = useState<number | null>(null);
  const [draftInput, setDraftInput] = useState('');
  const counterRef = useRef(1);
  const inputRef = useRef<HTMLInputElement | null>(null);
  const outputRef = useRef<HTMLDivElement | null>(null);

  const prompt = useMemo(() => `${getCurrentPath(fs)} $`, [fs]);

  const appendLine = (line: HistoryLine) => {
    setHistory((prev) => [...prev, line]);
    counterRef.current += 1;
  };

  const appendLines = (lines: string[], kind: 'output' | 'error') => {
    if (lines.length === 0) return;
    setHistory((prev) => [
      ...prev,
      ...lines.map((text, index) => ({
        id: counterRef.current + index,
        kind,
        text,
      })),
    ]);
    counterRef.current += lines.length;
  };

  const runCommand = (commandText: string) => {
    const trimmed = commandText.trimEnd();
    if (trimmed.length === 0) return;

    appendLine({ id: counterRef.current, kind: 'input', text: `${prompt} ${trimmed}` });
    setCommandHistory((prev) => [...prev, trimmed]);
    setHistoryIndex(null);
    setDraftInput('');
    const parsed = parseCommand(trimmed);
    if ('error' in parsed) {
      if (parsed.error !== 'Empty command') {
        appendLines([parsed.error], 'error');
      }
      return;
    }
    const result = executeCommand(parsed, {
      fs,
      openUrl: createOpenUrlHandler(),
      theme,
      setTheme: onThemeChange,
    });

    if (result.clear) {
      setHistory([]);
      return;
    }

    if (result.nextFs) {
      onFsChange(result.nextFs);
    }

    if (result.error) {
      appendLines([result.error], 'error');
    }

    if (result.output && result.output.length > 0) {
      appendLines(result.output, 'output');
    }
  };

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    runCommand(input);
    setInput('');
    setCursor(0);
  };

  const handleHistoryUp = () => {
    if (commandHistory.length === 0) return;
    if (historyIndex === null) {
      setDraftInput(input);
      const nextIndex = commandHistory.length - 1;
      setHistoryIndex(nextIndex);
      const nextValue = commandHistory[nextIndex];
      setInput(nextValue);
      setCursor(nextValue.length);
      return;
    }
    const nextIndex = Math.max(0, historyIndex - 1);
    setHistoryIndex(nextIndex);
    const nextValue = commandHistory[nextIndex];
    setInput(nextValue);
    setCursor(nextValue.length);
  };

  const handleHistoryDown = () => {
    if (commandHistory.length === 0 || historyIndex === null) return;
    const nextIndex = historyIndex + 1;
    if (nextIndex >= commandHistory.length) {
      setHistoryIndex(null);
      setInput(draftInput);
      setCursor(draftInput.length);
      return;
    }
    setHistoryIndex(nextIndex);
    const nextValue = commandHistory[nextIndex];
    setInput(nextValue);
    setCursor(nextValue.length);
  };

  const longestCommonPrefix = (values: string[]): string => {
    if (values.length === 0) return '';

    let prefix = values[0];
    for (let i = 1; i < values.length; i += 1) {
      const value = values[i];
      let j = 0;
      const max = Math.min(prefix.length, value.length);
      while (j < max && prefix[j] === value[j]) {
        j += 1;
      }
      prefix = prefix.slice(0, j);
      if (prefix.length === 0) return '';
    }

    return prefix;
  };

  const replaceInputToken = (start: number, end: number, value: string) => {
    const nextInput = input.slice(0, start) + value + input.slice(end);
    setInput(nextInput);
    setCursor(start + value.length);
  };

  const completeCommandName = (token: string, tokenStart: number, tokenEnd: number) => {
    const matches = getCommandNames()
      .filter((name) => name.startsWith(token))
      .sort((a, b) => a.localeCompare(b));

    if (matches.length === 0) return;

    if (matches.length === 1) {
      replaceInputToken(tokenStart, tokenEnd, `${matches[0]} `);
      return;
    }

    const prefix = longestCommonPrefix(matches);
    if (prefix.length > token.length) {
      replaceInputToken(tokenStart, tokenEnd, prefix);
      return;
    }

    appendLines([matches.join('  ')], 'output');
  };

  const completePath = (
    commandName: string,
    token: string,
    tokenStart: number,
    tokenEnd: number
  ) => {
    const lastSlash = token.lastIndexOf('/');
    const basePath = lastSlash >= 0 ? token.slice(0, lastSlash + 1) : '';
    const nameFragment = lastSlash >= 0 ? token.slice(lastSlash + 1) : token;
    const lookupPath = basePath === ''
      ? '.'
      : (basePath === '/' ? '/' : basePath.slice(0, -1));

    const directoryOnlyCommands = new Set(['cd', 'ls', 'mkdir']);
    const fileOnlyCommands = new Set<string>();

    const entries = listDirectory(lookupPath, fs);
    if (!entries) return;

    const matches = entries
      .filter((entry) => entry.name.startsWith(nameFragment))
      .filter((entry) => {
        if (directoryOnlyCommands.has(commandName)) return isDirectory(entry);
        if (fileOnlyCommands.has(commandName)) return !isDirectory(entry);
        return true;
      })
      .map((entry) => ({
        text: `${basePath}${entry.name}${isDirectory(entry) ? '/' : ''}`,
        isDirectory: isDirectory(entry),
      }))
      .sort((a, b) => {
        if (a.isDirectory !== b.isDirectory) {
          return a.isDirectory ? -1 : 1;
        }
        return a.text.localeCompare(b.text);
      });

    if (matches.length === 0) return;

    if (matches.length === 1) {
      const [match] = matches;
      replaceInputToken(tokenStart, tokenEnd, match.isDirectory ? match.text : `${match.text} `);
      return;
    }

    const options = matches.map((match) => match.text);
    const prefix = longestCommonPrefix(options);
    if (prefix.length > token.length) {
      replaceInputToken(tokenStart, tokenEnd, prefix);
      return;
    }

    appendLines([options.join('  ')], 'output');
  };

  const handleTabCompletion = () => {
    const tokenStart = (() => {
      let i = cursor;
      while (i > 0 && !/\s/.test(input[i - 1])) {
        i -= 1;
      }
      return i;
    })();
    const tokenEnd = (() => {
      let i = cursor;
      while (i < input.length && !/\s/.test(input[i])) {
        i += 1;
      }
      return i;
    })();

    const token = input.slice(tokenStart, tokenEnd);
    const beforeToken = input.slice(0, tokenStart).trim();
    const previousTokens = beforeToken.length > 0 ? beforeToken.split(/\s+/) : [];

    if (previousTokens.length === 0) {
      completeCommandName(token, tokenStart, tokenEnd);
      return;
    }

    completePath(previousTokens[0], token, tokenStart, tokenEnd);
  };

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Tab') {
      event.preventDefault();
      handleTabCompletion();
      return;
    }

    if (event.key === 'ArrowUp') {
      event.preventDefault();
      handleHistoryUp();
      return;
    }

    if (event.key === 'ArrowDown') {
      event.preventDefault();
      handleHistoryDown();
      return;
    }

    if (event.key === 'Home' || (event.ctrlKey && event.key.toLowerCase() === 'a')) {
      event.preventDefault();
      setCursor(0);
      return;
    }

    if (event.key === 'End' || (event.ctrlKey && event.key.toLowerCase() === 'e')) {
      event.preventDefault();
      setCursor(input.length);
      return;
    }

    if (event.ctrlKey && event.key.toLowerCase() === 'l') {
      event.preventDefault();
      setHistory([]);
      return;
    }

    if (event.key === 'ArrowLeft') {
      event.preventDefault();
      setCursor((prev) => Math.max(0, prev - 1));
      return;
    }

    if (event.key === 'ArrowRight') {
      event.preventDefault();
      setCursor((prev) => Math.min(input.length, prev + 1));
      return;
    }

    if (event.key === 'Backspace') {
      if (cursor === 0) {
        event.preventDefault();
        return;
      }
      event.preventDefault();
      const nextValue = input.slice(0, cursor - 1) + input.slice(cursor);
      setInput(nextValue);
      setCursor(cursor - 1);
      return;
    }

    if (event.key === 'Delete') {
      if (cursor >= input.length) {
        event.preventDefault();
        return;
      }
      event.preventDefault();
      const nextValue = input.slice(0, cursor) + input.slice(cursor + 1);
      setInput(nextValue);
      setCursor(cursor);
    }
  };

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const nextValue = event.target.value;
    const selectionStart = event.target.selectionStart ?? nextValue.length;
    setInput(nextValue);
    setCursor(selectionStart);
    if (historyIndex !== null) {
      setHistoryIndex(null);
      setDraftInput('');
    }
  };

  const focusInput = () => {
    inputRef.current?.focus();
  };

  const handleBlur = () => {
    window.setTimeout(() => {
      if (document.activeElement !== inputRef.current) {
        focusInput();
      }
    }, 0);
  };

  const syncCursorFromEvent = (event: React.SyntheticEvent<HTMLInputElement>) => {
    const target = event.currentTarget;
    const selectionStart = target.selectionStart ?? target.value.length;
    setCursor(selectionStart);
  };

  useEffect(() => {
    const inputEl = inputRef.current;
    if (!inputEl) return;
    const safeCursor = Math.min(cursor, input.length);
    if (inputEl.selectionStart !== safeCursor || inputEl.selectionEnd !== safeCursor) {
      inputEl.setSelectionRange(safeCursor, safeCursor);
    }
  }, [cursor, input]);

  useEffect(() => {
    if (!outputRef.current) return;
    outputRef.current.scrollTop = outputRef.current.scrollHeight;
  }, [history]);

  useEffect(() => {
    focusInput();
  }, []);

  return (
    <div className="terminal-shell" onMouseDown={focusInput}>
      <div
        ref={outputRef}
        className="px-4 py-4 h-112 overflow-y-auto font-mono text-sm space-y-1"
      >
        {history.length === 0 ? (
          <div className="terminal-muted"> </div>
        ) : (
          history.map((line) => (
            <div
              key={line.id}
              className={line.kind === 'error' ? 'terminal-error' : 'terminal-text'}
            >
              {line.text}
            </div>
          ))
        )}
      </div>
      <form onSubmit={handleSubmit} className="px-4 py-3">
        <div className="flex items-center gap-2 font-mono text-sm">
          <span className="terminal-text">{prompt}</span>
          <input
            ref={inputRef}
            value={input}
            onChange={handleChange}
            onKeyDown={handleKeyDown}
            onClick={syncCursorFromEvent}
            onKeyUp={syncCursorFromEvent}
            onBlur={handleBlur}
            className="terminal-input flex-1 bg-transparent outline-none"
            placeholder=""
            autoComplete="off"
            spellCheck={false}
          />
        </div>
      </form>
    </div>
  );
}
