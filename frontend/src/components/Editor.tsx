import { useEffect, useMemo } from "react";
import { RemoteFile } from "./editor/utils/FileManager";
import { Socket } from "socket.io-client";
import SideBar from "./editor/components/SideBar";
type EditorProps = {
  files: RemoteFile[];
  selectedFile?: File;
  socket: Socket;
  onSelect: (file: File) => void;
};
export const Editor = ({
  files,
  selectedFile,
  socket,
  onSelect,
}: EditorProps) => {
  const rootDir = useMemo(() => {
    return buildFileTree(files);
  }, [files]);
  useEffect(() => {
    if (!selectedFile) {
      onSelect(rootDir.files[0]);
    }
  }, [selectedFile]);

  return (
    <div>
      <div>
        {/* main */}
        <SideBar>
            <FileTree></FileTree>
        </SideBar>
      </div>
    </div>
  );
};
