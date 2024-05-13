all: build

build: 
	cargo build

check: build
	cargo check

run: build
	cargo run

test: build
	cargo test

clean:
	cargo clean

docker-build:
	docker-compose build

docker-test:
	docker-compose run make test

docker-run:
	docker-compose up -d

docker-down:
	docker-compose down

docker-clean:
	docker-compose rm --force

distclean: clean docker-clean

.PHONY: all build check run test clean distclean \
		docker-build docker-test docker-run docker-down docker-clean

.SILENT: clean docker-clean distclean
