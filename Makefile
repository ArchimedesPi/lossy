help:
	@echo "Makefile for building Lossy"
	@echo "Usage: make [ all | clean | build | run | run ]"
	@echo ""
	@echo

all:
	@echo "Building Kernel"
	make -C ./kernel

run:
	@echo "Running [dev] OS in QEMU"
	# Nothing yet

clean:
	make -C ./kernel clean