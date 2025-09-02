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

✅ Checklist for LLM Coding Agent: Low Cognitive Load Code

1. General
	•	✅ Always write boring, obvious, predictable code.
	•	❌ Don’t optimize for cleverness, brevity, or novelty.

⸻

2. Conditionals
	•	✅ Break complex conditions into descriptive intermediate variables.
	•	✅ Prefer early returns over nested if blocks.
	•	❌ Don’t write compound conditions with multiple operators inline.
	•	❌ Don’t deeply nest conditional branches.

⸻

3. Functions / Classes / Modules
	•	✅ Prefer deep modules: powerful implementation hidden behind a simple interface.
	•	✅ Keep module boundaries clear and intuitive.
	•	❌ Don’t create many tiny, shallow modules that require lots of cross-referencing.
	•	❌ Don’t fragment functionality across dozens of small classes or files.

⸻

4. Inheritance & Composition
	•	✅ Use composition for reuse.
	•	❌ Don’t create long inheritance chains.
	•	❌ Don’t bury logic across multiple superclasses.

⸻

5. Services / Architecture
	•	✅ Default to a modular monolith.
	•	✅ Only introduce microservices when independent deployment is necessary.
	•	❌ Don’t split into many microservices prematurely.
	•	❌ Don’t design based on “trendy architectures” without practical justification.

⸻

6. Responsibility
	•	✅ Define responsibility in terms of a single stakeholder or user group.
	•	❌ Don’t split modules by arbitrary “one thing” definitions.
	•	❌ Don’t design abstractions that serve no clear stakeholder.

⸻

7. DRY (Don’t Repeat Yourself)
	•	✅ Allow small, local duplication if it keeps code simpler.
	•	✅ Prefer explicit, self-contained code over abstract coupling.
	•	❌ Don’t over-abstract shared code prematurely.
	•	❌ Don’t pull in heavy libraries for trivial helpers.

⸻

8. Dependencies & Frameworks
	•	✅ Treat dependencies as if you must debug them yourself.
	•	✅ Keep business logic independent of frameworks.
	•	❌ Don’t couple your code tightly to framework “magic.”
	•	❌ Don’t introduce dependencies unless the payoff is clear.

⸻

9. Abstractions
	•	✅ Add abstractions only when a real, current extension point exists.
	•	✅ Use dependency inversion and information hiding.
	•	❌ Don’t add unnecessary architecture layers (Hexagonal, Onion, etc.) for fashion.
	•	❌ Don’t multiply indirections without reason.

⸻

10. Naming & Communication
	•	✅ Use descriptive names and self-descriptive codes.
	•	✅ Prefer clear human terms (e.g., “login” / “permissions”).
	•	❌ Don’t force developers to memorize numeric mappings (e.g., HTTP 401/403 distinctions for business logic).
	•	❌ Don’t use jargon where simpler terms exist.

⸻

11. Language & Features
	•	✅ Stick to simple, idiomatic features.
	•	✅ Use the smallest expressive subset of the language.
	•	❌ Don’t overuse advanced or obscure language features.
	•	❌ Don’t introduce multiple ways to do the same thing.

⸻

12. Mental Models & Team Onboarding
	•	✅ Optimize for newcomers: code should be graspable in hours, not weeks.
	•	✅ Favor linear, obvious reading flow.
	•	❌ Don’t rely on “familiarity” — familiar ≠ simple.
	•	❌ Don’t use clever hacks or non-idiomatic tricks.

⸻

13. Defaults to Prefer
	•	✅ CRUD-style architecture.
	•	✅ Monolith with modular boundaries.
	•	✅ Clear entry-point “wiring” functions.
	•	✅ Self-contained deep modules.

⸻

👉 Use this checklist every time you generate code.
If a rule conflicts with another, choose the option that reduces cognitive load the most.

⸻

