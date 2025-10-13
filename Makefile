GREEN = \033[0;32m
RED = \033[0;31m
BRIGHT_BLACK = \033[0;90m
RESET = \033[0m
NO_OUTPUT = > /dev/null 2>&1

.PHONY: all build re clean fclean up down logs release test check \
	migrate-run migrate-revert migrate-re \
	docker-up docker-down docker-logs docker-clean docker-re docker-logs \
	cargo-build cargo-run cargo-build-release cargo-run-release cargo-clean cargo-re

all: build cargo-run
build: docker-up-build migrate-run cargo-build
re: clean all
clean: down cargo-clean
fclean: docker-clean cargo-clean

release: docker-up-build migrate-run cargo-build-release cargo-run-release
test: build cargo-test

up: docker-up
down: docker-down
logs: docker-logs

check:
	@docker -v $(NO_OUTPUT) && echo "$(GREEN)Docker installed$(RESET)" || (echo -n "$(RED)Docker uninstalled!$(RESET) " && echo "$(BRIGHT_BLACK)follow the instructions at: https://docs.docker.com/engine/install/$(RESET)")
	@cargo -v $(NO_OUTPUT) && echo "$(GREEN)Cargo (Rust) installed$(RESET)" || (echo -n "$(RED)Cargo uninstalled!$(RESET) " && echo "$(BRIGHT_BLACK)run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh$(RESET)")
	@sqlx --version $(NO_OUTPUT) && echo "$(GREEN)sqlx-cli installed$(RESET)" || (echo -n "$(RED)sqlx-cli uninstalled!$(RESET) " && echo "$(BRIGHT_BLACK)run: cargo install sqlx-cli$(RESET)")

cargo-build:
	@cargo build

cargo-run:
	@cargo run

cargo-build-release:
	@cargo build --release

cargo-run-release:
	@cargo run --release

cargo-test:
	@cargo test

cargo-clean:
	@cargo clean

cargo-re: cargo-clean cargo-run

# Docker
docker-up:
	@docker compose up --detach
	@sleep 1

docker-up-build:
	@docker compose up --build --detach
	@sleep 1

docker-down:
	@docker compose down

docker-logs:
	@docker compose logs -f

docker-clean:
	@docker compose down --rmi all -v

docker-re:
	@docker compose restart
	@sleep 1

# Migrations
migrate-run:
	@sqlx migrate run

migrate-revert:
	@sqlx migrate revert --target-version 0

migrate-re: migrate-revert migrate-run