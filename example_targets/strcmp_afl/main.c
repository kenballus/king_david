#include <string.h> // for strcmp
#include <unistd.h> // for read, STDIN_FILENO

int main(void) {
    char buf[4] = {};
    read(STDIN_FILENO, buf, sizeof(buf));
    if (buf[0] == 'd') {
        if (buf[1] == 'a') {
            if (buf[2] == 'l' || buf[2] == 'v') {
                if (buf[3] == 'e') {
                    __builtin_trap();
                }
            }
        }
    }
}
