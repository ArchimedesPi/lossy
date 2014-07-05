#include <os.h>

extern "C"
int strlen(char *s) {
	int i = 0;
	while ( *s++)
		i++;
	return i;
}

extern "C"
int strcmp(const char *dst, char *src) {
	int i = 0;

	while ((dst[i] == src[i])) {
		if (src[i++] == 0)
			return 0;
	}

	return 1;
}

extern "C"
void strcat(void *dest,const void *src) {
	memcpy((char*)((int)dest+(int)strlen((char*)dest)),(char*)src,strlen((char*)src));
}