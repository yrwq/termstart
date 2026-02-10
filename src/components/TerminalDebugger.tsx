import { useMemo, useRef, useState } from 'react';
import type { FileSystem, FileSystemNode } from '../filesystem';
import {
  changeDirectory,
  createDirectory,
  createFile,
  deleteDirectory,
  deleteFile,
  getCurrentPath,
  isDirectory,
  isFile,
  resolvePath,
} from '../filesystem';

type TerminalDebuggerProps = {
  fs: FileSystem;
  onFsChange: (next: FileSystem) => void;
};

type HistoryLine = {
  id: number;
  kind: 'input' | 'output' | 'error';
  text: string;
};

type CommandResult = {
  output?: string[];
  error?: string;
  nextFs?: FileSystem;
  clear?: boolean;
};

const DEFAULT_URL = 'about:blank';

function tokenize(input: string): string[] {
  const tokens: string[] = [];
  let current = '';
  let inQuotes = false;
  let quoteChar: '"' | '\'' | null = null;

  for (let i = 0; i < input.length; i += 1) {
    const char = input[i];

    if ((char === '"' || char === '\'') && !inQuotes) {
      inQuotes = true;
      quoteChar = char as '"' | '\'';
      continue;
    }

    if (inQuotes && char === quoteChar) {
      inQuotes = false;
      quoteChar = null;
      continue;
    }

    if (!inQuotes && /\s/.test(char)) {
      if (current.length > 0) {
        tokens.push(current);
        current = '';
      }
      continue;
    }

    current += char;
  }

  if (current.length > 0) {
    tokens.push(current);
  }

  return tokens;
}

function formatNodeName(node: FileSystemNode): string {
  if (isDirectory(node)) return `${node.name}/`;
  return node.name;
}

function listDirectory(node: FileSystemNode): string[] | null {
  if (!isDirectory(node)) return null;
  const items = Array.from(node.children.values())
    .map((child) => formatNodeName(child))
    .sort((a, b) => a.localeCompare(b));
  return items;
}

function renderTree(node: FileSystemNode, prefix = ''): string[] {
  if (isFile(node)) {
    return [`${prefix}${node.name}`];
  }

  const lines: string[] = [];
  const children = Array.from(node.children.values()).sort((a, b) => a.name.localeCompare(b.name));
  const label = node.name === '' ? '/' : `${node.name}/`;
  lines.push(`${prefix}${label}`);

  const childPrefix = `${prefix}  `;
  for (const child of children) {
    lines.push(...renderTree(child, childPrefix));
  }

  return lines;
}

function parsePathArg(args: string[]): string | null {
  if (args.length === 0) return null;
  return args[0];
}

function executeCommand(input: string, fs: FileSystem): CommandResult {
  const tokens = tokenize(input.trim());
  if (tokens.length === 0) {
    return { output: [] };
  }

  const [command, ...args] = tokens;

  switch (command) {
    case 'help': {
      return {
        output: [
          'Available commands:',
          '  help                    Show this help message',
          '  pwd                     Print current directory',
          '  ls [path]                List directory contents',
          '  tree [path]              Render directory tree',
          '  cd <path>                Change directory',
          '  mkdir [-p] <path>        Create directory (use -p for recursive)',
          '  touch <path> [url]       Create file (defaults url to about:blank)',
          '  cat <path>               Show file url',
          '  rm <path>                Delete file',
          '  rmdir [-r] <path>        Delete directory (use -r for recursive)',
          '  clear                    Clear terminal output',
        ],
      };
    }

    case 'pwd': {
      return { output: [getCurrentPath(fs)] };
    }

    case 'ls': {
      const target = parsePathArg(args) ?? '.';
      const node = resolvePath(target, fs.currentDirectory, fs.root);
      if (!node) return { error: `ls: cannot access '${target}': No such file or directory` };
      const items = listDirectory(node);
      if (!items) return { error: `ls: '${target}' is not a directory` };
      return { output: items.length === 0 ? ['(empty)'] : items };
    }

    case 'tree': {
      const target = parsePathArg(args) ?? '.';
      const node = resolvePath(target, fs.currentDirectory, fs.root);
      if (!node) return { error: `tree: cannot access '${target}': No such file or directory` };
      return { output: renderTree(node) };
    }

    case 'cd': {
      const target = parsePathArg(args);
      if (!target) return { error: 'cd: missing operand' };
      const nextFs = changeDirectory(target, fs);
      if (!nextFs) return { error: `cd: ${target}: Not a directory` };
      return { nextFs, output: [] };
    }

    case 'mkdir': {
      const recursive = args.includes('-p') || args.includes('--parents');
      const pathArg = args.find((arg) => !arg.startsWith('-'));
      if (!pathArg) return { error: 'mkdir: missing operand' };
      const nextFs = createDirectory(pathArg, recursive, fs);
      if (!nextFs) return { error: `mkdir: cannot create directory '${pathArg}'` };
      return { nextFs, output: [] };
    }

    case 'touch': {
      const pathArg = args[0];
      if (!pathArg) return { error: 'touch: missing file operand' };
      const url = args[1] ?? DEFAULT_URL;
      const nextFs = createFile(pathArg, url, fs);
      if (!nextFs) return { error: `touch: cannot create file '${pathArg}'` };
      return { nextFs, output: [] };
    }

    case 'cat': {
      const pathArg = parsePathArg(args);
      if (!pathArg) return { error: 'cat: missing file operand' };
      const node = resolvePath(pathArg, fs.currentDirectory, fs.root);
      if (!node) return { error: `cat: ${pathArg}: No such file or directory` };
      if (!isFile(node)) return { error: `cat: ${pathArg}: Is a directory` };
      return { output: [node.url] };
    }

    case 'rm': {
      const pathArg = parsePathArg(args);
      if (!pathArg) return { error: 'rm: missing file operand' };
      const nextFs = deleteFile(pathArg, fs);
      if (!nextFs) return { error: `rm: cannot remove '${pathArg}'` };
      return { nextFs, output: [] };
    }

    case 'rmdir': {
      const recursive = args.includes('-r') || args.includes('-R') || args.includes('--recursive');
      const pathArg = args.find((arg) => !arg.startsWith('-'));
      if (!pathArg) return { error: 'rmdir: missing operand' };
      const nextFs = deleteDirectory(pathArg, recursive, fs);
      if (!nextFs) return { error: `rmdir: failed to remove '${pathArg}'` };
      return { nextFs, output: [] };
    }

    case 'clear': {
      return { clear: true };
    }

    default: {
      return { error: `Unknown command: ${command}. Type 'help' for commands.` };
    }
  }
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

    const result = executeCommand(trimmed, fs);

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
