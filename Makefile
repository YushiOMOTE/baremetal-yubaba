.PHONY: setup build run clean

qemu ?= qemu-system-x86_64

target-dir = target/helloworld/release
bootloader-target-dir = target/x86_64-bootloader/release

kernel = $(target-dir)/helloworld
bootloader = $(bootloader-target-dir)/bootloader
bootloader-bin = $(bootloader-target-dir)/bootloader.bin
bootloader-dir = bootloader

fonts-dir = fontgen

qemu-opt ?= -m 4G
qemu-drive-opt ?= -serial stdio -drive format=raw,file=$(bootloader-dir)/$(bootloader-bin)

fonts = src/font.bin

setup:
	git submodule update --init
	cargo install cargo-xbuild
	cargo install cargo-binutils
	cd $(bootloader-dir); rustup component add llvm-tools-preview
	cd $(bootloader-dir); rustup component add rust-src

build: $(bootloader-bin)

run: build
	$(qemu) $(qemu-opt) $(qemu-drive-opt)

$(fonts): fontgen/src/main.rs fonts/src/lib.rs
	cd $(fonts-dir); cargo run --release

$(kernel): helloworld.json Cargo.toml $(wildcard src/*) $(fonts)
	cargo xbuild --release --target $<

$(bootloader): export KERNEL = ../$(kernel)
$(bootloader): export KERNEL_MANIFEST = ../Cargo.toml
$(bootloader): $(kernel)
	cd $(bootloader-dir); cargo xbuild --features binary,map_physical_memory --release

$(bootloader-bin): $(bootloader)
	cd $(bootloader-dir); rust-objcopy -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 $< $@

clean:
	rm -rf target $(bootloader-dir)/target
