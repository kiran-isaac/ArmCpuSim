#include "syscalls.h"

void svc_exit(unsigned code) { __asm("svc 0"); }

void svc_puts(const char *s) { __asm("svc 0"); }

void svc_gets() { __asm("svc 1"); }