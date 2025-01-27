#include "syscalls.h"

void svc_exit(unsigned code) { __asm("svc 0"); }

void svc_puts(const char *s) { __asm("svc 1"); }

void svc_gets(const char buf[]) { __asm("svc 2"); }