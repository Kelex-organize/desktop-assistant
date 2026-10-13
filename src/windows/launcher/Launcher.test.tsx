import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, act, waitFor } from "@testing-library/react";
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

  it("does not hide the window when another key is pressed", () => {
    render(<Launcher />);

    fireEvent.keyDown(screen.getByRole("textbox"), { key: "a" });

    expect(tauri.hide).not.toHaveBeenCalled();
  });

  it("hides the window when it loses focus", () => {
    render(<Launcher />);

    act(() => tauri.emitFocus({ payload: false }));

    expect(tauri.hide).toHaveBeenCalledTimes(1);
  });

  it("focuses the input when the window gets focus", () => {
    render(<Launcher />);
    const input = screen.getByRole("textbox");
    input.blur();
    expect(document.activeElement).not.toBe(input);

    act(() => tauri.emitFocus({ payload: true }));

    expect(document.activeElement).toBe(input);
  });

  it("stops listening for focus changes when it unmounts", async () => {
    const { unmount } = render(<Launcher />);

    unmount();

    await waitFor(() => expect(tauri.unlisten).toHaveBeenCalledTimes(1));
  });
});
