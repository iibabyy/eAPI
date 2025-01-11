all:
	make -s all_docker
	make -s all_database
	make -s all_backend

clean:
	make -s clean_backend

fclean:
	make -s fclean_backend
	make -s fclean_database
	make -s fclean_docker

all_backend: backend_run
clean_backend: backend_clean
fclean_backend: backend_fclean

all_docker: docker_d
fclean_docker: docker_clean

all_database: migration_run
fclean_database: migration_revert


###		BACKEND		###

PROJECT_NAME = myapp
BACKEND_PATH = app/backend/srcs/
BACKEND_BIN = app/backend/srcs/target/release/$(PROJECT_NAME)
BACKEND_DEBUG_BIN = app/backend/srcs/target/debug/$(PROJECT_NAME)
BACKEND_MANIFEST_PATH = --manifest-path $(BACKEND_PATH)/Cargo.toml

backend_run: $(BACKEND_BIN)
	./$(BACKEND_BIN)

backend_debug_run: $(BACKEND_DEBUG_BIN)
	cargo run $(BACKEND_MANIFEST_PATH)

backen_build: $(BACKEND_BIN)
backen_debug_build: $(BACKEND_DEBUG_BIN)

$(BACKEND_BIN):
	cargo build --release $(BACKEND_MANIFEST_PATH)

$(BACKEND_DEBUG_BIN):
	cargo build $(BACKEND_MANIFEST_PATH)

backend_clean:
	mv $(BACKEND_BIN) .
	cargo clean $(BACKEND_MANIFEST_PATH)
	mkdir -p $(BACKEND_PATH)/target/release
	mv $(PROJECT_NAME) $(BACKEND_PATH)/target/release/

backend_debug_clean:
	mv $(BACKEND_DEBUG_BIN) .
	cargo clean $(BACKEND_MANIFEST_PATH)
	mkdir -p $(BACKEND_PATH)/target/debug
	mv $(PROJECT_NAME) $(BACKEND_PATH)/target/debug/

backend_fclean:
	cargo clean $(BACKEND_MANIFEST_PATH)

###		DATABASE	###

MIGRATION_PATH = app/migrations

migration_run:
	sqlx migrate run --source $(MIGRATION_PATH)

migration_revert:
	sqlx migrate revert --source $(MIGRATION_PATH)

###		DOCKER		###

docker_up: #volume_up
	docker compose -f app/docker-compose.yml up --build

docker_down:
	docker compose -f app/docker-compose.yml down

docker_d: #volume_up
	docker compose -f app/docker-compose.yml up --build -d

docker_stop:
	docker compose -f app/docker-compose.yml stop

docker_start:
	docker compose -f app/docker-compose.yml start

docker_re: docker_down docker_up
docker_red: docker_down docker_d

docker_volume_up:
	mkdir -p /lib/myapp/data

docker_volume_down:
	rm -rf /lib/myapp/data

# network_up:
# 	docker network create app-network

# network_down:
# 	docker network rm app-network

docker_clean:
	docker system prune --all