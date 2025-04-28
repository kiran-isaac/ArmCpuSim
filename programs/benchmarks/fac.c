#include "../syscalls/syscalls.h"

unsigned factorial(unsigned n) {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

int main() {
    unsigned result = factorial(10);

    if (result == 3628800) {
        svc_exit(0);
    } else {
        svc_exit(1);
    }
}