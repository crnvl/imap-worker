FROM rust:latest

WORKDIR /usr/src
COPY . . 

RUN cargo build --release

CMD ["./target/release/worker"]