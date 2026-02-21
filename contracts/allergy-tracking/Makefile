.PHONY: build test clean optimize deploy-testnet coverage fmt lint

# Build the contract
build:
	cargo build --target wasm32-unknown-unknown --release

# Run tests
test:
	cargo test -- --nocapture

# Run tests with coverage
coverage:
	cargo tarpaulin --out Html --output-dir coverage

# Clean build artifacts
clean:
	cargo clean

# Optimize WASM binary
optimize: build
	soroban contract optimize \
		--wasm ../../target/wasm32-unknown-unknown/release/allergy_tracking.wasm

# Format code
fmt:
	cargo fmt

# Run clippy linter
lint:
	cargo clippy -- -D warnings

# Check code without building
check:
	cargo check

# Deploy to testnet (requires DEPLOYER_SECRET environment variable)
deploy-testnet: optimize
	soroban contract deploy \
		--wasm ../../target/wasm32-unknown-unknown/release/allergy_tracking.optimized.wasm \
		--source deployer \
		--network testnet

# Run all quality checks
quality: fmt lint test

# Build documentation
docs:
	cargo doc --no-deps --open

# Watch for changes and run tests
watch:
	cargo watch -x test
