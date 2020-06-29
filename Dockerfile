FROM rustlang/rust:nightly-buster AS build

WORKDIR /app

COPY . /app

RUN rustup --version
RUN rustup install nightly-2019-04-16 && \
    rustup default nightly-2019-04-16

RUN rustc --version && \
    rustup --version && \
    cargo --version

RUN cargo clean && cargo build --release
RUN strip ./target/release/raider

FROM debian:buster-slim

WORKDIR /usr/src/raider

COPY ./res/assets/ ./res/assets/
COPY --from=build /app/target/release/raider /usr/local/bin/raider

RUN apt-get update
RUN apt-get install -y libssl-dev default-libmysqlclient-dev

CMD [ "raider", "-c", "/etc/raider.cfg" ]

EXPOSE 8080
