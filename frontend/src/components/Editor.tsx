import { useEffect, useMemo, useState } from "react";
import SideBar from "./editor/components/SideBar";
import { Code } from "./editor/Code";
import styled from "@emotion/styled";
import { File, buildFileTree, RemoteFile } from "./editor/utils/FileManager";
import { FileTree } from "./editor/components/FileTree";
import { Socket } from "socket.io-client";

// credits - https://codesandbox.io/s/monaco-tree-pec7u
export const Editor = ({
  files,
  onSelect,
  selectedFile,
  socket,
}: {
  files: RemoteFile[];
  onSelect: (file: File) => void;
  selectedFile: File | undefined;
  socket: Socket | null;
}) => {
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
      <Main>
        <SideBar>
          <FileTree
            rootDir={rootDir}
            selectedFile={selectedFile}
            onSelect={onSelect}
          />
        </SideBar>
        <Code socket={socket} selectedFile={selectedFile} />
      </Main>
    </div>
  );
};

const Main = styled.main`
  display: flex;
`;
