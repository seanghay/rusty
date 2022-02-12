FROM rust

WORKDIR /usr/src

RUN USER=root cargo new app

WORKDIR /usr/src/app

COPY Cargo.toml .

COPY Cargo.lock .

RUN cargo install --path .

COPY ./src ./src

RUN cargo build --release

# move binary
RUN mv target/release/http-server /bin

# delete source
RUN rm -rf /usr/src/app

EXPOSE 8000

CMD ["/bin/http-server"]