#include "../syscalls/syscalls.h"

int func2() {
    return 2;
}

int main() {
    char * bingus = "SHPONGLEDONGLE";
    long x = (long)bingus + func2();
    int a = 10;
    int b = 3;
    int c = a / b;
    float f = 100;
    float g = 3;
    float h = f / g;
    svc_exit(0);
}