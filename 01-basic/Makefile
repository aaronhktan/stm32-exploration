CC = arm-none-eabi-gcc

ASFLAGS += -c
ASFLAGS += -O0
ASFLAGS += -mthumb
ASFLAGS += -Wall

LFLAGs += -mthumb
LFLAGS += -Wall
LFLAGS += --specs=nosys.specs
LFLAGS += -nostdlib
LFLAGS += -lgcc

AS_SRC = ./core.S

OBJS = $(AS_SRC:.S=.o)

f031k6: ASFLAGS += -mcpu=cortex-m0
f031k6: LFLAGS += -mcpu=cortex-m0
f031k6: LFLAGS += -T./stm32f031k6.ld

f446re: ASFLAGS += -mcpu=cortex-m4
f446re: LFLAGS += -mcpu=cortex-m4
f446re: LFLAGS += -T./stm32f446re.ld

%.o: %.S FORCE
	$(CC) -x assembler-with-cpp $(ASFLAGS) $< -o $@
FORCE:

f031k6 f446re: $(OBJS)
	$(CC) $^ $(LFLAGS) -o $@.elf

.PHONY: clean
clean:
	rm -f $(OBJS)
	rm -f f446re.elf
	rm -f f031k6.elf
