# PressKeyCheater

Press Key Cheater is a lightweight tool that simulates keyboard input based on predefined command sequences. It is designed for automation, testing, and scripting use cases where keystroke simulation is required.

## Features

- Executes sequences of character inputs and instructions.
- Supports commands like `GOTO`, `SLEEP`, `EXIT`, and `LOG`.
- Allows specifying input sequences via environment variables.
- Includes a helper utility (`show-key`) for debugging key inputs.
- Map keys using https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes

## Release

- update version in `Cargo.toml`
- quality checks (run for each cargo)
```bash
cargo update
cargo test
cargo bench
cargo check
cargo clippy
cargo fmt -- --check
cargo fix
##
## requires: rustup install nightly
## cargo install cargo-udeps --locked 
cargo +nightly udeps --all-targets
## requires git config (won't work in devcontainer)
cargo audit
## to reclaim disk space
cargo clean
```
- build
```bash
# rustup default stable 
cargo clean

# for target/x86_64-pc-windows-gnu/release/ai-discord-gamemaster
## FIXME doesn't work in devcontainer, works in local
## requires toolchain
## rustup target add x86_64-pc-windows-gnu
## sudo apt-get install mingw-w64
#cargo build --release
```
- push

## License

MIT