# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Rule-based interpreter: phrases like "abrí calculadora" or "open notepad" open the app, and plain app names still work

## [0.2.0] - 2026-10-15

### Added

- Command registry: register commands by name, validate parameters against a JSON Schema and run them
- `open_app` command: open Calculator, Notepad or File Explorer by name, from a closed list and without a shell
- Launcher window: borderless, centered, always-on-top bar that hides on Escape or when it loses focus
- Global shortcut `Ctrl+Alt+K` to show or hide the launcher from any app
- Run what is typed in the launcher: the text is sent to Rust, executed as a command and the result (or an error) is shown in the bar
- Tray icon with a Show/Quit menu; closing the main window keeps Kai running in the tray

## [0.1.0] - 2026-10-01

### Added

- Project scaffold with Tauri 2, React and TypeScript
- Linters and formatters: rustfmt, clippy, ESLint and Prettier
- Test setup: `cargo test` for Rust and Vitest for TypeScript
- CI with GitHub Actions on Windows (Rust checks, frontend checks and a Tauri build)
- Shared VS Code workspace settings and recommended extensions
- Project documentation in `docs/`
- README
- MIT license

[Unreleased]: https://github.com/Kelex-organize/desktop-assistant/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Kelex-organize/desktop-assistant/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Kelex-organize/desktop-assistant/releases/tag/v0.1.0
