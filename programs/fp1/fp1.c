#include "../syscalls/syscalls.h"
#include <stdio.h>

int main() {
    float f = 100;
    float g = 3;
    float h = f * g;

    char float_buf[100];
    sprintf(float_buf, "f = %f, g = %f, f*g = %f\n", f, g, h);
    svc_puts(float_buf);
    
    svc_exit(0);
}