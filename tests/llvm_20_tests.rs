#![cfg(feature = "llvm-20-or-greater")]

//! Test that we can parse the copy of `compatibility.ll` in LLVM 20's `test/Bitcode` directory

use either::Either;
use llvm_ir::{Constant, Instruction, Module, Type};
use llvm_ir::instruction::RMWBinOp;
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
    "tests/llvm_bc/compatibility-as-of-llvm-20.bc",
    compatibility_llvm_20
);

#[test]
fn ptr_auth() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/compatibility-as-of-llvm-20.bc");
    let module = Module::from_bc_path(path).expect("Failed to parse module");

    let auth_var = module
        .global_vars
        .iter()
        .find(|var| var.name == "auth_var".into())
        .expect("auth_var variable should exist");

    let auth_var_init = auth_var
        .initializer
        .clone()
        .expect("auth_var should be initialised");

    if let Constant::PtrAuth {
        ptr,
        key,
        disc,
        addr_disc,
    } = auth_var_init.as_ref()
    {
        assert_eq!(&format!("{ptr}"), "ptr @g1");
        assert_eq!(&format!("{key}"), "i32 0");
        assert_eq!(&format!("{disc}"), "i64 65535");
        assert_eq!(&format!("{addr_disc}"), "ptr null");
    } else {
        panic!(
            "auth_var initialised with non-ptrauth constant {:?}",
            auth_var_init
        );
    }
}

#[test]
fn scalable_vector_alloca() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/compatibility-as-of-llvm-20.bc");
    let module = Module::from_bc_path(path).expect("Failed to parse module");

    let func = module
        .functions
        .iter()
        .find(|func| func.name == "typesystem")
        .expect("typesystem function should exist");

    let mut found = false;
    for bb in &func.basic_blocks {
        for inst in &bb.instrs {
            if let Instruction::Alloca(alloca) = inst {
                if let Type::VectorType {
                    element_type,
                    num_elements,
                    scalable: true,
                } = alloca.allocated_type.as_ref()
                {
                    if *num_elements == 4 {
                        if let Type::IntegerType { bits: 32 } = element_type.as_ref() {
                            found = true;
                            break;
                        }
                    }
                }
            }
        }
        if found {
            break;
        }
    }

    assert!(found, "expected scalable vector alloca in typesystem");
}

// TODO: Other LLVM-20+ tests

#[test]
fn atomicrmw_usub_ops() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/llvm20_atomicrmw.bc");
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
        ops.contains(&RMWBinOp::USubCond),
        "expected atomicrmw usub_cond in llvm20_atomicrmw.bc"
    );
    assert!(
        ops.contains(&RMWBinOp::USubSat),
        "expected atomicrmw usub_sat in llvm20_atomicrmw.bc"
    );
}

#[test]
fn stepvector_intrinsic() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/llvm20_stepvector.bc");
    let module = Module::from_bc_path(path).expect("Failed to parse module");

    let func = module
        .functions
        .iter()
        .find(|func| func.name == "use_stepvector")
        .expect("use_stepvector function should exist");

    let mut found = false;
    for bb in &func.basic_blocks {
        for inst in &bb.instrs {
            if let Instruction::Call(call) = inst {
                if let Either::Right(op) = &call.function {
                    let func_str = format!("{}", op);
                    if func_str.contains("@llvm.stepvector.v4i32") {
                        found = true;
                        break;
                    }
                }
            }
        }
        if found {
            break;
        }
    }

    assert!(
        found,
        "expected call to llvm.stepvector.v4i32 in llvm20_stepvector.bc"
    );
}
