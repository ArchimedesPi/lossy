#include "video/bios_term.h"

/* The `main` function called by boot.s */
void main() {
	BIOSVGAterm terminal = BIOSVGAterm();
	terminal.setColors(COLOR_LIGHT_GREY, COLOR_BLACK);
	terminal.clear();
	terminal.puts("Hello World\n");
}