FROM rust:1.45.0

WORKDIR /app
COPY . .

RUN cargo install --path .

# We want to run Docker commands in this container.
RUN apt-get update && apt-get -y install docker.io

CMD ["minebot"]