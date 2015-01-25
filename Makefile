CC=gcc
LD=ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386

arch ?= i386

all: disk.img

.SUFFIXES: .o .rs .asm

.PHONY: clean run

.rs.o:
	$(RUSTC) -O --target