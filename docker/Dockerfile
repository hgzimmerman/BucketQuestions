FROM rust:latest

RUN cargo install diesel_cli --no-default-features --features postgres --force

RUN curl -sL https://deb.nodesource.com/setup_10.x | bash
RUN apt update -y
RUN apt install nodejs -y


WORKDIR /usr/src/app/docker

EXPOSE 8080

VOLUME ["/usr/local/cargo"]

