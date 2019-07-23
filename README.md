# llvm-ir: LLVM IR in natural Rust data structures

[![Crates.io](http://meritbadge.herokuapp.com/llvm-ir)](https://crates.io/crates/llvm-ir)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/cdisselkoen/llvm-ir/master/LICENSE)

`llvm-ir` is a crate which seeks to provide a more Rust-y representation of
LLVM IR than crates like [`llvm-sys`] or [`inkwell`] which rely on continuous
FFI to the LLVM API.
It's based on the idea that an LLVM [`Instruction`] shouldn't be an opaque
datatype (which is under-the-hood just an FFI pointer), but rather an `enum`
with variants like [`Add`], [`Call`], and [`Store`].
Likewise, types like [`BasicBlock`], [`Function`], and [`Module`] should be
Rust structs with as much information as possible contained in them.

`llvm-ir` is intended for consumption of LLVM IR, and not necessarily
production of LLVM IR (yet).
That is, it is aimed at program analysis and related applications which want
to read and compute on LLVM IR; it's currently not set up to output its
`Module`s back into LLVM files, or to send them off to the LLVM library for
compiling.
If producing LLVM IR with `llvm-ir` interests you, contributions are welcome!
But if what you want is to have a nice read-oriented representation of LLVM
IR for working in pure Rust, that's exactly what `llvm-ir` can (attempt to)
provide today.

Currently, `llvm-ir` does rely on FFI to the LLVM API via `llvm-sys`, but
only for its initial parsing step.
Once a given LLVM file has been parsed in to create a [`Module`] data
structure, `llvm-ir` drops the LLVM FFI objects and makes no further FFI
calls.
This allows you to work with the resulting LLVM IR in pure Rust.

## Getting started
The easiest way to get started is to parse some existing LLVM IR into this
crate's data structures.
To do this, you need LLVM bitcode (`*.bc`) files; if you currently have C/C++
sources (say, `source.c`), you can generate `*.bc` files by running
```bash
clang -S -emit-llvm source.c -o source.ll
llvm-as source.ll -o source.bc
```
Then, in Rust, you can use `llvm-ir`'s `Module::from_bc_path` function:
```rust
use llvm_ir::Module;
use std::path::Path;

let path = Path::new("path/to/my/file.bc");
module = Module::from_bc_path(&path)?;
```

## Documentation
Documentation for `llvm-ir` can be found [here](https://cdisselkoen.github.io/llvm-ir),
or of course you can generate local documentation with `cargo doc --open`.
The documentation includes links to relevant parts of the LLVM documentation
when appropriate.

## Compatibility
Currently, `llvm-ir` only supports LLVM 8. Unfortunately, I'm only one
person. Again, contributions are welcome.

`llvm-ir` works on stable Rust, and requires Rust 1.36+.

## Limitations
A few features of LLVM IR are not yet represented in `llvm-ir`'s data
structures, most notably debug information (metadata), which `llvm-ir`
currently makes no attempt to recover.
LLVM files containing metadata can still be parsed in with no problems, but
the resulting `Module` structures will not contain any of the metadata.
Work-in-progress on fixing this can be found on the `metadata` branch of this
repo, but be warned that the `metadata` branch doesn't even build at the time
of this writing, let alone provide any meaningful functionality for crate
users.

A few other features are missing from `llvm-ir`'s data structures because
getters for them are apparently (to my knowledge) missing from the LLVM C API
and the Rust `llvm-sys` crate, only being present in the LLVM C++ API.
These include but are not limited to:

- the `nsw` and `nuw` flags on `Add`, `Sub`, `Mul`, and `Shl`, and likewise
the `exact` flag on `UDiv`, `SDiv`, `LShr`, and `AShr`. The C API has
functionality to set these flags and/or create new instructions specifying
values of these flags, but not (apparently, to my knowledge) to query the
values of these flags on existing instructions.
- the "fast-math flags" on various floating-point operations
- the specific opcode for the `AtomicRMW` instruction, i.e., `Xchg`, `Add`,
`Max`, `Min`, and the like. Again, the C API allows creating `AtomicRMW`
instructions with any of these opcodes, but it's unclear how to get the
opcode for an existing `AtomicRMW` instruction.
- contents of inline assembly functions
- information about the clauses in the variadic `LandingPad` instruction
- information about the operands of a `BlockAddress` constant expression
- the ["prefix data"](https://releases.llvm.org/8.0.0/docs/LangRef.html#prefix-data)
associated with a function

Work is in progress on rectifying at least some of these limitations.

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
[`BasicBlock`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/struct.BasicBlock.html
[`Function`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/struct.Function.html
[`Module`]: https://cdisselkoen.github.io/llvm-ir/llvm_ir/struct.Module.html
