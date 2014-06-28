help:
	@echo "Makefile for building Lossy"
	@echo "Usage: make [ all | clean | build | run ]"
	@echo ""
	@echo

all: build

build:
	@echo "Building Kernel"
	make -C ./kernel

run:
	@echo "Running [dev] OS in QEMU"
	helpers/qemu.sh lossy.img

clean:
	make -C ./kernel clean