// test load and store to stack-allocated, heap-allocated, and global variables

#include<stdlib.h>

volatile int global = 5;

void variables(volatile int byvalue, volatile int* ptr) {
  volatile int stack_alloc = 72;
  volatile int* heap_alloc = (volatile int*) malloc(sizeof(volatile int));

  // load and store to stack_alloc
  stack_alloc = stack_alloc + 5;

  // load and store to heap_alloc
  *heap_alloc = *heap_alloc + 5;

  // load and store to global
  global = global + 5;

  // load and store to parameter passed by-value
  byvalue = byvalue + 5;

  // load and store to pointer parameter
  *ptr = *ptr + 5;
}
