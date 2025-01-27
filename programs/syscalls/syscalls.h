#ifndef syscalls_h
#define syscalls_h

void svc_exit(unsigned code);

void svc_puts(const char *s);

void svc_gets(const char buf[]);

#endif // syscalls_h