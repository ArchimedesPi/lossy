

extern "C" {
	size_t strlen(char *str) {
		size_t ret = 0;
		while ( str[ret] != 0 )
			ret++;
		return ret;
	}

	int strcmp(const char *dst, char *src) {
		int i = 0;

		while ((dst[i] == src[i])) {
			if (src[i++] == 0)
				return 0;
		}

		return 1;
	}

	void strcat(void *dest,const void *src) {
		memcpy((char*)((int)dest+(int)strlen((char*)dest)),(char*)src,strlen((char*)src));
	}

}