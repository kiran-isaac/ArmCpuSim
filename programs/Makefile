C_FILES := $(wildcard ./*.c)
ARMET := ~/LLVM-ET-Arm-19.1.5-Linux-x86_64
CLANG_TARGET_OPTIONS := --target=armv6m-none-eabi -mfloat-abi=soft -mthumb
LINKER_SCRIPT := script.ld

%.dis : %.out
	$(ARMET)/bin/llvm-objdump -D  $< > $@ 

%.dis : %.o
	$(ARMET)/bin/llvm-objdump -d $< > $@

%.s: %.c
	$(ARMET)/bin/clang -O0 -S $(CLANG_TARGET_OPTIONS) $< -o $@

%.o: %.s
	$(ARMET)/bin/clang -c $(CLANG_TARGET_OPTIONS) $< -o $@

%.out: %.o
	$(ARMET)/bin/clang $(CLANG_TARGET_OPTIONS) syscalls/syscalls.o $< -o $@ -T $(LINKER_SCRIPT)

%.dis: %.c  
	$(MAKE) $(<:.c=.s)
	$(MAKE) $(<:.c=.o)
	$(MAKE) $(@:.out=.o)
	$(MAKE) $(@:.dis=.out)

all: $(C_FILES:.c=.dis) $(C_FILES:.c=.s) $(C_FILES:.c=.out)

.PHONY: clean
clean:
	rm -f *.bin *.o *.s *.dis