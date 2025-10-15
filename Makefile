ABORT_ON_EXIT = --abort-on-container-exit --exit-code-from eapi
ENV_FILE = --env-file .env

COMPOSE_FILE = docker/docker-compose.yml
DOCKER_COMPOSE = docker compose $(ENV_FILE) -f $(COMPOSE_FILE)

COMPOSE_DEV_FILE = docker/docker-compose-dev.yml
DOCKER_COMPOSE_DEV = docker compose $(ENV_FILE) -f $(COMPOSE_DEV_FILE)

COMPOSE_TEST_FILE = docker/docker-compose-tests.yml
DOCKER_COMPOSE_TESTS = $(DOCKER_COMPOSE) -f $(COMPOSE_TEST_FILE)

COMPOSE_ALL_TEST_FILE = docker/docker-compose-all-tests.yml
DOCKER_COMPOSE_ALL_TESTS = $(DOCKER_COMPOSE) -f $(COMPOSE_ALL_TEST_FILE)

API_SERVICE = eapi
DATABASE_SERVICE = db

# ------------------------------
# PHONY TARGETS
# ------------------------------
.PHONY: all detach build down clean fclean re logs dev dev-down test test-all \
    docker-up docker-build docker-up-detach docker-down docker-logs \
    docker-logs-db docker-logs-all docker-clean docker-clean-with-volumes

# ------------------------------
# MAIN TARGETS
# ------------------------------
all: docker-up
detach: docker-up-detach
build: docker-build
down: docker-down
clean: docker-clean
fclean: docker-clean-with-volumes
re: clean all
logs: docker-logs

# ------------------------------
# DEV TARGETS
# ------------------------------
dev:
	@$(DOCKER_COMPOSE_DEV) up --detach --build

dev-down:
	@$(DOCKER_COMPOSE_DEV) down --remove-orphans

# ------------------------------
# TEST TARGETS
# ------------------------------
test:
	@$(DOCKER_COMPOSE_TESTS) up $(API_SERVICE) --build $(ABORT_ON_EXIT)
	@$(DOCKER_COMPOSE_TESTS) down --remove-orphans

test-all:
	@$(DOCKER_COMPOSE_ALL_TESTS) up $(API_SERVICE) --build $(ABORT_ON_EXIT)
	@$(DOCKER_COMPOSE_ALL_TESTS) down --remove-orphans

# ------------------------------
# DOCKER TARGETS
# ------------------------------
docker-up:
	@$(DOCKER_COMPOSE) up $(API_SERVICE) --build $(ABORT_ON_EXIT)

docker-build:
	@$(DOCKER_COMPOSE) build

docker-up-detach:
	@$(DOCKER_COMPOSE) up --build --detach

docker-down:
	@$(DOCKER_COMPOSE) down --remove-orphans

docker-logs:
	@$(DOCKER_COMPOSE) logs $(API_SERVICE) -f

docker-logs-db:
	@$(DOCKER_COMPOSE) logs $(DATABASE_SERVICE) -f || true

docker-logs-all:
	@$(DOCKER_COMPOSE) logs -f || true

docker-clean:
	@$(DOCKER_COMPOSE) down --remove-orphans --rmi all
	@$(DOCKER_COMPOSE_TESTS) down --remove-orphans --rmi all
	@$(DOCKER_COMPOSE_ALL_TESTS) down --remove-orphans --rmi all

docker-clean-with-volumes:
	@$(DOCKER_COMPOSE) down --remove-orphans --rmi all -v
	@$(DOCKER_COMPOSE_TESTS) down --remove-orphans --rmi all -v
	@$(DOCKER_COMPOSE_ALL_TESTS) down --remove-orphans --rmi all -v