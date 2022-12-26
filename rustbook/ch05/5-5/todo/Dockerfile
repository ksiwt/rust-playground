FROM rust:1.56

WORKDIR /todo

COPY Cargo.toml Cargo.toml

RUN mkdir src

RUN echo "fn main(){}" > src/main.rs
RUN cargo build --relase
RUN cargo install --path .
COPY ./src ./src
COPY .templates ./templates
RUN rm -f target/relase/deps/todo*
RUN cargo build --relase

FROM debian:10.4

COPY --from=builder /todo/target/relase/todo /usr/local/bin/todo
CMD ["todo"]
