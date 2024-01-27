CLANG8=clang-8
CLANG9=clang-9
CLANG10=clang-10 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk
CLANG11=clang-11 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk
CLANG12=clang-12
CLANG13=clang-13
CLANG14=clang-14
CLANG15=clang-15
CLANG16=$(LLVM16PATH)/bin/clang
CLANG17=$(LLVM17PATH)/bin/clang
CLANGXX8=clang++-8
CLANGXX9=clang++-9
CLANGXX10=clang++-10 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk
CLANGXX11=clang++-11 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk
CLANGXX12=clang++-12
CLANGXX13=clang++-13
CLANGXX14=clang++-14
CLANGXX15=$(LLVM15PATH)/bin/clang++
CLANGXX16=$(LLVM16PATH)/bin/clang++
CLANGXX17=$(LLVM17PATH)/bin/clang++
RUSTC=rustc
RUSTFLAGS=--crate-type=lib
LLVMAS8=$(LLVM8PATH)/bin/llvm-as
LLVMAS9=$(LLVM9PATH)/bin/llvm-as
LLVMAS10=$(LLVM10PATH)/bin/llvm-as
LLVMAS11=$(LLVM11PATH)/bin/llvm-as
LLVMAS12=$(LLVM12PATH)/bin/llvm-as
LLVMAS13=$(LLVM13PATH)/bin/llvm-as
LLVMAS14=$(LLVM14PATH)/bin/llvm-as
LLVMAS15=$(LLVM15PATH)/bin/llvm-as
LLVMAS16=$(LLVM16PATH)/bin/llvm-as
LLVMAS17=$(LLVM17PATH)/bin/llvm-as

COBJS = \
	hello.bc hello.ll hello.bc-g hello.ll-g \
	loop.bc loop.ll \
	switch.bc switch.ll \
	linkedlist.bc linkedlist.ll linkedlist.bc-g linkedlist.ll-g \
	variables.bc variables.ll variables.bc-g variables.ll-g \
	issue_4.bc issue_4.ll \

CXXOBJS = throw.bc throw.ll \

RUSTOBJS = rust.bc rust.ll rust.bc-g rust.ll-g \

LLOBJS = param_and_func_attributes.ll.bc \
	fences.ll.bc \

.PHONY: all
all: \
	cxx \
	$(patsubst %,llvm8/%,$(COBJS)) \
	$(patsubst %,llvm9/%,$(COBJS)) \
	$(patsubst %,llvm10/%,$(COBJS)) \
	$(patsubst %,llvm11/%,$(COBJS)) \
	$(patsubst %,llvm12/%,$(COBJS)) \
	$(patsubst %,llvm13/%,$(COBJS)) \
	$(patsubst %,llvm14/%,$(COBJS)) \
	$(patsubst %,llvm15/%,$(COBJS)) \
	$(patsubst %,llvm16/%,$(COBJS)) \
	$(patsubst %,llvm17/%,$(COBJS)) \
	$(patsubst %,llvm8/%,$(LLOBJS)) \
	$(patsubst %,llvm9/%,$(LLOBJS)) \
	$(patsubst %,llvm10/%,$(LLOBJS)) \
	$(patsubst %,llvm11/%,$(LLOBJS)) \
	$(patsubst %,llvm12/%,$(LLOBJS)) \
	$(patsubst %,llvm13/%,$(LLOBJS)) \
	$(patsubst %,llvm14/%,$(LLOBJS)) \
	$(patsubst %,llvm15/%,$(LLOBJS)) \
	$(patsubst %,llvm16/%,$(LLOBJS)) \
	$(patsubst %,llvm17/%,$(LLOBJS)) \
	$(patsubst %,rust/%,$(RUSTOBJS)) \
	llvm11/float_types.bc llvm12/float_types.bc llvm13/float_types.bc \
	llvm14/float_types.bc llvm15/float_types.bc llvm16/float_types.bc \
	llvm17/float_types.bc \

.PHONY: cxx
cxx: \
	$(patsubst %,cxx-llvm8/%,$(CXXOBJS)) \
	$(patsubst %,cxx-llvm9/%,$(CXXOBJS)) \
	$(patsubst %,cxx-llvm10/%,$(CXXOBJS)) \
	$(patsubst %,cxx-llvm11/%,$(CXXOBJS)) \
	$(patsubst %,cxx-llvm12/%,$(CXXOBJS)) \
	$(patsubst %,cxx-llvm13/%,$(CXXOBJS)) \
	$(patsubst %,cxx-llvm14/%,$(CXXOBJS)) \
	$(patsubst %,cxx-llvm15/%,$(CXXOBJS)) \
	$(patsubst %,cxx-llvm16/%,$(CXXOBJS)) \
	$(patsubst %,cxx-llvm17/%,$(CXXOBJS)) \

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
llvm14/%.ll : %.c
	mkdir -p llvm14
	$(CLANG14) $(CFLAGS) -S -emit-llvm $^ -o $@
