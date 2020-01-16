# llvm-ir: LLVM IR in natural Rust data structures

[![Crates.io](http://meritbadge.herokuapp.com/llvm-ir)](https://crates.io/crates/llvm-ir)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/cdisselkoen/llvm-ir/master/LICENSE)

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
add it as a dependency in your `Cargo.toml`:
```toml
[dependencies]
llvm-ir = "0.4.1"
```

Then, the easiest way to get started is to parse some existing LLVM IR into
this crate's data structures.
To do this, you need LLVM bitcode (`*.bc`) files.
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
use std::path::Path;

let path = Path::new("path/to/my/file.bc");
let module = Module::from_bc_path(&path)?;
```

## Documentation
Documentation for `llvm-ir` can be found [here](https://cdisselkoen.github.io/llvm-ir),
or of course you can generate local documentation with `cargo doc --open`.
The documentation includes links to relevant parts of the LLVM documentation
when appropriate.

## Compatibility
Currently, `llvm-ir` only supports LLVM 9. However, it should probably "just
work" with LLVM 8 if you simply open the `llvm-ir` `Cargo.toml` and change
the line
```toml
llvm-sys = "90.0"
```
to
```toml
llvm-sys = "80.1"
```
then `cargo clean` and rebuild. LLVMs older than 8 are not supported.

`llvm-ir` works on stable Rust, and requires Rust 1.36+.

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

Additionally, you may want to ensure you are generating LLVM bitcode with
debuginfo; this will ensure that [`Instruction`]s, [`Terminator`]s,
[`GlobalVariable`]s, and [`Function`]s have valid [`DebugLoc`]s attached.
(See the [`HasDebugLoc`] trait.)
You can do this by passing the `-g` flag to `clang`, `clang++`, or `rustc`
when generating bitcode.

## Limitations
A few features of LLVM IR are not yet represented in `llvm-ir`'s data
structures.

Most notably, `llvm-ir` recovers debug-location metadata (for mapping back to
source locations), but makes no attempt to recover any other debug metadata.
LLVM files containing metadata can still be parsed in with no problems, but
the resulting `Module` structures will not contain any of the metadata,
except debug locations.
Work-in-progress on fixing this can be found on the `metadata` branch of this
repo, but be warned that the `metadata` branch doesn't even build at the time
of this writing, let alone provide any meaningful functionality for crate
users.

A few other features are missing from `llvm-ir`'s data structures because
getters for them are missing from the LLVM C API and the Rust `llvm-sys`
crate, only being present in the LLVM C++ API.
These include but are not limited to:

- the `nsw` and `nuw` flags on `Add`, `Sub`, `Mul`, and `Shl`, and likewise
the `exact` flag on `UDiv`, `SDiv`, `LShr`, and `AShr`. The C API has
functionality to set these flags and/or create new instructions specifying
values of these flags, but not to query the values of these flags on existing
instructions.
- the "fast-math flags" on various floating-point operations
- the specific opcode for the `AtomicRMW` instruction, i.e., `Xchg`, `Add`,
`Max`, `Min`, and the like. Again, the C API allows creating `AtomicRMW`
instructions with any of these opcodes, but has no way to get the opcode for
an existing `AtomicRMW` instruction.
- contents of inline assembly functions
- information about the clauses in the variadic `LandingPad` instruction
- information about the operands of a `BlockAddress` constant expression
- the "other labels" reachable from a `CallBr` terminator
- the ["prefix data"](https://releases.llvm.org/9.0.0/docs/LangRef.html#prefix-data)
associated with a function

These issues with the LLVM C API have also been reported as
[LLVM bug #42692](https://bugs.llvm.org/show_bug.cgi?id=42692).
As discussed there, the `AtomicRMW` opcode getters have been added in trunk
(and should be available in the LLVM 10 release), but the others remain open
problems.
Any contributions to filling these gaps in the C API are greatly appreciated!

## Acknowledgments
`llvm-ir` is heavily inspired by the [`llvm-hs-pure` Haskell package].
Most of the data structures in `llvm-ir` are essentially translations from
Haskell to Rust of the data structures in `llvm-hs-pure` (with some tweaks).
To a lesser extent, `llvm-ir` borrows from the larger [`llvm-hs` Haskell
package] as well.

[`llvm-sys`]: https://crates.io/crates/llvm-sys
[`inkwell`]: https://github.com/TheDan64/inkwell
[`llvm-hs-pure` Haskell package]: http://hackage.haskell.org/package/llvm-hs-pure
[`llvm-hs` Haskell package]: http://hackage.haskell.org/package/llvm-hs
[`Instruction`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/instruction/enum.Instruction.html
[`Add`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/instruction/struct.Add.html
[`Call`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/instruction/struct.Call.html
[`Store`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/instruction/struct.Store.html
[`BasicBlock`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/basicblock/struct.BasicBlock.html
[`Function`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/function/struct.Function.html
[`Module`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/module/struct.Module.html
[`Terminator`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/terminator/enum.Terminator.html
[`GlobalVariable`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/module/struct.GlobalVariable.html
[`DebugLoc`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/debugloc/struct.DebugLoc.html
[`HasDebugLoc`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/debugloc/trait.HasDebugLoc.html
