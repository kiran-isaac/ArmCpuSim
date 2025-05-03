#include "../syscalls/syscalls.h"

#define SAMPLE 615

int main() {
    volatile int arr[100000];
    int len = sizeof(arr) / sizeof(arr[0]);
    for (unsigned i = 0; i < len; i++) {
        arr[i] = (int) (i + 1);
    }

    svc_putint(arr[SAMPLE + 1]);
    svc_exit(arr[SAMPLE] != SAMPLE + 1);
}