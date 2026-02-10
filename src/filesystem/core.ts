import type { DirectoryNode, FileSystem, FileSystemNode } from './types';
import { isDirectory, isFile } from './types';

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

export function readFile(path: string, fs: FileSystem): string | null {
  const normalizedPath = path.trim();
  if (!normalizedPath) return null;

  const node = resolvePath(normalizedPath, fs.currentDirectory, fs.root);
  if (!node || !isFile(node)) return null;

  return node.url;
}

export function listDirectory(path: string | null, fs: FileSystem): FileSystemNode[] | null {
  const target = path && path.trim().length > 0 ? path.trim() : '.';
  const node = resolvePath(target, fs.currentDirectory, fs.root);
  if (!node || !isDirectory(node)) return null;
  return Array.from(node.children.values());
}

function getParentAndName(path: string): { parentPath: string; name: string } | null {
  const normalizedPath = path.trim();
  if (!normalizedPath) return null;

  const lastSlashIndex = normalizedPath.lastIndexOf('/');
  if (lastSlashIndex === -1) {
    return { parentPath: '', name: normalizedPath };
  }
  if (lastSlashIndex === 0) {
    return { parentPath: '/', name: normalizedPath.slice(1) };
  }
  return {
    parentPath: normalizedPath.slice(0, lastSlashIndex),
    name: normalizedPath.slice(lastSlashIndex + 1),
  };
}

function isDescendantOf(node: FileSystemNode, ancestor: FileSystemNode): boolean {
  let current: FileSystemNode | null = node;
  while (current) {
    if (current === ancestor) return true;
    current = current.parent;
  }
  return false;
}

export function moveNode(sourcePath: string, destinationPath: string, fs: FileSystem): FileSystem | null {
  const sourceNode = resolvePath(sourcePath, fs.currentDirectory, fs.root);
  if (!sourceNode || !sourceNode.parent) return null;

  const destinationNode = resolvePath(destinationPath, fs.currentDirectory, fs.root);

  let destinationParent: DirectoryNode | null = null;
  let destinationName = sourceNode.name;

  if (destinationNode) {
    if (isFile(destinationNode)) return null;
    destinationParent = destinationNode;
    destinationName = sourceNode.name;
  } else {
    const parsed = getParentAndName(destinationPath);
    if (!parsed || !parsed.name) return null;
    const parentNode = parsed.parentPath === ''
      ? fs.currentDirectory
      : resolvePath(parsed.parentPath, fs.currentDirectory, fs.root);
    if (!parentNode || !isDirectory(parentNode)) return null;
    destinationParent = parentNode;
    destinationName = parsed.name;
  }

  if (!destinationParent) return null;
  if (destinationParent.children.has(destinationName)) return null;

  if (isDirectory(sourceNode) && isDescendantOf(destinationParent, sourceNode)) {
    return null;
  }

  if (destinationParent === sourceNode.parent && destinationName === sourceNode.name) {
    return fs;
  }

  const newRoot = cloneFilesystem(fs.root);
  const clonedSource = findNodeInClone(sourceNode, newRoot, fs.root);
  const clonedSourceParent = findNodeInClone(sourceNode.parent, newRoot, fs.root) as DirectoryNode;
  const clonedDestinationParent = findNodeInClone(destinationParent, newRoot, fs.root) as DirectoryNode;

  if (!clonedSource || !clonedSourceParent || !clonedDestinationParent) return null;

  clonedSourceParent.children.delete(sourceNode.name);
  clonedSource.name = destinationName;
  clonedSource.parent = clonedDestinationParent;
  clonedDestinationParent.children.set(destinationName, clonedSource);

  const sourceAbsPath = getNodePath(sourceNode);
  const destinationParentPath = getNodePath(destinationParent);
  const destinationAbsPath =
    destinationParentPath === '/'
      ? `/${destinationName}`
      : `${destinationParentPath}/${destinationName}`;
  const currentPath = getNodePath(fs.currentDirectory);

  let newCurrentPath = currentPath;
  if (currentPath === sourceAbsPath || currentPath.startsWith(`${sourceAbsPath}/`)) {
    newCurrentPath = destinationAbsPath + currentPath.slice(sourceAbsPath.length);
  }

  let newCurrentDirectory = resolvePath(newCurrentPath, newRoot, newRoot) as DirectoryNode | null;
  if (!newCurrentDirectory || !isDirectory(newCurrentDirectory)) {
    newCurrentDirectory = newRoot;
  }

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
