export UID=$(shell id -u)
export GID=$(shell id -g)

all:
	mkdir -p tmp/.cargo
	docker-compose up --build

release-build:
	docker exec -it ki_client_1 yarn release
	docker exec -it ki_server_1 cargo build --release
	docker cp ki_server_1:/app/target/release/server ./production/server/app
	docker cp ki_server_1:/usr/local/cargo/bin/movine ./production/server/app
	rsync -a --delete ./client/dist ./production/web
	rsync -a --delete ./migrations ./production/server/app

deploy: release-build
	rsync -avz --delete production/ rep:sites/ki
	ssh rep "cd sites/ki && docker-compose up -d --build"

clean:
	docker-compose rm

clean-all: clean
	docker volume rm ki_postgresql_data

recreate-db:
	docker-compose down
	docker volume rm ki_postgresql_data
	$(MAKE) all

production-db-dump:
	ssh rep "docker exec ki_db_1 pg_dump -U ki -c ki_production"

movine-up:
	docker exec -w /app -it ki_server_1 movine up

movine-down:
	docker exec -w /app -it ki_server_1 movine down

movine-fix:
	docker exec -w /app -it ki_server_1 movine fix

movine-redo:
	docker exec -w /app -it ki_server_1 movine redo

movine-status:
	docker exec -w /app -it ki_server_1 movine status

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
