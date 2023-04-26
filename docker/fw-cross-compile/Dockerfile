FROM rust:1.67.1-slim-bullseye
WORKDIR /app
RUN rustup target add thumbv7m-none-eabi
RUN cargo install flip-link
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["cargo", "build", "--release"]