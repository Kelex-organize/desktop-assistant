# Kai

A desktop assistant for Windows that does everyday chores for you: open apps, set alarms and reminders, take notes, control Spotify, jump to pages like MetaTFT. You ask by typing in a Spotlight-style bar or, soon, by voice.

> **Status: early development.** The project skeleton, tooling and CI are in place; the assistant itself is not usable yet. See the [roadmap](docs/ROADMAP.md).

## What Kai is (and is not)

Kai is **not a chat app**. There is no conversation window. You ask for something, it does it, and it confirms in one short, friendly sentence.

AI will come later, and only as an *interpreter*: it turns what you type or say into app commands. Everything works without AI and without internet.

## Planned features

- **Quick bar:** a global shortcut opens a Spotlight-style input
- **Alarms and reminders** that ring even when the window is closed
- **Notes:** create, list, search
- **Open any installed app**
- **Basic Spotify control:** play/pause, next, previous
- **Shortcuts to web pages**, for example `items de Caitlyn` opens MetaTFT
- **Voice commands** with push-to-talk
- **A character** that lives in a corner of the screen (can be hidden)

## Tech stack

- [Tauri 2](https://tauri.app/): desktop shell (Rust + WebView2)
- **Rust** (`src-tauri/`): all the logic (commands, interpreter, background tasks, storage)
- **React + TypeScript + Vite** (`src/`): UI only
- **SQLite** for module data
- Windows 10/11 only

## Getting started

### Requirements

- Windows 10 or 11
- [Node.js](https://nodejs.org/) (LTS)
- [Rust](https://rustup.rs/) (`stable-msvc` toolchain)
- Visual Studio Build Tools with the **Desktop development with C++** workload
- WebView2 (already included in Windows 10/11)

See the [Tauri prerequisites](https://tauri.app/start/prerequisites/) for details.

### Run in development

```bash
npm install
npm run tauri dev
```

The first run compiles all Rust dependencies and can take several minutes.

### Build

```bash
npm run tauri build
```

## Development

Before opening a pull request, run the same checks CI runs.

Rust (from `src-tauri/`):

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Frontend (from the repository root):

```bash
npm run format:check
npm run lint
npm run typecheck
npm test
```

Contributions follow GitHub Flow: one issue per task, one branch per issue, [Conventional Commits](https://www.conventionalcommits.org/), and a pull request with green CI before merging. Details in [`docs/FLUJO_DE_TRABAJO.md`](docs/FLUJO_DE_TRABAJO.md).

## Documentation

The project documentation lives in [`docs/`](docs/) and is written in Spanish:

- [`VISION.md`](docs/VISION.md): what Kai is and is not
- [`ARQUITECTURA.md`](docs/ARQUITECTURA.md): how the app is organized
- [`FLUJO_DE_TRABAJO.md`](docs/FLUJO_DE_TRABAJO.md): git, branches, PRs, tests, CI
- [`ROADMAP.md`](docs/ROADMAP.md): milestones and backlog
- [`DECISIONES.md`](docs/DECISIONES.md): decisions and the reasons behind them

## License

[MIT](LICENSE). You are free to use, copy, modify and redistribute this code, as long as you keep the copyright notice.
