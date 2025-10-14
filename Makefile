ABORT_ON_EXIT = --abort-on-container-exit --exit-code-from eapi
DOCKER_COMPOSE = docker compose -f docker-compose.yml
TEST_COMPOSE_FILE = docker-compose-tests.yml
ALL_TEST_COMPOSE_FILE = docker-compose-all-tests.yml
UP = up --build

.PHONY: all detach down clean fclean re logs test test-all \
	docker-up docker-up-detach docker-down docker-logs docker-clean docker-clean-volumes

all: docker-up

detach: docker-up-detach

down: docker-down

clean: docker-clean

fclean: docker-clean-volumes

re: clean all

logs: docker-logs

test:
	@$(DOCKER_COMPOSE) -f $(TEST_COMPOSE_FILE) up --build $(ABORT_ON_EXIT) eapi
	@$(DOCKER_COMPOSE) -f $(TEST_COMPOSE_FILE) down

test-all:
	@$(DOCKER_COMPOSE) -f $(ALL_TEST_COMPOSE_FILE) up --build $(ABORT_ON_EXIT) eapi
	@$(DOCKER_COMPOSE) -f $(ALL_TEST_COMPOSE_FILE) down

# Docker 
docker-up:
	@$(DOCKER_COMPOSE) up --build $(ABORT_ON_EXIT)

docker-up-detach:
	@$(DOCKER_COMPOSE) up --build --detach

docker-down:
	@$(DOCKER_COMPOSE) down

docker-logs:
	@$(DOCKER_COMPOSE) logs -f

docker-clean:
	@$(DOCKER_COMPOSE) down --rmi all

docker-clean-volumes:
	@$(DOCKER_COMPOSE) down --rmi all -v