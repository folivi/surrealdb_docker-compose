FROM rust:latest as builder

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./

COPY src ./src
RUN cargo build --release

FROM ubuntu:22.04 as runner

COPY --from=builder /usr/src/app/target/release/surreal-compose ./

EXPOSE 7777
CMD ["./surreal-compose"]