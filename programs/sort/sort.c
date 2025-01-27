#include "../syscalls/syscalls.h"
#include <stdio.h>

void swap(char* arr, int i, int j) {
    char temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

__attribute__((noinline)) void bubble_sort(char arr[], int n) {
  for (int i = 0; i < 6; i++) {
    // Last i elements are already in place, so the loop
    // will only num n - i - 1 times
    for (int j = 0; j < n - i - 1; j++) {
        svc_puts("1\n");
        if (arr[j] > arr[j + 1])
            swap(arr, j, j + 1);
        svc_puts("2\n");
    }
    svc_puts("Outer loop\n");
  }
}

int main() {
    char arr[] = {10, 2, 17, 19, 7, 5};
    unsigned len = 6;
    bubble_sort(arr, len);

    svc_exit(arr[5]);
}