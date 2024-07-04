FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv ./target/release/argo-sync ./argo-sync

FROM debian:stable-slim AS runtime
RUN  apt update -y && apt upgrade -y && apt install openssl -y
WORKDIR /app
COPY --from=builder /app/argo-sync /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/argo-sync"]
