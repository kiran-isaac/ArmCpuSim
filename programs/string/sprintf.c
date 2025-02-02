#include "../syscalls/syscalls.h"
#include <stdio.h>

int main() {
  char hw_buf[100];
  sprintf(hw_buf, "hello_world");
  svc_puts(hw_buf);

  svc_exit(0);
}