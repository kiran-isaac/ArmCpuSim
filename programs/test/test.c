#include "../syscalls/syscalls.h"

int func2() {
    return 2;
}

int main() {
    int a = 10;
    int b = 3;
    int c = a / b;
    svc_exit(c);
}