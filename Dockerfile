FROM rust:latest

COPY . .

ENV ENV=prod

RUN cargo build --release

CMD ["./target/release/pictx"]