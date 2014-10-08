# Define all the things up here
# so that when we include config.mk farther down
# it just overwrites the things that are different in config.mk

# The QEMU command we're using. Arch specific. Left independant of arch because of weird naming
# and becuase I didn't want a huge conditional in a Makefile
QEMU=qemu-system-i386
# Yeah, I use NASM. It has better syntax.
NASM=nasm
# The rust compiler.
RUSTC=rustc
# Linker. Independent of arch because of same reasons in QEMU.
LD=i386-elf-ld


# Tell Make that the following names are *not filenames* and should *never* be parsed as such
.PHONY: all run debugrun

# Run everything in this order. Called by default when only `make` is executed.
all: run

run:
	$(QEMU) 
