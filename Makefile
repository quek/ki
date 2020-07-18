export UID=$(shell id -u)
export GID=$(shell id -g)

all:
	mkdir -p tmp/.cargo
	docker-compose up --build

release-build:
	docker exec -it ki_client_1 yarn release
	docker exec -it ki_server_1 cargo build --release
	docker cp ki_server_1:/app/target/release/server ./production/server/app
	docker cp ki_server_1:/usr/local/cargo/bin/diesel ./production/server/app
	cp -a ./client/dist ./production/web
	cp -a ./server/migrations ./production/server/app

deploy: release-build
	rsync -avz --delete production/ rep:sites/ki
	ssh rep "cd sites/ki && docker-compose up -d --build"

clean:
	docker-compose rm

clean-all: clean
	docker volume rm ki_postgresql_data

migration-run:
	docker exec -it ki_server_1 diesel migration run

psql:
	docker exec -it ki_db_1 psql -U ki ki_development
