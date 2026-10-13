import { beforeEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, act, waitFor } from "@testing-library/react";
import Launcher from "./Launcher";

type FocusEvent = { payload: boolean };

// `vi.mock` is hoisted above the imports, so the fakes it uses must be created
// with `vi.hoisted` to exist by then.
const tauri = vi.hoisted(() => ({
  hide: vi.fn(),
  unlisten: vi.fn(),
  emitFocus: vi.fn() as (event: FocusEvent) => void,
  runText: vi.fn(),
}));

vi.mock("../../lib/api", () => ({ runText: tauri.runText }));

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

  function typeAndSubmit(text: string) {
    const input = screen.getByRole("textbox");
    fireEvent.change(input, { target: { value: text } });
    fireEvent.keyDown(input, { key: "Enter" });
  }

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

  it("sends the typed text to runText", async () => {
    tauri.runText.mockResolvedValue("Running the Calculator");
    render(<Launcher />);

    typeAndSubmit("calculator");

    await waitFor(() => expect(tauri.runText).toHaveBeenCalledWith("calculator"));
    await waitFor(() => expect(tauri.hide).toHaveBeenCalledTimes(1), { timeout: 2000 });
  });

  it("shows the confirmation and then hides the window", async () => {
    tauri.runText.mockResolvedValue("Running the Calculator");
    render(<Launcher />);

    typeAndSubmit("calculator");

    expect(await screen.findByText("Running the Calculator")).toBeTruthy();
    await waitFor(() => expect(tauri.hide).toHaveBeenCalledTimes(1), { timeout: 2000 });
  });

  it("shows the error and keeps the window visible", async () => {
    tauri.runText.mockRejectedValue('I don\'t know an app called "asdf"');
    render(<Launcher />);

    typeAndSubmit("asdf");

    const message = await screen.findByText('I don\'t know an app called "asdf"');
    expect(message.className).toContain("launcher-message-error");
    expect(tauri.hide).not.toHaveBeenCalled();
  });

  it("clears the text and the message when the window loses focus", async () => {
    tauri.runText.mockRejectedValue("Type what you want to do");
    render(<Launcher />);
    typeAndSubmit("asdf");
    await screen.findByText("Type what you want to do");

    act(() => tauri.emitFocus({ payload: false }));

    expect(screen.queryByText("Type what you want to do")).toBeNull();
    expect((screen.getByRole("textbox") as HTMLInputElement).value).toBe("");
  });
});
