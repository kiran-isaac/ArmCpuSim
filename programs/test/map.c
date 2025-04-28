#include "../syscalls/syscalls.h"
#include <stdlib.h>

typedef int (*map_fn)(int);

void map(int *array, int len, map_fn f) {
    for (int i = 0; i < len; i++) {
        array[i] = f(array[i]);
    }
}

int add_one(int x) {
    return x + 1;
}

int square(int x) {
    return x * x;
}

int main() {
    volatile int arr[10000];
    int len = sizeof(arr) / sizeof(arr[0]);
    for (unsigned i = 0; i < len; i++) {
        arr[i] = (int) (i + 1);
    }
    map(arr, len, square);

    for (unsigned i = 0; i < len; i++) {
        svc_putint(arr[i]);
        svc_puts("\n");
    }

    svc_exit(0);
}