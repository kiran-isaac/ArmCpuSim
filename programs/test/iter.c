#include "../syscalls/syscalls.h"

int main() {
    int i = 0;
    while (i < 100) {
        i++;
    }
    svc_exit(i);
}