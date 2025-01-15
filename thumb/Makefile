OBJCOPY := arm-none-eabi-objcopy
C_FILES := $(wildcard *.c)

%.dis : %.elf
	arm-none-eabi-objdump -s $< > $@

%.s : %.c
	clang -S --target=armv6-none-eabi -mcpu=cortex-m0 -mthumb $< -o $@ -O0

%.elf : %.s
	clang -c --target=armv6-none-eabi -mcpu=cortex-m0 -mthumb $< -o $@ -O0

all: $(C_FILES:.c=.dis) $(C_FILES:.c=.s)

.PHONY: clean
clean:
	rm -f *.bin *.elf *.s *.dis