prepare: # setup git hooks overrides
	./configs/git/setup.sh
build:
	cargo build --release