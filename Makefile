# Targets
.PHONY: all build test format lint clean

all: build test format lint clean

build:
	cargo build --manifest-path=crud/Cargo.toml

test:
	cargo test --quiet --manifest-path ./crud/Cargo.toml

format:
	cargo fmt --manifest-path ./crud/Cargo.toml

lint:
	cargo clippy --quiet --manifest-path ./crud/Cargo.toml

clean:
	cargo clean --manifest-path ./crud/Cargo.toml

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