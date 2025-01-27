#include "../syscalls/syscalls.h"
#include <string.h>

int main() {
  char buf[20] = "Hello, ";
  strcat(buf, "World!\n");
  svc_puts(buf);
  svc_exit(0);
}