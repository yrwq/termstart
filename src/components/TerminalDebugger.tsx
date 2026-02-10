import { useEffect, useMemo, useRef, useState } from 'react';
import type { FileSystem } from '../filesystem';
import {
  getCurrentPath,
} from '../filesystem';
import { executeCommand } from '../terminal/commands';
import { parseCommand } from '../terminal/parser';

type TerminalDebuggerProps = {
  fs: FileSystem;
  onFsChange: (next: FileSystem) => void;
};

type HistoryLine = {
  id: number;
  kind: 'input' | 'output' | 'error';
  text: string;
};


function createOpenUrlHandler(): (url: string) => boolean {
  return (url: string) => {
    const opened = window.open(url, '_blank', 'noopener,noreferrer');
    return Boolean(opened);
  };
}

export function TerminalDebugger({ fs, onFsChange }: TerminalDebuggerProps) {
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
    const result = executeCommand(parsed, { fs, openUrl: createOpenUrlHandler() });

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

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
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

  return (
    <div className="bg-stone-900 text-orange-200 rounded-xl shadow-xl">
      <div
        ref={outputRef}
        className="px-4 py-4 h-[28rem] overflow-y-auto font-mono text-sm space-y-1"
      >
        {history.length === 0 ? (
          <div className="text-orange-200"> </div>
        ) : (
          history.map((line) => (
            <div
              key={line.id}
              className={line.kind === 'error' ? 'text-rose-400' : 'text-orange-200'}
            >
              {line.text}
            </div>
          ))
        )}
      </div>
      <form onSubmit={handleSubmit} className="px-4 py-3">
        <div className="flex items-center gap-2 font-mono text-sm">
          <span className="text-orange-200">{prompt}</span>
          <input
            ref={inputRef}
            value={input}
            onChange={handleChange}
            onKeyDown={handleKeyDown}
            onClick={syncCursorFromEvent}
            onKeyUp={syncCursorFromEvent}
            className="flex-1 bg-transparent outline-none text-orange-200 placeholder:text-orange-200/50"
            placeholder="type here"
            autoComplete="off"
            spellCheck={false}
          />
        </div>
      </form>
    </div>
  );
}
