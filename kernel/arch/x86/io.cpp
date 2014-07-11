#include <os.h>

int strlen(const char *str) {
	int ret = 0;
	while ( str[ret] != 0 )
		ret++;
	return ret;
}

////////
// Constructor
VGATerminal::VGATerminal() {
	cursorY = 0;
	cursorX = 0;
	buffer = (uint16_t*) 0xB8000;
}

/////////
// Low level stuff

// Getters
size_t VGATerminal::getX() {
	return cursorX;
}

size_t VGATerminal::getY() {
	return cursorY;
}

// Setters
void VGATerminal::setX(size_t _x) {
	cursorX = _x;
}

void VGATerminal::setY(size_t _y) {
	cursorY = _y;
}

void VGATerminal::setCursor(size_t _x, size_t _y) {
	cursorX = _x;
	cursorY = _y;
}

// Really low level - Returns a VGA entry
uint16_t VGATerminal::make_vga_entry(char c, uint8_t color) {
	uint16_t c16 = c;
	uint16_t color16 = color;
	return c16 | color16 << 8;
}


// Handle colors
void VGATerminal::setColors(enum Color fg, enum Color bg) {
	colors = make_color(fg, bg);
}

uint8_t VGATerminal::make_color(enum Color fg, enum Color bg) {
	return fg | bg << 4;
}


// Clear the display
void VGATerminal::clear() {
	// Oh. Have to reset the darn cursor
	cursorX = 0;
	cursorY = 0;
	// This breaks *something*, i'm not sure what yet. Maybe it doesn't init the colors? [C++ NOOB]
	//memcpy((uint16_t*)VGARAM, 0, SCREENSIZE);

	for (size_t i = 0; i < vga_width; i++) {
		for (size_t j = 0; j < vga_height; j++) {
			setCursor(i, j);
			putat(' ', colors);
		}
	}
}


// Put a single char at the cursor location in a color
void VGATerminal::putat(char c, uint8_t color) {
	const size_t index = cursorY * vga_width + cursorX;
	buffer[index] = make_vga_entry(c, color);
}

void VGATerminal::putc(char c) {
	putat(c, colors);
	if (++cursorX == vga_width) {
		cursorX = 0;
		if (++cursorY == vga_height) {
			cursorY = 0;
		}
	}
}

void VGATerminal::puts(const char* data) {
	int datalen = strlen(data);
	for (int i = 0; i < datalen; i++) {
		putc(data[i]);
	}
}