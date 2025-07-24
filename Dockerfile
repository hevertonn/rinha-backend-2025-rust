FROM rust:1.88.0-bullseye

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

ENTRYPOINT [ "rinha-backend-2025" ]