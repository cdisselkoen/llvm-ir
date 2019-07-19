// this program tests the LLVM switch statement

#include <stdio.h>

int has_a_switch(int x) {
  int y = -1;
  switch (x) {
    case 0: y = 3; break;
    case 1: y = 5; break;
    case 13: y = -7; break;
    case 26: y = -5; break;
    case 33: y = 1; break;
    case 142: y = -33; break;
    case 1678: y = 77; break;
    case 88: y = 0; break;
    case 101: y = -3; break;
    default: printf("reached default\n"); break;
  }
  return y + x;
}
