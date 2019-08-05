CC=clang-8
CFLAGS=-O3

.PHONY: all
all: hello.bc hello.ll \
		loop.bc loop.ll \
		switch.bc switch.ll \
		linkedlist.bc linkedlist.ll \

%.ll : %.c
	$(CC) $(CFLAGS) -S -emit-llvm $^ -o $@

%.bc : %.c
	$(CC) $(CFLAGS) -c -emit-llvm $^ -o $@

# use -O0 on linkedlist.c
linkedlist.ll : linkedlist.c
	$(CC) -O0 -S -emit-llvm $^ -o $@
linkedlist.bc : linkedlist.c
	$(CC) -O0 -c -emit-llvm $^ -o $@

.PHONY: clean
clean:
	find . -name "*.ll" | xargs rm
	find . -name "*.bc" | xargs rm
	find . -name "*~" | xargs rm
