export interface FileSystemNode {
  name: string;
  type: 'file' | 'directory';
  parent: DirectoryNode | null;
}

export interface FileNode extends FileSystemNode {
  type: 'file';
  url: string;
}

export interface DirectoryNode extends FileSystemNode {
  type: 'directory';
  children: Map<string, FileSystemNode>;
}

export interface FileSystem {
  root: DirectoryNode;
  currentDirectory: DirectoryNode;
}

export function isFile(node: FileSystemNode): node is FileNode {
  return node.type === 'file';
}

export function isDirectory(node: FileSystemNode): node is DirectoryNode {
  return node.type === 'directory';
}