llvm15/%.ll : %.c
	mkdir -p llvm15
	$(CLANG15) $(CFLAGS) -S -emit-llvm $^ -o $@
llvm16/%.ll : %.c
	mkdir -p llvm16
	$(CLANG16) $(CFLAGS) -S -emit-llvm $^ -o $@
llvm17/%.ll : %.c
	mkdir -p llvm17
	$(CLANG17) $(CFLAGS) -S -emit-llvm $^ -o $@


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
llvm14/%.bc : %.c
	mkdir -p llvm14
	$(CLANG14) $(CFLAGS) -c -emit-llvm $^ -o $@
llvm15/%.bc : %.c
	mkdir -p llvm15
	$(CLANG15) $(CFLAGS) -c -emit-llvm $^ -o $@
llvm16/%.bc : %.c
	mkdir -p llvm16
	$(CLANG16) $(CFLAGS) -c -emit-llvm $^ -o $@
llvm17/%.bc : %.c
	mkdir -p llvm17
	$(CLANG17) $(CFLAGS) -c -emit-llvm $^ -o $@

cxx-llvm8/%.ll : %.cpp
	mkdir -p cxx-llvm8
	$(CLANGXX8) $(CFLAGS) -S -emit-llvm $^ -o $@
cxx-llvm9/%.ll : %.cpp
	mkdir -p cxx-llvm9
	$(CLANGXX9) $(CFLAGS) -S -emit-llvm $^ -o $@
cxx-llvm10/%.ll : %.cpp
	mkdir -p cxx-llvm10
	$(CLANGXX10) $(CFLAGS) -S -emit-llvm $^ -o $@
cxx-llvm11/%.ll : %.cpp
	mkdir -p cxx-llvm11
	$(CLANGXX11) $(CFLAGS) -S -emit-llvm $^ -o $@
cxx-llvm12/%.ll : %.cpp
	mkdir -p cxx-llvm12
	$(CLANGXX12) $(CFLAGS) -S -emit-llvm $^ -o $@
cxx-llvm13/%.ll : %.cpp
	mkdir -p cxx-llvm13
	$(CLANGXX13) $(CFLAGS) -S -emit-llvm $^ -o $@
cxx-llvm14/%.ll : %.cpp
	mkdir -p cxx-llvm14
	$(CLANGXX14) $(CFLAGS) -S -emit-llvm $^ -o $@
cxx-llvm15/%.ll : %.cpp
	mkdir -p cxx-llvm15
	$(CLANGXX15) $(CFLAGS) -S -emit-llvm $^ -o $@
cxx-llvm16/%.ll : %.cpp
	mkdir -p cxx-llvm16
	$(CLANGXX16) $(CFLAGS) -S -emit-llvm $^ -o $@
cxx-llvm17/%.ll : %.cpp
	mkdir -p cxx-llvm17
	$(CLANGXX17) $(CFLAGS) -S -emit-llvm $^ -o $@

cxx-llvm8/%.bc : %.cpp
	mkdir -p cxx-llvm8
	$(CLANGXX8) $(CFLAGS) -c -emit-llvm $^ -o $@
cxx-llvm9/%.bc : %.cpp
	mkdir -p cxx-llvm9
	$(CLANGXX9) $(CFLAGS) -c -emit-llvm $^ -o $@
cxx-llvm10/%.bc : %.cpp
	mkdir -p cxx-llvm10
	$(CLANGXX10) $(CFLAGS) -c -emit-llvm $^ -o $@
cxx-llvm11/%.bc : %.cpp
	mkdir -p cxx-llvm11
	$(CLANGXX11) $(CFLAGS) -c -emit-llvm $^ -o $@
cxx-llvm12/%.bc : %.cpp
	mkdir -p cxx-llvm12
	$(CLANGXX12) $(CFLAGS) -c -emit-llvm $^ -o $@
cxx-llvm13/%.bc : %.cpp
	mkdir -p cxx-llvm13
	$(CLANGXX13) $(CFLAGS) -c -emit-llvm $^ -o $@
cxx-llvm14/%.bc : %.cpp
	mkdir -p cxx-llvm14
	$(CLANGXX14) $(CFLAGS) -c -emit-llvm $^ -o $@
cxx-llvm15/%.bc : %.cpp
	mkdir -p cxx-llvm15
	$(CLANGXX15) $(CFLAGS) -c -emit-llvm $^ -o $@
cxx-llvm16/%.bc : %.cpp
	mkdir -p cxx-llvm16
	$(CLANGXX16) $(CFLAGS) -c -emit-llvm $^ -o $@
