FROM rust:1.56.0-slim AS build
WORKDIR /usr/src

# Add target for static binding
RUN rustup target add x86_64-unknown-linux-musl
RUN USER=root cargo new wi-is-up
WORKDIR /usr/src/wi-is-up
COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=build /usr/local/cargo/bin/wi-is-up .
USER 1000
ENTRYPOINT ["/wi-is-up"]
