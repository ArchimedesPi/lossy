#include <stddef.h>
#include <stdint.h>
#include "vga_colors.h"

// Super simple graphics driver implementation

class BIOSVGAterm {
public:
	static const size_t vga_width = 80;
	static const size_t vga_height = 25;
	uint8_t make_color(enum vga_color fg, enum vga_color bg);
	uint8_t make_vga_entry(char c, uint8_t color);
	BIOSVGAterm();
	void setColors(enum vga_color fg, enum vga_color bg);
	void clear();
	void putat(char c, uint8_t color, size_t x, size_t y);
	void putc(char c);
	void puts(const char* data);
private:
	size_t row;
	size_t column;
	uint8_t colors;
	uint16_t* buffer;
};