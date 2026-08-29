CC := clang
CFLAGS := -std=c23

.PHONY: all clean

all: main

main: main.c
	$(CC) $(CFLAGS) $^ -o $@

clean: main
	rm -f $^
