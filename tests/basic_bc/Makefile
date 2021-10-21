CLANG8=clang-8
CLANG9=clang-9
CLANG10=clang-10 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk
CLANG11=clang-11 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk
CLANG12=clang-12
CLANG13=clang-13
RUSTC=rustc
RUSTFLAGS=--crate-type=lib
LLVMAS8=$(LLVM8PATH)/bin/llvm-as
LLVMAS9=$(LLVM9PATH)/bin/llvm-as
LLVMAS10=$(LLVM10PATH)/bin/llvm-as
LLVMAS11=$(LLVM11PATH)/bin/llvm-as
LLVMAS12=$(LLVM12PATH)/bin/llvm-as
LLVMAS13=$(LLVM13PATH)/bin/llvm-as

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
	$(patsubst %,llvm11/%,$(COBJS)) \
	$(patsubst %,llvm12/%,$(COBJS)) \
	$(patsubst %,llvm13/%,$(COBJS)) \
	$(patsubst %,llvm8/%,$(LLOBJS)) \
	$(patsubst %,llvm9/%,$(LLOBJS)) \
	$(patsubst %,llvm10/%,$(LLOBJS)) \
	$(patsubst %,llvm11/%,$(LLOBJS)) \
	$(patsubst %,llvm12/%,$(LLOBJS)) \
	$(patsubst %,llvm13/%,$(LLOBJS)) \
	$(patsubst %,rust/%,$(RUSTOBJS)) \
	llvm11/float_types.bc llvm12/float_types.bc llvm13/float_types.bc \

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
llvm11/%.ll : %.c
	mkdir -p llvm11
	$(CLANG11) $(CFLAGS) -S -emit-llvm $^ -o $@
llvm12/%.ll : %.c
	mkdir -p llvm12
	$(CLANG12) $(CFLAGS) -S -emit-llvm $^ -o $@
llvm13/%.ll : %.c
	mkdir -p llvm13
	$(CLANG13) $(CFLAGS) -S -emit-llvm $^ -o $@

llvm8/%.bc : %.c
	mkdir -p llvm8
	$(CLANG8) $(CFLAGS) -c -emit-llvm $^ -o $@
llvm9/%.bc : %.c
	mkdir -p llvm9
	$(CLANG9) $(CFLAGS) -c -emit-llvm $^ -o $@
llvm10/%.bc : %.c
	mkdir -p llvm10
	$(CLANG10) $(CFLAGS) -c -emit-llvm $^ -o $@
llvm11/%.bc : %.c
	mkdir -p llvm11
	$(CLANG11) $(CFLAGS) -c -emit-llvm $^ -o $@
llvm12/%.bc : %.c
	mkdir -p llvm12
	$(CLANG12) $(CFLAGS) -c -emit-llvm $^ -o $@
llvm13/%.bc : %.c
	mkdir -p llvm13
	$(CLANG13) $(CFLAGS) -c -emit-llvm $^ -o $@

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
llvm11/%.ll-g : %.c
	mkdir -p llvm11
	$(CLANG11) $(CFLAGS) -g -S -emit-llvm $^ -o $@
llvm12/%.ll-g : %.c
	mkdir -p llvm12
	$(CLANG12) $(CFLAGS) -g -S -emit-llvm $^ -o $@
llvm13/%.ll-g : %.c
	mkdir -p llvm13
	$(CLANG13) $(CFLAGS) -g -S -emit-llvm $^ -o $@

llvm8/%.bc-g : %.c
	mkdir -p llvm8
	$(CLANG8) $(CFLAGS) -g -c -emit-llvm $^ -o $@
llvm9/%.bc-g : %.c
	mkdir -p llvm9
	$(CLANG9) $(CFLAGS) -g -c -emit-llvm $^ -o $@
llvm10/%.bc-g : %.c
	mkdir -p llvm10
	$(CLANG10) $(CFLAGS) -g -c -emit-llvm $^ -o $@
llvm11/%.bc-g : %.c
	mkdir -p llvm11
	$(CLANG11) $(CFLAGS) -g -c -emit-llvm $^ -o $@
llvm12/%.bc-g : %.c
	mkdir -p llvm12
	$(CLANG12) $(CFLAGS) -g -c -emit-llvm $^ -o $@
llvm13/%.bc-g : %.c
	mkdir -p llvm13
	$(CLANG13) $(CFLAGS) -g -c -emit-llvm $^ -o $@

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
llvm11/linkedlist.% : CFLAGS := -O0
llvm12/linkedlist.% : CFLAGS := -O0
llvm13/linkedlist.% : CFLAGS := -O0

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
llvm11/param_and_func_attributes.ll.bc : param_and_func_attributes.ll
	mkdir -p llvm11
	$(LLVMAS11) $< -o $@
# use the llvm12 version of the file for LLVM 12
llvm12/param_and_func_attributes.ll.bc : param_and_func_attributes_llvm12.ll
	mkdir -p llvm12
	$(LLVMAS12) $< -o $@
# use the llvm13+ version of the file for LLVM 13+
llvm13/param_and_func_attributes.ll.bc : param_and_func_attributes_llvm13+.ll
	mkdir -p llvm13
	$(LLVMAS13) $< -o $@

# assemble this one directly from .ll, and only for LLVM 11+
llvm11/float_types.bc : float_types.ll
	mkdir -p llvm11
	$(LLVMAS11) $< -o $@
llvm12/float_types.bc : float_types.ll
	mkdir -p llvm12
	$(LLVMAS12) $< -o $@
llvm13/float_types.bc : float_types.ll
	mkdir -p llvm13
	$(LLVMAS13) $< -o $@

.PHONY: clean
clean:
	-rm -rf llvm8 llvm9 llvm10 llvm11 llvm12 llvm13 rust
	-find . -name "*~" | xargs rm
