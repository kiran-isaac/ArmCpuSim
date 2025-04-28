#include "../syscalls/syscalls.h"

#include <stdio.h>


#define SIZE 20

int main() {
    int matrix1[SIZE][SIZE];
    int matrix2[SIZE][SIZE];
    volatile int result[SIZE][SIZE];

    // Initialize matrix1 and matrix2
    for (int i = 0; i < SIZE; i++) {
        for (int j = 0; j < SIZE; j++) {
            matrix1[i][j] = i + j * 2 + 4;
            matrix2[i][j] = i * j - 2;
            result[i][j] = 0;
        }
    }

    // Matrix multiplication
    for (int i = 0; i < SIZE; i++) {
        for (int j = 0; j < SIZE; j++) {
            for (int k = 0; k < SIZE; k++) {
                result[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }

    svc_exit(0);
}