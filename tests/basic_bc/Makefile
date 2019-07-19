CC=clang-8
CFLAGS=-O3
LLVMAS=llvm-as

.PHONY: all
all: hello.bc hello.ll \
		loop.bc loop.ll \
		switch.bc switch.ll \

%.ll : %.c
	$(CC) $(CFLAGS) -S -emit-llvm $^ -o $@

%.bc : %.ll
	$(LLVMAS) $^ -o $@

.PHONY: clean
clean:
	find . -name "*.ll" | xargs rm
	find . -name "*.bc" | xargs rm
	find . -name "*~" | xargs rm
