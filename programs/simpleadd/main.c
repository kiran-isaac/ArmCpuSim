#include "../syscalls/syscalls.h"

int func3() { return 2; }

int func2() { return 2; }

int main() {
    int x = func2() * func3();
    x = x * 2;
    svc_exit(x);
}