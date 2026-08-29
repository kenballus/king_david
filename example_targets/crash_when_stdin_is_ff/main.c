#include <unistd.h> // for read, STDIN_FILENO

int main(void) {
    unsigned char c = 0;
    read(STDIN_FILENO, &c, sizeof(c));

    if (c == 0xff) {
        return *(volatile int *)nullptr;
    }
}
