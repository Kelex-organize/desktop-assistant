import { useEffect, useRef, useState, type KeyboardEvent } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { runText } from "../../lib/api";

function Launcher() {
  const inputRef = useRef<HTMLInputElement>(null);
  const [text, setText] = useState("");
  const [message, setMessage] = useState("");
  const [isError, setIsError] = useState(false);

  async function submit() {
    try {
      const result = await runText(text);
      setIsError(false);
      setMessage(result);
      await new Promise((resolve) => setTimeout(resolve, 800));
      await getCurrentWindow().hide();
    } catch (error) {
      setIsError(true);
      setMessage(typeof error === "string" ? error : "Something went wrong");
    }
  }

  function handleKeyDown(e: KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Escape") {
      void getCurrentWindow().hide();
    } else if (e.key === "Enter") {
      void submit();
    }
  }

  useEffect(() => {
    const unlisten = getCurrentWindow().onFocusChanged(({ payload: focused }) => {
      if (focused) {
        inputRef.current?.focus();
      } else {
        setText("");
        setMessage("");
        setIsError(false);
        void getCurrentWindow().hide();
      }
    });

    return () => {
      void unlisten.then((fn) => fn());
    };
  }, []);

  return (
    <div className="launcher">
      <input
        ref={inputRef}
        className="launcher-input"
        placeholder="What do you need?"
        autoFocus
        value={text}
        onChange={(e) => setText(e.target.value)}
        onKeyDown={handleKeyDown}
      />
      <p className={isError ? "launcher-message launcher-message-error" : "launcher-message"}>
        {message}
      </p>
    </div>
  );
}

export default Launcher;
