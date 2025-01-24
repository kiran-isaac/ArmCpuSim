C_FILES := $(wildcard ./*.c)
ARMBIN := ~/arm-et-bin
CLANG_TARGET_OPTIONS := --target=armv6-none-eabi -mfloat-abi=soft -mcpu=cortex-m0 -mthumb

%.dis : %.out
	$(ARMBIN)/llvm-objdump -d $< > $@

%.dis : %.o
	$(ARMBIN)/llvm-objdump -d $< > $@

%.s : %.c
	$(ARMBIN)/clang -Os -S $(CLANG_TARGET_OPTIONS) $< -o $@

%.o : %.s
	$(ARMBIN)/clang -c $(CLANG_TARGET_OPTIONS) $< -o $@

%.a : %.o
	$(ARMBIN)/clang $(CLANG_TARGET_OPTIONS) $< -o $@ -T /home/kiran/LLVM-ET-Arm-19.1.5-Linux-x86_64/lib/clang-runtimes/arm-none-eabi/armv6m_soft_nofp/lib/picolibc.ld

%.out : %.o syscalls/syscalls.o
	$(ARMBIN)/clang -e main $(CLANG_TARGET_OPTIONS) syscalls/syscalls.o $< -o $@ -T /home/kiran/LLVM-ET-Arm-19.1.5-Linux-x86_64/lib/clang-runtimes/arm-none-eabi/armv6m_soft_nofp/lib/picolibc.ld

all: $(C_FILES:.c=.dis) $(C_FILES:.c=.s) $(C_FILES:.c=.out)

syscalls: syscalls/syscalls.o syscalls/syscalls.dis

.PHONY: clean
clean:
	rm -f *.bin *.o *.s *.dis