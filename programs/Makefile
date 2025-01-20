C_FILES := $(wildcard ./*.c)
ARMBIN := ~/arm-et-bin

%.dis : %.out
	$(ARMBIN)/llvm-objdump -d $< > $@

%.s : %.c
	$(ARMBIN)/clang -S --target=armv6-none-eabi -mfloat-abi=soft -O0 -mcpu=cortex-m0 -mthumb $< -o $@

%.o : %.s
	$(ARMBIN)/clang -c --target=armv6-none-eabi -mfloat-abi=soft -mcpu=cortex-m0 -mthumb $< -o $@

%.out : %.o
	$(ARMBIN)/clang -e main --target=armv6-none-eabi -mfloat-abi=soft -mcpu=cortex-m0 -mthumb $< -o $@ -T /home/kiran/LLVM-ET-Arm-19.1.5-Linux-x86_64/lib/clang-runtimes/arm-none-eabi/armv6m_soft_nofp/lib/picolibc.ld

all: $(C_FILES:.c=.dis) $(C_FILES:.c=.s) $(C_FILES:.c=.out)

.PHONY: clean
clean:
	rm -f *.bin *.o *.s *.dis