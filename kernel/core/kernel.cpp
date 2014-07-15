#include <os.h>


// The `main` function called by boot.s 


extern "C"
void kmain() {
	IO io = IO();
	io.terminal.setColors(VGATerminal::Black, VGATerminal::LightGrey);
	io.terminal.clear();
	io.terminal.setCursor(0,0);
	io.terminal.puts(KERNEL_NAME " - " KERNEL_VERSION " -- " KERNEL_DATE " " KERNEL_TIME "\n");
	io.terminal.puts("This is a second line!\n");
	io.terminal.putc('\n');
	io.terminal.puts("And a third (actually fourth) line! Isn't this exciting?\n");
}