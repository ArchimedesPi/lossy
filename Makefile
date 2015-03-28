CC=gcc
LD=ld
RUSTC=rustc
ASM=as
QEMU=qemu-system-i386
KERNEL=kernel.elf

RUSTCFLAGS=-O --target i686-unknown-linux-gnu -Z no-landing-pads --emit=obj
ASMFLAGS=--march=i386 --32

ARCH=i386

include ./src/$(ARCH)/Makefile
include ./src/kernel/Makefile

.PHONY: clean run

all: $(KERNEL)

$(KERNEL): $(OBJS)

%.o: %.asm
				$(ASM) $(NASMFLAGS) -o $@ $<

%.o: %.rs
				$(RUSTC) $(RUSTCFLAGS) -o $@ $<
