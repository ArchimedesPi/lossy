// Super simple command line driver thingy. Will replace sometime.
// Does text mode VGA in a "terminal".

// Start the include fence
#ifndef IO_H
#define IO_H

#include <stddef.h>
#include <stdint.h>

#define VGARAM 0xB8000
#define SCREENSIZE 0xFA0

class VGATerminal {
public:
	// Constants
	static const size_t vga_width = 80;
	static const size_t vga_height = 25;

	// Enumerate possible colors
	enum Color {
		Black = 0,
		Blue = 1,
		Green = 2,
		Cyan = 3,
		Red = 4,
		Magenta = 5,
		Brown = 6,
		LightGrey = 7,
		DarkGrey = 8,
		LightBlue = 9,
		LightGreen = 10,
		LightCyan = 11,
		LightRed = 12,
		LightMagenta = 13,
		LightBrown = 14,
		White = 15
	};

	/////////////
	// Screen API

	// Color handler
	uint8_t make_color(enum Color fg, enum Color bg);

	// Constructor
	VGATerminal();

	// Coloring API
	void setColors(enum Color fg, enum Color bg);
	
	// Low level control
	void clear(); // Clearing
	void putat(char c, uint8_t color, size_t x, size_t y); // Put char at location in color
	
	// Low level accessors
	size_t getX();
	size_t getY();

	// Low level setters
	void setX(size_t _x);
	void setY(size_t _y);

	void setCursor(size_t _x, size_t _y);


	// High level stuff
	void putc(char c);
	void puts(const char* data);
private:
	//////////
	// Private stuff. They don't need to see it!
	size_t cursorX; // Cursor x
	size_t cursorY; // Cursor y
	uint8_t colors; // Background and foreground colors for the screen
	
	// VGA buffer [text mode]
	uint16_t* buffer;
	
	// Really low level stuff
	uint16_t make_vga_entry(char c, uint8_t color); // This one returns a proper VGA entry
};

// Close fence
#endif