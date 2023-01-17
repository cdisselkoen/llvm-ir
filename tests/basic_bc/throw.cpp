#include <cstdio>
#include <stdexcept>

int main() {
  try {
    throw std::logic_error("hello, exceptions!");
  } catch (const std::logic_error &e) {
    printf("%p\n", (void *) &e);
  }
  return 0;
}