import type { DirectoryNode, FileSystem, FileSystemNode } from './types';
import { isDirectory } from './types';

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

export function getNodePath(node: FileSystemNode): string {
  const parts: string[] = [];
  let current: FileSystemNode | null = node;
  
  while (current !== null && current.parent !== null) {
    parts.unshift(current.name);
    current = current.parent;
  }
  
  return '/' + parts.join('/');
}

export function getCurrentPath(fs: FileSystem): string {
  return getNodePath(fs.currentDirectory);
}

export function changeDirectory(path: string, fs: FileSystem): FileSystem | null {
  const resolved = resolvePath(path, fs.currentDirectory, fs.root);
  
  if (!resolved || !isDirectory(resolved)) {
    return null;
  }
  
  return {
    root: fs.root,
    currentDirectory: resolved,
  };
}

export function createFile(path: string, url: string, fs: FileSystem): FileSystem | null {
  const normalizedPath = path.trim();
  if (!normalizedPath) return null;

  const lastSlashIndex = normalizedPath.lastIndexOf('/');
  let parentPath: string;
  let fileName: string;

  if (lastSlashIndex === -1) {
    parentPath = '';
    fileName = normalizedPath;
  } else if (lastSlashIndex === 0) {
    parentPath = '/';
    fileName = normalizedPath.slice(1);
  } else {
    parentPath = normalizedPath.slice(0, lastSlashIndex);
    fileName = normalizedPath.slice(lastSlashIndex + 1);
  }

  if (!fileName) return null;

  const parentNode = parentPath === ''
    ? fs.currentDirectory
    : resolvePath(parentPath, fs.currentDirectory, fs.root);

  if (!parentNode || !isDirectory(parentNode)) return null;
  if (parentNode.children.has(fileName)) return null;

  const newRoot = cloneFilesystem(fs.root);
  const newParent = findNodeInClone(parentNode, newRoot, fs.root) as DirectoryNode;
  
  if (!newParent) return null;

  const newFile: import('./types').FileNode = {
    name: fileName,
    type: 'file',
    url,
    parent: newParent,
  };

  newParent.children.set(fileName, newFile);
  const newCurrentDirectory = findNodeInClone(fs.currentDirectory, newRoot, fs.root) as DirectoryNode;

  return {
    root: newRoot,
    currentDirectory: newCurrentDirectory,
  };
}

export function createDirectory(path: string, recursive: boolean, fs: FileSystem): FileSystem | null {
  const normalizedPath = path.trim();
  if (!normalizedPath) return null;

  const isAbsolute = normalizedPath.startsWith('/');
  const components = normalizedPath.split('/').filter(c => c !== '' && c !== '.');

  if (components.length === 0) return null;

  const newRoot = cloneFilesystem(fs.root);
  let currentNode: DirectoryNode = isAbsolute ? newRoot : findNodeInClone(fs.currentDirectory, newRoot, fs.root) as DirectoryNode;

  if (!currentNode) return null;

  for (let i = 0; i < components.length; i++) {
    const component = components[i];

    if (component === '..') {
      if (currentNode.parent !== null) {
        currentNode = currentNode.parent;
      }
      continue;
    }

    const existingChild = currentNode.children.get(component);

    if (existingChild) {
      if (i === components.length - 1) return null;
      if (!isDirectory(existingChild)) return null;
      currentNode = existingChild;
    } else {
      if (i < components.length - 1 && !recursive) return null;

      const newDir: DirectoryNode = {
        name: component,
        type: 'directory',
        parent: currentNode,
        children: new Map(),
      };

      currentNode.children.set(component, newDir);
      currentNode = newDir;
    }
  }

  const newCurrentDirectory = findNodeInClone(fs.currentDirectory, newRoot, fs.root) as DirectoryNode;

  return {
    root: newRoot,
    currentDirectory: newCurrentDirectory,
  };
}

function cloneFilesystem(node: DirectoryNode, parent: DirectoryNode | null = null): DirectoryNode {
  const clonedDir: DirectoryNode = {
    name: node.name,
    type: 'directory',
    parent,
    children: new Map(),
  };

  for (const [name, child] of node.children) {
    if (isDirectory(child)) {
      const clonedChild = cloneFilesystem(child, clonedDir);
      clonedDir.children.set(name, clonedChild);
    } else {
      const clonedFile: import('./types').FileNode = {
        name: child.name,
        type: 'file',
        url: (child as import('./types').FileNode).url,
        parent: clonedDir,
      };
      clonedDir.children.set(name, clonedFile);
    }
  }

  return clonedDir;
}

function findNodeInClone(
  originalNode: FileSystemNode,
  clonedRoot: DirectoryNode,
  _originalRoot: DirectoryNode
): FileSystemNode | null {
  const path = getNodePath(originalNode);
  return resolvePath(path, clonedRoot, clonedRoot);
}

export function deleteFile(path: string, fs: FileSystem): FileSystem | null {
  const normalizedPath = path.trim();
  if (!normalizedPath) return null;

  const node = resolvePath(normalizedPath, fs.currentDirectory, fs.root);

  if (!node) return null;
  if (isDirectory(node)) return null;
  if (!node.parent) return null;

  const newRoot = cloneFilesystem(fs.root);
  const parentPathStr = getNodePath(node.parent);

  const newParent = resolvePath(parentPathStr, newRoot, newRoot) as DirectoryNode;
  if (!newParent || !isDirectory(newParent)) return null;

  newParent.children.delete(node.name);

  const newCurrentDirectory = findNodeInClone(fs.currentDirectory, newRoot, fs.root) as DirectoryNode;

  return {
    root: newRoot,
    currentDirectory: newCurrentDirectory,
  };
}


export function deleteDirectory(path: string, recursive: boolean, fs: FileSystem): FileSystem | null {
  const normalizedPath = path.trim();
  if (!normalizedPath) return null;

  const node = resolvePath(normalizedPath, fs.currentDirectory, fs.root);

  if (!node) return null;
  if (!isDirectory(node)) return null;
  if (!node.parent) return null;

  if (!recursive && node.children.size > 0) return null;

  const newRoot = cloneFilesystem(fs.root);
  const parentPathStr = getNodePath(node.parent);

  const newParent = resolvePath(parentPathStr, newRoot, newRoot) as DirectoryNode;
  if (!newParent || !isDirectory(newParent)) return null;

  newParent.children.delete(node.name);

  let newCurrentDirectory = findNodeInClone(fs.currentDirectory, newRoot, fs.root) as DirectoryNode;

  if (!newCurrentDirectory) {
    newCurrentDirectory = newRoot;
  }

  return {
    root: newRoot,
    currentDirectory: newCurrentDirectory,
  };
}

