FROM almalinux:8 AS rust-base
RUN yum -y groupinstall 'Development Tools'
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

FROM rust-base AS chef
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef 
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build -p project-dirs-bin --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN touch /.is-docker
USER nobody
WORKDIR /app
COPY --from=builder /app/target/release/project-dirs-bin /usr/local/bin
ENTRYPOINT ["/usr/local/bin/project-dirs-bin"]
