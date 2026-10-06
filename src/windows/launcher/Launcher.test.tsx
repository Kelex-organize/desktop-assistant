import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import Launcher from "./Launcher";

type FocusEvent = { payload: boolean };

// `vi.mock` is hoisted above the imports, so the fakes it uses must be created
// with `vi.hoisted` to exist by then.
const tauri = vi.hoisted(() => ({
  hide: vi.fn(),
  unlisten: vi.fn(),
  // Filled in by the fake `onFocusChanged`: lets a test simulate a focus change.
  emitFocus: vi.fn() as (event: FocusEvent) => void,
}));

vi.mock("@tauri-apps/api/window", () => ({
  getCurrentWindow: () => ({
    hide: tauri.hide,
    onFocusChanged: (callback: (event: FocusEvent) => void) => {
      tauri.emitFocus = callback;
      return Promise.resolve(tauri.unlisten);
    },
  }),
}));

describe("Launcher", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("hides the window when Escape is pressed", () => {
    render(<Launcher />);

    fireEvent.keyDown(screen.getByRole("textbox"), { key: "Escape" });

    expect(tauri.hide).toHaveBeenCalledTimes(1);
  });

  it.todo("does not hide the window when another key is pressed");
  it.todo("hides the window when it loses focus");
  it.todo("focuses the input when the window gets focus");
  it.todo("stops listening for focus changes when it unmounts");
});
