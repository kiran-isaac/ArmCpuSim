#include "../syscalls/syscalls.h"
#include <string.h>

int main() {
  while (1) {
    char password_buf[20];
    svc_puts("Enter password: ");

    svc_gets(password_buf);
    svc_puts("You entered: ");
    svc_puts(password_buf);
    svc_puts("\n");

    if (strcmp(password_buf, "password") == 0) {
      svc_puts("Correct password!\n");
      svc_exit(0);
    } else {
      svc_puts("Incorrect password!\n");
    }
  }
}