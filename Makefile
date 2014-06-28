help:
	@echo "Makefile for building Lossy"
	@echo "Usage: make [ all | clean | build | run | copy | loop ]"
	@echo ""
	@echo

all: build

build:
	@echo "Building Kernel"
	make -C ./kernel

copy:
	make -C ./kernel copy

loop:
	make -C ./kernel install

run:
	@echo "Running [dev] OS in QEMU"
	helpers/qemu.sh lossy.img

clean:
	make -C ./kernel clean