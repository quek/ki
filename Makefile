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

.PHONY : arysn_cli
arysn_cli:
	mkdir -p arysn_cli/generated
	mkdir -p generated
	docker exec -it ki_server_1 cargo run --package arysn_cli

ar: arysn_cli
	cp arysn_cli/generated/user.rs client/src/generated/user.rs
	cp arysn_cli/generated/post.rs client/src/generated/post.rs
	cp arysn_cli/generated/user.rs server/src/generated/user.rs
	cp arysn_cli/generated/user_impl.rs server/src/generated/user_impl.rs
	cp arysn_cli/generated/post.rs server/src/generated/post.rs
	cp arysn_cli/generated/post_impl.rs server/src/generated/post_impl.rs
