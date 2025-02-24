#![cfg(feature = "llvm-19-or-greater")]

//! Test that we can parse the copy of `compatibility.ll` in LLVM 19's `test/Bitcode` directory

use llvm_ir::{Constant, Module};
use std::path::Path;

macro_rules! llvm_test {
    ($path:expr, $func:ident) => {
        #[test]
        #[allow(non_snake_case)]
        fn $func() {
            let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
            let path = Path::new($path);
            let _ = Module::from_bc_path(&path).expect("Failed to parse module");
        }
    };
}

llvm_test!(
    "tests/llvm_bc/compatibility-as-of-llvm-19.bc",
    compatibility_llvm_19
);

#[test]
fn ptr_auth() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/compatibility-as-of-llvm-19.bc");
    let module = Module::from_bc_path(path).expect("Failed to parse module");

    let auth_var = module.global_vars.iter().find(|var| var.name == "auth_var".into())
        .expect("auth_var variable should exist");

    let auth_var_init = auth_var.initializer.clone().expect("auth_var should be initialised");

    if let Constant::PtrAuth { ptr, key, disc, addr_disc } = auth_var_init.as_ref() {
        assert_eq!(&format!("{ptr}"), "ptr @g1");
        assert_eq!(&format!("{key}"), "i32 0");
        assert_eq!(&format!("{disc}"), "i64 65535");
        assert_eq!(&format!("{addr_disc}"), "ptr null");
    }
    else {
        panic!("auth_var initialised with non-ptrauth constant {:?}", auth_var_init);
    }
}

// TODO: Other LLVM-19+ tests
