#include "io.h"

// The `main` function called by boot.s 
extern "C"
void kmain() {
	VGATerminal terminal = VGATerminal();
	terminal.setColors(VGATerminal::LightGrey, VGATerminal::Black);
	terminal.clear();
	terminal.puts("Hello World\n");
}