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

// tests for nuw, nsw, and exact flags, whose getters were added to the LLVM C API in LLVM 17
use llvm_ir::instruction;
use std::convert::TryInto;

#[test]
fn nuw_nsw_exact() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/compatibility-as-of-llvm-17.ll");
    let module = Module::from_ir_path(&path).expect("Failed to parse module");
    let func = module
        .get_func_by_name("instructions.binops")
        .expect("Failed to find function");
    let bb = func.basic_blocks
        .get(0)
        .expect("expected function to have a basic block");
    let add: &instruction::Add = &bb
        .instrs[0]
        .clone()
        .try_into()
        .expect("Expected an add");
    assert_eq!(add.nuw, false);
    assert_eq!(add.nsw, false);
    assert_eq!(&format!("{add}"), "%1 = add i8 %op1, i8 %op2");
    let add: &instruction::Add = &bb
        .instrs[1]
        .clone()
        .try_into()
        .expect("Expected an add");
    assert_eq!(add.nuw, true);
    assert_eq!(add.nsw, false);
    assert_eq!(&format!("{add}"), "%2 = add nuw i8 %op1, i8 %op2");
    let add: &instruction::Add = &bb
        .instrs[2]
        .clone()
        .try_into()
        .expect("Expected an add");
    assert_eq!(add.nuw, false);
    assert_eq!(add.nsw, true);
    assert_eq!(&format!("{add}"), "%3 = add nsw i8 %op1, i8 %op2");
    let add: &instruction::Add = &bb
        .instrs[3]
        .clone()
        .try_into()
        .expect("Expected an add");
    assert_eq!(add.nuw, true);
    assert_eq!(add.nsw, true);
    assert_eq!(&format!("{add}"), "%4 = add nuw nsw i8 %op1, i8 %op2");

    let sub: &instruction::Sub = &bb
        .instrs[4]
        .clone()
        .try_into()
        .expect("Expected a sub");
    assert_eq!(sub.nuw, false);
    assert_eq!(sub.nsw, false);
    assert_eq!(&format!("{sub}"), "%5 = sub i8 %op1, i8 %op2");
    let sub: &instruction::Sub = &bb
        .instrs[5]
        .clone()
        .try_into()
        .expect("Expected a sub");
    assert_eq!(sub.nuw, true);
    assert_eq!(sub.nsw, false);
    assert_eq!(&format!("{sub}"), "%6 = sub nuw i8 %op1, i8 %op2");
    let sub: &instruction::Sub = &bb
        .instrs[6]
        .clone()
        .try_into()
        .expect("Expected a sub");
    assert_eq!(sub.nuw, false);
    assert_eq!(sub.nsw, true);
    assert_eq!(&format!("{sub}"), "%7 = sub nsw i8 %op1, i8 %op2");
    let sub: &instruction::Sub = &bb
        .instrs[7]
        .clone()
        .try_into()
        .expect("Expected a sub");
    assert_eq!(sub.nuw, true);
    assert_eq!(sub.nsw, true);
    assert_eq!(&format!("{sub}"), "%8 = sub nuw nsw i8 %op1, i8 %op2");

    let mul: &instruction::Mul = &bb
        .instrs[8]
        .clone()
        .try_into()
        .expect("Expected a mul");
    assert_eq!(mul.nuw, false);
    assert_eq!(mul.nsw, false);
    assert_eq!(&format!("{mul}"), "%9 = mul i8 %op1, i8 %op2");
    let mul: &instruction::Mul = &bb
        .instrs[9]
        .clone()
        .try_into()
        .expect("Expected a mul");
    assert_eq!(mul.nuw, true);
    assert_eq!(mul.nsw, false);
    assert_eq!(&format!("{mul}"), "%10 = mul nuw i8 %op1, i8 %op2");
    let mul: &instruction::Mul = &bb
        .instrs[10]
        .clone()
        .try_into()
        .expect("Expected a mul");
    assert_eq!(mul.nuw, false);
    assert_eq!(mul.nsw, true);
    assert_eq!(&format!("{mul}"), "%11 = mul nsw i8 %op1, i8 %op2");
    let mul: &instruction::Mul = &bb
        .instrs[11]
        .clone()
        .try_into()
        .expect("Expected a mul");
    assert_eq!(mul.nuw, true);
    assert_eq!(mul.nsw, true);
    assert_eq!(&format!("{mul}"), "%12 = mul nuw nsw i8 %op1, i8 %op2");

    let udiv: &instruction::UDiv = &bb
        .instrs[12]
        .clone()
        .try_into()
        .expect("Expected a udiv");
    assert_eq!(udiv.exact, false);
    assert_eq!(&format!("{udiv}"), "%13 = udiv i8 %op1, i8 %op2");
    let udiv: &instruction::UDiv = &bb
        .instrs[13]
        .clone()
        .try_into()
        .expect("Expected a udiv");
    assert_eq!(udiv.exact, true);
    assert_eq!(&format!("{udiv}"), "%14 = udiv exact i8 %op1, i8 %op2");

    let sdiv: &instruction::SDiv = &bb
        .instrs[14]
        .clone()
        .try_into()
        .expect("Expected an sdiv");
    assert_eq!(sdiv.exact, false);
    assert_eq!(&format!("{sdiv}"), "%15 = sdiv i8 %op1, i8 %op2");
    let sdiv: &instruction::SDiv = &bb
        .instrs[15]
        .clone()
        .try_into()
        .expect("Expected an sdiv");
    assert_eq!(sdiv.exact, true);
    assert_eq!(&format!("{sdiv}"), "%16 = sdiv exact i8 %op1, i8 %op2");
}
