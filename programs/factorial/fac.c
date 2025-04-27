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
    svc_exit(result);

    if (result == 3628800) {
//        svc_puts("Yipee!!!!\n");
        svc_exit(0);
    } else {
//        svc_puts("Expected 3628800, got ");
//        svc_putint(result);
//        svc_puts("\n");
        svc_exit(1);
    }
}