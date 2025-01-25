FROM rust:bullseye

RUN apt update -y \
&& apt upgrade -y

RUN cargo install cargo-watch \
	&& cargo install sqlx-cli --no-default-features --features native-tls,postgres

COPY ./srcs /app
COPY ./api_entrypoint.sh /api_entrypoint.sh
COPY ./migrations /var/lib/migrations
RUN chmod +x /api_entrypoint.sh

CMD [ "bash", "/api_entrypoint.sh" ]