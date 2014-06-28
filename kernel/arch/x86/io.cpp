#include "io.h"

size_t strlen(const char* str)
{
	size_t ret = 0;
	while ( str[ret] != 0 )
		ret++;
	return ret;
}

////////
// Constructor
VGATerminal::VGATerminal() {
	y = 0;
	x = 0;
	buffer = (uint16_t*) 0xB8000;
}

/////////
// Low level stuff

// Getters
size_t VGATerminal::getX() {
	return x;
}

size_t VGATerminal::getY() {
	return y;
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
	for ( size_t y = 0; y < vga_height; y++ ) {
		for ( size_t x = 0; x < vga_width; x++) {
			const size_t index = y * vga_width + x;
			buffer[index] = make_vga_entry(' ', colors);
		}
	}
}


// Put a single char at a location in a color
void VGATerminal::putat(char c, uint8_t color, size_t x, size_t y) {
	const size_t index = y * vga_width + x;
	buffer[index] = make_vga_entry(c, color);
}

void VGATerminal::putc(char c) {
	putat(c, colors, x, y);
	if (++x == vga_width) {
		x = 0;
		if (++y == vga_height) {
			y = 0;
		}
	}
}

void VGATerminal::puts(const char* data) {
	size_t datalen = strlen(data);
	for (size_t i = 0; i < datalen; i++) {
		putc(data[i]);
	}
}