SRCS_PATH = ./api/srcs

MANIFEST_PATH = --manifest-path $(SRCS_PATH)/Cargo.toml
CARGO_BUILD = cargo build $(MANIFEST_PATH)
CARGO_RUN = cargo run $(MANIFEST_PATH)

.PHONY: all build re clean fclean down logs \
	docker-up docker-up-build docker-up-detach docker-up-build-detach \
	docker-down docker-logs docker-clean docker-clean-volumes docker-re

all: docker-up

build: docker-up-build

detach: docker-up-detach

build-detach: docker-up-build-detach

down: docker-down

clean: docker-clean

fclean: docker-clean-volumes

re: clean all

logs: docker-logs

# Docker 
docker-up:
	@docker compose up

docker-up-build:
	@docker compose up --build

docker-up-detach:
	@docker compose up --detach

docker-up-build-detach:
	@docker compose up --build --detach

docker-down:
	@docker compose down

docker-logs:
	@docker compose logs -f

docker-clean:
	@docker compose down --rmi all

docker-clean-volumes:
	@docker compose down --rmi all -v

docker-re:
	@docker compose restart