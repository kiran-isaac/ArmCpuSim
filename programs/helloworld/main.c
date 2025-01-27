#include "../syscalls/syscalls.h"

int main() {
  svc_puts("Hello, World!\n");
  svc_exit(0);
}