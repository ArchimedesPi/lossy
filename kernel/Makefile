ARCH=x86
KERNEL=lossy.elf
INCLUDEDIR= -I. -I./core -I./arch/$(KERN_ARCH)

include ./arch/$(ARCH)/config.make

include ./core/Makefile
include ./arch/$(ARCH)/Makefile

CPPFLAGS := $(CPPFLAGS) -ffreestanding -O2 -Wall -Wextra
ASMFLAGS := $(ASMFLAGS)
PLATFORMS= `find ./arch/ -type d | sed "s/.*\///" | sort`

all: $(KERNEL)

$(KERNEL): $(OBJS)
	$(LD) $(LDFLAGS) -o $@ $^

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