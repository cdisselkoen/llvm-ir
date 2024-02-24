//! These tests simply ensure that we can parse all of the `.bc` files in LLVM 9's `test/Bitcode` directory without crashing.
//! We only include the `.bc` files which are new or have changed since LLVM 8 (older ones are covered in llvm_8_tests.rs).
//! Human-readable `.ll` versions of these files can be found in the LLVM repo at `test/Bitcode` on the git branch `release/9.x`.

use llvm_ir::Module;
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

llvm_test!("tests/llvm_bc/DISubprogram-v5.ll.bc", DISubprogram_v5);
llvm_test!("tests/llvm_bc/callbr.ll.bc", callbr);
//llvm_test!("tests/llvm_bc/invalid.ll.bc", invalid);  // we omit this .bc file because it is intentionally invalid.  Note that it existed in LLVM 8 but has changed; the checked-in version as of this writing is the LLVM 8 version
llvm_test!(
    "tests/llvm_bc/objectsize-upgrade-7.0.ll.bc",
    objectsize_upgrade_7_0
);
//llvm_test!("tests/llvm_bc/upgrade-clang-arc-use.ll.bc", upgrade_clang_arc_use);  // this file is weird because it includes an opaque struct type which is numbered, not named.  This shouldn't ever occur in real code (?), so we omit this test case
llvm_test!(
    "tests/llvm_bc/upgrade-global-dtors.ll.bc",
    upgrade_global_dtors
);
llvm_test!(
    "tests/llvm_bc/upgrade-vecreduce-intrinsics.ll.bc",
    upgrade_vecreduce_intrinsics
);

// also ensure that certain new-to-llvm-9 constructs were parsed correctly
use llvm_ir::{terminator, Name};
use std::convert::TryInto;

#[test]
fn callbr_parsing() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/callbr.ll.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    let func = module
        .get_func_by_name("test_asm_goto")
        .expect("Failed to find function");
    let bb = func
        .get_bb_by_name(&Name::from("entry"))
        .expect("Failed to find entry bb");
    let callbr: &terminator::CallBr = &bb
        .term
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected a callbr, got {:?}", &bb.term));
    assert!(callbr.function.is_left());
    assert_eq!(callbr.return_label, Name::from("normal"));
    #[cfg(feature = "llvm-14-or-lower")]
    let expected_fmt = "%0 = callbr <inline assembly>(i32 %x, blockaddr) to label %normal";
    #[cfg(feature = "llvm-15-or-greater")]
    let expected_fmt = "%0 = callbr <inline assembly>(i32 %x) to label %normal";
    assert_eq!(&format!("{}", callbr), expected_fmt);
}
