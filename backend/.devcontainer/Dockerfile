# syntax=docker/dockerfile:1.3-labs

FROM rust:1.64.0

RUN <<EOT
rustup component add rustfmt
cargo install cargo-watch sea-orm-cli
EOT
