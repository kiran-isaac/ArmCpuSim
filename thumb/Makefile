OBJCOPY := arm-none-eabi-objcopy
C_FILES := $(wildcard *.c)

%.dis : %.o
	arm-none-eabi-objdump -d $< > $@

%.s : %.c
	clang -S --target=armv6-none-eabi -mcpu=cortex-m0 -mthumb $< -o $@ -O0

%.o : %.s
	clang -c --target=armv6-none-eabi -mcpu=cortex-m0 -mthumb $< -o $@ -O0

all: $(C_FILES:.c=.dis) $(C_FILES:.c=.s) $(C_FILES:.c=.o)

.PHONY: clean
clean:
	rm -f *.bin *.o *.s *.dis