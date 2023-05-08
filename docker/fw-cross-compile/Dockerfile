FROM rust:1.69-slim-bullseye
WORKDIR /app
RUN rustup target add thumbv7m-none-eabi
RUN cargo install flip-link
ENTRYPOINT ["cargo", "build", "--release"]