#include "../syscalls/syscalls.h"
//
//int foo() {
//    for (char i = 0; i < 100; i++) {
//        
//    }
//    return 100;
//}
//
//int bar() {
//    return foo();
//}

int main() {
    volatile unsigned count = 0;
    volatile int take_branch = 1;
    
    for (unsigned i = 0; i < 10; i++) {
        if (take_branch) {
            count += 2;
        }
    }
    svc_exit(count);
    
}