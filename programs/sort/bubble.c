#include "../syscalls/syscalls.h"
#include <stdio.h>

void swap(char* arr, int i, int j) {
    char temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

void is_sorted(char *arr, int n) {
    for (int i = 0; i < n - 1; i++) {
        if (arr[i] > arr[i + 1]) {
            svc_puts("Array is not sorted\n");
            svc_exit(1);
            return;
        }
    }
    svc_puts("Array is sorted\n");
}

void bubble_sort(char arr[], int n) {
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < n - i - 1; j++) {
        if (arr[j] > arr[j + 1])
            swap(arr, j, j + 1);
    }
  }
}

int main() {
    char arr[] = {10, 2, 17, 19, 7, 5, 103, 91, 4, 3, 0, 201};
    unsigned len = sizeof(arr) / sizeof(arr[0]);
    bubble_sort(arr, len);
    is_sorted(arr, len);

    svc_exit(arr[0]);
}