cxx-llvm17/%.bc : %.cpp
	mkdir -p cxx-llvm17
	$(CLANGXX17) $(CFLAGS) -c -emit-llvm $^ -o $@

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
llvm14/%.ll-g : %.c
	mkdir -p llvm14
	$(CLANG14) $(CFLAGS) -g -S -emit-llvm $^ -o $@
llvm15/%.ll-g : %.c
	mkdir -p llvm15
	$(CLANG15) $(CFLAGS) -g -S -emit-llvm $^ -o $@
llvm16/%.ll-g : %.c
	mkdir -p llvm16
	$(CLANG16) $(CFLAGS) -g -S -emit-llvm $^ -o $@
llvm17/%.ll-g : %.c
	mkdir -p llvm17
	$(CLANG17) $(CFLAGS) -g -S -emit-llvm $^ -o $@

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
llvm14/%.bc-g : %.c
	mkdir -p llvm14
	$(CLANG14) $(CFLAGS) -g -c -emit-llvm $^ -o $@
llvm15/%.bc-g : %.c
	mkdir -p llvm15
	$(CLANG15) $(CFLAGS) -g -c -emit-llvm $^ -o $@
llvm16/%.bc-g : %.c
	mkdir -p llvm16
	$(CLANG16) $(CFLAGS) -g -c -emit-llvm $^ -o $@
llvm17/%.bc-g : %.c
	mkdir -p llvm17
	$(CLANG17) $(CFLAGS) -g -c -emit-llvm $^ -o $@

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
llvm14/linkedlist.% : CFLAGS := -O0
llvm15/linkedlist.% : CFLAGS := -O0
llvm16/linkedlist.% : CFLAGS := -O0
llvm17/linkedlist.% : CFLAGS := -O0

# assemble this one directly from .ll
llvm8/fences.ll.bc : fences.ll
	mkdir -p llvm8
	$(LLVMAS8) $< -o $@
llvm9/fences.ll.bc : fences.ll
	mkdir -p llvm9
	$(LLVMAS9) $< -o $@
llvm10/fences.ll.bc : fences.ll
	mkdir -p llvm10
	$(LLVMAS10) $< -o $@
llvm11/fences.ll.bc : fences.ll
	mkdir -p llvm11
	$(LLVMAS11) $< -o $@
llvm12/fences.ll.bc : fences.ll
	mkdir -p llvm12
	$(LLVMAS12) $< -o $@
llvm13/fences.ll.bc : fences.ll
	mkdir -p llvm13
	$(LLVMAS13) $< -o $@
llvm14/fences.ll.bc : fences.ll
	mkdir -p llvm14
	$(LLVMAS14) $< -o $@
llvm15/fences.ll.bc : fences.ll
	mkdir -p llvm15
	$(LLVMAS15) $< -o $@
llvm16/fences.ll.bc : fences.ll
	mkdir -p llvm16
	$(LLVMAS16) $< -o $@
llvm17/fences.ll.bc : fences.ll
	mkdir -p llvm17
	$(LLVMAS17) $< -o $@

# assemble this one directly from .ll as well
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
# use the llvm13+ version of the file for LLVM 13-15
llvm13/param_and_func_attributes.ll.bc : param_and_func_attributes_llvm13+.ll
	mkdir -p llvm13
	$(LLVMAS13) $< -o $@
llvm14/param_and_func_attributes.ll.bc : param_and_func_attributes_llvm13+.ll
	mkdir -p llvm14
	$(LLVMAS14) $< -o $@
llvm15/param_and_func_attributes.ll.bc : param_and_func_attributes_llvm13+.ll
	mkdir -p llvm15
	$(LLVMAS15) $< -o $@
# use the llvm16+ version of the file for LLVM 16+
llvm16/param_and_func_attributes.ll.bc : param_and_func_attributes_llvm16+.ll
	mkdir -p llvm16
	$(LLVMAS16) $< -o $@
llvm17/param_and_func_attributes.ll.bc : param_and_func_attributes_llvm16+.ll
	mkdir -p llvm17
	$(LLVMAS17) $< -o $@

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
llvm14/float_types.bc : float_types.ll
	mkdir -p llvm14
	$(LLVMAS14) $< -o $@
llvm15/float_types.bc : float_types.ll
	mkdir -p llvm15
	$(LLVMAS15) $< -o $@
llvm16/float_types.bc : float_types.ll
	mkdir -p llvm16
	$(LLVMAS16) $< -o $@
llvm17/float_types.bc : float_types.ll
	mkdir -p llvm17
	$(LLVMAS17) $< -o $@

.PHONY: clean
clean:
	-rm -rf llvm8 llvm9 llvm10 llvm11 llvm12 llvm13 llvm14 llvm15 llvm16 rust
	-find . -name "*~" | xargs rm
