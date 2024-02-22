export enum Type {
  FILE,
  DIRECTORY,
  DUMMY,
}
export interface RemoteFile {
  type: "file" | "dir";
  name: string;
  path: string;
}
interface CommonProps {
  id: string;
  type: Type;
  name: string;
  content?: string;
  path: string;
  parentId: string | undefined;
  depth: number;
}
export interface File extends CommonProps {}
export interface Directory extends CommonProps {
  files: File[];
  dirs: Directory[];
}

export function buildFileTree(data: RemoteFile[]): Directory {
  const dirs = data.filter((x) => x.type === "dir");
  const files = data.filter((x) => x.type === "file");
}
