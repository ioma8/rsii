**Project Overview**
- **Name:** `rsii` — Rust Smart Interactive Interface (CLI assistant).
- **Goal:** Turn a user’s natural-language prompt into a safe, ready-to-run shell command using OpenAI’s Chat Completions API with a single function tool.
- **Main Flow:**
  - Parses args; `--version` prints build-time version from `build.rs`-generated `version.rs`.
  - Loads config from `~/.rsii/config.toml` (model, API key, system prompt).
  - Gathers system info (`uname -a`) and builds a combined prompt.
  - Calls OpenAI chat with a required tool `call_command(command: string)`.
  - On tool call, copies the returned command to the clipboard and simulates paste (platform-specific helper).

**Repo Layout**
- `src/main.rs`: CLI entry, OpenAI request/response handling, tool definition, clipboard + paste logic, tests.
- `src/config.rs`: Config loader for `~/.rsii/config.toml`.
- `build.rs`: Ensures a default config exists on first build and generates `version.rs` with the crate version.
- `default.config.toml`: Template copied to `~/.rsii/config.toml` if none exists.
- `build_binaries.sh`: Convenience script to build and package macOS binaries for arm64/x86_64 and emit checksums.
- `FLOW.md`: Mermaid diagram of the request/response/tool-call flow.
- `README.md`: Features, install, usage.

**Configuration**
- **Location:** `~/.rsii/config.toml` created on build if missing (copied from `default.config.toml`).
- **Keys:**
  - `default.model`: OpenAI model name.
  - `default.api-key`: OpenAI API key.
  - `default.system-prompt`: System instructions for command generation.
- The binary reads only the user’s home config; repo files are for bootstrapping and docs.

**Build & Run**
- **Check:** `cargo check` — fast validation of changes.
- **Build (debug):** `cargo build`.
- **Build (release):** `cargo build --release`.
- **Run:** `cargo run -- "Your query here"`.
- **Version:** `rsii --version` prints the crate version baked in at build time.

**Packaging**
- Use `./build_binaries.sh` to:
  - Build for `aarch64-apple-darwin` and `x86_64-apple-darwin` targets.
  - Tarball outputs into `releases/` and print SHA-256 checksums.
- Requires respective Rust targets installed (e.g., `rustup target add aarch64-apple-darwin x86_64-apple-darwin`).

**Tests**
- `src/main.rs` includes unit tests for:
  - System info retrieval.
  - Config loading.
  - Tool function shape.
  - Error path for API call with dummy credentials.
- Run with `cargo test`.

**Notes & Gotchas**
- `build.rs` will create `~/.rsii/config.toml` if absent; subsequent edits should be made in the home file, not in-repo templates.
- The program simulates paste via OS-specific commands (macOS: `osascript`, Linux: `xdotool`, Windows: PowerShell SendKeys).
- Network access is required at runtime for OpenAI API calls.
- Keep secrets out of the repo: ensure your personal `~/.rsii/config.toml` contains real credentials.

