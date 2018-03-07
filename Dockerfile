FROM rustlang/rust:nightly

WORKDIR /usr/src/raider
COPY ./res/assets/ ./res/assets/

RUN cargo install raider-server
CMD [ "raider", "-c", "/etc/raider.cfg" ]

EXPOSE 8080
