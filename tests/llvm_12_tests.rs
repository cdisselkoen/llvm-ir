#![cfg(feature = "llvm-12-or-greater")]

//! These tests simply ensure that we can parse all of the `.bc` files in LLVM 12's `test/Bitcode` directory without crashing.
//! We only include the `.bc` files which are new or have changed since LLVM 11 (older ones are covered in other llvm_*_tests.rs).
//! Human-readable `.ll` versions of these files can be found in the LLVM repo at `test/Bitcode` at the git tag `llvmorg-12.0.0`.

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

llvm_test!(
    "tests/llvm_bc/aarch64-bf16-upgrade.ll.bc",
    aarch64_bf16_upgrade
);
llvm_test!("tests/llvm_bc/arm-bf16-upgrade.ll.bc", arm_bf16_upgrade);
llvm_test!(
    "tests/llvm_bc/compatibility-as-of-llvm-12.bc",
    compatibility_llvm_12
);
llvm_test!(
    "tests/llvm_bc/upgrade-ptr-annotation.ll.bc",
    upgrade_ptr_annotation
);
llvm_test!(
    "tests/llvm_bc/upgrade-var-annotation.ll.bc",
    upgrade_var_annotation
);

// also ensure that the new-to-llvm-12 `Constant::Poison` was parsed correctly

use llvm_ir::{instruction, instruction::UnaryOp, types::FPType, Constant, Name, Operand, Type};
use std::convert::TryInto;

#[test]
fn constant_poison() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/compatibility-as-of-llvm-12.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");

    let const_struct = module
        .get_global_var_by_name(&Name::from("const.struct"))
        .expect("Couldn't find @const.struct");
    let initializer = const_struct
        .initializer
        .as_ref()
        .expect("@const.struct should have an initializer");
    match initializer.as_ref() {
        Constant::Struct { values, .. } => match &values[2].as_ref() {
            Constant::Poison(ty) => match ty.as_ref() {
                Type::IntegerType { bits: 64 } => {}, // pass
                ty => panic!("Poison of wrong type: expected i64, got {}", ty),
            },
            c => panic!("Expected constant poison, got {}", c),
        },
        c => panic!("Expected a constant struct, got {}", c),
    }

    let conversions = module
        .get_func_by_name("instructions.conversions")
        .expect("Couldn't find function @instructions.conversions");
    let bb = &conversions.basic_blocks[0];
    let fptrunc: instruction::FPTrunc = bb.instrs[7].clone().try_into().unwrap();
    match fptrunc.get_operand() {
        Operand::ConstantOperand(c) => match c.as_ref() {
            Constant::Poison(ty) => match ty.as_ref() {
                Type::FPType(FPType::Single) => {}, // pass
                ty => panic!("Poison of wrong type: expected float, got {}", ty),
            },
            c => panic!("Expected constant poison, got {}", c),
        },
        o => panic!("Expected constant operand, got {}", o),
    }
}
