#![cfg(feature = "llvm-17-or-greater")]

//! Test that we can parse the copy of `compatibility.ll` in LLVM 17's `test/Bitcode` directory

use llvm_ir::Module;
use std::path::Path;

macro_rules! llvm_ll_test {
    ($path:expr, $func:ident) => {
        #[test]
        #[allow(non_snake_case)]
        fn $func() {
            let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
            let path = Path::new($path);
            let _ = Module::from_ir_path(&path).expect("Failed to parse module");
        }
    };
}

llvm_ll_test!(
    "tests/llvm_bc/compatibility-as-of-llvm-17.ll",
    compatibility_llvm_17
);
