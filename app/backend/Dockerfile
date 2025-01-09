FROM rust:bullseye

RUN apt update -y
RUN apt upgrade -y

COPY ./srcs /app/myapp

WORKDIR /app/myapp

# CMD ["bash"]
CMD [ "cargo", "run" ]
