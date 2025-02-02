#include "../syscalls/syscalls.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
  char hw_buf[100]; 
  sprintf(hw_buf, "%d\n",11);
  svc_puts(hw_buf);

  svc_exit(0);
}