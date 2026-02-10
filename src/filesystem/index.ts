export type {
  FileSystemNode,
  FileNode,
  DirectoryNode,
  FileSystem,
} from './types';

export {
  isFile,
  isDirectory,
} from './types';

export {
  createEmptyFileSystem,
  resolvePath,
  getCurrentPath,
  changeDirectory,
  getNodePath,
  createFile,
  createDirectory,
} from './core';
