FROM rust:latest

WORKDIR /app
RUN mkdir src 

RUN echo 'fn main() { println!("tmp for cargo build"); }' >> ./src/main.rs; 
RUN echo "// tmp lib for build" >> ./src/lib.rs; 

RUN cargo install cargo-watch

COPY Cargo.toml Cargo.lock ./

COPY database database

RUN cargo build
RUN rm -rf src

CMD cargo watch -w src -x run

