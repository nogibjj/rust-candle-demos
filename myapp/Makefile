verify:
	nvidia-smi --query-gpu=compute_cap --format=csv

install:
	#CPU target
	cargo add --git https://github.com/huggingface/candle.git candle-core
build:
	cargo build

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

clean:
	cargo clean

run:
	cargo run 

all: format lint test run