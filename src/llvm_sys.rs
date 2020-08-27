#[cfg(feature = "llvm-8")]
pub use llvm_sys_80 as llvm_sys;
#[cfg(feature = "llvm-9")]
pub use llvm_sys_90 as llvm_sys;
#[cfg(feature = "llvm-10")]
pub use llvm_sys_100 as llvm_sys;

pub use llvm_sys::core::*;
pub use llvm_sys::prelude::*;
