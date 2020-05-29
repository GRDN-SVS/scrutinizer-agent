FROM rust:slim-stretch

RUN apt update && apt install libssl-dev pkg-config build-essential libpq-dev -y

WORKDIR /usr/src/scrutinizer-agent
COPY . .

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/scrutinizer-agent"]
