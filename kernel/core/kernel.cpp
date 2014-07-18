#include <os.h>


// The `main` function called by boot.s 


extern "C"
void kmain() {
	extern IO io = IO();
	io.terminal.setColors(VGATerminal::Black, VGATerminal::LightGrey);
	io.terminal.clear();
	io.terminal.setCursor(0,0);
	io.terminal.puts(KERNEL_NAME " - " KERNEL_VERSION " -- " KERNEL_DATE " " KERNEL_TIME "\n");
	io.terminal.puts("Licensed under the " KERNEL_LICENSE "\n")
}