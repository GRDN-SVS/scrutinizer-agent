FROM rust:latest-apline

WORKDIR /usr/src/scrutinizer-agent
COPY . .

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/scrutinizer-agent"]