#include "../syscalls/syscalls.h"
#include <string.h>

int main() {
  char buf[20] = "Hello, ";
  strcat(buf, "World!\n");
  svc_puts(buf);

//   char *hello = "Hello, \0";
//   char *world = "World!\n";
//   char buf2[20];
//   strcpy(buf2, hello);
//   strcat(buf2, world);
//   svc_puts(buf2);

  char buf2[20] = "Hello, ";
  strcat(buf2, "World!\n");
  svc_puts(buf2);

  svc_exit(strcmp(buf, buf2) == 0);
}