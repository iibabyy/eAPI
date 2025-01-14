all:
	make -s all_docker
# make -s all_database
# make -s all_api

clean:
	make -s clean_api

fclean:
	make -s fclean_api
	make -s fclean_docker
# make -s fclean_database

all_api: api_run
clean_api: api_clean
fclean_api: api_fclean

all_docker: docker_d
fclean_docker: docker_clean

all_database: migration_run
fclean_database: migration_revert


###		api		###

PROJECT_NAME = myapp
api_PATH = app/api/srcs/
api_BIN = app/api/srcs/target/release/$(PROJECT_NAME)
api_DEBUG_BIN = app/api/srcs/target/debug/$(PROJECT_NAME)
api_MANIFEST_PATH = --manifest-path $(api_PATH)/Cargo.toml

api_run: $(api_BIN)
	./$(api_BIN)

api_debug_run: $(api_DEBUG_BIN)
	cargo run $(api_MANIFEST_PATH)

api_build:
	cargo build --release $(api_MANIFEST_PATH)

api_debug_build:
	cargo build $(api_MANIFEST_PATH)

$(api_BIN):
	cargo build --release $(api_MANIFEST_PATH)

$(api_DEBUG_BIN):
	cargo build $(api_MANIFEST_PATH)

api_clean:
	mv $(api_BIN) $(api_PATH)
	cargo clean $(api_MANIFEST_PATH)
	mkdir -p $(api_PATH)/target/release
	mv $(api_PATH)/$(PROJECT_NAME) $(api_PATH)/target/release/

api_debug_clean:
	mv $(api_DEBUG_BIN) $(api_PATH)
	cargo clean $(api_MANIFEST_PATH)
	mkdir -p $(api_PATH)/target/debug
	mv $(api_PATH)/$(PROJECT_NAME) $(api_PATH)/target/debug/

api_fclean:
	cargo clean $(api_MANIFEST_PATH)

###		DATABASE	###

MIGRATION_PATH = app/api/migrations

migration_run:
	sqlx migrate run --source $(MIGRATION_PATH)

migration_revert:
	sqlx migrate revert --source $(MIGRATION_PATH)

###		DOCKER		###

docker: #volume_up
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