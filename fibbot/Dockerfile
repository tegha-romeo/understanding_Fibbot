FROM rust:1.60

WORKDIR /usr/src/fibbot
COPY . .

RUN cargo build --release

CMD ["./target/release/fibbot"]