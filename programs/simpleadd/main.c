#include "../syscalls/syscalls.h"

int func3() { return 2 + 3; }

int func2() { return 2; }

int main() {
    int x = func2() * func3();
//    char* str = "Hello, World!\n";
//    x = x * 2;
//    svc_putint(x);
    svc_exit(x);
}