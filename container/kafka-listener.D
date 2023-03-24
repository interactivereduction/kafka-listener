FROM rust:latest

COPY . .

RUN cargo install --path .

CMD ["kafka-listener"]