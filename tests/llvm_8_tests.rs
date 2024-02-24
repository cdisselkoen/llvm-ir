//! These tests simply ensure that we can parse all of the `.bc` files in LLVM 8's `test/Bitcode` directory without crashing.
//! Human-readable `.ll` versions of these files can be found in the LLVM repo at `test/Bitcode` on the git branch `release_80`.

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
    "tests/llvm_bc/aggregateInstructions.3.2.ll.bc",
    aggregateInstructions
);
llvm_test!(
    "tests/llvm_bc/atomic-no-syncscope.ll.bc",
    atomic_no_syncscope
);
llvm_test!("tests/llvm_bc/attributes-3.3.ll.bc", attributes);
llvm_test!(
    "tests/llvm_bc/auto_upgrade_intrinsics.bc",
    auto_upgrade_intrinsics
);
llvm_test!(
    "tests/llvm_bc/avr-calling-conventions.ll.bc",
    avr_calling_conventions
);
llvm_test!(
    "tests/llvm_bc/binaryFloatInstructions.3.2.ll.bc",
    binaryFloatInstructions
);
llvm_test!(
    "tests/llvm_bc/binaryIntInstructions.3.2.ll.bc",
    binaryIntInstructions
);
llvm_test!(
    "tests/llvm_bc/bitwiseInstructions.3.2.ll.bc",
    bitwiseInstructions
);
llvm_test!(
    "tests/llvm_bc/calling-conventions.3.2.ll.bc",
    calling_conventions
);
llvm_test!("tests/llvm_bc/case-ranges-3.3.ll.bc", case_ranges);
llvm_test!("tests/llvm_bc/cmpxchg-upgrade.ll.bc", cmpxchg_upgrade);
llvm_test!("tests/llvm_bc/cmpxchg.3.6.ll.bc", cmpxchg);
llvm_test!("tests/llvm_bc/compatibility-3.6.ll.bc", compatibility_3_6);
llvm_test!("tests/llvm_bc/compatibility-3.7.ll.bc", compatibility_3_7);
llvm_test!("tests/llvm_bc/compatibility-3.8.ll.bc", compatibility_3_8);
llvm_test!("tests/llvm_bc/compatibility-3.9.ll.bc", compatibility_3_9);
llvm_test!("tests/llvm_bc/compatibility-4.0.ll.bc", compatibility_4_0);
llvm_test!("tests/llvm_bc/compatibility-5.0.ll.bc", compatibility_5_0);
llvm_test!("tests/llvm_bc/compatibility-6.0.ll.bc", compatibility_6_0);
llvm_test!("tests/llvm_bc/constantsTest.3.2.ll.bc", constantsTest);
llvm_test!(
    "tests/llvm_bc/conversionInstructions.3.2.ll.bc",
    conversionInstructions
);
llvm_test!(
    "tests/llvm_bc/DICompileUnit-no-DWOId.ll.bc",
    DICompileUnit_no_DWOId
);
llvm_test!("tests/llvm_bc/DIExpression-4.0.ll.bc", DIExpression_4_0);
llvm_test!(
    "tests/llvm_bc/DIExpression-aggresult.ll.bc",
    DIExpression_aggresult
);
llvm_test!("tests/llvm_bc/DIExpression-deref.ll.bc", DIExpression_deref);
llvm_test!(
    "tests/llvm_bc/DIExpression-minus-upgrade.ll.bc",
    DIExpression_minus_upgrade
);
llvm_test!(
    "tests/llvm_bc/diglobalvariable-3.8.ll.bc",
    diglobalvariable_3_8
);
llvm_test!(
    "tests/llvm_bc/DIGlobalVariableExpression.ll.bc",
    DIGlobalVariableExpression
);
llvm_test!(
    "tests/llvm_bc/DIGlobalVariableExpression2.ll.bc",
    DIGlobalVariableExpression2
);
llvm_test!(
    "tests/llvm_bc/dilocalvariable-3.9.ll.bc",
    dilocalvariable_3_9
);
llvm_test!(
    "tests/llvm_bc/DILocalVariable-explicit-tags.ll.bc",
    DILocalVariable_explicit_tags
);
llvm_test!(
    "tests/llvm_bc/DILocation-implicit-code.ll.bc",
    DILocation_implicit_code
);
llvm_test!("tests/llvm_bc/DINamespace.ll.bc", DINamespace);
llvm_test!(
    "tests/llvm_bc/DISubprogram-distinct-definitions.ll.bc",
    DISubprogram_distinct_definitions
);
llvm_test!("tests/llvm_bc/DISubprogram-v4.ll.bc", DISubprogram_v4);
llvm_test!("tests/llvm_bc/disubrange-v0.ll.bc", disubrange_v0);
llvm_test!("tests/llvm_bc/dityperefs-3.8.ll.bc", dityperefs);
llvm_test!("tests/llvm_bc/drop-debug-info.3.5.ll.bc", drop_debug_info);
llvm_test!(
    "tests/llvm_bc/function-local-metadata.3.5.ll.bc",
    function_local_metadata
);
llvm_test!("tests/llvm_bc/global-variables.3.2.ll.bc", global_variables);
llvm_test!(
    "tests/llvm_bc/highLevelStructure.3.2.ll.bc",
    highLevelStructure
);
// llvm_test!("tests/llvm_bc/invalid.ll.bc", invalid);  // we omit this .bc file because it is intentionally invalid
llvm_test!("tests/llvm_bc/linkage-types-3.2.ll.bc", linkage_types);
llvm_test!(
    "tests/llvm_bc/local-linkage-default-visibility.3.4.ll.bc",
    local_linkage_default_visibility
);
llvm_test!("tests/llvm_bc/memInstructions.3.2.ll.bc", memInstructions);
llvm_test!("tests/llvm_bc/metadata-source.ll.bc", metadata_source);
llvm_test!("tests/llvm_bc/metadata.3.5.ll.bc", metadata);
llvm_test!("tests/llvm_bc/miscInstructions.3.2.ll.bc", miscInstructions);
// llvm_test!("tests/llvm_bc/null-type.ll.bc", null_type);  // we omit this .bc file because it is intentionally invalid
llvm_test!("tests/llvm_bc/old-aliases.ll.bc", old_aliases);
// llvm_test!("tests/llvm_bc/pr18704.ll.bc", pr18704);  // we omit this .bc file because it is intentionally invalid
llvm_test!(
    "tests/llvm_bc/standardCIntrinsic.3.2.ll.bc",
    standardCIntrinsic
);
llvm_test!(
    "tests/llvm_bc/terminatorInstructions.3.2.ll.bc",
    terminatorInstructions
);
llvm_test!(
    "tests/llvm_bc/thinlto-summary-local-5.0.ll.bc",
    thinlto_summary_local
);
llvm_test!(
    "tests/llvm_bc/upgrade-dbg-checksum.ll.bc",
    upgrade_dbg_checksum
);
llvm_test!("tests/llvm_bc/upgrade-dbg-value.ll.bc", upgrade_dbg_value);
llvm_test!(
    "tests/llvm_bc/upgrade-debug-info-for-profiling.ll.bc",
    upgrade_debug_info_for_profiling
);
llvm_test!(
    "tests/llvm_bc/upgrade-global-ctors.ll.bc",
    upgrade_global_ctors
);
llvm_test!(
    "tests/llvm_bc/upgrade-importedentity.ll.bc",
    upgrade_importedentity
);
llvm_test!(
    "tests/llvm_bc/upgrade-loop-metadata.ll.bc",
    upgrade_loop_metadata
);
llvm_test!(
    "tests/llvm_bc/upgrade-objcretainrelease-asm.ll.bc",
    upgrade_objcretainrelease_asm
);
llvm_test!(
    "tests/llvm_bc/upgrade-objcretainrelease.ll.bc",
    upgrade_objcretainrelease
);
llvm_test!(
    "tests/llvm_bc/upgrade-pointer-address-space.ll.bc",
    upgrade_pointer_address_space
);
llvm_test!(
    "tests/llvm_bc/upgrade-subprogram-this.ll.bc",
    upgrade_subprogram_this
);
llvm_test!("tests/llvm_bc/upgrade-subprogram.ll.bc", upgrade_subprogram);
llvm_test!(
    "tests/llvm_bc/variableArgumentIntrinsic.3.2.ll.bc",
    variableArgumentIntrinsic
);
llvm_test!(
    "tests/llvm_bc/vectorInstructions.3.2.ll.bc",
    vectorInstructions
);
llvm_test!(
    "tests/llvm_bc/visibility-styles.3.2.ll.bc",
    visibility_styles
);
llvm_test!(
    "tests/llvm_bc/weak-cmpxchg-upgrade.ll.bc",
    weak_cmpxchg_upgrade
);
llvm_test!("tests/llvm_bc/weak-macho-3.5.ll.bc", weak_macho);

