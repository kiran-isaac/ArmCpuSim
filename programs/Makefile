C_FILES := $(wildcard ./*.c)
ARMET := ~/LLVM-ET-Arm-19.1.5-Linux-x86_64
CLANG_TARGET_OPTIONS := --target=armv6m-none-eabi -mfloat-abi=soft -mthumb
LINKER_SCRIPT := script.ld

%.dis : %.out
	$(ARMET)/bin/llvm-objdump -D -S  $< > $@ 
	
%.out: %.s
	$(ARMET)/bin/clang -g $(CLANG_TARGET_OPTIONS) $< -o $@ -T $(LINKER_SCRIPT)

%.out: %.c
	$(ARMET)/bin/clang -O0 -g $(CLANG_TARGET_OPTIONS) $< -o $@ -T $(LINKER_SCRIPT)

%.s: %.c
	$(ARMET)/bin/clang -O0 -S -g $(CLANG_TARGET_OPTIONS) $< -o $@

all: $(C_FILES:.c=.dis) $(C_FILES:.c=.s) $(C_FILES:.c=.out)

.PHONY: clean
clean:
	rm -f *.bin *.o *.s *.dis