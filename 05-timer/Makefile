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
CFLAGS_F031K6 = -DF031K6

LFLAGs += -mthumb
LFLAGS += -Wall
LFLAGS += --specs=nosys.specs
LFLAGS += -nostdlib
LFLAGS += -lgcc

AS_SRC = ./src/core.S
C_SRC = ./src/main.c
C_SRC += ./src/timer.c
VECT_TBL_F031K6 = ./src/vector_table_f031k6.S
VECT_TBL_F446RE = ./src/vector_table_f446re.S

INCLUDE = -I./

OBJS_F031K6 = $(AS_SRC:.S=.o)
OBJS_F031K6 += $(VECT_TBL_F031K6:.S=.o)
OBJS_F031K6 += $(C_SRC:.c=F031K6.o)

INCLUDE_F031K6 = $(INCLUDE)
INCLUDE_F031K6 += -I./include_f031k6

OBJS_F446RE = $(AS_SRC:.S=.o)
OBJS_F446RE += $(VECT_TBL_F446RE:.S=.o)
OBJS_F446RE += $(C_SRC:.c=F446RE.o)

INCLUDE_F446RE = $(INCLUDE)
INCLUDE_F446RE += -I./include_f446re

.PHONY: all
all: f031k6 f446re

f031k6: ASFLAGS += -mcpu=cortex-m0
f031k6: CFLAGS += -mcpu=cortex-m0
f031k6: LFLAGS += -mcpu=cortex-m0
f031k6: LFLAGS += -T./ld/stm32f031k6.ld

f446re: ASFLAGS += -mcpu=cortex-m4
f446re: CFLAGS += -mcpu=cortex-m4
f446re: LFLAGS += -mcpu=cortex-m4
f446re: LFLAGS += -T./ld/stm32f446re.ld

%.o: %.S FORCE
	$(CC) -x assembler-with-cpp $(ASFLAGS) $< -o $@

%F031K6.o: %.c FORCE
	$(CC) -c $(CFLAGS) $(CFLAGS_F031K6) $(INCLUDE_F031K6) $< -o $@
%F446RE.o: %.c FORCE
	$(CC) -c $(CFLAGS) $(INCLUDE_F446RE) $< -o $@
FORCE:

f031k6: f031k6.elf
	$(OC) -S -O binary $< $@.bin
	$(OS) $<

f031k6.elf: $(OBJS_F031K6)
	$(CC) $^ $(LFLAGS) -o $@

f446re: f446re.elf
	$(OC) -S -O binary $< $@.bin
	$(OS) $<

f446re.elf: $(OBJS_F446RE)
	$(CC) $^ $(LFLAGS) -o $@

.PHONY: clean
clean:
	rm -f $(OBJS_F446RE)
	rm -f $(OBJS_F031K6)
	rm -f f031k6.elf
	rm -f f031k6.bin
	rm -f f446re.elf
	rm -f f446re.bin
