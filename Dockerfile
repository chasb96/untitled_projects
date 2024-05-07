FROM rust AS build_host
WORKDIR /src

RUN USER=root cargo new --bin projects
WORKDIR /src/projects

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/projects*

COPY ./src ./src
RUN cargo build --release

WORKDIR /src

FROM rust:slim

RUN apt-get update
RUN apt-get install -y libpq-dev

WORKDIR /src

COPY --from=build_host /src/projects/target/release/projects ./projects

CMD ["./projects"]
