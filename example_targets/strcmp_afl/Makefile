CC := afl-cc
CFLAGS := -std=c23 -O3

.PHONY: all clean

all: main

main: main.c
	$(CC) $(CFLAGS) $^ -o $@

clean: main
	rm -f $^
