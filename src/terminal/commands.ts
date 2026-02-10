import type { FileSystem } from '../filesystem';
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
};

type CommandDefinition = {
  name: string;
  description: string;
  usage: string;
  run: (command: ParsedCommand, context: CommandContext) => CommandResult;
};

const commandList: CommandDefinition[] = [
  {
    name: 'help',
    description: 'Show available commands or help for one command',
    usage: 'help [command]',
    run: (command) => {
      if (command.args.length === 0) {
        return {
          output: [
            'Commands:',
            ...commandList.map((cmd) => `  ${cmd.name.padEnd(8)} ${cmd.description}`),
          ],
        };
      }

      const target = command.args[0];
      const match = commandList.find((cmd) => cmd.name === target);
      if (!match) {
        return { error: `help: no help topics match '${target}'` };
      }

      return {
        output: [
          `${match.name}: ${match.description}`,
          `Usage: ${match.usage}`,
        ],
      };
    },
  },
  {
    name: 'pwd',
    description: 'Print the current directory',
    usage: 'pwd',
    run: (_command, context) => ({ output: [getCurrentPath(context.fs)] }),
  },
  {
    name: 'ls',
    description: 'List directory contents',
    usage: 'ls [path]',
    run: (command, context) => {
      const target = command.args[0] ?? '.';
      const node = resolvePath(target, context.fs.currentDirectory, context.fs.root);
      if (!node) return { error: `ls: cannot access '${target}': No such file or directory` };
      if (!isDirectory(node)) return { error: `ls: '${target}' is not a directory` };
      const entries = listDirectory(target, context.fs) ?? [];
      const items = entries
        .map((child) => (isDirectory(child) ? `${child.name}/` : child.name))
        .sort((a, b) => a.localeCompare(b));
      return { output: items.length === 0 ? ['(empty)'] : items };
    },
  },
  {
    name: 'cd',
    description: 'Change directory',
    usage: 'cd [path]',
    run: (command, context) => {
      const target = command.args[0] ?? '/';
      const nextFs = changeDirectory(target, context.fs);
      if (!nextFs) return { error: `cd: ${target}: Not a directory` };
      return { nextFs, output: [] };
    },
  },
  {
    name: 'mkdir',
    description: 'Create a directory',
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
    description: 'Create a bookmark file with URL',
    usage: 'touch <path> <url>',
    run: (command, context) => {
      const pathArg = command.args[0];
      const url = command.args[1];
      if (!pathArg) return { error: 'touch: missing file operand' };
      if (!url) return { error: 'touch: missing URL operand' };
      const nextFs = createFile(pathArg, url, context.fs);
      if (!nextFs) return { error: `touch: cannot create file '${pathArg}'` };
      return { nextFs, output: [] };
    },
  },
  {
    name: 'rm',
    description: 'Remove a file (use -r for directories)',
    usage: 'rm [-r] <path>',
    run: (command, context) => {
      const pathArg = command.args[0];
      if (!pathArg) return { error: 'rm: missing operand' };
      const recursive = hasFlag(command, '-r', '-R', '--recursive');
      const node = resolvePath(pathArg, context.fs.currentDirectory, context.fs.root);
      if (!node) return { error: `rm: cannot remove '${pathArg}': No such file or directory` };
      if (isDirectory(node)) {
        if (!recursive) return { error: `rm: cannot remove '${pathArg}': Is a directory` };
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
    description: 'Move or rename a file or directory',
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
    description: 'Show bookmark URL',
    usage: 'cat <path>',
    run: (command, context) => {
      const pathArg = command.args[0];
      if (!pathArg) return { error: 'cat: missing file operand' };
      const node = resolvePath(pathArg, context.fs.currentDirectory, context.fs.root);
      if (!node) return { error: `cat: ${pathArg}: No such file or directory` };
      if (!isFile(node)) return { error: `cat: ${pathArg}: Is a directory` };
      const value = readFile(pathArg, context.fs);
      if (!value) return { error: `cat: ${pathArg}: No such file or directory` };
      return { output: [value] };
    },
  },
  {
    name: 'open',
    description: 'Open bookmark URL in a new tab',
    usage: 'open <path>',
    run: (command, context) => {
      const pathArg = command.args[0];
      if (!pathArg) return { error: 'open: missing file operand' };
      const node = resolvePath(pathArg, context.fs.currentDirectory, context.fs.root);
      if (!node) return { error: `open: ${pathArg}: No such file or directory` };
      if (!isFile(node)) return { error: `open: ${pathArg}: Is a directory` };
      const opened = context.openUrl(node.url);
      if (!opened) return { error: `open: failed to open ${node.url}` };
      return { output: [`Opened ${node.url}`] };
    },
  },
  {
    name: 'clear',
    description: 'Clear terminal output',
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
    return { error: `Unknown command: ${parsed.name}. Type 'help' for commands.` };
  }

  try {
    return command.run(parsed, context);
  } catch (error) {
    console.error(error);
    return { error: `Error executing ${parsed.name}` };
  }
}

export function getCommandNames(): string[] {
  return commandList.map((cmd) => cmd.name);
}

export function getCommandDefinition(name: string): CommandDefinition | undefined {
  return resolveCommand(name);
}
