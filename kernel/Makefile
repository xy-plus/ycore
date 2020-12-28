target := riscv64imac-unknown-none-elf

all: run

run: build

build:
	cargo build --target $(target)

env:
	cargo install cargo-binutils
	rustup component add llvm-tools-preview
	rustup target add $(target)
