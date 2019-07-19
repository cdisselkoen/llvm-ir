// a simple program which loops over an array; exercises more of LLVM than hello.c

void loop(int a, int b) {
  volatile int arr[10] = {0};  // 'volatile' prevents memory ops from being optimized out
  if (b <= 10) {
    for(int i = 0; i < b; i++) {
      arr[i] = a + 3;
      if(i > 0) arr[i-1] = arr[i-1] + b;
    }
  }
}
