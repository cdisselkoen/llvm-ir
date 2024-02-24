#[cfg(feature = "llvm-10")]
pub use llvm_sys_100 as llvm_sys;
#[cfg(feature = "llvm-11")]
pub use llvm_sys_110 as llvm_sys;
#[cfg(feature = "llvm-12")]
pub use llvm_sys_120 as llvm_sys;
#[cfg(feature = "llvm-13")]
pub use llvm_sys_130 as llvm_sys;
#[cfg(feature = "llvm-14")]
pub use llvm_sys_140 as llvm_sys;
#[cfg(feature = "llvm-15")]
pub use llvm_sys_150 as llvm_sys;
#[cfg(feature = "llvm-16")]
pub use llvm_sys_160 as llvm_sys;
#[cfg(feature = "llvm-17")]
pub use llvm_sys_170 as llvm_sys;
#[cfg(feature = "llvm-9")]
pub use llvm_sys_90 as llvm_sys;

pub use llvm_sys::core::*;
pub use llvm_sys::prelude::*;
