arch ?= x86_64

target ?= $(arch)-unknown-none-gnu
target_file ?= src/arch/$(arch)/$(target).json

kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg


asm_src := $(wildcard src/arch/$(arch)/*.asm)
asm_obj := $(patsubst src/arch/$(arch)/%.asm, \
    build/arch/$(arch)/%.o, $(asm_src))

rust_kernel := target/$(target)/debug/liblossy.a

.PHONY: all clean run iso

all: $(kernel)

clean:
	rm -r build
	rm -r target

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	mkdir -p build/isofiles/boot/grub
	cp $(kernel) build/isofiles/boot/kernel.bin
	cp $(grub_cfg) build/isofiles/boot/grub
	grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	rm -r build/isofiles

$(kernel): libcore cargo $(rust_kernel) $(asm_obj) $(linker_script)
	ld -n --gc-sections -T $(linker_script) -o $(kernel) $(asm_obj) $(rust_kernel)

cargo:
	cargo rustc --target $(target_file) -- -Z no-landing-pads -C no-redzone

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@

rustc-nightly-src.tar.gz:
	curl -L -O https://static.rust-lang.org/dist/rustc-nightly-src.tar.gz

libcore/lib.rs: rustc-nightly-src.tar.gz libcore_nofp.patch
	tar -xmf rustc-nightly-src.tar.gz rustc-nightly/src/libcore --transform 's~^rustc-nightly/src/~~'
	patch -p0 < libcore_nofp.patch

build/libcore.rlib: libcore/lib.rs
	rustc -o $@ --target $(target_file) --cfg disable-float -C soft-float --crate-type=lib --emit=link,dep-info $<
