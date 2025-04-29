
CLANG_TARGET_OPTIONS := --target=armv6m-none-eabi -mfloat-abi=soft -mthumb
LINKER_SCRIPT := script.ld

ifeq ($(OS),Windows_NT)
    ARMET := $(USERPROFILE)/LLVM-ET-Arm-19.1.5
else
    ARMET := $(HOME)/LLVM-ET-Arm-19.1.5
endif

%.dis : %.out
	$(ARMET)/bin/llvm-objdump -D -S  $< > $@

%.out: %.c
	$(ARMET)/bin/clang -O2 -g $(CLANG_TARGET_OPTIONS) $< -o $@ -T $(LINKER_SCRIPT)

%.out: %.s
	$(ARMET)/bin/clang -g $(CLANG_TARGET_OPTIONS) $< -o $@ -T $(LINKER_SCRIPT)

%.s: %.c
	$(ARMET)/bin/clang -O0 -S $(CLANG_TARGET_OPTIONS) $< -o $@

all: $(C_FILES:.c=.dis) $(C_FILES:.c=.s) $(C_FILES:.c=.out)

bubblebench:
	$(ARMET)/bin/clang -O0 -g $(CLANG_TARGET_OPTIONS) benchmarks/bubble.c -o benchmarks/bubble.out -T $(LINKER_SCRIPT)
	$(ARMET)/bin/llvm-objdump -D -S  benchmarks/bubble.out > benchmarks/bubble.dis

matmulbench:
	$(ARMET)/bin/clang -O1 -g $(CLANG_TARGET_OPTIONS) benchmarks/matmul.c -o benchmarks/matmul.out -T $(LINKER_SCRIPT)
	$(ARMET)/bin/llvm-objdump -D -S  benchmarks/matmul.out > benchmarks/matmul.dis

facbench:
	$(ARMET)/bin/clang -O1 -g $(CLANG_TARGET_OPTIONS) benchmarks/fac.c -o benchmarks/fac.out -T $(LINKER_SCRIPT)
	$(ARMET)/bin/llvm-objdump -D -S  benchmarks/fac.out > benchmarks/fac.dis

fibbench:
	$(ARMET)/bin/clang -O0 -g $(CLANG_TARGET_OPTIONS) benchmarks/fib.c -o benchmarks/fib.out -T $(LINKER_SCRIPT)
	$(ARMET)/bin/llvm-objdump -D -S  benchmarks/fib.out > benchmarks/fib.dis

makelistbench:
	$(ARMET)/bin/clang -O3 -g $(CLANG_TARGET_OPTIONS) benchmarks/makelist.c -o benchmarks/makelist.out -T $(LINKER_SCRIPT)
	$(ARMET)/bin/llvm-objdump -D -S  benchmarks/makelist.out > benchmarks/makelist.dis

bench: bubblebench matmulbench facbench makelistbench fibbench

.PHONY: clean
clean:
	rm -f *.bin *.o *.s *.dis