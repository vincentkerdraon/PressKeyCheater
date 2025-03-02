# PressKeyCheater

Press Key Cheater is a lightweight tool that simulates keyboard input based on predefined command sequences. 
It is designed for automation in idle video games.

## Features

*Windows only!*

- Executes sequences of character inputs and instructions.
- Supports commands like `GOTO`, `SLEEP`, `EXIT`, and `LOG`.
- Allows specifying input sequences via environment variables.
- Includes a helper utility (`show-key`) for debugging key inputs.
- Access to side mouse buttons, arrows, Left Windows or Right Windows, Start Mail, Volume Up ...
- Map keys using https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes

## Example

- Write "hello", using delimiter `.`
`SEQUENCE="VK H.VK E.VK L.VK L.VK O" DELIMITER="."`
- Write "hello" every 1s (using default delimiter)
`SEQUENCE="VK H#VK E#VK L#VK L#VK O#VK SPACE#SLEEPS 1#GOTO 0"`
- Write "aAa" (using `_` for keydown and `^` for key up)
`SEQUENCE="VK A#_VK SHIFT#VK A#^VK SHIFT#VK A"`

## A word of caution

The program doesn't understand the sequence or the active window. For example, this sequence is dangerous if executed in the file explorer:
- Ctrl+A (select All)
- Maj+Del (permanently delete selected files)
- Enter (validate)

## Dependency

See winapi: https://docs.rs/winapi/latest/winapi/
- Can only be compiled and executed on windows

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
- build done by github actions, creating artifact

## License

MIT