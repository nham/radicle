BUILD_DIR = build

all:
	mkdir -p $(BUILD_DIR)
	rustc --out-dir=$(BUILD_DIR) -O src/radicle.rs

test:
	mkdir -p $(BUILD_DIR)
	rustc --test --out-dir=$(BUILD_DIR) -O src/radicle.rs
