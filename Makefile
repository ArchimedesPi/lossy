# arch to build for.
# supported values:
# * x86_64

ARCH ?= x86_64

ifeq ($(ARCH), x86_64)
	TRIPLE ?= x86_64-elf
endif

RUSTC ?= rustc
LD := $(TRIPLE)-gcc
AS := $(TRIPLE)-as
OBJCOPY := $(TRIPLE)-objcopy

PLATFORMS=`find ./src/arch/ -type d | sed "s/.*\///" | grep -v _common | sort`

LINKERSCRIPT := src/arch/$(ARCH)/link.ld
RSTARGETSPEC := src/arch/$(ARCH)/target.json

LINKERFLAGS := -T $(LINKERSCRIPT) -Xlinker --gc-sections -ffreestanding -nostdlib -z max-page-size=0x1000
ASFLAGS :=

RUSTFLAGS := -O --cfg arch__$(ARCH) --target=$(TARGETSPEC) --cfg disable_float

ifeq ($(ARCH), amd64)
	RUSTFLAGS += -C soft-float
endif

KERNEL := build/kernel.$(ARCH).bin

include src/arch/$(ARCH)/Makefile

.PHONY: all clean help platforms

all: $(KERNEL)

%.o: %.s
	$(AS) $(ASFLAGS) -o $@ $<

$(KERNEL): $(OBJS)
	$(LD) $(LINKERFLAGS) -o $@ $^
ifeq ($(ARCH), x86_64)
	mv $@ $@.elf64
	$(OBJCOPY) $@.elf64 -F elf32-i386 $@
endif

clean:
	rm -rfv $(shell find src/ -type f -name "*.o")
	rm $(KERNEL)

help:
	@echo "Lossy Makefile"
	@echo "See LICENSE for license"
	@echo "Output is, by default, $(KERNEL)"
	@echo "Currently supported platforms:"
	@echo $(PLATFORMS)
	@echo
	@echo "Tasks:"
	@echo "all clean help platforms qemu qemudbg"

platforms:
	@echo $(PLATFORMS)