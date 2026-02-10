import { useMemo, useState, useRef } from 'react';
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
  const [history, setHistory] = useState<HistoryLine[]>([ ]);
  const counterRef = useRef(3);

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
  };

  return (
    <div className="bg-stone-900 text-orange-200 rounded-xl shadow-xl">
      <div className="px-4 py-4 h-[28rem] overflow-y-auto font-mono text-sm space-y-1">
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
            value={input}
            onChange={(event) => setInput(event.target.value)}
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
