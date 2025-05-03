#include "../syscalls/syscalls.h"

void swap(unsigned *arr, int i, int j) {
    int temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

void bubble_sort(unsigned arr[], int n) {
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n - i - 1; j++) {
            if (arr[j] > arr[j + 1])
              swap(arr, j, j + 1);
        }
    }
}

void assert_is_sorted(unsigned *arr, unsigned n) {
    for (int i = 0; i < n - 1; i++) {
        if (arr[i] > arr[i + 1]) {
            svc_puts("Array is not sorted\n");
            svc_exit(1);
        }
    }
    svc_puts("Array is sorted\n");
}

int main() {
    unsigned arr[] = {
//        20, 19, 18, 17, 16, 15, 14, 13, 12, 11,
        10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0
    };
    unsigned len = sizeof(arr) / sizeof(unsigned);
    bubble_sort(arr, len);
    assert_is_sorted(arr, len);

    svc_exit(arr[0]);
}