# llvm-ir: LLVM IR in natural Rust data structures

[![crates.io](http://img.shields.io/crates/v/llvm-ir.svg)](https://crates.io/crates/llvm-ir)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/cdisselkoen/llvm-ir/main/LICENSE)

`llvm-ir` seeks to provide a Rust-y representation of LLVM IR.
It's based on the idea that an LLVM [`Instruction`] shouldn't be an opaque
datatype, but rather an `enum` with variants like [`Add`], [`Call`], and
[`Store`].
Likewise, types like [`BasicBlock`], [`Function`], and [`Module`] should be
Rust structs containing as much information as possible.

Unlike other safe LLVM bindings such as [`inkwell`], `llvm-ir` does not rely
on continuous FFI to the LLVM API.
It uses the LLVM API only for its initial parsing step, to pull in all the
data it needs to construct its rich representation of LLVM IR.
Once `llvm-ir` creates a [`Module`] data structure by parsing an LLVM file
(using the excellent [`llvm-sys`] low-level LLVM bindings), it drops the LLVM
FFI objects and makes no further FFI calls.
This allows you to work with the resulting LLVM IR in pure safe Rust.

`llvm-ir` is intended for consumption of LLVM IR, and not necessarily
production of LLVM IR (yet).
That is, it is aimed at program analysis and related applications which want
to read and analyze LLVM IR.
In the future, perhaps `llvm-ir` could be able to output its `Module`s back
into LLVM files, or even send them directly to the LLVM library for compiling.
If this interests you, contributions are welcome!
(Or in the meantime, check out [`inkwell`] for a different safe interface for
producing LLVM IR.)
But if you're looking for a nice read-oriented representation of LLVM IR for
working in pure Rust, that's exactly what `llvm-ir` can provide today.

## Getting started
This crate is on [crates.io](https://crates.io/crates/llvm-ir), so you can simply
add it as a dependency in your `Cargo.toml`, selecting the feature corresponding
to the LLVM version you want:
```toml
[dependencies]
llvm-ir = { version = "0.11.0", features = ["llvm-17"] }
```

Currently, the supported LLVM versions are `llvm-9`, `llvm-10`, `llvm-11`,
`llvm-12`, `llvm-13`, `llvm-14`, `llvm-15`, `llvm-16`, and `llvm-17`.

Then, the easiest way to get started is to parse some existing LLVM IR into
this crate's data structures.
To do this, you need LLVM bitcode (`*.bc`) or text-format IR (`*.ll`) files.
If you currently have C/C++ sources (say, `source.c`), you can generate
`*.bc` files with `clang`'s `-c` and `-emit-llvm` flags:
```bash
clang -c -emit-llvm source.c -o source.bc
```

Alternately, to compile Rust sources to LLVM bitcode, you can use `rustc`'s
`--emit=llvm-bc` flag.

In either case, once you have a bitcode file, then you can use `llvm-ir`'s
`Module::from_bc_path` function:
```rust
use llvm_ir::Module;
let module = Module::from_bc_path("path/to/my/file.bc")?;
```
or if you have a text-format IR file, you can use `Module::from_ir_path()`.

You may also be interested in the [`llvm-ir-analysis`] crate, which computes
control-flow graphs, dominator trees, etc for `llvm-ir` functions.

[`llvm-ir-analysis`]: https://crates.io/crates/llvm-ir-analysis

## Documentation
Documentation for `llvm-ir` can be found [on docs.rs](https://docs.rs/llvm-ir),
or of course you can generate local documentation with `cargo doc --open`.
The documentation includes links to relevant parts of the LLVM documentation
when appropriate.

Note that some data structures differ slightly depending on your choice of
LLVM version. The docs.rs documentation is generated with the `llvm-10`
feature; for other LLVM versions, you can get appropriate documentation with
`cargo doc --features=llvm-<x> --open` where `<x>` is the LLVM version you're
using.

## Compatibility
Starting with `llvm-ir` 0.7.0, LLVM versions are selected by a Cargo feature
flag. This means that a single crate version can be used for any supported LLVM
version. Currently, `llvm-ir` supports LLVM versions 9 through 17, selected by
feature flags `llvm-9` through `llvm-17`.

You should select the LLVM version corresponding to the version of the LLVM
library you are linking against (i.e., that is available on your system.)
Newer LLVMs should be able to read bitcode produced by older LLVMs, so you
should be able to use this crate to parse bitcode older than the LLVM version
you select via crate feature, even bitcode produced by LLVMs older than LLVM 9.
However, this is not extensively tested by us.

`llvm-ir` works on stable Rust. As of this writing, it requires Rust 1.65+.

## Development/Debugging
For development or debugging, you may want LLVM text-format (`*.ll`) files in
addition to `*.bc` files.

For C/C++ sources, you can generate these by passing `-S -emit-llvm` to
`clang`, instead of `-c -emit-llvm`.
E.g.,
```bash
clang -S -emit-llvm source.c -o source.ll
```

For Rust sources, you can use `rustc`'s `--emit=llvm-ir` flag.

Additionally, you may want to pass the `-g` flag to `clang`, `clang++`, or
`rustc` when generating bitcode.
This will generate LLVM bitcode with debuginfo, which will ensure that
[`Instruction`]s, [`Terminator`]s, [`GlobalVariable`]s, and [`Function`]s
have valid [`DebugLoc`]s attached. (See the [`HasDebugLoc`] trait.)
Also note that these `DebugLoc`s are only available in LLVM 9 and newer;
previous versions of LLVM had a bug in this interface in the C API which
would cause segfaults.

## Limitations
A few features of LLVM IR are not yet represented in `llvm-ir`'s data
structures.

Most notably, `llvm-ir` recovers debug-location metadata (for mapping back to
source locations), but makes no attempt to recover any other debug metadata.
LLVM files containing metadata can still be parsed in with no problems, but
the resulting `Module` structures will not contain any of the metadata,
except debug locations.

A few other features are missing from `llvm-ir`'s data structures because
getters for them are missing from the LLVM C API and the Rust `llvm-sys`
crate, only being present in the LLVM C++ API.
These include but are not limited to:

- the "fast-math flags" on various floating-point operations
- contents of inline assembly functions
- information about the clauses in the variadic `LandingPad` instruction
- information about the operands of a `BlockAddress` constant expression
- information about `TargetExtType` types
- the ["prefix data"](https://releases.llvm.org/16.0.0/docs/LangRef.html#prefix-data)
associated with a function
- the values of constant integers which are larger than 64 bits (and don't
fit in 64 bits) -- see [#5](https://github.com/cdisselkoen/llvm-ir/issues/5)
- the "other labels" reachable from a `CallBr` terminator (which was
introduced in LLVM 9)
- (LLVM 16 and lower -- fixed in LLVM 17 and later) the `nsw` and `nuw` flags on
`Add`, `Sub`, `Mul`, and `Shl`, and likewise the `exact` flag on `UDiv`, `SDiv`,
`LShr`, and `AShr`. The C API has functionality to create new instructions
specifying values of these flags, but not to query the values of these flags on
existing instructions.
- (LLVM 9 and lower -- fixed in LLVM 10 and later) the opcode for the
`AtomicRMW` instruction, i.e., `Xchg`, `Add`, `Max`, `Min`, and the like.

More discussion about this is in
[LLVM bug #42692](https://bugs.llvm.org/show_bug.cgi?id=42692).
Any contributions to filling these gaps in the C API are greatly appreciated!

## Acknowledgments
`llvm-ir` took its original inspiration from the [`llvm-hs-pure` Haskell package].
Most of the data structures in the original release of `llvm-ir` were
essentially translations from Haskell to Rust of the data structures in
`llvm-hs-pure` (with some tweaks).

## Changelog for 0.7.0

`llvm-ir` 0.7.0 includes several fairly major changes from previous
versions, which are outlined here.

- LLVM versions are now selected via Cargo features. You must select exactly
one of the features `llvm-8`, `llvm-9`, or `llvm-10`. Previously, we had the
`0.6.x` branch for LLVM 10, the `0.5.x` branch for LLVM 9, and didn't
officially support LLVM 8. Now, a single release supports LLVM 8, 9, and 10.
  - (Note: Versions of this crate beyond 0.7.0 have added support for later LLVM
  versions as well. For instance, 0.7.3 and later also support LLVM 11; and
  0.7.5 and later also support LLVM 12. Crate version 0.11.0 removed support
  for LLVM 8.)
- [`FunctionAttribute`] and [`ParameterAttribute`] are now proper enums with
descriptive variants such as `NoInline`, `StackProtect`, etc. Previously,
attributes were opaque numeric codes which were difficult to interpret.
- Several changes to improve runtime performance and especially memory
consumption, particularly when parsing large LLVM modules. This involves a
number of breaking changes to the public interface:
  - Most users of [`Type`] now own a [`TypeRef`] rather than a `Type` directly.
  This includes `Operand::LocalOperand`, `GlobalVariable`, many variants of
  `Instruction`, many variants of `Constant`, and some variants of `Type`
  itself, among others. See the documentation on [`TypeRef`].
  - Similarly, most users of [`Constant`] now own a [`ConstantRef`] rather
  than a `Constant` directly. See the documentation on [`ConstantRef`].
  - To get the type of [`Typed`] objects, the provided `.get_type()` method
  now requires an additional argument; most users will probably prefer
  [`module.type_of()`] (or `module.types.type_of()`).
  - `Type::NamedStructType` no longer carries a weak reference to its inner
  type; instead, you can look up the name using
  [`module.types.named_struct_def()`] to get the definition for any named
  struct type in the module.
- The required Rust version increased from 1.36+ to 1.39+.
  - (Note: Versions of this crate beyond 0.7.0 have increased this requirement
  further.  For the current required Rust version, see "Compatibility" above.)

[`llvm-sys`]: https://crates.io/crates/llvm-sys
[`inkwell`]: https://github.com/TheDan64/inkwell
[`llvm-hs-pure` Haskell package]: http://hackage.haskell.org/package/llvm-hs-pure
[`llvm-hs` Haskell package]: http://hackage.haskell.org/package/llvm-hs
[`Instruction`]: https://docs.rs/llvm-ir/latest/llvm_ir/instruction/enum.Instruction.html
[`Add`]: https://docs.rs/llvm-ir/latest/llvm_ir/instruction/struct.Add.html
[`Call`]: https://docs.rs/llvm-ir/latest/llvm_ir/instruction/struct.Call.html
[`Store`]: https://docs.rs/llvm-ir/latest/llvm_ir/instruction/struct.Store.html
[`BasicBlock`]: https://docs.rs/llvm-ir/latest/llvm_ir/basicblock/struct.BasicBlock.html
[`Function`]: https://docs.rs/llvm-ir/latest/llvm_ir/function/struct.Function.html
[`Module`]: https://docs.rs/llvm-ir/latest/llvm_ir/module/struct.Module.html
[`Terminator`]: https://docs.rs/llvm-ir/latest/llvm_ir/terminator/enum.Terminator.html
[`GlobalVariable`]: https://docs.rs/llvm-ir/latest/llvm_ir/module/struct.GlobalVariable.html
[`DebugLoc`]: https://docs.rs/llvm-ir/latest/llvm_ir/debugloc/struct.DebugLoc.html
[`HasDebugLoc`]: https://docs.rs/llvm-ir/latest/llvm_ir/debugloc/trait.HasDebugLoc.html
[`FunctionAttribute`]: https://docs.rs/llvm-ir/latest/llvm_ir/function/enum.FunctionAttribute.html
[`ParameterAttribute`]: https://docs.rs/llvm-ir/latest/llvm_ir/function/enum.ParameterAttribute.html
[`Type`]: https://docs.rs/llvm-ir/latest/llvm_ir/types/enum.Type.html
[`TypeRef`]: https://docs.rs/llvm-ir/latest/llvm_ir/types/struct.TypeRef.html
[`Typed`]: https://docs.rs/llvm-ir/latest/llvm_ir/types/struct.TypeRef.html
[`Constant`]: https://docs.rs/llvm-ir/latest/llvm_ir/constant/enum.Constant.html
[`ConstantRef`]: https://docs.rs/llvm-ir/latest/llvm_ir/constant/struct.ConstantRef.html
[`module.type_of()`]: https://docs.rs/llvm-ir/latest/llvm_ir/module/struct.Module.html#method.type_of
[`module.types.named_struct_def()`]: https://docs.rs/llvm-ir/latest/llvm_ir/types/struct.Types.html#method.named_struct_def
