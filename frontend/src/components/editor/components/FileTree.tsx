import { networkInterfaces } from "os";
import { Directory, sortDir, File, sortFile } from "../utils/FileManager";
import React, { useState } from "react";
import styled from "@emotion/styled";
import { getIcon } from "./Icon";

interface FileTreeProps {
  rootDir: Directory;
  selectedFile: File | undefined;
  onSelect: (file: File) => void;
}

interface SubTreeProps {
  directory: Directory;
  selectedFile: File | undefined;
  onSelect: (file: File) => void;
}

export const FileTree = (props: FileTreeProps) => {
  return <SubTree directory={props.rootDir} {...props} />;
};

const SubTree = (props: SubTreeProps) => {
  return (
    <div>
      {props.directory.dirs.sort(sortDir).map((dir) => (
        <React.Fragment key={dir.id}>
          <DirDiv
            directory={dir}
            selectedFile={props.selectedFile}
            onSelect={props.onSelect}
          />
        </React.Fragment>
      ))}
      {props.directory.files.sort(sortFile).map((file) => (
        <React.Fragment key={file.id}>
          <FileDiv
            file={file}
            selectedFile={props.selectedFile}
            onClick={() => props.onSelect(file)}
          />
        </React.Fragment>
      ))}
    </div>
  );
};
const FileDiv = ({
  file,
  icon,
  selectedFile,
  onClick,
}: {
  file: File | Directory;
  icon?: string;
  selectedFile: File | undefined;
  onClick: () => void;
}) => {
  const isSelected = (selectedFile && selectedFile.id === file.id) as boolean;
  const depth = file.depth;
  return (
    <Div depth={depth} isSelected={isSelected} onClick={onClick}>
      <FileIcon name={icon} extension={file.name.split(".").pop() || ""} />
      <span style={{ marginLeft: 1 }}>{file.name}</span>
    </Div>
  );
};

const Div = styled.div<{
  depth: number;
  isSelected: boolean;
}>`
  display: flex;
  align-items: center;
  padding-left: ${(props) => props.depth * 16}px;
  background-color: ${(props) =>
    props.isSelected ? "#242424" : "transparent"};

  :hover {
    cursor: pointer;
    background-color: #242424;
  }
`;

const DirDiv = ({
  directory,
  selectedFile,
  onSelect,
}: {
  directory: Directory;
  selectedFile: File | undefined;
  onSelect: (file: File) => void;
}) => {
  let defaultOpen = false;
  if (selectedFile) {
    defaultOpen = isChildSelected(directory, selectedFile);
  }
  const [open, setOpen] = useState(defaultOpen);
  return <></>;
};

const isChildSelected = (dir: Directory, selectedFile: File) => {
  let res = false;
  function isChild(dir: Directory, file: File) {
    if (selectedFile.parentId === dir.id) {
      res = true;
      return;
    }
    if (selectedFile.parentId === "0") {
      res = false;
      return;
    }
    dir.dirs.forEach((item) => {
      isChild(item, file);
    });
  }
  isChild(dir, selectedFile);

  return res;
};

const FileIcon = ({
  extension,
  name,
}: {
  name?: string;
  extension?: string;
}) => {
  let icon = getIcon(extension || "", name || "");
  return <Span>{icon}</Span>;
};

const Span = styled.span`
  display: flex;
  width: 32px;
  height: 32px;
  justify-content: center;
  align-items: center;
`;
