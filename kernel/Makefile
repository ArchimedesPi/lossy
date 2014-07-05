ARCH=x86
KERNEL=lossy.elf
INCDIR= -I. -I./core -I./arch/$(ARCH)

include ./arch/$(ARCH)/config.make

include ./runtime/Makefile
include ./core/Makefile
include ./arch/$(ARCH)/Makefile


CPPFLAGS := $(CPPFLAGS)
ASMFLAGS := $(ASMFLAGS)
PLATFORMS= `find ./arch/ -type d | sed "s/.*\///" | sort`

all: $(KERNEL)

$(KERNEL): $(OBJS)
	$(LD) $(LDFLAGS) -o $@ $^

objs:
	@echo $(OBJS)

help:
	@echo "Lossy Kernel Makefile"
	@echo "Outputs "$(KERNEL)
	@echo "Usage: make [ all | clean ]"
	@echo "Supported platforms:"
	@echo $(PLATFORMS)
	@echo


%.o: %.cpp
	$(CPP) -c $< -o $@ $(CPPFLAGS)

%.o: %.asm
	$(ASM) $(ASMFLAGS) $< -o $@

clean:
	rm -f $(OBJS) $(KERNEL)

copy:
	cp lossy.elf ../hdfiles/boot/lossy.elf

install:
	cp -r ../hdfiles/* /mnt
	sync

run:
	../helpers/qemu.sh ../lossy.img

runelf:
	qemu-system-i386 -kernel $(KERNEL)