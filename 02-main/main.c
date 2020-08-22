// This should show up as 4 bytes between _sbss and _ebss
static int val_s = 0;
// This should show up as 4 bytes between _sdata and _edata
static volatile int other_val = 0xDEADBEEF;

int main(void) {
  int val = 0;

  while (1) {
    val_s += 2;
    val += 1;
    other_val = 0xDEADBEEF;
  }
}
