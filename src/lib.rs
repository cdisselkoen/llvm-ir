// this ensures that crate users generating docs with --no-deps will still
// properly get links to the public docs for llvm-ir's types
#![doc(html_root_url = "https://cdisselkoen.github.io/llvm-ir")]

#[macro_use]
mod from_llvm;
mod iterators;

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
