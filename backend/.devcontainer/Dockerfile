# syntax=docker/dockerfile:1.3-labs

FROM rust:1.64.0

RUN <<EOT
rustup component add rustfmt clippy
cargo install cargo-watch sea-orm-cli
EOT

# ENV sourcedir=backend
# ENV workdir=/app/backend
# WORKDIR $workdir
# COPY $sourcedir/Cargo.toml $sourcedir/Cargo.lock $workdir
# COPY $sourcedir/api/Cargo.toml $workdir/api/
# RUN cargo build
# COPY $sourcedir $workdir
