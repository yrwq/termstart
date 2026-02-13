import type { DirectoryNode, FileNode, FileSystem, FileSystemNode } from '@/filesystem/types';
import { isDirectory } from '@/filesystem/types';
import { createEmptyFileSystem, getCurrentPath, resolvePath } from '@/filesystem/core';

type SerializedFile = {
  name: string;
  type: 'file';
  url: string;
};

type SerializedDirectory = {
  name: string;
  type: 'directory';
  children: Array<SerializedDirectory | SerializedFile>;
};

type SerializedFileSystem = {
  root: SerializedDirectory;
  currentPath: string;
};

function serializeNode(node: FileSystemNode): SerializedDirectory | SerializedFile {
  if (!isDirectory(node)) {
    return {
      name: node.name,
      type: 'file',
      url: (node as FileNode).url,
    };
  }

  return {
    name: node.name,
    type: 'directory',
    children: Array.from(node.children.values()).map(serializeNode),
  };
}

export function serializeFileSystem(fs: FileSystem): string {
  const payload: SerializedFileSystem = {
    root: serializeNode(fs.root) as SerializedDirectory,
    currentPath: getCurrentPath(fs),
  };
  return JSON.stringify(payload);
}

function deserializeNode(
  node: SerializedDirectory | SerializedFile,
  parent: DirectoryNode | null
): FileSystemNode | null {
  if (node.type === 'file') {
    const fileNode: FileNode = {
      name: node.name,
      type: 'file',
      url: node.url,
      parent,
    };
    return fileNode;
  }

  const directory: DirectoryNode = {
    name: node.name,
    type: 'directory',
    parent,
    children: new Map(),
  };

  for (const child of node.children) {
    const childNode = deserializeNode(child, directory);
    if (!childNode) return null;
    directory.children.set(childNode.name, childNode);
  }

  return directory;
}

export function deserializeFileSystem(raw: string): FileSystem | null {
  let parsed: SerializedFileSystem;
  try {
    parsed = JSON.parse(raw) as SerializedFileSystem;
  } catch {
    return null;
  }

  if (!parsed || !parsed.root || parsed.root.type !== 'directory') {
    return null;
  }

  const root = deserializeNode(parsed.root, null);
  if (!root || !isDirectory(root)) return null;

  const currentPath = typeof parsed.currentPath === 'string' ? parsed.currentPath : '/';
  const currentNode = resolvePath(currentPath, root, root);

  return {
    root,
    currentDirectory: currentNode && isDirectory(currentNode) ? currentNode : root,
  };
}

export function createEmptyOrFallback(raw: string | null): FileSystem {
  if (!raw) return createEmptyFileSystem();
  const parsed = deserializeFileSystem(raw);
  return parsed ?? createEmptyFileSystem();
}
