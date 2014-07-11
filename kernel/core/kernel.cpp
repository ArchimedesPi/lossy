#include <os.h>


// The `main` function called by boot.s 


extern "C"
void kmain() {
	VGATerminal terminal = VGATerminal();
	terminal.setColors(VGATerminal::LightMagenta, VGATerminal::LightCyan);
	terminal.clear();
	terminal.setCursor(0,0);
	terminal.puts("Hello protected mode world!");
}