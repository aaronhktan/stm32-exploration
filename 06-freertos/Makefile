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
CFLAGS += -std=c99

LFLAGs += -mthumb
LFLAGS += -Wall
LFLAGS += -lgcc
LFLAGS += -lc

AS_SRC = ./src/core.S
C_SRC = ./src/main.c
C_SRC += ./src/timer.c
C_SRC += ./freertos/Source/portable/MemMang/heap_4.c
C_SRC += ./freertos/Source/portable/GCC/ARM_CM4F/port.c
C_SRC += ./freertos/Source/list.c
C_SRC += ./freertos/Source/tasks.c
C_SRC += ./freertos/Source/queue.c
VECT_TBL_F446RE = ./src/vector_table_f446re.S

INCLUDE = -I./
INCLUDE += -I./src

OBJS_F446RE = $(AS_SRC:.S=.o)
OBJS_F446RE += $(VECT_TBL_F446RE:.S=.o)
OBJS_F446RE += $(C_SRC:.c=F446RE.o)

INCLUDE_F446RE = $(INCLUDE)
INCLUDE_F446RE += -I./include_f446re
INCLUDE_F446RE += -I./freertos/Source/include
INCLUDE_F446RE += -I./freertos/Source/portable/GCC/ARM_CM4F

.PHONY: all
all: f446re

f446re: ASFLAGS += -mcpu=cortex-m4
f446re: CFLAGS += -mcpu=cortex-m4
f446re: CFLAGS += -mhard-float
f446re: CFLAGS += -mfloat-abi=hard
f446re: CFLAGS += -mfpu=fpv4-sp-d16
f446re: CFLAGS += -Os
f446re: CFLAGS += -fmessage-length=0 -fno-common
f446re: CFLAGS += -ffunction-sections -fdata-sections

f446re: LFLAGS += -mcpu=cortex-m4
f446re: LFLAGS += -T./ld/stm32f446re.ld
f446re: LFLAGS += -mhard-float
f446re: LFLAGS += -mfloat-abi=hard
f446re: LFLAGS += -mfpu=fpv4-sp-d16
f446re: LFLAGS += --static
f446re: LFLAGS += -Wl,--gc-sections

%.o: %.S FORCE
	$(CC) -x assembler-with-cpp $(ASFLAGS) $< -o $@

%F446RE.o: %.c FORCE
	$(CC) -c $(CFLAGS) $(INCLUDE_F446RE) $< -o $@
FORCE:

f446re: f446re.elf
	$(OC) -S -O binary $< $@.bin
	$(OS) $<

f446re.elf: $(OBJS_F446RE)
	$(CC) $^ $(LFLAGS) -o $@

.PHONY: clean
clean:
	rm -f $(OBJS_F446RE)
	rm -f f446re.elf
	rm -f f446re.bin
