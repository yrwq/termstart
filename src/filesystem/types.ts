/*
 * Base interface for all filesystem nodes
 */
export interface FileSystemNode {
  name: string;
  type: 'file' | 'directory';
  parent: DirectoryNode | null;
}

/*
 * File node containing a URL bookmark
 */
export interface FileNode extends FileSystemNode {
  type: 'file';
  url: string;
}

/*
 * Directory node containing child nodes
 */
export interface DirectoryNode extends FileSystemNode {
  type: 'directory';
  children: Map<string, FileSystemNode>;
}

/*
 * Complete filesystem state with root and current directory
 */
export interface FileSystem {
  root: DirectoryNode;
  currentDirectory: DirectoryNode;
}

/*
 * Type guard to check if a node is a file
 */
export function isFile(node: FileSystemNode): node is FileNode {
  return node.type === 'file';
}

/*
 * Type guard to check if a node is a directory
 */
export function isDirectory(node: FileSystemNode): node is DirectoryNode {
  return node.type === 'directory';
}
