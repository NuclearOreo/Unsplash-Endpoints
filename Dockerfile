FROM rust:1.64 as builder

ENV USER root
ARG unsplash_client_id

WORKDIR /app
COPY . .

RUN cargo install --path .

CMD ["unsplash-endpoints"]
