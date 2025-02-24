#![cfg(feature = "llvm-18-or-greater")]

//! Test that we can parse the copy of `compatibility.ll` in LLVM 18's `test/Bitcode` directory

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

#[test]
fn inline_assembly() {
    let _ = env_logger::builder().is_test(true).try_init();
    let path = Path::new("tests/llvm_bc/nop-inlineasm.ll.bc");
    let module = Module::from_bc_path(path).expect("Failed to parse module");
    let inst = &module.functions[0].basic_blocks[0].instrs[0];
    if let llvm_ir::Instruction::Call(call) = inst {
        assert!(call.function.is_left());
        let inline = call.function.clone().left().unwrap();
        assert_eq!(inline.assembly, "nop;");
        assert_eq!(inline.dialect, llvm_ir::instruction::AssemblyDialect::ATT);
        assert!(inline.has_side_effects);
        assert!(!inline.align_stack);
        assert_eq!(inline.constraints, "");
    } else {
        panic!("Expected InlineAsm instruction, got {:?}", inst);
    }
}

llvm_test!(
    "tests/llvm_bc/compatibility-as-of-llvm-18.bc",
    compatibility_llvm_18
);

// TODO: Other LLVM-18+ tests
