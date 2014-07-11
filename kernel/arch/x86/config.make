# Arch
COMP_ARCH=i686-elf

# Compilers, linkers, assemblers
CPP=$(COMP_ARCH)-g++
ASM=nasm
# Using gcc as the linker (for -lgcc)
LD=$(COMP_ARCH)-gcc

# Flags for the previous
CPPFLAGS= $(INCDIR) -ffreestanding -O2 -Wall -Wextra -g
ASMFLAGS= -felf 
LDFLAGS= -L ./ -T ./arch/$(ARCH)/linker.ld -ffreestanding -O2 -nostdlib -lgcc