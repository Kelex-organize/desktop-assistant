import { useEffect, useRef, type KeyboardEvent } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

function Launcher() {
  const inputRef = useRef<HTMLInputElement>(null);

  function hide(e: KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Escape") {
      void getCurrentWindow().hide();
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

  return (
    <div className="launcher">
      <input
        ref={inputRef}
        className="launcher-input"
        placeholder="¿Qué necesitás?"
        autoFocus
        onKeyDown={hide}
      />
    </div>
  );
}

export default Launcher;
