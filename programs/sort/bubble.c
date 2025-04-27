#include "../syscalls/syscalls.h"
#include <stdio.h>

void swap(unsigned *arr, int i, int j) {
  char temp = arr[i];
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

void assert_is_sorted(unsigned *arr, int n) {
  for (int i = 0; i < n - 1; i++) {
    if (arr[i] > arr[i + 1]) {
//      svc_puts("Array is not sorted\n");
      svc_exit(1);
      return;
    }
  }
  svc_puts("Array is sorted\n");
}

int main() {
  unsigned arr[] = {2, 1};
  bubble_sort(arr, 8);
//  assert_is_sorted(arr, len);

//  svc_exit(69);
//
  svc_exit(arr[0]);
}