.PHONY: all build run clean

# デフォルトのシーンファイル
SCENE ?= scenes/default.yaml

all: run

build:
	cargo build --release

run: build
	./target/release/ray_tracing $(SCENE)

# 使用例:
# make run SCENE=scenes/fog_test.yaml
# make SCENE=scenes/fog_test.yaml

clean:
	cargo clean
	rm -f output.ppm
