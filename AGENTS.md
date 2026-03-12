# AGENTS.md

## Cursor Cloud specific instructions

### Project overview

RSS Reporter is a Rust workspace with two crates:

- `rss-reporter-core` (library): RSS/Atom feed parsing into `SiteCtx` structs
- `rss-reporter-cli` (binary): CLI that fetches an RSS feed via `curl` subprocess and prints parsed results

### Toolchain requirements

- **Rust edition 2024** — requires `rustc >= 1.85.0`. The VM's default Rust may be older; the update script runs `rustup update stable` to ensure compatibility.
- `curl` must be available on PATH (pre-installed in the VM).

### Build / Test / Lint commands

| Action | Command |
|---|---|
| Build workspace | `cargo build` |
| Build core only | `cargo build -p rss-reporter-core` |
| Test core | `cargo test -p rss-reporter-core` |
| Lint (clippy) | `cargo clippy -p rss-reporter-core` |
| Format check | `cargo fmt --check` |
| Run CLI | `cargo run -p rss-reporter-cli` |

### Known issues

- `rss-reporter-cli` has a compile error in `main.rs:12`: `ctx.article_title.content` accesses a non-existent field — `article_title` is `String`, not `feed_rs::model::Text`. Fix: replace `ctx.article_title.content` with `ctx.article_title`.
- `cargo test --workspace` and `cargo clippy --workspace` will fail due to the CLI compile error. Use `-p rss-reporter-core` to scope to the working crate.
- The CLI fetches a hardcoded URL (`https://scour.ing/@xrm07/rss.xml`) and requires internet access at runtime.
