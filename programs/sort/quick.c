// https://www.geeksforgeeks.org/quick-sort-in-c/
// C program to implement Quick Sort Algorithm
#include "../syscalls/syscalls.h"

void swap(int *a, int *b) {
  int temp = *a;
  *a = *b;
  *b = temp;
}

int partition(int arr[], int low, int high) {
  // Initialize pivot to be the first element
  int p = arr[low];
  int i = low;
  int j = high;

  while (i < j) {

    // Find the first element greater than
    // the pivot (from starting)
    while (arr[i] <= p && i <= high - 1) {
      i++;
    }

    // Find the first element smaller than
    // the pivot (from last)
    while (arr[j] > p && j >= low + 1) {
      j--;
    }
    if (i < j) {
      swap(&arr[i], &arr[j]);
    }
  }
  swap(&arr[low], &arr[j]);
  return j;
}

void quick_sort(int arr[], int low, int high) {
  if (low < high) {

    // call partition function to find Partition Index
    int pi = partition(arr, low, high);

    // Recursively call quickSort() for left and right
    // half based on Partition Index
    quick_sort(arr, low, pi - 1);
    quick_sort(arr, pi + 1, high);
  }
}

void is_sorted(int *arr, int n) {
  for (int i = 0; i < n - 1; i++) {
    if (arr[i] > arr[i + 1]) {
      svc_puts("Array is not sorted\n");
      svc_exit(1);
      return;
    }
  }
  svc_puts("Array is sorted\n");
}

int main() {

  int arr[] = {2, 5, 3, 1, 0, 6};
  int n = sizeof(arr) / sizeof(arr[0]);

  // calling quickSort() to sort the given array
  quick_sort(arr, 0, n - 1);
  is_sorted(arr, n);

  //   for (int i = 0; i < n; i++)
  //     printf("%d ", arr[i]);

  svc_exit(arr[0]);
}