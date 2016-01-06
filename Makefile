arch ?= x86_64

target ?= $(arch)-unknown-linux-gnu

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

run: $(iso)
	qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	mkdir -p build/isofiles/boot/grub
	cp $(kernel) build/isofiles/boot/kernel.bin
	cp $(grub_cfg) build/isofiles/boot/grub
	grub-mkrescue -o $(iso) build/isofiles 2> /dev/null
	rm -r build/isofiles

$(kernel): cargo $(rust_kernel) $(asm_obj) $(linker_script)
	ld -n --gc-sections -T $(linker_script) -o $(kernel) $(asm_obj) $(rust_kernel)

cargo:
	cargo rustc --target $(target) -- -Z no-landing-pads -C no-redzone

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@
