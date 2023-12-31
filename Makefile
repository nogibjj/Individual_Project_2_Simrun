# Targets
.PHONY: all build test format lint clean

all: build test format lint clean

build:
	cargo build --release --manifest-path ./rust_crud/Cargo.toml

copy:
	cp rust_crud/target/release/rust_crud ./"Binary_Executable"

test:
	cargo test --quiet --manifest-path ./rust_crud/Cargo.toml

format:
	cargo fmt --manifest-path ./rust_crud/Cargo.toml

lint:
	cargo clippy --quiet --manifest-path ./rust_crud/Cargo.toml

clean:
	cargo clean --manifest-path ./rust_crud/Cargo.toml

# Generate and push changes to GitHub
generate_and_push:
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add query log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi