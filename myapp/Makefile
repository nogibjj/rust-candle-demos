verify:
	#shows compute cap (recommend >8)
	nvidia-smi --query-gpu=compute_cap --format=csv
	#shows Cuda compiler driver version
	nvcc --version
	
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