#include "../syscalls/syscalls.h"

#define N 100

unsigned long long fibonacci(int n) {
    if (n <= 0) return 0;
    if (n == 1) return 1;

    unsigned long long a = 0, b = 1, temp;
    for (int i = 2; i <= n; ++i) {
        temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}

int main() {
    unsigned arr[N] = {0};
    for (unsigned i = 0; i < N; i++) {
        arr[i] = fibonacci(i + 1);
    }

    svc_putint(arr[N - 1]);
    svc_exit(0);
}