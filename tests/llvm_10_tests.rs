#![cfg(feature = "llvm-10-or-greater")]

//! These tests simply ensure that we can parse all of the `.bc` files in LLVM 10's `test/Bitcode` directory without crashing.
//! We only include the `.bc` files which are new or have changed since LLVM 9 (older ones are covered in llvm_9_tests.rs or llvm_8_tests.rs).
//! Human-readable `.ll` versions of these files can be found in the LLVM repo at `test/Bitcode` at the git tag `llvmorg-10.0.1`.

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
    "tests/llvm_bc/aarch64-addp-upgrade.bc",
    aarch64_addp_upgrade
);
//llvm_test!("tests/llvm_bc/invalid-functionptr-align.ll.bc", invalid_functionptr_align);  // we omit this .bc file because it is intentionally invalid
//llvm_test!("tests/llvm_bc/invalid-type-for-null-constant.ll.bc", invalid_type_for_null_constant);  // we omit this .bc file because it is intentionally invalid
llvm_test!(
    "tests/llvm_bc/upgrade-arc-runtime-calls-bitcast.bc",
    upgrade_arc_runtime_calls_bitcast
);
llvm_test!(
    "tests/llvm_bc/upgrade-arc-runtime-calls-new.bc",
    upgrade_arc_runtime_calls_new
);
llvm_test!(
    "tests/llvm_bc/upgrade-arc-runtime-calls.bc",
    upgrade_arc_runtime_calls
);
llvm_test!(
    "tests/llvm_bc/upgrade-mrr-runtime-calls.bc",
    upgrade_mrr_runtime_calls
);

// also ensure that new-to-llvm-10 constructs -- specifically, freeze
// instructions, AtomicRMWBinOps, and the `weak` field on CmpXchg -- were parsed
// correctly
use llvm_ir::instruction::RMWBinOp;
use llvm_ir::{instruction, Constant, ConstantRef, Name, Operand};
use std::convert::TryInto;

/// LLVM 10 added the Freeze instruction; ensure that that was parsed correctly
#[test]
fn freeze() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/compatibility.ll.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    let func = module
        .get_func_by_name("instructions.other")
        .expect("Failed to find function");
    let bb = func
        .get_bb_by_name(&Name::from("exit"))
        .expect("Failed to find exit bb");
    let freeze: &instruction::Freeze = &bb.instrs[6]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected a freeze, got {:?}", &bb.instrs[6]));
    assert_eq!(
        freeze.operand,
        Operand::LocalOperand {
            name: Name::from("op1"),
            ty: module.types.i32()
        }
    );
    assert_eq!(freeze.dest, Name::from(31));
    assert_eq!(&format!("{}", freeze), "%31 = freeze i32 %op1");
    let freeze: &instruction::Freeze = &bb.instrs[7]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected a freeze, got {:?}", &bb.instrs[7]));
    assert_eq!(
        freeze.operand,
        Operand::ConstantOperand(ConstantRef::new(Constant::Int {
            bits: 32,
            value: 10
        }))
    );
    assert_eq!(freeze.dest, Name::from(32));
    assert_eq!(&format!("{}", freeze), "%32 = freeze i32 10");
    let freeze: &instruction::Freeze = &bb.instrs[9]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected a freeze, got {:?}", &bb.instrs[9]));
    #[cfg(feature = "llvm-11-or-greater")]
    assert_eq!(
        freeze.operand,
        Operand::LocalOperand {
            name: Name::from("vop"),
            ty: module.types.vector_of(module.types.i32(), 2, false),
        }
    );
    #[cfg(feature = "llvm-10-or-lower")]
    assert_eq!(
        freeze.operand,
        Operand::LocalOperand {
            name: Name::from("vop"),
            ty: module.types.vector_of(module.types.i32(), 2),
        }
    );
    assert_eq!(freeze.dest, Name::from(34));
    assert_eq!(&format!("{}", freeze), "%34 = freeze <2 x i32> %vop");
}

