#include "io.h"

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
void VGATerminal::setX(_x) {
	cursorX = _x;
}

void VGATerminal::setY(_y) {
	cursorY = _y;
}

void VGATerminal::setCursor(_x, _y) {
	cursorX = _x;
	cursorY = _y;
}

// Really low level - Returns a VGA entry
uint8_t VGATerminal::make_vga_entry(char c, uint8_t color) {
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
	//memcpy((uint16_t*)VGARAM, 0, SCREENSIZE); Commented until further notice ;)

	for (; cursorX < vga_width; cursorX++) {
		for (; cursorY < vga_height; cursorY++) {
			putat(' ', colors); // Yeah, slow. But it works!
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
	size_t datalen = strlen(data);
	for (size_t i = 0; i < datalen; i++) {
		putc(data[i]);
	}
}