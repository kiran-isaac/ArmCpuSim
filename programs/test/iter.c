#include "../syscalls/syscalls.h"

int main() {
    int i = 0;
    while (i < 5) {
        i++;
    }
    svc_exit(i);
}