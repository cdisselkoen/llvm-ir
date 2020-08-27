// this ensures that crate users generating docs with --no-deps will still
// properly get links to the public docs for llvm-ir's types
// it was especially necessary when the docs.rs docs weren't working for any
// llvm-sys consumers; now that we have docs.rs as the official docs, I'm not
// sure if this is necessary or helpful anymore
#![doc(html_root_url = "https://docs.rs/llvm-ir/0.7.0")]

#[macro_use]
mod from_llvm;
mod iterators;
mod llvm_sys;

pub mod basicblock;
pub use basicblock::BasicBlock;
pub mod constant;
pub use constant::{Constant, ConstantRef};
#[cfg(LLVM_VERSION_9_OR_GREATER)]
pub mod debugloc;
#[cfg(LLVM_VERSION_9_OR_GREATER)]
pub use debugloc::{DebugLoc, HasDebugLoc};
pub mod function;
pub use function::Function;
pub mod instruction;
pub use instruction::Instruction;
// pub mod metadata;
// pub use metadata::Metadata;
pub mod module;
pub use module::Module;
pub mod name;
pub use name::Name;
pub mod operand;
pub use operand::Operand;
pub mod predicates;
pub use predicates::{FPPredicate, IntPredicate};
pub mod terminator;
pub use terminator::Terminator;
pub mod types;
pub use types::{Type, TypeRef};
