#ifndef syscalls_h
#define syscalls_h

#include <stddef.h>

extern char __heap_start;  // Defined in the linker script
extern char __heap_end;    // Defined in the linker script
// Pointer to track the current position in the heap
static char* heap_ptr = &__heap_start;

static inline void svc_exit(unsigned code) {
    register unsigned r0 __asm__("r0") = code;
    __asm volatile ("svc 0" :  : "r"(r0) : "memory");
};

static inline void svc_puts(const char *s) {
     register const char *r0 __asm__("r0") = s;
     __asm volatile ("svc 1" :  : "r"(r0) : "memory");
};

static inline void svc_putint(int n) {
     register int r0 __asm__("r0") = n;
     __asm volatile ("svc 3" :  : "r"(r0) : "memory");
}

extern void* sbrk(ptrdiff_t increment) {
    char* prev_heap_ptr = heap_ptr;

    // Check if the increment is positive (malloc style)
    if (increment > 0) {
        // Ensure we don’t exceed the heap boundary
        if (heap_ptr + increment > &__heap_end) {
            return (void*)-1;  // Return error if the heap is full
        }

        // Update the heap pointer to the new position
        heap_ptr += increment;
    }

    // Return the previous heap position as the allocated memory
    return (void*)prev_heap_ptr;
}

#endif // syscalls_h