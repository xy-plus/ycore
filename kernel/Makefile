MODE ?= release
BOARD ?= qemu
SBI ?= rustsbi
ARCH ?= riscv64
SMP ?= 4

.PHONY: all clean build kernel run

BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin
# BOOTLOADER := default # opensbi
TARGET := riscv64gc-unknown-none-elf
KERNEL_ELF := target/$(TARGET)/$(MODE)/kernel
KERNEL_BIN := $(KERNEL_ELF).bin
KERNEL_ENTRY_PA := 0x80200000
OBJCOPY := rust-objcopy --binary-architecture=riscv64

all: run

run: build $(BOARD)

$(BOARD):
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-smp $(SMP)

build: $(KERNEL_BIN)

$(KERNEL_BIN): kernel
	$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	cargo build --$(MODE)

env:
	cargo install cargo-binutils
	rustup component add llvm-tools-preview
	rustup target add $(TARGET)
	mkdir -p ../bootloader
	wget https://github.com/luojia65/rustsbi/releases/download/v0.1.0/rustsbi-qemu.zip -O ../bootloader/rustsbi-qemu.zip
	unzip ../bootloader/rustsbi-qemu.zip -d../bootloader

clean:
	cargo clean
