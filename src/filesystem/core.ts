import type { DirectoryNode, FileSystem, FileSystemNode } from './types';
import { isDirectory } from './types';

/*
 * Creates an empty filesystem with a root directory
 */
export function createEmptyFileSystem(): FileSystem {
  const root: DirectoryNode = {
    name: '',
    type: 'directory',
    parent: null,
    children: new Map(),
  };

  return {
    root,
    currentDirectory: root,
  };
}

/*
 * Resolves a path to a filesystem node
 * Supports both absolute paths (starting with /) and relative paths
 */
export function resolvePath(
  path: string,
  current: DirectoryNode,
  root: DirectoryNode
): FileSystemNode | null {
  if (!path || path.trim() === '') {
    return current;
  }

  let node: FileSystemNode = path.startsWith('/') ? root : current;
  const components = path.split('/').filter(c => c !== '');
  
  if (path === '/' || (path.startsWith('/') && components.length === 0)) {
    return root;
  }

  for (const component of components) {
    if (component === '.') {
      continue;
    }
    
    if (component === '..') {
      if (node.parent !== null) {
        node = node.parent;
      }
      continue;
    }
    
    if (!isDirectory(node)) {
      return null;
    }
    
    const child = node.children.get(component);
    if (!child) {
      return null;
    }
    
    node = child;
  }

  return node;
}

/*
 * Get the absolute path of a node in the filesystem
 */
export function getNodePath(node: FileSystemNode): string {
  const parts: string[] = [];
  let current: FileSystemNode | null = node;
  
  while (current !== null && current.parent !== null) {
    parts.unshift(current.name);
    current = current.parent;
  }
  
  return '/' + parts.join('/');
}

/*
 * Get the current working directory path
 */
export function getCurrentPath(fs: FileSystem): string {
  return getNodePath(fs.currentDirectory);
}

/*
 * Change the current directory (immutable operation)
 * Returns a new FileSystem with updated currentDirectory
 */
export function changeDirectory(path: string, fs: FileSystem): FileSystem | null {
  const resolved = resolvePath(path, fs.currentDirectory, fs.root);
  
  if (!resolved) {
    return null;
  }
  
  if (!isDirectory(resolved)) {
    return null;
  }
  
  return {
    root: fs.root,
    currentDirectory: resolved,
  };
}
