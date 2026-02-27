import type { FileSystem } from '@/filesystem';
import {
  changeDirectory,
  createDirectory,
  createFile,
  deleteDirectory,
  deleteFile,
  getCurrentPath,
  isDirectory,
  isFile,
  listDirectory,
  moveNode,
  readFile,
  resolvePath,
} from '../filesystem';
import type { ParsedCommand } from './parser';
import { hasFlag } from './parser';

type CommandResult = {
  output?: string[];
  error?: string;
  nextFs?: FileSystem;
  clear?: boolean;
};

type CommandContext = {
  fs: FileSystem;
  openUrl: (url: string) => boolean;
  theme: string;
  setTheme: (next: string) => void;
};

type CommandDefinition = {
  name: string;
  description: string;
  usage: string;
  run: (command: ParsedCommand, context: CommandContext) => CommandResult;
};

const AVAILABLE_THEMES = [
  'amber',
  'gruvbox',
  'paper',
  'dracula',
  'nord',
  'tokyo-night',
  'solarized-dark',
  'monokai',
];

function normalizeUrl(input: string): string {
  const trimmed = input.trim();
  if (trimmed === '') return trimmed;
  if (/^[a-zA-Z][a-zA-Z0-9+.-]*:\/\//.test(trimmed)) return trimmed;
  if (trimmed.startsWith('//')) return `https:${trimmed}`;
  return `https://${trimmed}`;
}

function formatCommandHelp(command: CommandDefinition): string[] {
  return [
    `${command.name} - ${command.description}`,
    `usage: ${command.usage}`,
  ];
}

const commandList: CommandDefinition[] = [
  {
    name: 'help',
    description: 'list available commands',
    usage: 'help',
    run: (command) => {
      if (command.args.length > 0) {
        return { error: "help: this command only lists commands. use 'man <command>'" };
      }

      const names = commandList
        .slice()
        .sort((a, b) => a.name.localeCompare(b.name))
        .map((cmd) => cmd.name);
      return { output: [names.join('  '), "run man <command> for usage"] };
    },
  },
  {
    name: 'man',
    description: 'show help for a command',
    usage: 'man <command>',
    run: (command) => {
      if (command.args.length === 0) {
        return { error: 'man: missing command name' };
      }
      const target = command.args[0];
      const match = resolveCommand(target);
      if (!match) {
        return { error: `man: ${target} not found` };
      }
      return { output: formatCommandHelp(match) };
    },
  },
  {
    name: 'pwd',
    description: 'print the current directory',
    usage: 'pwd',
    run: (_command, context) => ({ output: [getCurrentPath(context.fs)] }),
  },
  {
    name: 'ls',
    description: 'list directory contents',
    usage: 'ls [path]',
    run: (command, context) => {
      const target = command.args[0] ?? '.';
      const node = resolvePath(target, context.fs.currentDirectory, context.fs.root);
      if (!node) return { error: `ls: cannot access '${target}': no such file or directory` };
      if (!isDirectory(node)) return { error: `ls: '${target}' is not a directory` };
      const entries = listDirectory(target, context.fs) ?? [];
      const items = entries
        .map((child) => (isDirectory(child) ? `${child.name}/` : child.name))
        .sort((a, b) => a.localeCompare(b));
      return { output: items };
    },
  },
  {
    name: 'cd',
    description: 'change directory',
    usage: 'cd [path]',
    run: (command, context) => {
      const target = command.args[0] ?? '/';
      const nextFs = changeDirectory(target, context.fs);
      if (!nextFs) return { error: `cd: ${target}: not a directory` };
      return { nextFs, output: [] };
    },
  },
  {
    name: 'mkdir',
    description: 'create a directory',
    usage: 'mkdir [-p] <path>',
    run: (command, context) => {
      const recursive = hasFlag(command, '-p', '--parents');
      const pathArg = command.args[0];
      if (!pathArg) return { error: 'mkdir: missing operand' };
      const nextFs = createDirectory(pathArg, recursive, context.fs);
      if (!nextFs) return { error: `mkdir: cannot create directory '${pathArg}'` };
      return { nextFs, output: [] };
    },
  },
  {
    name: 'touch',
    description: 'create a bookmark file with URL',
    usage: 'touch <path> <url>',
    run: (command, context) => {
      const pathArg = command.args[0];
      const url = command.args[1];
      if (!pathArg) return { error: 'touch: missing file operand' };
      if (!url) return { error: 'touch: missing URL operand' };
      const normalizedUrl = normalizeUrl(url);
      const nextFs = createFile(pathArg, normalizedUrl, context.fs);
      if (!nextFs) return { error: `touch: cannot create file '${pathArg}'` };
      return { nextFs, output: [] };
    },
  },
  {
    name: 'rm',
    description: 'remove a file (use -r for directories)',
    usage: 'rm [-r] <path>',
    run: (command, context) => {
      const pathArg = command.args[0];
      if (!pathArg) return { error: 'rm: missing operand' };
      const recursive = hasFlag(command, '-r', '-R', '--recursive');
      const node = resolvePath(pathArg, context.fs.currentDirectory, context.fs.root);
      if (!node) return { error: `rm: cannot remove '${pathArg}': no such file or directory` };
      if (isDirectory(node)) {
        if (!recursive) return { error: `rm: cannot remove '${pathArg}': is a directory` };
        const nextFs = deleteDirectory(pathArg, true, context.fs);
        if (!nextFs) return { error: `rm: failed to remove '${pathArg}'` };
        return { nextFs, output: [] };
      }
      const nextFs = deleteFile(pathArg, context.fs);
      if (!nextFs) return { error: `rm: cannot remove '${pathArg}'` };
      return { nextFs, output: [] };
    },
  },
  {
    name: 'mv',
    description: 'move or rename a file or directory',
    usage: 'mv <source> <destination>',
    run: (command, context) => {
      const source = command.args[0];
      const destination = command.args[1];
      if (!source || !destination) return { error: 'mv: missing operand' };
      const nextFs = moveNode(source, destination, context.fs);
      if (!nextFs) return { error: `mv: cannot move '${source}'` };
      return { nextFs, output: [] };
    },
  },
  {
    name: 'cat',
    description: 'show bookmark URL',
    usage: 'cat <path>',
    run: (command, context) => {
      const pathArg = command.args[0];
      if (!pathArg) return { error: 'cat: missing file operand' };
      const node = resolvePath(pathArg, context.fs.currentDirectory, context.fs.root);
      if (!node) return { error: `cat: ${pathArg}: no such file or directory` };
      if (!isFile(node)) return { error: `cat: ${pathArg}: is a directory` };
      const value = readFile(pathArg, context.fs);
      if (!value) return { error: `cat: ${pathArg}: no such file or directory` };
      return { output: [value] };
    },
  },
  {
    name: 'open',
    description: 'open bookmark URL in a new tab',
    usage: 'open <path>',
    run: (command, context) => {
      const pathArg = command.args[0];
      if (!pathArg) return { error: 'open: missing file operand' };
      const node = resolvePath(pathArg, context.fs.currentDirectory, context.fs.root);
      if (!node) return { error: `open: ${pathArg}: no such file or directory` };
      if (!isFile(node)) return { error: `open: ${pathArg}: is a directory` };
      const opened = context.openUrl(node.url);
      if (!opened) return { error: `open: failed to open ${node.url}` };
      return { output: [] };
    },
  },
  {
    name: 'theme',
    description: 'list or set the terminal theme',
    usage: 'theme [name|list]',
    run: (command, context) => {
      if (command.args.length === 0) {
        return { output: [context.theme] };
      }

      const arg = command.args[0];
      if (arg === 'list') {
        return { output: AVAILABLE_THEMES };
      }

      if (!AVAILABLE_THEMES.includes(arg)) {
        return { error: `theme: unknown theme '${arg}'` };
      }

      context.setTheme(arg);
      return { output: [arg] };
    },
  },
  {
    name: 'clear',
    description: 'clear terminal output',
    usage: 'clear',
    run: () => ({ clear: true }),
  },
];

function resolveCommand(name: string): CommandDefinition | undefined {
  return commandList.find((cmd) => cmd.name === name);
}

export function executeCommand(parsed: ParsedCommand, context: CommandContext): CommandResult {
  const command = resolveCommand(parsed.name);
  if (!command) {
    return { error: `${parsed.name}: command not found` };
  }

  try {
    return command.run(parsed, context);
  } catch (error) {
    console.error(error);
    return { error: `error executing ${parsed.name}` };
  }
}

export function getCommandNames(): string[] {
  return commandList.map((cmd) => cmd.name);
}

export function getCommandDefinition(name: string): CommandDefinition | undefined {
  return resolveCommand(name);
}
