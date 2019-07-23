CC=clang-8
CFLAGS=-O3
LLVMAS=llvm-as

.PHONY: all
all: hello.bc hello.ll \
		loop.bc loop.ll \
		switch.bc switch.ll \
		linkedlist.bc linkedlist.ll \

%.ll : %.c
	$(CC) $(CFLAGS) -S -emit-llvm $^ -o $@

%.bc : %.ll
	$(LLVMAS) $^ -o $@

# use -O0 on linkedlist.c
linkedlist.ll : linkedlist.c
	$(CC) -O0 -S -emit-llvm $^ -o $@

.PHONY: clean
clean:
	find . -name "*.ll" | xargs rm
	find . -name "*.bc" | xargs rm
	find . -name "*~" | xargs rm
