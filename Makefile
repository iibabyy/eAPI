all:
	make -s all_docker
	make -s migration_run
# make -s all_database
# make -s all_api

clean:
	make -s clean_api

fclean:
	make -s fclean_api
	make -s fclean_docker
	make -s migration_revert

###		DOCKER		###

# start containers in background
docker:
	docker compose up --build --detach

# start containers
docker_run:
	docker compose up --build

# stop containers
docker_down:
	docker compose down

# cleanup docker cache
docker_clean:
	(docker stop $(docker ps -a -q) && docker rm $(docker ps -a -q) && docker rmi $(docker images -a -q) && docker volume prune -f && docker container prune -f && docker system prune --all --force --volumes && docker volume rm -f $(docker volume ls | grep -v DRIVER | tr -s " " | cut -d " "  -f 2 | tr "\n" " ")) 2>>/dev/null


###		DATABASE	###

MIGRATION_PATH = backend/requirements/migrations

migrate:
	sqlx migrate run --source $(MIGRATION_PATH)

migrate_revert:
	sqlx migrate revert --source $(MIGRATION_PATH) --target-version 0
