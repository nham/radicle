all:
	mkdir -p build
	rustc --out-dir=build -O src/radicle.rs
