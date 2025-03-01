FROM rust:1.85 AS builder

RUN apt-get update && apt-get install -y \
    mingw-w64

# Add the target architecture
RUN rustup target add x86_64-pc-windows-gnu

WORKDIR /workspace
COPY . .

RUN cargo build --release --target x86_64-pc-windows-gnu

# Build done, so set the final image to be minimal
FROM debian:bullseye-slim

COPY --from=builder /workspace/target/x86_64-pc-windows-gnu/release/your_binary_name /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/your_binary_name"]
