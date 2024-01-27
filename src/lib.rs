// this ensures that crate users generating docs with --no-deps will still
// properly get links to the public docs for llvm-ir's types
// it was especially necessary when the docs.rs docs weren't working for any
// llvm-sys consumers; now that we have docs.rs as the official docs, I'm not
// sure if this is necessary or helpful anymore
#![doc(html_root_url = "https://docs.rs/llvm-ir/0.8.2")]

#[macro_use]
mod from_llvm;
mod iterators;
#[rustfmt::skip]
mod llvm_sys;

pub mod basicblock;
pub use basicblock::BasicBlock;
pub mod constant;
pub use constant::{Constant, ConstantRef};
pub mod debugloc;
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

macro_rules! case {
    ($feature:expr) => {
        if cfg!(feature = $feature) {
            return $feature.strip_prefix("llvm-").unwrap();
        }
    };
}

/// Returns the LLVM version for which `llvm-ir` was configured.
pub fn llvm_version() -> &'static str {
    case!("llvm-9");
    case!("llvm-10");
    case!("llvm-11");
    case!("llvm-12");
    case!("llvm-13");
    case!("llvm-14");
    case!("llvm-15");
    case!("llvm-16");
    case!("llvm-17");
    unreachable!()
}
