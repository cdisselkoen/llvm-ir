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

#[test]
fn disjoint_or() {
    use llvm_ir::instruction;
    use std::convert::TryInto;
    
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/compatibility-as-of-llvm-18.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    let func = module
        .get_func_by_name("instructions.bitwise_binops")
        .expect("Failed to find function");
    let bb = func.basic_blocks
        .get(0)
        .expect("expected function to have a basic block");

    let non_disjoint_or: &instruction::Or = &bb
        .instrs[9]
        .clone()
        .try_into()
        .expect("Expected an or");

    let disjoint_or: &instruction::Or = &bb
        .instrs[11]
        .clone()
        .try_into()
        .expect("Expected an or");
        
    assert_eq!(non_disjoint_or.disjoint, false);
    assert_eq!(disjoint_or.disjoint, true);
}

/*
llvm_18_nneg.bc:

define void @test_nneg(i32 %val) {
entry:
  %0 = zext i32 %val to i64
  %1 = zext nneg i32 %val to i64
  ret void
}
*/

#[test]
fn nneg_zext() {
    use llvm_ir::instruction;
    use std::convert::TryInto;
    
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/llvm_18_nneg.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    let func = module
        .get_func_by_name("test_nneg")
        .expect("Failed to find function");
    let bb = func.basic_blocks
        .get(0)
        .expect("expected function to have a basic block");

    let non_nneg_zext: &instruction::ZExt = &bb
        .instrs[0]
        .clone()
        .try_into()
        .expect("Expected a zext");

    let nneg_zext: &instruction::ZExt = &bb
        .instrs[1]
        .clone()
        .try_into()
        .expect("Expected a zext");
        
    assert_eq!(non_nneg_zext.nneg, false);
    assert_eq!(nneg_zext.nneg, true);
}

llvm_test!(
    "tests/llvm_bc/compatibility-as-of-llvm-18.bc",
    compatibility_llvm_18
);

// TODO: Other LLVM-18+ tests