use either::Either;
use llvm_ir::instruction::{Atomicity, MemoryOrdering, SynchronizationScope};
#[cfg(feature = "llvm-14-or-lower")]
use llvm_ir::types::NamedStructDef;
use llvm_ir::*;
use std::convert::TryInto;

/// Additionally ensure that certain constructs were parsed correctly
/// (these constructs don't currently appear in any of the basic_tests)
#[test]
#[allow(non_snake_case)]
#[allow(clippy::cognitive_complexity)]
fn DILocation_implicit_code_extra_checks() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/DILocation-implicit-code.ll.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    let func = module
        .get_func_by_name("_Z5test1v")
        .expect("Failed to find function");

    let entry = func
        .get_bb_by_name(&Name::from("entry"))
        .expect("Failed to find entry bb");
    let invoke: &terminator::Invoke = &entry
        .term
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an invoke, got {:?}", &entry.term));
    if let Either::Right(Operand::ConstantOperand(cref)) = &invoke.function {
        if let Constant::GlobalReference { name, .. } = cref.as_ref() {
            assert_eq!(name, &Name::from("_ZN1A3fooEi"));
        } else {
            panic!(
                "Expected invoke.function to be a GlobalReference; instead it was another kind of Constant: {:?}",
                cref
            );
        }
    } else {
        panic!(
            "Expected invoke.function to be a GlobalReference; instead it was {:?}",
            &invoke.function
        );
    }
    assert_eq!(invoke.arguments.len(), 2);
    if let Operand::LocalOperand { name, ty } = &invoke.arguments[0].0 {
        assert_eq!(name, &Name::from("a"));
        #[cfg(feature = "llvm-14-or-lower")]
        if let Type::PointerType { pointee_type, .. } = ty.as_ref() {
            if let Type::NamedStructType { name } = pointee_type.as_ref() {
                match module.types.named_struct_def(name) {
                    None => panic!("Failed to find {} in module.types", name),
                    Some(NamedStructDef::Opaque) => {
                        panic!("Didn't expect {} to be an opaque struct type", name)
                    },
                    Some(NamedStructDef::Defined(ty)) => {
                        assert_eq!(ty, &module.types.struct_of(vec![module.types.i8()], false));
                    },
                }
            } else {
                panic!("Expected invoke.arguments[0].0 to be a pointer to a Type::NamedStructType; instead it was a pointer to a {:?}", **pointee_type);
            }
        } else {
            panic!(
                "Exected invoke.arguments[0].0 to be of PointerType; instead it was {:?}",
                ty.as_ref()
            );
        }
        #[cfg(feature = "llvm-15-or-greater")]
        assert!(matches!(ty.as_ref(), Type::PointerType { .. }));
    } else {
        panic!(
            "Expected invoke.arguments[0].0 to be a local operand; instead it was {:?}",
            &invoke.arguments[0].0
        );
    }
    assert_eq!(
        invoke.arguments[1].0,
        Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 32, value: 0 }))
    );
    assert_eq!(invoke.return_label, Name::from("invoke.cont"));
    assert_eq!(invoke.exception_label, Name::from("lpad"));
    #[cfg(feature = "llvm-14-or-lower")]
    let expected_fmt = "%0 = invoke @_ZN1A3fooEi(%struct.A* %a, i32 0) to label %invoke.cont unwind label %lpad (with debugloc)";
    #[cfg(feature = "llvm-15-or-greater")]
    let expected_fmt = "%0 = invoke @_ZN1A3fooEi(ptr %a, i32 0) to label %invoke.cont unwind label %lpad (with debugloc)";
    assert_eq!(&format!("{}", invoke), expected_fmt);

    // For the rest of the function, our numbered variables are one-off the
    // numbers in the .ll file in the LLVM repo.
    // For instance, the .ll file in the LLVM repo has %0 as the dest of the
    // first 'landingpad' instruction, while we number it %1.
    // The discrepancy is because we assign %0 to be the result of the invoke
    // terminator in the entry block. Although this particular .ll file doesn't
    // assign the result of the invoke, the LLVM 8/9/10 docs on 'invoke' -- and
    // in particular the examples -- are clear that 'invoke' does produce a
    // result.

    let lpad = func
        .get_bb_by_name(&Name::from("lpad"))
        .expect("Failed to find lpad bb");
    let landingpad: &instruction::LandingPad = &lpad.instrs[0]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected a landingpad, got {:?}", &lpad.instrs[0]));
    let expected_landingpad_resultty = module.types.struct_of(
        vec![
            #[cfg(feature = "llvm-14-or-lower")]
            module.types.pointer_to(module.types.i8()),
            #[cfg(feature = "llvm-15-or-greater")]
            module.types.pointer(),
            module.types.i32(),
        ],
        false,
    );
    assert_eq!(landingpad.result_type, expected_landingpad_resultty);
    assert_eq!(landingpad.clauses.len(), 1);
    assert_eq!(landingpad.cleanup, false);
    assert_eq!(landingpad.dest, Name::Number(1));
    #[cfg(feature = "llvm-14-or-lower")]
    let expected_fmt = "%1 = landingpad { i8*, i32 } (with debugloc)";
    #[cfg(feature = "llvm-15-or-greater")]
    let expected_fmt = "%1 = landingpad { ptr, i32 } (with debugloc)";
    assert_eq!(&format!("{}", landingpad), expected_fmt);
    let eval: &instruction::ExtractValue = &lpad.instrs[1]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an extractvalue, got {:?}", &lpad.instrs[1]));
    assert_eq!(
        eval.aggregate,
        Operand::LocalOperand {
            name: Name::Number(1),
            ty: expected_landingpad_resultty.clone()
        }
    );
    assert_eq!(eval.indices.len(), 1);
    assert_eq!(eval.indices[0], 0);
    assert_eq!(eval.dest, Name::Number(2));
    #[cfg(feature = "llvm-14-or-lower")]
    let expected_fmt = "%2 = extractvalue { i8*, i32 } %1, 0 (with debugloc)";
    #[cfg(feature = "llvm-15-or-greater")]
    let expected_fmt = "%2 = extractvalue { ptr, i32 } %1, 0 (with debugloc)";
    assert_eq!(&format!("{}", eval), expected_fmt);

    // From this point on, our numbers are off by 2 instead of 1, due to
    // numbering the invoke terminator in the 'catch' block.
    // See notes above.

    let lpad1 = func
        .get_bb_by_name(&Name::from("lpad1"))
        .expect("Failed to find lpad1 bb");
    let landingpad: &instruction::LandingPad = &lpad1.instrs[0]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected a landingpad, got {:?}", &lpad1.instrs[0]));
    assert_eq!(landingpad.result_type, expected_landingpad_resultty);
    assert_eq!(landingpad.clauses.len(), 0);
    assert_eq!(landingpad.cleanup, true);
    #[cfg(feature = "llvm-14-or-lower")]
    let expected_fmt = "%10 = landingpad { i8*, i32 } cleanup (with debugloc)";
    #[cfg(feature = "llvm-15-or-greater")]
    let expected_fmt = "%10 = landingpad { ptr, i32 } cleanup (with debugloc)";
    assert_eq!(&format!("{}", landingpad), expected_fmt);
    let eval: &instruction::ExtractValue = &lpad1.instrs[3]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an extractvalue, got {:?}", &lpad.instrs[3]));
    assert_eq!(
        eval.aggregate,
        Operand::LocalOperand {
            name: Name::Number(10),
            ty: expected_landingpad_resultty.clone()
        }
    );
    assert_eq!(eval.indices.len(), 1);
    assert_eq!(eval.indices[0], 1);
    assert_eq!(eval.dest, Name::Number(12));
    #[cfg(feature = "llvm-14-or-lower")]
    let expected_fmt = "%12 = extractvalue { i8*, i32 } %10, 1 (with debugloc)";
    #[cfg(feature = "llvm-15-or-greater")]
    let expected_fmt = "%12 = extractvalue { ptr, i32 } %10, 1 (with debugloc)";
    assert_eq!(&format!("{}", eval), expected_fmt);

    let trycont = func
        .get_bb_by_name(&Name::from("try.cont"))
        .expect("Failed to find trycont bb");
    let u: &terminator::Unreachable = &trycont
        .term
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an unreachable, got {:?}", &trycont.term));
    assert_eq!(&format!("{}", u), "unreachable (with debugloc)");

    let ehresume = func
        .get_bb_by_name(&Name::from("eh.resume"))
        .expect("Failed to find ehresume bb");
    let ival: &instruction::InsertValue = &ehresume.instrs[2]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an insertvalue, got {:?}", &ehresume.instrs[2]));
    assert_eq!(
        ival.aggregate,
        Operand::ConstantOperand(ConstantRef::new(Constant::Undef(
            expected_landingpad_resultty.clone()
        )))
    );
    assert_eq!(
        ival.element,
        Operand::LocalOperand {
            name: Name::from("exn4"),
            #[cfg(feature = "llvm-14-or-lower")]
            ty: module.types.pointer_to(module.types.i8()),
            #[cfg(feature = "llvm-15-or-greater")]
            ty: module.types.pointer(),
        }
    );
    assert_eq!(ival.indices.len(), 1);
    assert_eq!(ival.indices[0], 0);
    assert_eq!(ival.dest, Name::from("lpad.val"));
    #[cfg(feature = "llvm-14-or-lower")]
    let expected_fmt = "%lpad.val = insertvalue { i8*, i32 } undef, i8* %exn4, 0 (with debugloc)";
    #[cfg(feature = "llvm-15-or-greater")]
    let expected_fmt = "%lpad.val = insertvalue { ptr, i32 } undef, ptr %exn4, 0 (with debugloc)";
    assert_eq!(&format!("{}", ival), expected_fmt);
    let ival2: &instruction::InsertValue = &ehresume.instrs[3]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an insertvalue, got {:?}", &ehresume.instrs[3]));
    assert_eq!(
        ival2.aggregate,
        Operand::LocalOperand {
            name: Name::from("lpad.val"),
            ty: expected_landingpad_resultty.clone()
        }
    );
    assert_eq!(
        ival2.element,
        Operand::LocalOperand {
            name: Name::from("sel5"),
            ty: module.types.i32()
        }
    );
    assert_eq!(ival2.indices.len(), 1);
    assert_eq!(ival2.indices[0], 1);
    assert_eq!(ival2.dest, Name::from("lpad.val6"));
    #[cfg(feature = "llvm-14-or-lower")]
    let expected_fmt = "%lpad.val6 = insertvalue { i8*, i32 } %lpad.val, i32 %sel5, 1 (with debugloc)";
    #[cfg(feature = "llvm-15-or-greater")]
    let expected_fmt = "%lpad.val6 = insertvalue { ptr, i32 } %lpad.val, i32 %sel5, 1 (with debugloc)";
    assert_eq!(&format!("{}", ival2), expected_fmt);
    let resume: &terminator::Resume = &ehresume
        .term
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected a resume, got {:?}", &ehresume.term));
    assert_eq!(
        resume.operand,
        Operand::LocalOperand {
            name: Name::from("lpad.val6"),
            ty: expected_landingpad_resultty.clone()
        }
    );
    #[cfg(feature = "llvm-14-or-lower")]
    let expected_fmt = "resume { i8*, i32 } %lpad.val6 (with debugloc)";
    #[cfg(feature = "llvm-15-or-greater")]
    let expected_fmt = "resume { ptr, i32 } %lpad.val6 (with debugloc)";
    assert_eq!(&format!("{}", resume), expected_fmt);
}

