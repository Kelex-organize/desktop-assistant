import { useRef } from "react";

function Launcher() {
  const inputRef = useRef<HTMLInputElement>(null);

  // TODO(#9): hide the window when Escape is pressed.
  // TODO(#9): hide the window when it loses focus.
  // TODO(#9): focus the input every time the window gets focus.

  return (
    <div className="launcher">
      <input ref={inputRef} className="launcher-input" placeholder="¿Qué necesitás?" autoFocus />
    </div>
  );
}

export default Launcher;
