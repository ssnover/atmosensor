FROM rust:1.70-slim-bullseye
WORKDIR /app
RUN rustup default nightly
RUN rustup target add thumbv7m-none-eabi
RUN cargo install flip-link
ENTRYPOINT ["cargo", "build", "--release"]