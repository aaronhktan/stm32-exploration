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
CFLAGS += -Os
CFLAGS += -fmessage-length=0 -fno-common
CFLAGS += -ffunction-sections -fdata-sections
CFLAGS_F031K6 = -DF031K6

LFLAGs += -mthumb
LFLAGS += -Wall
LFLAGS += -lgcc
LFLAGS += -lc
LFLAGS += --static
LFLAGS += -Wl,--gc-sections

AS_SRC = ./src/core.S
C_SRC = ./src/main.c
C_SRC += ./src/timer.c
C_SRC += ./freertos/Source/portable/MemMang/heap_4.c
C_SRC += ./freertos/Source/list.c
C_SRC += ./freertos/Source/tasks.c
C_SRC += ./freertos/Source/queue.c
VECT_TBL_F031K6 = ./src/vector_table_f031k6.S
VECT_TBL_F446RE = ./src/vector_table_f446re.S

INCLUDE = -I./
INCLUDE += -I./src
INCLUDE += -I./freertos/Source/include

F031K6_C_SRC += ./freertos/Source/portable/GCC/ARM_CM0/port.c
OBJS_F031K6 = $(AS_SRC:.S=.o)
OBJS_F031K6 += $(VECT_TBL_F031K6:.S=.o)
OBJS_F031K6 += $(C_SRC:.c=F031K6.o)
OBJS_F031K6 += $(F031K6_C_SRC:.c=F031K6.o)

INCLUDE_F031K6 = $(INCLUDE)
INCLUDE_F031K6 += -I./include_f031k6
INCLUDE_F031K6 += -I./freertos/Source/portable/GCC/ARM_CM0

F446RE_C_SRC = ./freertos/Source/portable/GCC/ARM_CM4F/port.c
OBJS_F446RE = $(AS_SRC:.S=.o)
OBJS_F446RE += $(VECT_TBL_F446RE:.S=.o)
OBJS_F446RE += $(C_SRC:.c=F446RE.o)
OBJS_F446RE += $(F446RE_C_SRC:.c=F446RE.o)

INCLUDE_F446RE = $(INCLUDE)
INCLUDE_F446RE += -I./include_f446re
INCLUDE_F446RE += -I./freertos/Source/portable/GCC/ARM_CM4F

.PHONY: all
all: f031k6 f446re

f031k6: ASFLAGS += -mcpu=cortex-m0
f031k6: CFLAGS += -mcpu=cortex-m0
f031k6: CFLAGS += -msoft-float
f031k6: CFLAGS += -mfloat-abi=soft

f031k6: LFLAGS += -mcpu=cortex-m0
f031k6: LFLAGS += -T./ld/stm32f031k6.ld
f031k6: LFLAGS += -msoft-float
f031k6: LFLAGS += -mfloat-abi=soft

f446re: ASFLAGS += -mcpu=cortex-m4
f446re: CFLAGS += -mcpu=cortex-m4
f446re: CFLAGS += -mhard-float
f446re: CFLAGS += -mfloat-abi=hard
f446re: CFLAGS += -mfpu=fpv4-sp-d16

f446re: LFLAGS += -mcpu=cortex-m4
f446re: LFLAGS += -T./ld/stm32f446re.ld
f446re: LFLAGS += -mhard-float
f446re: LFLAGS += -mfloat-abi=hard
f446re: LFLAGS += -mfpu=fpv4-sp-d16

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
