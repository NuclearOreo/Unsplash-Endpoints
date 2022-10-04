FROM rust:1.64 as builder
ENV USER root
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM rust:1.64 as runner
COPY --from=builder /usr/local/cargo/bin/unsplash-endpoints /usr/local/bin/unsplash-endpoints
EXPOSE 8000
CMD ["unsplash-endpoints"]