/// LLVM 10 added the ability to get the AtomicRMW operation to the C API, so we test that functionality
#[test]
fn atomicrmw_binops() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/compatibility.ll.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");

    let func = module
        .get_func_by_name("atomics")
        .expect("Failed to find function");
    let bb = &func.basic_blocks[0];
    let atomic_xchg: &instruction::AtomicRMW = &bb.instrs[8]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[8]));
    assert_eq!(atomic_xchg.operation, RMWBinOp::Xchg);
    let atomic_add: &instruction::AtomicRMW = &bb.instrs[9]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[9]));
    assert_eq!(atomic_add.operation, RMWBinOp::Add);
    let atomic_sub: &instruction::AtomicRMW = &bb.instrs[10]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[10]));
    assert_eq!(atomic_sub.operation, RMWBinOp::Sub);
    let atomic_and: &instruction::AtomicRMW = &bb.instrs[11]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[11]));
    assert_eq!(atomic_and.operation, RMWBinOp::And);
    let atomic_nand: &instruction::AtomicRMW = &bb.instrs[12]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[12]));
    assert_eq!(atomic_nand.operation, RMWBinOp::Nand);
    let atomic_or: &instruction::AtomicRMW = &bb.instrs[13]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[13]));
    assert_eq!(atomic_or.operation, RMWBinOp::Or);
    let atomic_xor: &instruction::AtomicRMW = &bb.instrs[14]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[14]));
    assert_eq!(atomic_xor.operation, RMWBinOp::Xor);
    let atomic_max: &instruction::AtomicRMW = &bb.instrs[15]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[15]));
    assert_eq!(atomic_max.operation, RMWBinOp::Max);
    let atomic_min: &instruction::AtomicRMW = &bb.instrs[16]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[16]));
    assert_eq!(atomic_min.operation, RMWBinOp::Min);
    let atomic_umax: &instruction::AtomicRMW = &bb.instrs[17]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[17]));
    assert_eq!(atomic_umax.operation, RMWBinOp::UMax);
    let atomic_umin: &instruction::AtomicRMW = &bb.instrs[18]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[18]));
    assert_eq!(atomic_umin.operation, RMWBinOp::UMin);

    let func = module
        .get_func_by_name("fp_atomics")
        .expect("Failed to find function");
    let bb = &func.basic_blocks[0];
    let atomic_xchg: &instruction::AtomicRMW = &bb.instrs[0]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[0]));
    assert_eq!(atomic_xchg.operation, RMWBinOp::Xchg);
    let atomic_fadd: &instruction::AtomicRMW = &bb.instrs[1]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[1]));
    assert_eq!(atomic_fadd.operation, RMWBinOp::FAdd);
    let atomic_fsub: &instruction::AtomicRMW = &bb.instrs[2]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an AtomicRMW, got {:?}", &bb.instrs[2]));
    assert_eq!(atomic_fsub.operation, RMWBinOp::FSub);
}

/// LLVM 10 added the ability to get the `weak` option for CmpXchg through the C API, so we test that functionality
#[test]
fn weak_cmpxchg() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/compatibility.ll.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    let func = module
        .get_func_by_name("atomics")
        .expect("Failed to find function");
    let bb = &func.basic_blocks[0];
    let cmpxchg_notweak: &instruction::CmpXchg = &bb.instrs[0]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected a CmpXchg, got {:?}", &bb.instrs[0]));
    assert_eq!(cmpxchg_notweak.weak, false);
    let cmpxchg_notweak: &instruction::CmpXchg = &bb.instrs[1]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected a CmpXchg, got {:?}", &bb.instrs[1]));
    assert_eq!(cmpxchg_notweak.weak, false);
    let cmpxchg_weak: &instruction::CmpXchg = &bb.instrs[5]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected a CmpXchg, got {:?}", &bb.instrs[5]));
    assert_eq!(cmpxchg_weak.weak, true);
    let cmpxchg_weak: &instruction::CmpXchg = &bb.instrs[7]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected a CmpXchg, got {:?}", &bb.instrs[7]));
    assert_eq!(cmpxchg_weak.weak, true);
}
