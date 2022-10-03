FROM rust as builder
ENV USER root
WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust
COPY --from=builder /app/target/release/unsplash-endpoints .
EXPOSE 8000
CMD ["/unsplash-endpoints"]