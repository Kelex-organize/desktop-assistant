import { useEffect, useRef, useState, type KeyboardEvent } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

function Launcher() {
  const inputRef = useRef<HTMLInputElement>(null);
  const [text, setText] = useState("");
  // TODO(#12): add the setters when you need them: `[message, setMessage]`, `[isError, setIsError]`.
  const [message] = useState("");
  const [isError] = useState(false);

  // TODO(#12): run `text` with `runText` (src/lib/api.ts), show the confirmation
  // or the error in `message`, and hide the launcher on success.
  async function submit() {
    throw new Error("TODO");
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
        void getCurrentWindow().hide();
      }
    });

    return () => {
      void unlisten.then((fn) => fn());
    };
  }, []);

  // TODO(#12): clear `text` and `message` when the launcher is hidden.

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
