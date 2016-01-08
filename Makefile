arch ?= x86_64

target ?= $(arch)-unknown-none-gnu

kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg


asm_src := $(wildcard src/arch/$(arch)/*.asm)
asm_obj := $(patsubst src/arch/$(arch)/%.asm, \
    build/arch/$(arch)/%.o, $(asm_src))

rust_kernel := target/$(target)/debug/liblossy.a

.PHONY: all clean run iso runtime

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

$(kernel): cargo $(rust_kernel) $(asm_obj) $(linker_script)
	ld -n --gc-sections -T $(linker_script) -o $(kernel) $(asm_obj) $(rust_kernel)

cargo:
	cargo rustc --target $(target) -- -Z no-landing-pads -C no-redzone

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@


####### building the runtime #######

installed_target_libs := \
	$(shell multirust which rustc | \
		sed s,bin/rustc,lib/rustlib/$(target)/lib,)

runtime_libs := \
	$(installed_target_libs)/libcore.rlib #\
	# $(installed_target_libs)/liballoc.rlib \
	# $(installed_target_libs)/librustc_unicode.rlib \
	# $(installed_target_libs)/libcollections.rlib

runtime: $(runtime_libs)

patch_libcore:
	cd rust/src && patch -p0 < ../../libcore_nofp.patch

$(installed_target_libs):
	mkdir -p $(installed_target_libs)

$(installed_target_libs)/%.rlib: rust/src/%/lib.rs $(installed_target_libs)
	rustc --verbose --target $(target) \
		-Z no-landing-pads \
		--cfg disable_float \
		--out-dir $(installed_target_libs) \
		$<

