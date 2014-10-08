.PHONY: all checkenv

all: checkenv

checkenv:
	@[[ -d "$(RUST_CHECKOUT)" ]] || echo "Please set RUST_CHECKOUT to a valid directory."
	@[[ -e "$(APPLICATION_PATH)" ]] || echo "APPLICATION_PATH is set to a file that does not exist."
	@[[ -x "$(CC)" ]] || echo "Please make sure GCC_PREFIX is set correctly (can't execute GCC)."
	@[[ -x "$(LD)" ]] || echo "Please make sure GCC_PREFIX is set correctly (can't execute LD)."
	@[[ -x "$(RC)" ]] || echo "Please make sure RUST_ROOT is set correctly (can't execute the Rust compiler)."
