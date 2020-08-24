CLANG8=clang-8
CLANG9=clang-9
CLANG10=clang-10
RUSTC=rustc
RUSTFLAGS=--crate-type=lib
LLVMAS8=$(LLVM8PATH)/bin/llvm-as
LLVMAS9=$(LLVM9PATH)/bin/llvm-as
LLVMAS10=$(LLVMPATH)/bin/llvm-as

COBJS = \
	hello.bc hello.ll hello.bc-g hello.ll-g \
	loop.bc loop.ll \
	switch.bc switch.ll \
	linkedlist.bc linkedlist.ll linkedlist.bc-g linkedlist.ll-g \
	variables.bc variables.ll variables.bc-g variables.ll-g \
	issue_4.bc issue_4.ll \

RUSTOBJS = rust.bc rust.ll rust.bc-g rust.ll-g \

LLOBJS = param_and_func_attributes.ll.bc \

.PHONY: all
all: \
	$(patsubst %,llvm8/%,$(COBJS)) \
	$(patsubst %,llvm9/%,$(COBJS)) \
	$(patsubst %,llvm10/%,$(COBJS)) \
	$(patsubst %,llvm8/%,$(LLOBJS)) \
	$(patsubst %,llvm9/%,$(LLOBJS)) \
	$(patsubst %,llvm10/%,$(LLOBJS)) \
	$(patsubst %,rust/%,$(RUSTOBJS)) \

# default to -O3 unless overridden by a more specific rule
%: CFLAGS := -O3

llvm8/%.ll : %.c
	mkdir -p llvm8
	$(CLANG8) $(CFLAGS) -S -emit-llvm $^ -o $@
llvm9/%.ll : %.c
	mkdir -p llvm9
	$(CLANG9) $(CFLAGS) -S -emit-llvm $^ -o $@
llvm10/%.ll : %.c
	mkdir -p llvm10
	$(CLANG10) $(CFLAGS) -S -emit-llvm $^ -o $@

llvm8/%.bc : %.c
	mkdir -p llvm8
	$(CLANG8) $(CFLAGS) -c -emit-llvm $^ -o $@
llvm9/%.bc : %.c
	mkdir -p llvm9
	$(CLANG9) $(CFLAGS) -c -emit-llvm $^ -o $@
llvm10/%.bc : %.c
	mkdir -p llvm10
	$(CLANG10) $(CFLAGS) -c -emit-llvm $^ -o $@

rust/%.ll : %.rs
	mkdir -p rust
	$(RUSTC) $(RUSTFLAGS) --emit=llvm-ir $^ -o $@

rust/%.bc : %.rs
	mkdir -p rust
	$(RUSTC) $(RUSTFLAGS) --emit=llvm-bc $^ -o $@

# with debug information
llvm8/%.ll-g : %.c
	mkdir -p llvm8
	$(CLANG8) $(CFLAGS) -g -S -emit-llvm $^ -o $@
llvm9/%.ll-g : %.c
	mkdir -p llvm9
	$(CLANG9) $(CFLAGS) -g -S -emit-llvm $^ -o $@
llvm10/%.ll-g : %.c
	mkdir -p llvm10
	$(CLANG10) $(CFLAGS) -g -S -emit-llvm $^ -o $@

llvm8/%.bc-g : %.c
	mkdir -p llvm8
	$(CLANG8) $(CFLAGS) -g -c -emit-llvm $^ -o $@
llvm9/%.bc-g : %.c
	mkdir -p llvm9
	$(CLANG9) $(CFLAGS) -g -c -emit-llvm $^ -o $@
llvm10/%.bc-g : %.c
	mkdir -p llvm10
	$(CLANG10) $(CFLAGS) -g -c -emit-llvm $^ -o $@

rust/%.ll-g : %.rs
	mkdir -p rust
	$(RUSTC) $(RUSTFLAGS) -g --emit=llvm-ir $^ -o $@

rust/%.bc-g : %.rs
	mkdir -p rust
	$(RUSTC) $(RUSTFLAGS) -g --emit=llvm-bc $^ -o $@

# use -O0 on linkedlist.c
llvm8/linkedlist.% : CFLAGS := -O0
llvm9/linkedlist.% : CFLAGS := -O0
llvm10/linkedlist.% : CFLAGS := -O0

# assemble this one directly from .ll
llvm8/param_and_func_attributes.ll.bc : param_and_func_attributes.ll
	mkdir -p llvm8
	$(LLVMAS8) $< -o $@
llvm9/param_and_func_attributes.ll.bc : param_and_func_attributes.ll
	mkdir -p llvm9
	$(LLVMAS9) $< -o $@
llvm10/param_and_func_attributes.ll.bc : param_and_func_attributes.ll
	mkdir -p llvm10
	$(LLVMAS10) $< -o $@

.PHONY: clean
clean:
	-rm -rf llvm8 llvm9 llvm10 rust
	-find . -name "*~" | xargs rm
