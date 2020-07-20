CC = arm-none-eabi-gcc
OC = arm-none-eabi-objcopy
OD = arm-none-eabi-objdump
OS = arm-none-eabi-size

ASFLAGS += -c
ASFLAGS += -O0
ASFLAGS += -mthumb
ASFLAGS += -Wall

CFLAGS += -mthumb
CFLAGS += -Wall
CFLAGS += -g
CFLAGS += --specs=nosys.specs

LFLAGs += -mthumb
LFLAGS += -Wall
LFLAGS += --specs=nosys.specs
LFLAGS += -nostdlib
LFLAGS += -lgcc

AS_SRC = ./core.S
VECT_TBL = ./vector_table.S
C_SRC = ./main.c

OBJS = $(AS_SRC:.S=.o)
OBJS += $(VECT_TBL:.S=.o)
OBJS += $(C_SRC:.c=.o)

.PHONY: all
all: f031k6

f031k6: ASFLAGS += -mcpu=cortex-m0
f031k6: CFLAGS += -mcpu=cortex-m0
f031k6: LFLAGS += -mcpu=cortex-m0
f031k6: LFLAGS += -T./stm32f031k6.ld

%.o: %.S FORCE
	$(CC) -x assembler-with-cpp $(ASFLAGS) $< -o $@

%.o: %.c FORCE
	$(CC) -c $(CFLAGS) $(INCLUDE) $< -o $@
FORCE:

f031k6: f031k6.elf
	$(OC) -S -O binary $< $@.bin
	$(OS) $<

f031k6.elf: $(OBJS)
	$(CC) $^ $(LFLAGS) -o $@

.PHONY: clean
clean:
	rm -f $(OBJS)
	rm -f f031k6.elf
