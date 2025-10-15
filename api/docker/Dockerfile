# -----------------------------
# BUILD STAGE
# -----------------------------
FROM rust:1.90 AS builder
WORKDIR /usr/src/eapi

# Cache dependencies
COPY ./Cargo.toml ./tools/dummy.rs ./
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml && rm ./dummy.rs

COPY . .

RUN cargo build --release

# -----------------------------
# FINAL IMAGE
# -----------------------------
FROM debian:bookworm-slim
WORKDIR /usr/src/eapi

RUN apt-get update && \
    apt-get install -y postgresql-client && \
    rm -rf /var/lib/apt/lists/*

COPY ./migrations ./migrations
COPY ./tools ./tools

RUN chmod +x ./tools/entrypoint.sh

COPY --from=builder /usr/src/eapi/target/release/eapi /usr/local/bin/eapi

EXPOSE 8080

ENTRYPOINT ["./tools/entrypoint.sh"]
CMD ["eapi"]
