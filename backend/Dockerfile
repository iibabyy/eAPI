FROM rust:bullseye

WORKDIR /app

RUN apt update -y \
&& apt upgrade -y

RUN cargo install cargo-watch \
	&& cargo install sqlx-cli --no-default-features --features native-tls,postgres

COPY ./api_entrypoint.sh /etc/api_entrypoint.sh
RUN chmod +x /etc/api_entrypoint.sh


CMD [ "bash", "/etc/api_entrypoint.sh" ]