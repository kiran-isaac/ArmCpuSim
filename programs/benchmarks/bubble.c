#include "../syscalls/syscalls.h"

void swap(char *arr, int i, int j) {
  int temp = arr[i];
  arr[i] = arr[j];
  arr[j] = temp;
}

void bubble_sort(char arr[], int n) {
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < n - i - 1; j++) {
      if (arr[j] > arr[j + 1])
        swap(arr, j, j + 1);
    }
  }
}

void assert_is_sorted(char *arr, unsigned n) {
  for (int i = 0; i < n - 1; i++) {
    if (arr[i] > arr[i + 1]) {
//      svc_puts("Array is not sorted\n");
      svc_exit(1);
    }
  }
//  svc_puts("Array is sorted\n");
}

int main() {
  char arr[] = {
     10, 9, 8, 7, 6, 5, 4, 3, 2, 1
  };
  unsigned len = sizeof(arr) / sizeof(char);
  bubble_sort(arr, len);
  assert_is_sorted(arr, len);

  svc_exit(arr[0]);
}