#include "../syscalls/syscalls.h"

#define N_SAMPLES 5

int main() {
    volatile int arr[100000];
    int len = sizeof(arr) / sizeof(arr[0]);
    for (unsigned i = 0; i < len; i++) {
        arr[i] = (int) (i + 1);
    }

    // Just to prove this does something
    for (unsigned i = 0; i < N_SAMPLES; i++) {
        svc_putint(arr[i * 1000]);
        svc_puts("\n");
    }

    svc_exit(0);
}