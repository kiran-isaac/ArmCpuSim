#include "../syscalls/syscalls.h"

#define ROWS1 2
#define COLS1 3
#define ROWS2 3
#define COLS2 2

int main() {
    int matrix1[ROWS1][COLS1] = {
        {1, 2, 3},
        {4, 5, 6}
    };

    int matrix2[ROWS2][COLS2] = {
        {7, 8},
        {9, 10},
        {11, 12}
    };

    volatile int result[ROWS1][COLS2] = {0};

    // Matrix multiplication
    for (int i = 0; i < ROWS1; i++) {
        for (int j = 0; j < COLS2; j++) {
            for (int k = 0; k < COLS1; k++) {
                result[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }

    svc_exit(0);
}