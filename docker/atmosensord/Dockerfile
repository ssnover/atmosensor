FROM rust:1.69-slim-bullseye as atmosensor-dev
WORKDIR /app
RUN apt-get update -y \
    && apt install -y --no-install-recommends \
    lld \
    clang \
    pkg-config \
    libssl-dev
ENTRYPOINT ["cargo", "build", "--release"]

FROM atmosensor-dev as atmosensord-build

COPY atmosensor-host-apps /app
RUN cargo build --release --bin atmosensord
ENTRYPOINT ["target/release/atmosensord"]

FROM debian:bullseye-slim as runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=atmosensord-build /app/target/release/atmosensord atmosensord