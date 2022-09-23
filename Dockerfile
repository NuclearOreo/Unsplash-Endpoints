FROM rust as builder
ENV USER root
WORKDIR /app
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /app/target/release/unsplash-endpoints /unsplash-endpoints
CMD ["/unsplash-endpoints"]