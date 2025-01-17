OBJCOPY := arm-none-eabi-objcopy
C_FILES := $(wildcard ./*/*.c)

%.dis : %.out
	arm-none-eabi-objdump -d $< > $@

%.s : %.c
	clang -S --target=armv6-none-eabi -mcpu=cortex-m0 -mthumb $< -o $@ -O0

%.o : %.s
	clang -c --target=armv6-none-eabi -mcpu=cortex-m0 -mthumb $< -o $@ -O0

%.out : %.o
	arm-none-eabi-ld -e main -o $@ $<

all: $(C_FILES:.c=.dis) $(C_FILES:.c=.s) $(C_FILES:.c=.out)

.PHONY: clean
clean:
	rm -f *.bin *.o *.s *.dis