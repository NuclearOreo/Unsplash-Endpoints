# Unsplash Endpoint

Basic API to communicate with Unsplash for my personal website. Didn't want my website to directly communicate with Unsplash so created this API. Went with rust as an experiment and exploration of rust support, tooling and performance.

## Cargo Command
- Running: `cargo run`
- Building: `cargo build`
- Build for Release: `cargo build --release`

EndPoint:
- Health Check: **GET** `<host>/`
- Get Photos: **GET** `<host>/unsplash/get_photos`
  - URL Params:
      - page_number: integer
      - pre_page: integer

## Coverage

Using [cargo-llvm-cov](https://crates.io/crates/cargo-llvm-cov/0.1.13) for code coverage

### Installation

- `rustup component add llvm-tools-preview --toolchain nightly`
- `cargo install cargo-llvm-cov`

### Command
- Prints Summary: `cargo llvm-cov`
- Generates HTML: `cargo llvm-cov --html`