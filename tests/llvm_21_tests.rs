#![cfg(feature = "llvm-21-or-greater")]

//! Test that we can parse the copy of `compatibility.ll` in LLVM 21's `test/Bitcode` directory

use llvm_ir::Module;
use llvm_ir::function::ParameterAttribute;
use llvm_ir::instruction::RMWBinOp;
use llvm_ir::Instruction;
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
    "tests/llvm_bc/compatibility-as-of-llvm-21.bc",
    compatibility_llvm_21
);

#[test]
fn atomicrmw_fmaximum_fminimum() {
    let _ = env_logger::builder().is_test(true).try_init();
    let path = Path::new("tests/llvm_bc/llvm21_atomicrmw_fmaximum.bc");
    let module = Module::from_bc_path(path).expect("Failed to parse module");

    let func = module
        .functions
        .iter()
        .find(|func| func.name == "test")
        .expect("test function should exist");

    let mut ops = Vec::new();
    for bb in &func.basic_blocks {
        for inst in &bb.instrs {
            if let Instruction::AtomicRMW(rmw) = inst {
                ops.push(rmw.operation);
            }
        }
    }

    assert!(
        ops.contains(&RMWBinOp::FMaximum),
        "expected atomicrmw fmaximum in llvm21_atomicrmw_fmaximum.bc"
    );
    assert!(
        ops.contains(&RMWBinOp::FMinimum),
        "expected atomicrmw fminimum in llvm21_atomicrmw_fmaximum.bc"
    );
}

#[test]
fn captures_none_attribute() {
    let _ = env_logger::builder().is_test(true).try_init();
    let path = Path::new("tests/llvm_bc/llvm21_captures.bc");
    let module = Module::from_bc_path(path).expect("Failed to parse module");

    let func = module
        .functions
        .iter()
        .find(|func| func.name == "captures_none")
        .expect("captures_none function should exist");

    let param = func
        .parameters
        .get(0)
        .expect("captures_none should have one parameter");

    let found = param.attributes.iter().any(|attr| {
        matches!(
            attr,
            ParameterAttribute::StringAttribute {
                kind,
                value
            } if kind == "captures" && value == "none"
        )
    });

    assert!(found, "expected captures(none) parameter attribute");
}
