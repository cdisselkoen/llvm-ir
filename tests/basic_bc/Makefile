CC=clang-9
CFLAGS=-O3
RUSTC=rustc
RUSTFLAGS=--crate-type=lib

.PHONY: all
all: hello.bc hello.ll hello.bc-g hello.ll-g \
		loop.bc loop.ll \
		switch.bc switch.ll \
		linkedlist.bc linkedlist.ll linkedlist.bc-g linkedlist.ll-g \
		variables.bc variables.ll variables.bc-g variables.ll-g \
		rust.bc rust.ll rust.bc-g rust.ll-g \
		issue_4.bc issue_4.ll \
		param_and_func_attributes.ll.bc \

%.ll : %.c
	$(CC) $(CFLAGS) -S -emit-llvm $^ -o $@

%.bc : %.c
	$(CC) $(CFLAGS) -c -emit-llvm $^ -o $@

%.ll : %.rs
	$(RUSTC) $(RUSTFLAGS) --emit=llvm-ir $^ -o $@

%.bc : %.rs
	$(RUSTC) $(RUSTFLAGS) --emit=llvm-bc $^ -o $@

# with debug information
%.ll-g : %.c
	$(CC) $(CFLAGS) -g -S -emit-llvm $^ -o $@

%.bc-g : %.c
	$(CC) $(CFLAGS) -g -c -emit-llvm $^ -o $@

%.ll-g : %.rs
	$(RUSTC) $(RUSTFLAGS) -g --emit=llvm-ir $^ -o $@

%.bc-g : %.rs
	$(RUSTC) $(RUSTFLAGS) -g --emit=llvm-bc $^ -o $@

# use -O0 on linkedlist.c
linkedlist.ll : linkedlist.c
	$(CC) -O0 -S -emit-llvm $^ -o $@
linkedlist.bc : linkedlist.c
	$(CC) -O0 -c -emit-llvm $^ -o $@
linkedlist.ll-g : linkedlist.c
	$(CC) -O0 -g -S -emit-llvm $^ -o $@
linkedlist.bc-g : linkedlist.c
	$(CC) -O0 -g -c -emit-llvm $^ -o $@

# assemble this one directly from .ll
param_and_func_attributes.ll.bc : param_and_func_attributes.ll
	llvm-as $< -o $@

.PHONY: clean
clean:
	find . -name "*.ll" | xargs rm
	find . -name "*.bc" | xargs rm
	find . -name "*.ll-g" | xargs rm
	find . -name "*.bc-g" | xargs rm
	find . -name "*~" | xargs rm
