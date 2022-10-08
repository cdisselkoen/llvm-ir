#![cfg(feature = "llvm-11-or-greater")]

//! These tests simply ensure that we can parse all of the `.bc` files in LLVM 11's `test/Bitcode` directory without crashing.
//! We only include the `.bc` files which are new or have changed since LLVM 10 (older ones are covered in other llvm_*_tests.rs).
//! Human-readable `.ll` versions of these files can be found in the LLVM repo at `test/Bitcode` at the git tag `llvmorg-11.0.0`.

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

macro_rules! llvm_test_should_panic {
    ($path:expr, $func:ident, $msg:expr) => {
        #[test]
        #[should_panic(expected = $msg)]
        #[allow(non_snake_case)]
        fn $func() {
            let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
            let path = Path::new($path);
            let _ = Module::from_bc_path(&path).expect("Failed to parse module");
        }
    };
}

llvm_test!("tests/llvm_bc/DIEnumerator-10.0.ll.bc", di_enumerator);
llvm_test!(
    "tests/llvm_bc/DIModule-clang-module.ll.bc",
    di_module_clang_module
);
llvm_test!(
    "tests/llvm_bc/DIModule-fortran-module.ll.bc",
    di_module_fortran_module
);
llvm_test!(
    "tests/llvm_bc/DITemplateParameter-5.0.ll.bc",
    di_template_parameter
);
llvm_test!("tests/llvm_bc/dataLocation.ll.bc", data_location);
llvm_test!("tests/llvm_bc/fortranSubrange.ll.bc", fortran_subrange);
llvm_test!(
    "tests/llvm_bc/fortranSubrangeBackward.ll.bc",
    fortran_subrange_backward
);
llvm_test!(
    "tests/llvm_bc/thinlto-function-summary-paramaccess.ll.bc",
    thinlto_function_summary_paramaccess
);
llvm_test!(
    "tests/llvm_bc/upgrade-garbage-collection-for-objc.ll.bc",
    upgrade_garbage_collection_for_objc
);
llvm_test!(
    "tests/llvm_bc/upgrade-garbage-collection-for-swift.ll.bc",
    upgrade_garbage_collection_for_swift
);
#[cfg(feature = "llvm-13-or-lower")] // starting with LLVM 14, this file is optimized to not contain a Constant::ShuffleVector
llvm_test_should_panic!(
    "tests/llvm_bc/vscale-round-trip.ll.bc",
    vscale_round_trip,
    "Constant::ShuffleVector, which is not supported"
);
llvm_test_should_panic!(
    "tests/llvm_bc/vscale-shuffle.ll.bc",
    vscale_shuffle,
    "Constant::ShuffleVector, which is not supported"
);

// also ensure that new-to-llvm-11 constructs -- specifically, BFloat types and
// scalable vector types -- were parsed correctly

// LLVM 11 doesn't appear to have any files in test/Bitcode that use the new
// BFloat types, so we wrote our own and tested it in basic_bc.
// That means this test file only worries about scalable vector types.

#[cfg(feature = "llvm-13-or-lower")] // starting with LLVM 14, this file is optimized to not contain a Constant::ShuffleVector
#[test]
#[should_panic(expected = "Constant::ShuffleVector, which is not supported")]
fn scalable_vector_insts() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/vscale-round-trip.ll.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");

    use llvm_ir::{instruction, Constant, ConstantRef, Name, Operand};
    use std::convert::TryInto;
    let func = module
        .get_func_by_name("non_const_shufflevector")
        .expect("Failed to find function");
    assert_eq!(
        module.type_of(&func.return_type),
        module.types.vector_of(module.types.i32(), 4, true),
    );
    let shufflevector: &instruction::ShuffleVector = &func
        .basic_blocks
        .get(0)
        .unwrap()
        .instrs
        .get(0)
        .unwrap()
        .clone()
        .try_into()
        .expect("Expected a ShuffleVector instruction");
    assert_eq!(
        module.type_of(shufflevector),
        module.types.vector_of(module.types.i32(), 4, true),
    );
    assert_eq!(
        &shufflevector.operand0,
        &Operand::LocalOperand {
            name: Name::from("lhs"),
            ty: module.types.vector_of(module.types.i32(), 4, true),
        }
    );
    assert_eq!(
        &shufflevector.operand1,
        &Operand::LocalOperand {
            name: Name::from("rhs"),
            ty: module.types.vector_of(module.types.i32(), 4, true),
        }
    );
    assert_eq!(
        &shufflevector.mask,
        &ConstantRef::new(Constant::AggregateZero(module.types.vector_of(
            module.types.i32(),
            4,
            true
        )))
    );
}

#[cfg(feature = "llvm-13-or-lower")] // starting with LLVM 14, this file is optimized to not contain a Constant::ShuffleVector
#[test]
#[should_panic(expected = "Constant::ShuffleVector, which is not supported")]
fn scalable_vector_consts() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/vscale-round-trip.ll.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");

    use llvm_ir::{constant, terminator, Constant, ConstantRef, Operand};
    use std::convert::TryInto;
    let func = module
        .get_func_by_name("const_shufflevector")
        .expect("Failed to find function");
    assert_eq!(
        module.type_of(&func.return_type),
        module.types.vector_of(module.types.i32(), 4, true),
    );
    let ret_inst: &terminator::Ret = &func
        .basic_blocks
        .get(0)
        .unwrap()
        .term
        .clone()
        .try_into()
        .expect("Expected a Ret instruction");
    let ret_op: &Operand = &ret_inst.return_operand.as_ref().unwrap();
    assert_eq!(
        module.type_of(ret_op),
        module.types.vector_of(module.types.i32(), 4, true),
    );
    assert_eq!(
        ret_op,
        &Operand::ConstantOperand(ConstantRef::new(Constant::ShuffleVector(
            constant::ShuffleVector {
                operand0: ConstantRef::new(Constant::AggregateZero(module.types.vector_of(
                    module.types.i32(),
                    2,
                    true
                ))),
                operand1: ConstantRef::new(Constant::Undef(module.types.vector_of(
                    module.types.i32(),
                    2,
                    true
                ))),
                mask: ConstantRef::new(Constant::AggregateZero(module.types.vector_of(
                    module.types.i32(),
                    4,
                    true
                ))),
            }
        )))
    );
}
