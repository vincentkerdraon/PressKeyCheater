# PressKeyCheater



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
## sudo apt-get install mingw-w64 libopus-dev
#cargo build --release --target x86_64-pc-windows-gnu
```
- push

## License

MIT