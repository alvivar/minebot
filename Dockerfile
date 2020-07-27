FROM rust:1.45.0

WORKDIR /app
COPY . .

RUN cargo install --path .
RUN apt-get update && apt-get -y install docker.io

CMD ["minebot"]