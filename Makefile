.PHONY: all build run clean

all: run

build:
	cargo build --release

run: build
	./target/release/ray_tracing

clean:
	cargo clean
	rm -f output.ppm
