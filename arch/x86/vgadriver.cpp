#include "bios_term.h"

#include "support/r_strings.h"

uint8_t BIOSVGAterm::make_color(enum vga_color fg, enum vga_color bg) {
	return fg | bg << 4;
}

uint8_t BIOSVGAterm::make_vga_entry(char c, uint8_t color) {
	uint16_t c16 = c;
	uint16_t color16 = color;
	return c16 | color16 << 8;
}

BIOSVGAterm::BIOSVGAterm() {
	row = 0;
	column = 0;
	buffer = (uint16_t*) 0xB8000;
}

void BIOSVGAterm::setColors(enum vga_color fg, enum vga_color bg) {
	colors = make_color(fg, bg);
}

void BIOSVGAterm::clear() {
	for ( size_t y = 0; y < vga_height; y++ ) {
		for ( size_t x = 0; x < vga_width; x++) {
			const size_t index = y * vga_width + x;
			buffer[index] = make_vga_entry(' ', colors);
		}
	}
}

void BIOSVGAterm::putat(char c, uint8_t color, size_t x, size_t y) {
	const size_t index = y * vga_width + x;
	buffer[index] = make_vga_entry(c, color);
}

void BIOSVGAterm::putc(char c) {
	putat(c, colors, column, row);
	if (++column == vga_width) {
		column = 0;
		if (++row == vga_height) {
			row = 0;
		}
	}
}

void BIOSVGAterm::puts(const char* data) {
	size_t datalen = strlen(data);
	for (size_t i = 0; i < datalen; i++) {
		putc(data[i]);
	}
}