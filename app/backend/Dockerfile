FROM rust:bullseye

RUN apt update -y \
&& apt upgrade -y

RUN cargo install cargo-watch \
	&& cargo install sqlx-cli --no-default-features --features native-tls,postgres

COPY ./srcs /app
COPY migrations /var/lib/migrations
COPY ./backend_entrypoint.sh /backend_entrypoint.sh
RUN chmod +x /backend_entrypoint.sh

CMD [ "bash", "/backend_entrypoint.sh" ]