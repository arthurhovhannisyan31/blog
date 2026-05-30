#.PHONY: prepare
prepare: # setup git hooks overrides
	./configs/git/setup.sh
sqlx-prepare:
	cargo sqlx prepare --workspace
# Exclude blog-fe, needs to be bundled by DX
build:
	cargo build \
		-p blog-cli \
		-p blog-client \
		-p blog-server \
		-p common \
		-p proto-generator
build-release:
	make build --release