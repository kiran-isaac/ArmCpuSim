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

int main() {
    malloc(1);
//    *x = 10;
////    volatile int arr[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
////    int len = sizeof(arr) / sizeof(arr[0]);
////    map(arr, len, add_one);
////
////    for (char i = 0; i < len; i++) {
////        svc_putint(arr[i]);
////    }
//    svc_exit(*x);
}