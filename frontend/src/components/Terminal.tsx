import { useEffect, useRef } from "react";
import { Socket } from "socket.io-client";
import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
const fitAddon = new FitAddon();
function ab2str(buf: string) {
  //@ts-ignore
  return String.fromCharCode.apply(null, new Uint8Array(buf));
}
const OPTIONS_TERM = {
  useStyle: true,
  screenKeys: true,
  cursorBlink: true,
  cols: 200,
  theme: {
    background: "black",
  },
};
const TerminalComponent = ({ socket }: { socket: Socket | null }) => {
  const terminalRef = useRef(null);
  useEffect(() => {
    if (!terminalRef || !terminalRef.current || !socket) {
      return;
    }
    socket.emit("requestTerminal");
    socket.on("terminal", terminalHandler);
    const term = new Terminal(OPTIONS_TERM);
    term.loadAddon(fitAddon);
    term.open(terminalRef.current);
    fitAddon.fit();
    function terminalHandler({ data }: { data: any }) {
      if (data instanceof ArrayBuffer) {
        console.error(data);
        // @ts-ignore
        console.log(ab2str(data));
        // @ts-ignore
        term.write(ab2str(data));
      }
    }
    term.onData((data) => {
      socket.emit("terminalData", {
        data,
      });
    });

    socket.emit("terminalData", {
      data: "\n",
    });

    return () => {
      socket.off("terminal");
    };
  }, [terminalRef]);
  return (
    <div
      style={{ width: "40vw", height: "400px", textAlign: "left" }}
      ref={terminalRef}
    ></div>
  );
};

export default TerminalComponent;
