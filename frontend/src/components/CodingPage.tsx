import { Socket, io } from "socket.io-client";
import Terminal from "./Terminal";
import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { RemoteFile } from "./editor/utils/FileManager";
import axios from "axios";
const useSocket = (replId: string) => {
  const [socket, setSocket] = useState<Socket | null>(null);
  useEffect(() => {
    const newSocket = io(`ws://${replId}.peetcode.com`);
    setSocket(newSocket);

    return () => {
      newSocket.disconnect();
    };
  }, [replId]);

  return socket;
};
const CodingPage = () => {
  const [podCreated, setPodCreated] = useState(false);
  const [searchParams] = useSearchParams();
  const replId = searchParams.get("replId") ?? "";

  useEffect(() => {
    if (replId) {
      axios
        .post(`http://localhost:3002/start`, { replId })
        .then(() => setPodCreated(true))
        .catch((err) => console.error(err));
    }
  }, []);

  if (!podCreated) {
    return <>Booting...</>;
  }
  return <CodingPagePostPodCreation />;
};

const CodingPagePostPodCreation = () => {
  const [searchParams] = useSearchParams();
  const [fileStructure, setFileStructure] = useState<RemoteFile[]>([]);
  const [loaded, setLoaded] = useState(false);

  const replId = searchParams.get("replId") ?? "";
  const socket = useSocket(replId);
  useEffect(() => {
    if (socket) {
      socket.on("loaded", ({ rootContent }: { rootContent: RemoteFile[] }) => {
        setLoaded(true);
        setFileStructure(rootContent);
      });
    }
  }, [socket]);
  if (!loaded) {
    return <span>"Loading..."</span>;
  }

  return (
    <div>
      <div>
        {/* button Container */}
        <button>See output</button>
      </div>
      <div>
        {/* workspace */}
        <div>
            {/* lefta panel */}
            
        </div>
      </div>
    </div>
  );
};

export default CodingPage;