#[test]
fn atomics() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = Path::new("tests/llvm_bc/compatibility-6.0.ll.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    let func = module
        .get_func_by_name("atomics")
        .expect("Failed to find function");
    let bb = &func.basic_blocks[0];
    let cmpxchg: &instruction::CmpXchg = &bb.instrs[0]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected a cmpxchg, got {:?}", &bb.instrs[0]));
    assert_eq!(
        cmpxchg.address,
        Operand::LocalOperand {
            name: Name::from("word"),
            #[cfg(feature = "llvm-14-or-lower")]
            ty: module.types.pointer_to(module.types.i32()),
            #[cfg(feature = "llvm-15-or-greater")]
            ty: module.types.pointer(),
        }
    );
    assert_eq!(
        cmpxchg.expected,
        Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 32, value: 0 }))
    );
    assert_eq!(
        cmpxchg.replacement,
        Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 32, value: 4 }))
    );
    assert_eq!(cmpxchg.dest, Name::from("cmpxchg.0"));
    assert_eq!(cmpxchg.volatile, false);
    assert_eq!(
        cmpxchg.atomicity,
        Atomicity {
            synch_scope: SynchronizationScope::System,
            mem_ordering: MemoryOrdering::Monotonic
        }
    );
    assert_eq!(cmpxchg.failure_memory_ordering, MemoryOrdering::Monotonic);
    #[cfg(feature = "llvm-14-or-lower")]
    let expected_fmt = "%cmpxchg.0 = cmpxchg i32* %word, i32 0, i32 4 monotonic monotonic";
    #[cfg(feature = "llvm-15-or-greater")]
    let expected_fmt = "%cmpxchg.0 = cmpxchg ptr %word, i32 0, i32 4 monotonic monotonic";
    assert_eq!(&format!("{}", cmpxchg), expected_fmt);
    let atomicrmw: &instruction::AtomicRMW = &bb.instrs[8]
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Expected an atomicrmw, got {:?}", &bb.instrs[8]));
    assert_eq!(
        atomicrmw.address,
        Operand::LocalOperand {
            name: Name::from("word"),
            #[cfg(feature = "llvm-14-or-lower")]
            ty: module.types.pointer_to(module.types.i32()),
            #[cfg(feature = "llvm-15-or-greater")]
            ty: module.types.pointer(),
        }
    );
    assert_eq!(
        atomicrmw.value,
        Operand::ConstantOperand(ConstantRef::new(Constant::Int {
            bits: 32,
            value: 12
        }))
    );
    assert_eq!(atomicrmw.dest, Name::from("atomicrmw.xchg"));
    assert_eq!(module.type_of(atomicrmw), module.types.i32());
    #[cfg(feature = "llvm-9-or-lower")]
    let expected_fmt = "%atomicrmw.xchg = atomicrmw i32* %word, i32 12 not_atomic";
    #[cfg(all(feature = "llvm-10-or-greater", feature = "llvm-14-or-lower"))]
    let expected_fmt = "%atomicrmw.xchg = atomicrmw xchg i32* %word, i32 12 monotonic"; // I'm not sure why it's not_atomic for LLVM 9 and lower, but monotonic for LLVM 10+
    #[cfg(feature = "llvm-15-or-greater")]
    let expected_fmt = "%atomicrmw.xchg = atomicrmw xchg ptr %word, i32 12 monotonic"; // I'm not sure why it's not_atomic for LLVM 9 and lower, but monotonic for LLVM 10+
    assert_eq!(&format!("{}", atomicrmw), expected_fmt);
}
