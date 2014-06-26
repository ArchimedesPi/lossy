#include <stddef.h>
#include <stdint.h>

// Super simple graphics driver implementation

enum vga_color {
	COLOR_BLACK = 0,
	COLOR_BLUE = 1,
	COLOR_GREEN = 2,
	COLOR_CYAN = 3,
	COLOR_RED = 4,
	COLOR_MAGENTA = 5,
	COLOR_BROWN = 6,
	COLOR_LIGHT_GREY = 7,
	COLOR_DARK_GREY = 8,
	COLOR_LIGHT_BLUE = 9,
	COLOR_LIGHT_GREEN = 10,
	COLOR_LIGHT_CYAN = 11,
	COLOR_LIGHT_RED = 12,
	COLOR_LIGHT_MAGENTA = 13,
	COLOR_LIGHT_BROWN = 14,
	COLOR_WHITE = 15,
};

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