FROM rust:1.69-slim-bullseye as build-env
WORKDIR /app
ADD . /app
RUN apt update -y && apt install -y make libssl-dev pkg-config
RUN rustup component add rustfmt clippy
RUN cargo fmt --all -- --check  && cargo clippy
RUN RUSTFLAGS="-C link-arg=-s" cargo build --locked --no-default-features --release

FROM gcr.io/distroless/cc-debian11:nonroot-amd64
LABEL org.opencontainers.image.description "Slash Bot Discord for Price using Coingecko API"
ENV TZ="Asia/Jakarta" RUST_LOG="info"
WORKDIR /app
COPY --from=build-env /app/target/release/cryptors-price-slash-bot /app
CMD ["./cryptors-price-slash-bot"]