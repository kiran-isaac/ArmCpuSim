#ifndef syscalls_h
#define syscalls_h

static inline void svc_exit(unsigned code) { __asm("svc 0"); };

static inline void svc_puts(const char *s) { __asm("svc 1"); };

static inline void svc_gets(const char buf[]) { __asm("svc 2"); }

static inline void svc_putint(int n) { __asm("svc 3"); }

#endif // syscalls_h