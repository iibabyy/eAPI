FROM rust:1.90 AS builder

WORKDIR /usr/src/eapi
COPY ./srcs .

RUN cargo install --path .

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y postgresql-client && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/eapi /usr/local/bin/eapi

COPY ./migrations /migrations

COPY ./tools/wait_for_db.sh /init.sh
RUN chmod +x /init.sh

ENTRYPOINT [ "/init.sh" ]
CMD ["eapi"]