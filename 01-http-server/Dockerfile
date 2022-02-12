FROM rust

WORKDIR /usr/src

# create an empty Cargo project
RUN USER=root cargo new app

# change working directory to the project
WORKDIR /usr/src/app

COPY Cargo.toml .
COPY Cargo.lock .

# Install required dependencies
RUN cargo install --path .

# Copy source coded
COPY ./src ./src

# Build the binary
RUN cargo build --release

# move binary
RUN mv target/release/http-server /bin

# delete source, save space
RUN rm -rf /usr/src/app

EXPOSE 8000

CMD ["/bin/http-server"]