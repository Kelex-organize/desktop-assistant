import { invoke } from "@tauri-apps/api/core";

/**
 * Runs what the user typed. Resolves with the confirmation to show; rejects
 * with a message for the user (already readable) if it could not be done.
 */
export function runText(text: string): Promise<string> {
  return invoke<string>("run_text", { text });
}
