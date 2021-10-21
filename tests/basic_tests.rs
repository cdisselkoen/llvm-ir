use either::Either;
use itertools::Itertools;
use llvm_ir::function::{FunctionAttribute, ParameterAttribute};
use llvm_ir::instruction;
use llvm_ir::module::{Alignment, Endianness, Mangling, PointerLayout};
use llvm_ir::terminator;
use llvm_ir::types::{FPType, NamedStructDef};
#[cfg(feature="llvm-9-or-greater")]
use llvm_ir::HasDebugLoc;
use llvm_ir::Instruction;
use llvm_ir::IntPredicate;
use llvm_ir::Module;
use llvm_ir::Name;
use llvm_ir::Operand;
use llvm_ir::Terminator;
use llvm_ir::Type;
use llvm_ir::{Constant, ConstantRef};
use std::convert::TryInto;
use std::path::{Path, PathBuf};

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

const BC_DIR: &str = "tests/basic_bc/";

// Test against bitcode compiled with the same version of LLVM
#[cfg(feature = "llvm-8")]
fn llvm_bc_dir() -> PathBuf {
    Path::new(BC_DIR).join("llvm8")
}
#[cfg(feature = "llvm-9")]
fn llvm_bc_dir() -> PathBuf {
    Path::new(BC_DIR).join("llvm9")
}
#[cfg(feature = "llvm-10")]
fn llvm_bc_dir() -> PathBuf {
    Path::new(BC_DIR).join("llvm10")
}
#[cfg(feature = "llvm-11")]
fn llvm_bc_dir() -> PathBuf {
    Path::new(BC_DIR).join("llvm11")
}
#[cfg(feature = "llvm-12")]
fn llvm_bc_dir() -> PathBuf {
    Path::new(BC_DIR).join("llvm12")
}
#[cfg(feature = "llvm-13")]
fn llvm_bc_dir() -> PathBuf {
    Path::new(BC_DIR).join("llvm13")
}

fn rust_bc_dir() -> PathBuf {
    Path::new(BC_DIR).join("rust")
}

#[test]
fn hellobc() {
    init_logging();
    let path = llvm_bc_dir().join("hello.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    assert_eq!(&module.name, &path.to_str().unwrap());
    assert_eq!(module.source_file_name, "hello.c");
    #[cfg(feature="llvm-10-or-lower")]
    assert_eq!(module.target_triple, Some("x86_64-apple-macosx10.16.0".into()));
    #[cfg(feature="llvm-11-or-greater")]
    assert_eq!(module.target_triple, Some("x86_64-apple-macosx11.0.0".into()));
    assert_eq!(module.functions.len(), 1);
    let func = &module.functions[0];
    assert_eq!(func.name, "main");
    assert_eq!(func.parameters.len(), 0);
    assert_eq!(func.is_var_arg, false);
    assert_eq!(func.return_type, module.types.int(32));
    assert_eq!(func.basic_blocks.len(), 1);
    let bb = &func.basic_blocks[0];
    assert_eq!(bb.name, Name::Number(0));
    assert_eq!(bb.instrs.len(), 0);
    let ret: &terminator::Ret = &bb
        .term
        .clone()
        .try_into()
        .unwrap_or_else(|_| panic!("Terminator should be a Ret but is {:?}", &bb.term));
    assert_eq!(
        ret.return_operand,
        Some(Operand::ConstantOperand(ConstantRef::new(Constant::Int {
            bits: 32,
            value: 0
        })))
    );
    assert_eq!(&format!("{}", ret), "ret i32 0");

    #[cfg(feature="llvm-9-or-greater")]
    {
        // this file was compiled without debuginfo, so nothing should have a debugloc
        assert_eq!(func.debugloc, None);
        assert_eq!(ret.debugloc, None);
    }
}

// this test relates to the version of the file compiled with debuginfo
#[cfg(feature="llvm-9-or-greater")]
#[test]
fn hellobcg() {
    init_logging();
    let path = llvm_bc_dir().join("hello.bc-g");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    assert_eq!(&module.name, &path.to_str().unwrap());
    assert_eq!(module.source_file_name, "hello.c");
    let debug_filename = "hello.c";
    let debug_directory = "/Users/craig/llvm-ir/tests/basic_bc"; // this is what appears in the checked-in copy of hello.bc-g

    let func = &module.functions[0];
    assert_eq!(func.name, "main");
    let debugloc = func.get_debug_loc().as_ref().expect("Expected main() to have a debugloc");
    assert_eq!(debugloc.line, 3);
    assert_eq!(debugloc.col, None);
    assert_eq!(debugloc.filename, debug_filename);
    assert_eq!(debugloc.directory.as_ref().map(|s| s.as_str()), Some(debug_directory));

    let bb = &func.basic_blocks[0];
    let ret: &terminator::Ret = &bb.term.clone().try_into().unwrap_or_else(|_| panic!("Terminator should be a Ret but is {:?}", &bb.term));
    let debugloc = ret.get_debug_loc().as_ref().expect("expected the Ret to have a debugloc");
    assert_eq!(debugloc.line, 4);
    assert_eq!(debugloc.col, Some(3));
    assert_eq!(debugloc.filename, debug_filename);
    assert_eq!(debugloc.directory.as_ref().map(|s| s.as_str()), Some(debug_directory));
    assert_eq!(&format!("{}", ret), "ret i32 0 (with debugloc)");
}

#[test]
#[allow(clippy::cognitive_complexity)]
fn loopbc() {
    init_logging();
    let path = llvm_bc_dir().join("loop.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");

    // get function and check info on it
    assert_eq!(module.functions.len(), 1);
    let func = &module.functions[0];
    assert_eq!(func.name, "loop");
    assert_eq!(func.parameters.len(), 2);
    assert_eq!(func.is_var_arg, false);
    assert_eq!(func.return_type, module.types.void());
    assert_eq!(
        module.type_of(func),
        module.types.func_type(
            module.types.void(),
            vec![module.types.i32(), module.types.i32()],
            false,
        )
    );
    assert_eq!(module.get_func_by_name("loop"), Some(func));

    // get parameters and check info on them
    let param0 = &func.parameters[0];
    let param1 = &func.parameters[1];
    assert_eq!(param0.name, Name::Number(0));
    assert_eq!(param1.name, Name::Number(1));
    assert_eq!(param0.ty, module.types.i32());
    assert_eq!(param1.ty, module.types.i32());
    assert_eq!(module.type_of(param0), module.types.i32());
    assert_eq!(module.type_of(param1), module.types.i32());

    // get basic blocks and check their names
    // different LLVM versions end up with different numbers of basic blocks for this function
    #[cfg(feature="llvm-9-or-lower")]
    let bbs = {
        assert_eq!(func.basic_blocks.len(), 6);
        let bb2 = &func.basic_blocks[0];
        let bb7 = &func.basic_blocks[1];
        let bb10 = &func.basic_blocks[2];
        let bb14 = &func.basic_blocks[3];
        let bb19 = &func.basic_blocks[4];
        let bb22 = &func.basic_blocks[5];
        assert_eq!(bb2.name, Name::Number(2));
        assert_eq!(bb7.name, Name::Number(7));
        assert_eq!(bb10.name, Name::Number(10));
        assert_eq!(bb14.name, Name::Number(14));
        assert_eq!(bb19.name, Name::Number(19));
        assert_eq!(bb22.name, Name::Number(22));
        assert_eq!(func.get_bb_by_name(&Name::Number(2)), Some(bb2));
        assert_eq!(func.get_bb_by_name(&Name::Number(19)), Some(bb19));
        vec![bb2, bb7, bb10, bb14, bb19, bb22]
    };
    #[cfg(feature = "llvm-10")]
    let bbs = {
        assert_eq!(func.basic_blocks.len(), 4);
        let bb2 = &func.basic_blocks[0];
        let bb7 = &func.basic_blocks[1];
        let bb12 = &func.basic_blocks[2];
        let bb21 = &func.basic_blocks[3];
        assert_eq!(bb2.name, Name::Number(2));
        assert_eq!(bb7.name, Name::Number(7));
        assert_eq!(bb12.name, Name::Number(12));
        assert_eq!(bb21.name, Name::Number(21));
        assert_eq!(func.get_bb_by_name(&Name::Number(2)), Some(bb2));
        assert_eq!(func.get_bb_by_name(&Name::Number(12)), Some(bb12));
        vec![bb2, bb7, bb12, bb21]
    };
    #[cfg(feature="llvm-11")]
    let bbs = {
        assert_eq!(func.basic_blocks.len(), 4);
        let bb2 = &func.basic_blocks[0];
        let bb7 = &func.basic_blocks[1];
        let bb14 = &func.basic_blocks[2];
        let bb24 = &func.basic_blocks[3];
        assert_eq!(bb2.name, Name::Number(2));
        assert_eq!(bb7.name, Name::Number(7));
        assert_eq!(bb14.name, Name::Number(14));
        assert_eq!(bb24.name, Name::Number(24));
        assert_eq!(func.get_bb_by_name(&Name::Number(2)), Some(bb2));
        assert_eq!(func.get_bb_by_name(&Name::Number(14)), Some(bb14));
        vec![bb2, bb7, bb14, bb24]
    };
    #[cfg(feature="llvm-12")]
    let bbs = {
        assert_eq!(func.basic_blocks.len(), 8); // LLVM 12+ seems to do some unrolling in this example that previous LLVMs didn't
        let bb2 = &func.basic_blocks[0];
        let bb7 = &func.basic_blocks[1];
        let bb12 = &func.basic_blocks[2];
        let bb17 = &func.basic_blocks[3];
        let bb19 = &func.basic_blocks[4];
        let bb47 = &func.basic_blocks[7];
        // actually have 8 BBs, but we only use the first five and the last one
        assert_eq!(bb2.name, Name::Number(2));
        assert_eq!(bb7.name, Name::Number(7));
        assert_eq!(bb12.name, Name::Number(12));
        assert_eq!(bb17.name, Name::Number(17));
        assert_eq!(bb19.name, Name::Number(19));
        assert_eq!(bb47.name, Name::Number(47));
        vec![bb2, bb7, bb12, bb17, bb19, bb47]
    };
    #[cfg(feature="llvm-13-or-greater")]
    let bbs = {
        assert_eq!(func.basic_blocks.len(), 9);
        let bb2 = &func.basic_blocks[0];
        let bb6 = &func.basic_blocks[1];
        let bb12 = &func.basic_blocks[3];
        let bb17 = &func.basic_blocks[4];
        let bb19 = &func.basic_blocks[5];
        let bb47 = &func.basic_blocks[8];
        // actually have 9 BBs, but we only use these selected 6
        assert_eq!(bb2.name, Name::Number(2));
        assert_eq!(bb6.name, Name::Number(6));
        assert_eq!(bb12.name, Name::Number(12));
        assert_eq!(bb17.name, Name::Number(17));
        assert_eq!(bb19.name, Name::Number(19));
        assert_eq!(bb47.name, Name::Number(47));
        vec![bb2, bb6, bb12, bb17, bb19, bb47]
    };

    // check details about the instructions in basic block %2
    let alloca: &instruction::Alloca = &bbs[0].instrs[0]
        .clone()
        .try_into()
        .expect("Should be an alloca");
    assert_eq!(alloca.dest, Name::Number(3));
    let allocated_type = module.types.array_of(
        module.types.i32(),
        10,
    );
    assert_eq!(alloca.allocated_type, allocated_type);
    assert_eq!(
        alloca.num_elements,
        Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 32, value: 1 })) // One element, which is an array of 10 elements. Not 10 elements, each of which are i32.
    );
    assert_eq!(alloca.alignment, 16);
    assert_eq!(module.type_of(alloca), module.types.pointer_to(allocated_type.clone()));
    assert_eq!(module.type_of(&alloca.num_elements), module.types.i32());
    assert_eq!(&format!("{}", alloca), "%3 = alloca [10 x i32], align 16");
    let bitcast: &instruction::BitCast = &bbs[0].instrs[1]
        .clone()
        .try_into()
        .expect("Should be a bitcast");
    assert_eq!(bitcast.dest, Name::Number(4));
    assert_eq!(bitcast.to_type, module.types.pointer_to(module.types.i8()));
    assert_eq!(
        bitcast.operand,
        Operand::LocalOperand {
            name: Name::Number(3),
            ty: module.types.pointer_to(allocated_type.clone())
        }
    );
    assert_eq!(module.type_of(bitcast), module.types.pointer_to(module.types.i8()));
    assert_eq!(module.type_of(&bitcast.operand), module.types.pointer_to(allocated_type.clone()));
    assert_eq!(&format!("{}", bitcast), "%4 = bitcast [10 x i32]* %3 to i8*");
    let lifetimestart: &instruction::Call =
        &bbs[0].instrs[2].clone().try_into().expect("Should be a call");
    if let Either::Right(Operand::ConstantOperand(cref)) = &lifetimestart.function {
        if let Constant::GlobalReference { ref name, ref ty } = cref.as_ref() {
            assert_eq!(module.type_of(&lifetimestart.function), module.types.pointer_to(ty.clone()));  // lifetimestart.function should be a constant function pointer
            assert_eq!(*name, Name::from("llvm.lifetime.start.p0i8"));
            if let Type::FuncType { result_type, param_types, is_var_arg } = ty.as_ref() {
                assert_eq!(result_type, &module.types.void());
                assert_eq!(param_types, &vec![module.types.i64(), module.types.pointer_to(module.types.i8())]);
                assert_eq!(*is_var_arg, false);
            } else {
                panic!("lifetimestart.function has unexpected type {:?}", ty);
            }
        } else {
            panic!(
                "lifetimestart.function not a GlobalReference as expected; it is actually another kind of Constant: {:?}",
                cref
            );
        }
    } else {
        panic!(
            "lifetimestart.function not a GlobalReference as expected; it is actually {:?}",
            &lifetimestart.function
        );
    }
    let arg0 = &lifetimestart.arguments.get(0).expect("Expected an argument 0");
    let arg1 = &lifetimestart.arguments.get(1).expect("Expected an argument 1");
    assert_eq!(arg0.0, Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 64, value: 40 } )));
    assert_eq!(arg1.0, Operand::LocalOperand { name: Name::Number(4), ty: module.types.pointer_to(module.types.i8()) } );
    assert_eq!(arg0.1, vec![]);  // should have no parameter attributes
    assert_eq!(arg1.1.len(), 1);  // should have one parameter attribute
    assert_eq!(lifetimestart.dest, None);
    assert_eq!(&format!("{}", lifetimestart), "call @llvm.lifetime.start.p0i8(i64 40, i8* %4)");
    let memset: &instruction::Call = &bbs[0].instrs[3].clone().try_into().expect("Should be a call");
    if let Either::Right(Operand::ConstantOperand(cref)) = &memset.function {
        if let Constant::GlobalReference { ref name, ref ty } = cref.as_ref() {
            assert_eq!(*name, Name::from("llvm.memset.p0i8.i64"));
            if let Type::FuncType { result_type, param_types, is_var_arg } = ty.as_ref() {
                assert_eq!(result_type, &module.types.void());
                assert_eq!(param_types, &vec![module.types.pointer_to(module.types.i8()), module.types.i8(), module.types.i64(), module.types.bool()]);
                assert_eq!(*is_var_arg, false);
            } else {
                panic!("memset.function has unexpected type {:?}", ty);
            }
        } else {
            panic!(
                "memset.function not a GlobalReference as expected; it is actually another kind of Constant: {:?}",
                cref
            );
        }
    } else {
        panic!(
            "memset.function not a GlobalReference as expected; it is actually {:?}",
            memset.function
        );
    }
    assert_eq!(memset.arguments.len(), 4);
    assert_eq!(memset.arguments[0].0, Operand::LocalOperand { name: Name::Number(4), ty: module.types.pointer_to(module.types.i8()) } );
    assert_eq!(memset.arguments[1].0, Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 8, value: 0 })));
    assert_eq!(memset.arguments[2].0, Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 64, value: 40 })));
    assert_eq!(memset.arguments[3].0, Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 1, value: 1 })));
    assert_eq!(memset.arguments[0].1.len(), 2); // should have two parameter attributes
    assert_eq!(&format!("{}", memset), "call @llvm.memset.p0i8.i64(i8* %4, i8 0, i64 40, i1 true)");
    #[cfg(feature="llvm-12-or-lower")] {
        let add: &instruction::Add = &bbs[0].instrs[4].clone().try_into().expect("Should be an add");
        assert_eq!(add.operand0, Operand::LocalOperand { name: Name::Number(1), ty: module.types.i32() } );
        assert_eq!(add.operand1, Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 32, value: 0x0000_0000_FFFF_FFFF })));
        assert_eq!(add.dest, Name::Number(5));
        assert_eq!(module.type_of(add), module.types.i32());
        assert_eq!(&format!("{}", add), "%5 = add i32 %1, i32 -1");
    }
    #[cfg(feature="llvm-13-or-greater")] {
        let add: &instruction::Add = &bbs[1].instrs[0].clone().try_into().expect("Should be an add");
        assert_eq!(add.operand0, Operand::LocalOperand { name: Name::Number(0), ty: module.types.i32() } );
        assert_eq!(add.operand1, Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 32, value: 3 })));
        assert_eq!(add.dest, Name::Number(7));
        assert_eq!(module.type_of(add), module.types.i32());
        assert_eq!(&format!("{}", add), "%7 = add i32 %0, i32 3");
    }
    #[cfg(feature="llvm-12-or-lower")] {
        let icmp: &instruction::ICmp = &bbs[0].instrs[5].clone().try_into().expect("Should be an icmp");
        assert_eq!(icmp.predicate, IntPredicate::ULT);
        assert_eq!(icmp.operand0, Operand::LocalOperand { name: Name::Number(5), ty: module.types.i32() } );
        assert_eq!(icmp.operand1, Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 32, value: 10 })));
        assert_eq!(module.type_of(icmp), module.types.bool());
        assert_eq!(&format!("{}", icmp), "%6 = icmp ult i32 %5, i32 10");
    }
    #[cfg(feature="llvm-13-or-greater")] {
        let icmp: &instruction::ICmp = &bbs[0].instrs[4].clone().try_into().expect("Should be an icmp");
        assert_eq!(icmp.predicate, IntPredicate::SLT);
        assert_eq!(icmp.operand0, Operand::LocalOperand { name: Name::Number(1), ty: module.types.i32() } );
        assert_eq!(icmp.operand1, Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 32, value: 11 })));
        assert_eq!(module.type_of(icmp), module.types.bool());
        assert_eq!(&format!("{}", icmp), "%5 = icmp slt i32 %1, i32 11");
    }
    let condbr: &terminator::CondBr = &bbs[0].term.clone().try_into().expect("Should be a condbr");
    #[cfg(feature="llvm-12-or-lower")]
    assert_eq!(condbr.condition, Operand::LocalOperand { name: Name::Number(6), ty: module.types.bool() } );
    #[cfg(feature="llvm-13-or-greater")]
    assert_eq!(condbr.condition, Operand::LocalOperand { name: Name::Number(5), ty: module.types.bool() } );
    #[cfg(feature="llvm-12-or-lower")]
    assert_eq!(condbr.true_dest, Name::Number(7));
    #[cfg(feature="llvm-13-or-greater")]
    assert_eq!(condbr.true_dest, Name::Number(6));
    let false_dest = if cfg!(feature="llvm-9-or-lower") {
        Name::Number(22)
    } else if cfg!(feature = "llvm-10") {
        Name::Number(21)
    } else if cfg!(feature = "llvm-11") {
        Name::Number(24)
    } else {
        Name::Number(47)
    };
    assert_eq!(condbr.false_dest, false_dest);
    assert_eq!(module.type_of(condbr), module.types.void());
    assert_eq!(
        &format!("{}", condbr),
        &format!(
            "br i1 %{}, label %{}, label %{}",
            if cfg!(feature="llvm-12-or-lower") { 6 } else { 5 },
            if cfg!(feature="llvm-12-or-lower") { 7 } else { 6 },
            if cfg!(feature="llvm-9-or-lower") { 22 }
            else if cfg!(feature = "llvm-10") { 21 }
            else if cfg!(feature = "llvm-11") { 24 }
            else { 47 }
        ),
    );

    // check details about certain instructions in basic block %7
    // not sure why LLVM 10+ puts a ZExt here instead of SExt. Maybe it can prove it's equivalent?
    // in LLVM 12+, the ZExt is in a different block
    #[cfg(feature="llvm-9-or-lower")]
    let ext: &instruction::SExt = &bbs[1].instrs[1].clone().try_into().expect("Should be a SExt");
    #[cfg(feature = "llvm-10")]
    let ext: &instruction::ZExt = &bbs[1].instrs[1].clone().try_into().expect("Should be a ZExt");
    #[cfg(feature="llvm-11")]
    let ext: &instruction::ZExt = &bbs[1].instrs[3].clone().try_into().expect("Should be a ZExt");
    #[cfg(feature="llvm-12-or-greater")]
    let ext: &instruction::ZExt = &bbs[2].instrs[0].clone().try_into().expect("Should be a ZExt");
    let ext_input = if cfg!(feature="llvm-10-or-lower") { Name::Number(1) } else if cfg!(any(feature="llvm-11",feature="llvm-12")) { Name::Number(10) } else { Name::Number(1) };
    let ext_dest = if cfg!(feature="llvm-10-or-lower") { Name::Number(9) } else if cfg!(feature="llvm-11") { Name::Number(11) } else { Name::Number(13) };
    assert_eq!(ext.operand, Operand::LocalOperand { name: ext_input, ty: module.types.i32() } );
    assert_eq!(ext.to_type, module.types.i64());
    assert_eq!(ext.dest, ext_dest);
    assert_eq!(module.type_of(ext), module.types.i64());
    #[cfg(feature="llvm-9-or-lower")]
    assert_eq!(&format!("{}", ext), "%9 = sext i32 %1 to i64");
    #[cfg(feature = "llvm-10")]
    assert_eq!(&format!("{}", ext), "%9 = zext i32 %1 to i64");
    #[cfg(feature="llvm-11")]
    assert_eq!(&format!("{}", ext), "%11 = zext i32 %10 to i64");
    #[cfg(feature="llvm-12")]
    assert_eq!(&format!("{}", ext), "%13 = zext i32 %10 to i64");
    #[cfg(feature="llvm-13-or-greater")]
    assert_eq!(&format!("{}", ext), "%13 = zext i32 %1 to i64");
    #[cfg(feature="llvm-9-or-lower")]
    {
        // LLVM 10 and 11 don't have a Br in this function
        let br: &terminator::Br = &bbs[1].term.clone().try_into().expect("Should be a Br");
        assert_eq!(br.dest, Name::Number(10));
        assert_eq!(&format!("{}", br), "br label %10");
    }
    #[cfg(feature="llvm-12-or-greater")]
    {
        let br: &terminator::Br = &bbs[3].term.clone().try_into().expect("Should be a Br");
        assert_eq!(br.dest, Name::Number(19));
        assert_eq!(&format!("{}", br), "br label %19");
    }

    // check details about certain instructions in basic block %10 (LLVM 9-) / %12 (LLVM 10) / %14 (LLVM 11) / %19 (LLVM 12+)
    #[cfg(feature="llvm-11-or-lower")]
    let phi: &instruction::Phi = &bbs[2].instrs[0].clone().try_into().expect("Should be a Phi");
    #[cfg(feature="llvm-12-or-greater")]
    let phi: &instruction::Phi = &bbs[4].instrs[0].clone().try_into().expect("Should be a Phi");
    let phi_dest = if cfg!(feature="llvm-9-or-lower") {
        Name::Number(11)
    } else if cfg!(feature = "llvm-10") {
        Name::Number(13)
    } else if cfg!(feature = "llvm-11") {
        Name::Number(15)
    } else {
        Name::Number(20)
    };
    assert_eq!(phi.dest, phi_dest);
    assert_eq!(phi.to_type, module.types.i64());
    #[cfg(feature="llvm-9-or-lower")]
    assert_eq!(
        phi.incoming_values,
        vec![
            (
                Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 64, value: 0 })),
                Name::Number(7)
            ),
            (
                Operand::LocalOperand { name: Name::Number(20), ty: module.types.i64() },
                Name::Number(19)
            ),
        ]
    );
    #[cfg(feature = "llvm-10")]
    assert_eq!(
        phi.incoming_values,
        vec![
            (
                Operand::LocalOperand { name: Name::Number(19), ty: module.types.i64() },
                Name::Number(12)
            ),
            (
                Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 64, value: 1 })),
                Name::Number(7)
            ),
        ]
    );
    #[cfg(feature="llvm-11")]
    assert_eq!(
        phi.incoming_values,
        vec![
            (
                Operand::LocalOperand { name: Name::Number(22), ty: module.types.i64() },
                Name::Number(14)
            ),
            (
                Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 64, value: 1 })),
                Name::Number(7)
            ),
        ]
    );
    #[cfg(feature="llvm-12-or-greater")]
    assert_eq!(
        phi.incoming_values,
        vec![
            (
                Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 64, value: 1 })),
                Name::Number(17)
            ),
            (
                Operand::LocalOperand { name: Name::Number(34), ty: module.types.i64() },
                Name::Number(19)
            ),
        ]
    );
    #[cfg(feature="llvm-9-or-lower")]
    assert_eq!(&format!("{}", phi), "%11 = phi i64 [ i64 0, %7 ], [ i64 %20, %19 ]");
    #[cfg(feature = "llvm-10")]
    assert_eq!(&format!("{}", phi), "%13 = phi i64 [ i64 %19, %12 ], [ i64 1, %7 ]");
    #[cfg(feature="llvm-11")]
    assert_eq!(&format!("{}", phi), "%15 = phi i64 [ i64 %22, %14 ], [ i64 1, %7 ]");
    #[cfg(feature="llvm-12-or-greater")]
    assert_eq!(&format!("{}", phi), "%20 = phi i64 [ i64 1, %17 ], [ i64 %34, %19 ]");

    #[cfg(feature="llvm-11-or-lower")]
    let gep: &instruction::GetElementPtr =
        &bbs[2].instrs[1].clone().try_into().expect("Should be a gep");
    #[cfg(feature="llvm-12-or-greater")]
    let gep: &instruction::GetElementPtr =
        &bbs[4].instrs[2].clone().try_into().expect("Should be a gep");
    assert_eq!(
        gep.address,
        Operand::LocalOperand {
            name: Name::Number(3),
            ty: module.types.pointer_to(allocated_type.clone())
        }
    );
    let gep_dest = if cfg!(feature="llvm-9-or-lower") {
        Name::Number(12)
    } else if cfg!(feature = "llvm-10") {
        Name::Number(14)
    } else if cfg!(feature = "llvm-11") {
        Name::Number(16)
    } else {
        Name::Number(22)
    };
    assert_eq!(gep.dest, gep_dest);
    assert_eq!(gep.in_bounds, true);
    let index = if cfg!(feature="llvm-9-or-lower") {
        Name::Number(11)
    } else if cfg!(feature = "llvm-10") {
        Name::Number(13)
    } else if cfg!(feature = "llvm-11") {
        Name::Number(15)
    } else {
        Name::Number(20)
    };
    assert_eq!(
        gep.indices,
        vec![
            Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 64, value: 0 })),
            Operand::LocalOperand {
                name: index,
                ty: module.types.i64()
            },
        ]
    );
    assert_eq!(module.type_of(gep), module.types.pointer_to(module.types.i32()));
    #[cfg(feature="llvm-9-or-lower")]
    assert_eq!(&format!("{}", gep), "%12 = getelementptr inbounds [10 x i32]* %3, i64 0, i64 %11");
    #[cfg(feature = "llvm-10")]
    assert_eq!(&format!("{}", gep), "%14 = getelementptr inbounds [10 x i32]* %3, i64 0, i64 %13");
    #[cfg(feature="llvm-11")]
    assert_eq!(&format!("{}", gep), "%16 = getelementptr inbounds [10 x i32]* %3, i64 0, i64 %15");
    #[cfg(feature="llvm-12-or-greater")]
    assert_eq!(&format!("{}", gep), "%22 = getelementptr inbounds [10 x i32]* %3, i64 0, i64 %20");
    #[cfg(feature="llvm-11-or-lower")]
    let store_inst = &bbs[2].instrs[2];
    #[cfg(feature="llvm-12-or-greater")]
    let store_inst = &bbs[4].instrs[3];
    let store: &instruction::Store = &store_inst.clone().try_into().expect("Should be a store");
    let address = if cfg!(feature="llvm-9-or-lower") {
        Name::Number(12)
    } else if cfg!(feature = "llvm-10") {
        Name::Number(14)
    } else if cfg!(feature = "llvm-11") {
        Name::Number(16)
    } else {
        Name::Number(22)
    };
    assert_eq!(store.address, Operand::LocalOperand { name: address, ty: module.types.pointer_to(module.types.i32()) });
    #[cfg(feature="llvm-12-or-lower")]
    assert_eq!(store.value, Operand::LocalOperand { name: Name::Number(8), ty: module.types.i32() });
    #[cfg(feature="llvm-13-or-greater")]
    assert_eq!(store.value, Operand::LocalOperand { name: Name::Number(7), ty: module.types.i32() });
    assert_eq!(store.volatile, true);
    assert_eq!(store.alignment, 4);
    assert_eq!(module.type_of(store), module.types.void());
    assert_eq!(store_inst.is_atomic(), false);
    #[cfg(feature="llvm-9-or-lower")]
    assert_eq!(&format!("{}", store), "store volatile i32 %8, i32* %12, align 4");
    #[cfg(feature = "llvm-10")]
    assert_eq!(&format!("{}", store), "store volatile i32 %8, i32* %14, align 4");
    #[cfg(feature="llvm-11")]
    assert_eq!(&format!("{}", store), "store volatile i32 %8, i32* %16, align 4");
    #[cfg(feature="llvm-12")]
    assert_eq!(&format!("{}", store), "store volatile i32 %8, i32* %22, align 4");
    #[cfg(feature="llvm-13-or-greater")]
    assert_eq!(&format!("{}", store), "store volatile i32 %7, i32* %22, align 4");

    // and finally other instructions of types we haven't seen yet
    let load_inst: &Instruction = if cfg!(feature="llvm-9-or-lower") {
        &bbs[3].instrs[2]
    } else if cfg!(feature = "llvm-10") {
        &bbs[2].instrs[5]
    } else if cfg!(feature = "llvm-11") {
        &bbs[2].instrs[6]
    } else {
        &bbs[4].instrs[7]
    };
    let load: &instruction::Load = &load_inst.clone().try_into().expect("Should be a load");
    let load_addr = if cfg!(feature="llvm-10-or-lower") {
        Name::Number(16)
    } else if cfg!(feature="llvm-11") {
        Name::Number(19)
    } else {
        Name::Number(25)
    };
    assert_eq!(load.address, Operand::LocalOperand { name: load_addr, ty: module.types.pointer_to(module.types.i32()) });
    #[cfg(feature="llvm-10-or-lower")]
    assert_eq!(load.dest, Name::Number(17));
    #[cfg(feature="llvm-11")]
    assert_eq!(load.dest, Name::Number(20));
    #[cfg(feature="llvm-12-or-greater")]
    assert_eq!(load.dest, Name::Number(26));
    assert_eq!(load.volatile, true);
    assert_eq!(load.alignment, 4);
    assert_eq!(module.type_of(load), module.types.i32());
    assert_eq!(load_inst.is_atomic(), false);
    #[cfg(feature="llvm-10-or-lower")]
    assert_eq!(&format!("{}", load), "%17 = load volatile i32* %16, align 4");
    #[cfg(feature="llvm-11")]
    assert_eq!(&format!("{}", load), "%20 = load volatile i32* %19, align 4");
    #[cfg(feature="llvm-12-or-greater")]
    assert_eq!(&format!("{}", load), "%26 = load volatile i32* %25, align 4");
    let ret: &Terminator = if cfg!(feature="llvm-9-or-lower") {
        &bbs[5].term
    } else if cfg!(feature="llvm-10") || cfg!(feature="llvm-11") {
        &bbs[3].term
    } else {
        &bbs[5].term
    };
    let ret: &terminator::Ret = &ret.clone().try_into().expect("Should be a ret");
    assert_eq!(ret.return_operand, None);
    assert_eq!(module.type_of(ret), module.types.void());
    assert_eq!(&format!("{}", ret), "ret void");
}

#[test]
fn switchbc() {
    init_logging();
    let path = llvm_bc_dir().join("switch.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    assert_eq!(module.functions.len(), 1);
    let func = &module.functions[0];
    assert_eq!(func.name, "has_a_switch");
    let bb = &func.basic_blocks[0];
    let switch: &terminator::Switch = &bb.term.clone().try_into().expect("Should be a switch");
    assert_eq!(switch.operand, Operand::LocalOperand { name: Name::Number(0), ty: module.types.i32() });
    assert_eq!(switch.dests.len(), 9);
    assert_eq!(switch.dests[0], (ConstantRef::new(Constant::Int { bits: 32, value: 0 }), Name::Number(12)));
    assert_eq!(switch.dests[1], (ConstantRef::new(Constant::Int { bits: 32, value: 1 }), Name::Number(2)));
    assert_eq!(switch.dests[2], (ConstantRef::new(Constant::Int { bits: 32, value: 13 }), Name::Number(3)));
    assert_eq!(switch.dests[3], (ConstantRef::new(Constant::Int { bits: 32, value: 26 }), Name::Number(4)));
    assert_eq!(switch.dests[4], (ConstantRef::new(Constant::Int { bits: 32, value: 33 }), Name::Number(5)));
    assert_eq!(switch.dests[5], (ConstantRef::new(Constant::Int { bits: 32, value: 142 }), Name::Number(6)));
    assert_eq!(switch.dests[6], (ConstantRef::new(Constant::Int { bits: 32, value: 1678 }), Name::Number(7)));
    assert_eq!(switch.dests[7], (ConstantRef::new(Constant::Int { bits: 32, value: 88 }), Name::Number(8)));
    assert_eq!(switch.dests[8], (ConstantRef::new(Constant::Int { bits: 32, value: 101 }), Name::Number(9)));
    assert_eq!(switch.default_dest, Name::Number(10));
    assert_eq!(
        &format!("{}", switch),
        "switch i32 %0, label %10 [ i32 0, label %12; i32 1, label %2; i32 13, label %3; i32 26, label %4; i32 33, label %5; i32 142, label %6; i32 1678, label %7; i32 88, label %8; i32 101, label %9; ]",
    );

    let phibb = &func
        .get_bb_by_name(&Name::Number(12))
        .expect("Failed to find bb %12");
    let phi: &instruction::Phi = &phibb.instrs[0].clone().try_into().expect("Should be a phi");
    assert_eq!(phi.incoming_values.len(), 10);
    assert_eq!(
        &format!("{}", phi),
        "%13 = phi i32 [ i32 -1, %10 ], [ i32 -3, %9 ], [ i32 0, %8 ], [ i32 77, %7 ], [ i32 -33, %6 ], [ i32 1, %5 ], [ i32 -5, %4 ], [ i32 -7, %3 ], [ i32 5, %2 ], [ i32 3, %1 ]",
    );
}

#[test]
fn variablesbc() {
    init_logging();
    let path = llvm_bc_dir().join("variables.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    assert_eq!(module.global_vars.len(), 1);
    let var = &module.global_vars[0];
    assert_eq!(var.name, Name::from("global"));
    assert_eq!(var.is_constant, false);
    assert_eq!(var.ty, module.types.pointer_to(module.types.i32()));
    assert_eq!(var.initializer, Some(ConstantRef::new(Constant::Int { bits: 32, value: 5 })));
    assert_eq!(var.alignment, 4);
    #[cfg(feature="llvm-9-or-greater")]
    assert!(var.get_debug_loc().is_none());  // this file was compiled without debuginfo

    assert_eq!(module.functions.len(), 1);
    let func = &module.functions[0];
    assert_eq!(func.name, "variables");
    let bb = &func.basic_blocks[0];
    let store: &instruction::Store = &bb.instrs[2].clone().try_into().expect("Should be a store");
    assert_eq!(store.address, Operand::LocalOperand { name: Name::Number(3), ty: module.types.pointer_to(module.types.i32()) });
    assert_eq!(module.type_of(store), module.types.void());
    assert_eq!(&format!("{}", store), "store volatile i32 %0, i32* %3, align 4");
    let load: &instruction::Load = &bb.instrs[8].clone().try_into().expect("Should be a load");
    assert_eq!(load.address, Operand::LocalOperand { name: Name::Number(4), ty: module.types.pointer_to(module.types.i32()) });
    assert_eq!(module.type_of(load), module.types.i32());
    assert_eq!(&format!("{}", load), "%8 = load volatile i32* %4, align 4");
    let global_load: &instruction::Load = &bb.instrs[14].clone().try_into().expect("Should be a load");
    assert_eq!(global_load.address, Operand::ConstantOperand(ConstantRef::new(Constant::GlobalReference { name: Name::from("global"), ty: module.types.i32() })));
    assert_eq!(module.type_of(global_load), module.types.i32());
    assert_eq!(&format!("{}", global_load), "%12 = load volatile i32* @global, align 4");
    let global_store: &instruction::Store = &bb.instrs[16].clone().try_into().expect("Should be a store");
    assert_eq!(global_store.address, Operand::ConstantOperand(ConstantRef::new(Constant::GlobalReference { name: Name::from("global"), ty: module.types.i32() })));
    assert_eq!(module.type_of(global_store), module.types.void());
    assert_eq!(&format!("{}", global_store), "store volatile i32 %13, i32* @global, align 4");
}

// this test relates to the version of the file compiled with debuginfo
#[cfg(feature="llvm-9-or-greater")]
#[test]
fn variablesbcg() {
    init_logging();
    let path = llvm_bc_dir().join("variables.bc-g");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    let debug_filename = "variables.c";
    let debug_directory = "/Users/craig/llvm-ir/tests/basic_bc"; // this is what appears in the checked-in copy of variables.bc-g

    // really all we want to check is the debugloc of the global variable.
    // other debuginfo stuff is covered in other tests
    assert_eq!(module.global_vars.len(), 1);
    let var = &module.global_vars[0];
    assert_eq!(var.name, Name::from("global"));
    let debugloc = var.get_debug_loc().as_ref().expect("expected the global to have a debugloc");
    assert_eq!(debugloc.line, 5);
    assert_eq!(debugloc.col, None);  // only `Instruction`s and `Terminator`s get column numbers
    assert_eq!(debugloc.filename, debug_filename);
    assert_eq!(debugloc.directory.as_ref().map(|s| s.as_str()), Some(debug_directory));
}

// this test checks for regression on issue #4
#[test]
fn issue4() {
    init_logging();
    let path = llvm_bc_dir().join("issue_4.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    assert_eq!(module.functions.len(), 1);
    let func = &module.functions[0];

    // not part of issue 4 proper, but let's check that we have the correct number of function attributes
    let expected_num_function_attributes = if cfg!(feature="llvm-8-or-lower") {
        21
    } else if cfg!(feature = "llvm-9") {
        // LLVM 9+ adds the attribute "nofree"
        22
    } else if cfg!(feature="llvm-10") || cfg!(feature="llvm-11") {
        // LLVM 10+ seems to have combined the two attributes
        // "no-frame-pointer-elim=true" and "no-frame-pointer-elim-non-leaf"
        // into a single attribute, "frame-pointer=all"
        21
    } else if cfg!(feature="llvm-12") {
        // LLVM 12+ adds "willreturn"
        22
    } else if cfg!(feature="llvm-13-or-greater") {
        // LLVM 13+ adds "mustprogress" and "nosync"
        // LLVM 13+ removes the following string attributes:
        //   "disable-tail-calls=false"
        //   "less-precise-fpmad=false"
        //   "no-infs-fp-math=false"
        //   "no-jump-tables=false"
        //   "no-nans-fp-math=false"
        //   "no-signed-zeroes-fp-math=false"
        //   "unsafe-fp-math=false"
        //   "use-soft-float=false"
        // for a net of -6 attributes
        16
    } else {
        panic!("Shouldn't reach this")
    };
    assert_eq!(func.function_attributes.len(), expected_num_function_attributes,
        "Expected {} function attributes but have {}: {:?}", expected_num_function_attributes, func.function_attributes.len(), func.function_attributes);
    // and that all but 6 of them are StringAttributes (5 of them for LLVM 8; 7 for LLVM 12; 9 for LLVM 13+)
    let expected_num_enum_attrs = if cfg!(feature="llvm-8-or-lower") {
        5 // missing "nofree"
    } else if cfg!(feature="llvm-9") || cfg!(feature="llvm-10") || cfg!(feature="llvm-11") {
        6
    } else if cfg!(feature="llvm-12") {
        7 // adds "willreturn"
    } else if cfg!(feature="llvm-13-or-greater") {
        9 // adds "mustprogress" and "nosync"
    } else {
        panic!("Shouldn't reach this")
    };
    let string_attrs = func.function_attributes.iter().filter(|attr| if let FunctionAttribute::StringAttribute { .. } = attr { true } else { false });
    assert_eq!(string_attrs.count(), expected_num_function_attributes - expected_num_enum_attrs);

    // now check that the first parameter has 3 attributes (4 in LLVM 11+) and the second parameter has 0
    assert_eq!(func.parameters.len(), 2);
    let first_param_attrs = &func.parameters[0].attributes;
    #[cfg(feature="llvm-10-or-lower")]
    assert_eq!(first_param_attrs.len(), 3);
    #[cfg(feature="llvm-11-or-greater")]
    assert_eq!(first_param_attrs.len(), 4);
    let second_param_attrs = &func.parameters[1].attributes;
    assert_eq!(second_param_attrs.len(), 0);

    // and that one of the parameter attributes is SRet
    #[cfg(feature="llvm-11-or-lower")]
    let is_sret = |attr: &ParameterAttribute| { match attr { ParameterAttribute::SRet => true, _ => false }};
    #[cfg(feature="llvm-12-or-greater")]
    let is_sret = |attr: &ParameterAttribute| { match attr { ParameterAttribute::SRet(_) => true, _ => false }};
    assert!(first_param_attrs.iter().any(is_sret));
}

#[test]
fn rustbc() {
    // This tests against the checked-in rust.bc, which was generated from the checked-in rust.rs with rustc 1.39.0
    init_logging();
    let path = rust_bc_dir().join("rust.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    let func = module.get_func_by_name("_ZN4rust9rust_loop17h3ed0672b8cf44eb1E").expect("Failed to find function");

    assert_eq!(func.parameters.len(), 3);
    assert_eq!(func.parameters[0].name, Name::from("a"));
    assert_eq!(func.parameters[1].name, Name::from("b"));
    assert_eq!(func.parameters[2].name, Name::from("v"));
    assert_eq!(func.parameters[0].ty, module.types.i64());
    assert_eq!(func.parameters[1].ty, module.types.i64());
    assert_eq!(func.parameters[2].ty, module.types.pointer_to(module.types.named_struct("alloc::vec::Vec<isize>")));

    let startbb = func.get_bb_by_name(&Name::from("start")).expect("Failed to find bb 'start'");
    let alloca_iter: &instruction::Alloca = &startbb.instrs[5].clone().try_into().expect("Should be an alloca");
    assert_eq!(alloca_iter.dest, Name::from("iter"));
    assert_eq!(&format!("{}", alloca_iter), "%iter = alloca { i64*, i64* }, align 8");
    let alloca_sum: &instruction::Alloca = &startbb.instrs[6].clone().try_into().expect("Should be an alloca");
    assert_eq!(alloca_sum.dest, Name::from("sum"));
    assert_eq!(&format!("{}", alloca_sum), "%sum = alloca i64, align 8");
    let store: &instruction::Store = &startbb.instrs[7].clone().try_into().expect("Should be a store");
    assert_eq!(store.address, Operand::LocalOperand { name: Name::from("sum"), ty: module.types.pointer_to(module.types.i64()) });
    assert_eq!(&format!("{}", store), "store i64 0, i64* %sum, align 8");
    let call: &instruction::Call = &startbb.instrs[8].clone().try_into().expect("Should be a call");
    let param_type = module.types.pointer_to(module.types.named_struct("alloc::vec::Vec<isize>"));
    let ret_type = module.types.struct_of(vec![
        module.types.pointer_to(module.types.array_of(module.types.i64(), 0)),
        module.types.i64(),
    ], false);
    if let Either::Right(Operand::ConstantOperand(cref)) = &call.function {
        if let Constant::GlobalReference { ref name, ref ty } = cref.as_ref() {
            assert_eq!(name, &Name::from("_ZN68_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h378128d7d9378466E"));
            match ty.as_ref() {
                Type::FuncType { result_type, param_types, is_var_arg } => {
                    assert_eq!(result_type, &ret_type);
                    assert_eq!(&param_types[0], &param_type);
                    assert_eq!(*is_var_arg, false);
                },
                _ => panic!("Expected called global to have FuncType, but got {:?}", ty),
            }
            assert_eq!(module.type_of(call), ret_type);
        } else {
            panic!(
                "call.function not a GlobalReference as expected; it is actually another kind of Constant: {:?}",
                cref
            );
        }
    } else {
        panic!(
            "call.function not a GlobalReference as expected; it is actually {:?}",
            call.function
        );
    }
    assert_eq!(call.arguments.len(), 1);
    assert_eq!(call.arguments[0].0, Operand::LocalOperand {
        name: Name::from("v"),
        ty: param_type,
    });
    assert_eq!(call.dest, Some(Name::Number(0)));
    assert_eq!(
        &format!("{}", call),
        "%0 = call @_ZN68_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h378128d7d9378466E(%alloc::vec::Vec<isize>* %v)",
    );

    #[cfg(feature="llvm-9-or-greater")]
    {
        // this file was compiled without debuginfo, so nothing should have a debugloc
        assert!(func.get_debug_loc().is_none());
        assert!(alloca_iter.get_debug_loc().is_none());
        assert!(alloca_sum.get_debug_loc().is_none());
        assert!(store.get_debug_loc().is_none());
        assert!(call.get_debug_loc().is_none());
    }
}

// this test relates to the version of the file compiled with debuginfo
#[cfg(feature="llvm-9-or-greater")]
#[test]
fn rustbcg() {
    init_logging();
    let path = rust_bc_dir().join("rust.bc-g");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    let debug_filename = "rust.rs";
    let debug_directory = "/Users/craig/llvm-ir/tests/basic_bc"; // this is what appears in the checked-in copy of rust.bc-g

    let func = module.get_func_by_name("_ZN4rust9rust_loop17h3ed0672b8cf44eb1E").expect("Failed to find function");
    let debugloc = func.get_debug_loc().as_ref().expect("Expected function to have a debugloc");
    assert_eq!(debugloc.line, 3);
    assert_eq!(debugloc.col, None);
    assert_eq!(debugloc.filename, debug_filename);
    assert_eq!(debugloc.directory.as_ref().map(|s| s.as_str()), Some(debug_directory));

    let startbb = func.get_bb_by_name(&Name::from("start")).expect("Failed to find bb 'start'");

    // the first 17 instructions in the function should not have debuglocs - they are just setting up the stack frame
    for i in 0..17 {
        assert!(startbb.instrs[i].get_debug_loc().is_none());
    }

    let store_debugloc = startbb.instrs[31].get_debug_loc().as_ref().expect("Expected this store to have a debugloc");
    assert_eq!(store_debugloc.line, 4);
    assert_eq!(store_debugloc.col, Some(18));
    assert_eq!(store_debugloc.filename, debug_filename);
    assert_eq!(debugloc.directory.as_ref().map(|s| s.as_str()), Some(debug_directory));
    assert_eq!(&format!("{}", startbb.instrs[31]), "store i64 0, i64* %sum, align 8 (with debugloc)");
    let call_debugloc = startbb.instrs[33].get_debug_loc().as_ref().expect("Expected this call to have a debugloc");
    assert_eq!(call_debugloc.line, 5);
    assert_eq!(call_debugloc.col, Some(13));
    assert_eq!(call_debugloc.filename, debug_filename);
    assert_eq!(debugloc.directory.as_ref().map(|s| s.as_str()), Some(debug_directory));
    assert_eq!(
        &format!("{}", startbb.instrs[33]),
        "%4 = call @_ZN68_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h378128d7d9378466E(%alloc::vec::Vec<isize>* %3) (with debugloc)",
    );
}

#[test]
fn simple_linked_list() {
    init_logging();
    let path = llvm_bc_dir().join("linkedlist.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");

    let struct_name: String = "struct.SimpleLinkedList".into();
    let structty = module.types.named_struct(&struct_name);
    match structty.as_ref() {
        Type::NamedStructType { name } => {
            assert_eq!(name, &struct_name);
        }
        ty => panic!("Expected {} to be NamedStructType, but got {:?}", struct_name, ty),
    }
    let structty_inner = match module.types.named_struct_def(&struct_name) {
        None => panic!(
            "Failed to find {} with module.types.named_struct_def(); have names {:?}",
            struct_name, module.types.all_struct_names().collect::<Vec<_>>()
        ),
        Some(NamedStructDef::Opaque) => panic!("{} should not be an opaque type", struct_name),
        Some(NamedStructDef::Defined(ty)) => ty,
    };
    if let Type::StructType { element_types, .. } = structty_inner.as_ref() {
        assert_eq!(element_types.len(), 2);
        assert_eq!(element_types[0], module.types.i32());
        if let Type::PointerType { pointee_type, .. } = element_types[1].as_ref() {
            if let Type::NamedStructType { name } = pointee_type.as_ref() {
                assert_eq!(name, &struct_name);
            } else {
                panic!(
                    "Expected pointee type to be a NamedStructType, got {:?}",
                    pointee_type
                );
            }
        } else {
            panic!(
                "Expected inner type to be a PointerType, got {:?}",
                element_types[1]
            );
        }
    } else {
        panic!(
            "Expected {} to be a StructType, got {:?}",
            struct_name, structty
        );
    }

    let func = module
        .get_func_by_name("simple_linked_list")
        .expect("Failed to find function");
    let alloca: &instruction::Alloca = &func.basic_blocks[0].instrs[1]
        .clone()
        .try_into()
        .expect("Should be an alloca");
    if let Type::NamedStructType { name } = alloca.allocated_type.as_ref() {
        assert_eq!(name, &struct_name);
    } else {
        panic!(
            "Expected alloca.allocated_type to be a NamedStructType, got {:?}",
            alloca.allocated_type
        );
    }
    assert_eq!(&format!("{}", alloca), "%3 = alloca %struct.SimpleLinkedList, align 8");

    let struct_name: String = "struct.SomeOpaqueStruct".into();
    let structty = module.types.named_struct(&struct_name);
    match structty.as_ref() {
        Type::NamedStructType { name } => {
            assert_eq!(name, &struct_name);
        }
        ty => panic!("Expected {} to be a NamedStructType, but got {:?}", struct_name, ty),
    }
    match module.types.named_struct_def(&struct_name) {
        None => panic!(
            "Failed to find {} with module.types.named_struct_def(); have names {:?}",
            struct_name, module.types.all_struct_names().collect::<Vec<_>>()
        ),
        Some(NamedStructDef::Opaque) => (),
        Some(NamedStructDef::Defined(def)) => panic!("{} should be an opaque type; got def {:?}", struct_name, def),
    }
    let func = module
        .get_func_by_name("takes_opaque_struct")
        .expect("Failed to find function");
    let paramty = &func.parameters[0].ty;
    match paramty.as_ref() {
        Type::PointerType { pointee_type, .. } => match pointee_type.as_ref() {
            Type::NamedStructType { name } => {
                assert_eq!(name, &struct_name);
            },
            ty => panic!("Expected parameter type to be pointer to named struct, but got pointer to {:?}", ty),
        },
        _ => panic!("Expected parameter type to be pointer type, but got {:?}", paramty),
    };
}

// this test relates to the version of the file compiled with debuginfo
#[cfg(feature="llvm-9-or-greater")]
#[test]
fn simple_linked_list_g() {
    init_logging();
    let path = llvm_bc_dir().join("linkedlist.bc-g");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    let debug_filename = "linkedlist.c";
    let debug_directory = "/Users/craig/llvm-ir/tests/basic_bc"; // this is what appears in the checked-in copy of linkedlist.bc-g

    let func = module.get_func_by_name("simple_linked_list").expect("Failed to find function");
    let debugloc = func.get_debug_loc().as_ref().expect("expected simple_linked_list to have a debugloc");
    assert_eq!(debugloc.line, 8);
    assert_eq!(debugloc.col, None);
    assert_eq!(debugloc.filename, debug_filename);
    assert_eq!(debugloc.directory.as_ref().map(|s| s.as_str()), Some(debug_directory));

    // the first seven instructions shouldn't have debuglocs - they are just setting up the stack frame
    for i in 0..7 {
        assert!(func.basic_blocks[0].instrs[i].get_debug_loc().is_none());
    }

    // the eighth instruction should have a debugloc
    let debugloc = func.basic_blocks[0].instrs[7].get_debug_loc().as_ref().expect("expected this instruction to have a debugloc");
    assert_eq!(debugloc.line, 8);
    assert_eq!(debugloc.col, Some(28));
    assert_eq!(debugloc.filename, debug_filename);
    assert_eq!(debugloc.directory.as_ref().map(|s| s.as_str()), Some(debug_directory));
    assert_eq!(
        &format!("{}", func.basic_blocks[0].instrs[7]),
        "call @llvm.dbg.declare(<metadata>, <metadata>, <metadata>) (with debugloc)",
    );

    // the tenth instruction should have a different debugloc
    let debugloc = func.basic_blocks[0].instrs[9].get_debug_loc().as_ref().expect("expected this instruction to have a debugloc");
    assert_eq!(debugloc.line, 9);
    assert_eq!(debugloc.col, Some(34));
    assert_eq!(debugloc.filename, debug_filename);
    assert_eq!(debugloc.directory.as_ref().map(|s| s.as_str()), Some(debug_directory));
    assert_eq!(
        &format!("{}", func.basic_blocks[0].instrs[9]),
        "%8 = getelementptr inbounds %struct.SimpleLinkedList* %3, i32 0, i32 0 (with debugloc)",
    );
}

#[test]
fn indirectly_recursive_type() {
    init_logging();
    let path = llvm_bc_dir().join("linkedlist.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");

    let struct_name_a: String = "struct.NodeA".into();
    let aty = module.types.named_struct(&struct_name_a);
    match aty.as_ref() {
        Type::NamedStructType { name } => {
            assert_eq!(name, &struct_name_a);
        }
        ty => panic!("Expected {} to be a NamedStructType, but got {:?}", struct_name_a, ty),
    }
    let aty_inner = match module.types.named_struct_def(&struct_name_a) {
        None => panic!(
            "Failed to find {} with module.types.named_struct_def(); have names {:?}",
            struct_name_a, module.types.all_struct_names().collect::<Vec<_>>()
        ),
        Some(NamedStructDef::Opaque) => panic!("{} should not be an opaque type", &struct_name_a),
        Some(NamedStructDef::Defined(ty)) => ty,
    };
    let struct_name_b: String = "struct.NodeB".into();
    let bty = module.types.named_struct(&struct_name_b);
    match bty.as_ref() {
        Type::NamedStructType { name } => {
            assert_eq!(name, &struct_name_b);
        }
        ty => panic!("Expected {} to be a NamedStructType, but got {:?}", struct_name_b, ty),
    }
    let bty_inner = match module.types.named_struct_def(&struct_name_b) {
        None => panic!(
            "Failed to find {} with module.types.named_struct_def(); have names {:?}",
            struct_name_b, module.types.all_struct_names().collect::<Vec<_>>()
        ),
        Some(NamedStructDef::Opaque) => panic!("{} should not be an opaque type", &struct_name_b),
        Some(NamedStructDef::Defined(ty)) => ty,
    };
    if let Type::StructType { element_types, .. } = aty_inner.as_ref() {
        assert_eq!(element_types.len(), 2);
        assert_eq!(element_types[0], module.types.i32());
        if let Type::PointerType { pointee_type, .. } = element_types[1].as_ref() {
            if let Type::NamedStructType { name } = pointee_type.as_ref() {
                assert_eq!(name, &struct_name_b);
            } else {
                panic!(
                    "Expected pointee type to be a NamedStructType, got {:?}",
                    pointee_type.as_ref()
                );
            }
        } else {
            panic!(
                "Expected inner type to be a PointerType, got {:?}",
                element_types[1]
            );
        }
    } else {
        panic!("Expected NodeA inner type to be a StructType, got {:?}", aty);
    }
    if let Type::StructType { element_types, .. } = bty_inner.as_ref() {
        assert_eq!(element_types.len(), 2);
        assert_eq!(element_types[0], module.types.i32());
        if let Type::PointerType { pointee_type, .. } = element_types[1].as_ref() {
            if let Type::NamedStructType { name } = pointee_type.as_ref() {
                assert_eq!(name, &struct_name_a);
            } else {
                panic!(
                    "Expected pointee type to be a NamedStructType, got {:?}",
                    pointee_type.as_ref()
                );
            }
        } else {
            panic!(
                "Expected inner type to be a PointerType, got {:?}",
                element_types[1]
            );
        }
    } else {
        panic!("Expected NodeB inner type to be a StructType, got {:?}", bty);
    }

    let func = module
        .get_func_by_name("indirectly_recursive_type")
        .expect("Failed to find function");
    let alloca_a: &instruction::Alloca = &func.basic_blocks[0].instrs[1]
        .clone()
        .try_into()
        .expect("Should be an alloca");
    let alloca_b: &instruction::Alloca = &func.basic_blocks[0].instrs[2]
        .clone()
        .try_into()
        .expect("Should be an alloca");
    if let Type::NamedStructType { name } = alloca_a.allocated_type.as_ref() {
        assert_eq!(name, &struct_name_a);
    } else {
        panic!(
            "Expected alloca_a.allocated_type to be a NamedStructType, got {:?}",
            alloca_a.allocated_type
        );
    }
    if let Type::NamedStructType { name } = alloca_b.allocated_type.as_ref() {
        assert_eq!(name, &struct_name_b);
    } else {
        panic!(
            "Expected alloca_b.allocated_type to be a NamedStructType, got {:?}",
            alloca_b.allocated_type
        );
    }
    assert_eq!(&format!("{}", alloca_a), "%3 = alloca %struct.NodeA, align 8");
    assert_eq!(&format!("{}", alloca_b), "%4 = alloca %struct.NodeB, align 8");
}

#[test]
fn param_and_func_attributes() {
    let _ = env_logger::builder().is_test(true).try_init(); // capture log messages with test harness
    let path = llvm_bc_dir().join("param_and_func_attributes.ll.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");

    // Return attributes
    let zeroext_fn = module.get_func_by_name("f.zeroext").unwrap();
    assert_eq!(zeroext_fn.return_attributes.len(), 1);
    assert_eq!(zeroext_fn.return_attributes[0], ParameterAttribute::ZeroExt);
    let signext_fn = module.get_func_by_name("f.signext").unwrap();
    assert_eq!(signext_fn.return_attributes.len(), 1);
    assert_eq!(signext_fn.return_attributes[0], ParameterAttribute::SignExt);
    let inreg_fn = module.get_func_by_name("f.inreg").unwrap();
    assert_eq!(inreg_fn.return_attributes.len(), 1);
    assert_eq!(inreg_fn.return_attributes[0], ParameterAttribute::InReg);
    let noalias_fn = module.get_func_by_name("f.noalias").unwrap();
    assert_eq!(noalias_fn.return_attributes.len(), 1);
    assert_eq!(noalias_fn.return_attributes[0], ParameterAttribute::NoAlias);
    let nonnull_fn = module.get_func_by_name("f.nonnull").unwrap();
    assert_eq!(nonnull_fn.return_attributes.len(), 1);
    assert_eq!(nonnull_fn.return_attributes[0], ParameterAttribute::NonNull);
    let deref4_fn = module.get_func_by_name("f.dereferenceable4").unwrap();
    assert_eq!(deref4_fn.return_attributes.len(), 1);
    assert_eq!(deref4_fn.return_attributes[0], ParameterAttribute::Dereferenceable(4));
    let deref8_fn = module.get_func_by_name("f.dereferenceable8").unwrap();
    assert_eq!(deref8_fn.return_attributes.len(), 1);
    assert_eq!(deref8_fn.return_attributes[0], ParameterAttribute::Dereferenceable(8));
    let deref4ornull_fn = module.get_func_by_name("f.dereferenceable4_or_null").unwrap();
    assert_eq!(deref4ornull_fn.return_attributes.len(), 1);
    assert_eq!(deref4ornull_fn.return_attributes[0], ParameterAttribute::DereferenceableOrNull(4));
    let deref8ornull_fn = module.get_func_by_name("f.dereferenceable8_or_null").unwrap();
    assert_eq!(deref8ornull_fn.return_attributes.len(), 1);
    assert_eq!(deref8ornull_fn.return_attributes[0], ParameterAttribute::DereferenceableOrNull(8));

    // Parameter attributes
    let f = module.get_func_by_name("f.param.zeroext").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    assert_eq!(param.attributes[0], ParameterAttribute::ZeroExt);
    let f = module.get_func_by_name("f.param.signext").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    assert_eq!(param.attributes[0], ParameterAttribute::SignExt);
    let f = module.get_func_by_name("f.param.inreg").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    assert_eq!(param.attributes[0], ParameterAttribute::InReg);
    let f = module.get_func_by_name("f.param.byval").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    #[cfg(feature="llvm-8-or-lower")]
    assert_eq!(param.attributes[0], ParameterAttribute::ByVal);
    #[cfg(all(feature="llvm-9-or-greater",feature="llvm-11-or-lower"))]
    assert_eq!(param.attributes[0], ParameterAttribute::UnknownAttribute);
    #[cfg(feature="llvm-12-or-greater")]
    match &param.attributes[0] {
        ParameterAttribute::ByVal(ty) => match ty.as_ref() {
            Type::StructType { element_types, is_packed: false } => {
                assert_eq!(element_types.len(), 2);
            }
            ty => panic!("Expected a StructType with is_packed=false, got {:?}", ty)
        }
        attr => panic!("Expected a ByVal, got {:?}", attr)
    }
    let f = module.get_func_by_name("f.param.inalloca").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    #[cfg(feature="llvm-12-or-lower")]
    assert_eq!(param.attributes[0], ParameterAttribute::InAlloca);
    #[cfg(feature="llvm-13-or-greater")]
    match &param.attributes[0] {
        ParameterAttribute::InAlloca(ty) => match ty.as_ref() {
            Type::IntegerType { bits: 8 } => {}
            ty => panic!("Expected i8, got {:?}", ty)
        }
        attr => panic!("Expected an InAlloca, got {:?}", attr)
    }
    let f = module.get_func_by_name("f.param.sret").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    #[cfg(feature="llvm-11-or-lower")]
    assert_eq!(param.attributes[0], ParameterAttribute::SRet);
    #[cfg(feature="llvm-12-or-greater")]
    match &param.attributes[0] {
        ParameterAttribute::SRet(ty) => match ty.as_ref() {
            Type::IntegerType { bits: 8 } => {}
            ty => panic!("Expected i8, got {:?}", ty)
        }
        attr => panic!("Expected an SRet, got {:?}", attr)
    }
    let f = module.get_func_by_name("f.param.noalias").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    assert_eq!(param.attributes[0], ParameterAttribute::NoAlias);
    let f = module.get_func_by_name("f.param.nocapture").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    assert_eq!(param.attributes[0], ParameterAttribute::NoCapture);
    let f = module.get_func_by_name("f.param.nest").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    assert_eq!(param.attributes[0], ParameterAttribute::Nest);
    let f = module.get_func_by_name("f.param.returned").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    assert_eq!(param.attributes[0], ParameterAttribute::Returned);
    let f = module.get_func_by_name("f.param.nonnull").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    assert_eq!(param.attributes[0], ParameterAttribute::NonNull);
    let f = module.get_func_by_name("f.param.dereferenceable").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    assert_eq!(param.attributes[0], ParameterAttribute::Dereferenceable(4));
    let f = module.get_func_by_name("f.param.dereferenceable_or_null").unwrap();
    assert_eq!(f.parameters.len(), 1);
    let param = &f.parameters[0];
    assert_eq!(param.attributes.len(), 1);
    assert_eq!(param.attributes[0], ParameterAttribute::DereferenceableOrNull(4));

    // Function attributes
    let f = module.get_func_by_name("f.alignstack4").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::AlignStack(4));
    let f = module.get_func_by_name("f.alignstack8").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::AlignStack(8));
    let f = module.get_func_by_name("f.alwaysinline").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::AlwaysInline);
    let f = module.get_func_by_name("f.cold").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::Cold);
    let f = module.get_func_by_name("f.convergent").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::Convergent);
    let f = module.get_func_by_name("f.inlinehint").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::InlineHint);
    let f = module.get_func_by_name("f.jumptable").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::JumpTable);
    let f = module.get_func_by_name("f.minsize").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::MinimizeSize);
    let f = module.get_func_by_name("f.naked").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::Naked);
    let f = module.get_func_by_name("f.nobuiltin").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::NoBuiltin);
    let f = module.get_func_by_name("f.noduplicate").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::NoDuplicate);
    let f = module.get_func_by_name("f.noimplicitfloat").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::NoImplicitFloat);
    let f = module.get_func_by_name("f.nonlazybind").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::NonLazyBind);
    let f = module.get_func_by_name("f.noredzone").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::NoRedZone);
    let f = module.get_func_by_name("f.noreturn").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::NoReturn);
    let f = module.get_func_by_name("f.nounwind").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::NoUnwind);
    let f = module.get_func_by_name("f.optnone").unwrap();
    assert_eq!(f.function_attributes.len(), 2);
    assert_eq!(f.function_attributes[0], FunctionAttribute::NoInline);
    assert_eq!(f.function_attributes[1], FunctionAttribute::OptNone);
    let f = module.get_func_by_name("f.optsize").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::OptSize);
    let f = module.get_func_by_name("f.readnone").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::ReadNone);
    let f = module.get_func_by_name("f.readonly").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::ReadOnly);
    let f = module.get_func_by_name("f.returns_twice").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::ReturnsTwice);
    let f = module.get_func_by_name("f.safestack").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::SafeStack);
    let f = module.get_func_by_name("f.sanitize_address").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::SanitizeAddress);
    let f = module.get_func_by_name("f.sanitize_memory").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::SanitizeMemory);
    let f = module.get_func_by_name("f.sanitize_thread").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::SanitizeThread);
    let f = module.get_func_by_name("f.ssp").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::StackProtect);
    let f = module.get_func_by_name("f.sspreq").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::StackProtectReq);
    let f = module.get_func_by_name("f.sspstrong").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::StackProtectStrong);
    let f = module.get_func_by_name("f.thunk").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::StringAttribute { kind: "thunk".into(), value: "".into() });
    let f = module.get_func_by_name("f.uwtable").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::UWTable);
    let f = module.get_func_by_name("f.kvpair").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::StringAttribute { kind: "cpu".into(), value: "cortex-a8".into() });
    let f = module.get_func_by_name("f.norecurse").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::NoRecurse);
    let f = module.get_func_by_name("f.inaccessiblememonly").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::InaccessibleMemOnly);
    let f = module.get_func_by_name("f.inaccessiblemem_or_argmemonly").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::InaccessibleMemOrArgMemOnly);
    let f = module.get_func_by_name("f.strictfp").unwrap();
    assert_eq!(f.function_attributes.len(), 1);
    assert_eq!(f.function_attributes[0], FunctionAttribute::StrictFP);
}

#[cfg(feature="llvm-11-or-greater")]
#[test]
fn float_types() {
    init_logging();
    let path = llvm_bc_dir().join("float_types.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");

    let f = module.get_func_by_name("takes_half").unwrap();
    assert_eq!(module.type_of(f.parameters.get(0).unwrap()), module.types.fp(FPType::Half));
    let f = module.get_func_by_name("takes_bfloat").unwrap();
    assert_eq!(module.type_of(f.parameters.get(0).unwrap()), module.types.fp(FPType::BFloat));
    let f = module.get_func_by_name("takes_float").unwrap();
    assert_eq!(module.type_of(f.parameters.get(0).unwrap()), module.types.fp(FPType::Single));
    let f = module.get_func_by_name("takes_double").unwrap();
    assert_eq!(module.type_of(f.parameters.get(0).unwrap()), module.types.fp(FPType::Double));
    let f = module.get_func_by_name("takes_fp128").unwrap();
    assert_eq!(module.type_of(f.parameters.get(0).unwrap()), module.types.fp(FPType::FP128));
    let f = module.get_func_by_name("takes_x86_fp80").unwrap();
    assert_eq!(module.type_of(f.parameters.get(0).unwrap()), module.types.fp(FPType::X86_FP80));
    let f = module.get_func_by_name("takes_ppc_fp128").unwrap();
    assert_eq!(module.type_of(f.parameters.get(0).unwrap()), module.types.fp(FPType::PPC_FP128));

    let f = module.get_func_by_name("returns_half").unwrap();
    assert_eq!(f.return_type, module.types.pointer_to(module.types.fp(FPType::Half)));
    let f = module.get_func_by_name("returns_bfloat").unwrap();
    assert_eq!(f.return_type, module.types.pointer_to(module.types.fp(FPType::BFloat)));
    let f = module.get_func_by_name("returns_float").unwrap();
    assert_eq!(f.return_type, module.types.pointer_to(module.types.fp(FPType::Single)));
    let f = module.get_func_by_name("returns_double").unwrap();
    assert_eq!(f.return_type, module.types.pointer_to(module.types.fp(FPType::Double)));
    let f = module.get_func_by_name("returns_fp128").unwrap();
    assert_eq!(f.return_type, module.types.pointer_to(module.types.fp(FPType::FP128)));
    let f = module.get_func_by_name("returns_x86_fp80").unwrap();
    assert_eq!(f.return_type, module.types.pointer_to(module.types.fp(FPType::X86_FP80)));
    let f = module.get_func_by_name("returns_ppc_fp128").unwrap();
    assert_eq!(f.return_type, module.types.pointer_to(module.types.fp(FPType::PPC_FP128)));
}

#[test]
fn datalayouts() {
    init_logging();
    let path = llvm_bc_dir().join("hello.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    let data_layout = &module.data_layout;

    #[cfg(feature="llvm-10-or-greater")] {
        assert_eq!(&data_layout.layout_str, "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128");
        assert_eq!(&data_layout.endianness, &Endianness::LittleEndian);
        assert_eq!(&data_layout.mangling, &Some(Mangling::MachO));
        assert_eq!(data_layout.alignments.ptr_alignment(270), &PointerLayout { size: 32, alignment: Alignment { abi: 32, pref: 32 }, index_size: 32 });
        assert_eq!(data_layout.alignments.ptr_alignment(271), &PointerLayout { size: 32, alignment: Alignment { abi: 32, pref: 32 }, index_size: 32 });
        assert_eq!(data_layout.alignments.ptr_alignment(272), &PointerLayout { size: 64, alignment: Alignment { abi: 64, pref: 64 }, index_size: 64 });
        assert_eq!(data_layout.alignments.ptr_alignment(0), &PointerLayout { size: 64, alignment: Alignment { abi: 64, pref: 64 }, index_size: 64 });
        assert_eq!(data_layout.alignments.ptr_alignment(33), &PointerLayout { size: 64, alignment: Alignment { abi: 64, pref: 64 }, index_size: 64 });
        assert_eq!(data_layout.alignments.int_alignment(64), &Alignment { abi: 64, pref: 64 });
        assert_eq!(data_layout.alignments.int_alignment(7), &Alignment { abi: 8, pref: 8 });
        assert_eq!(data_layout.alignments.int_alignment(26), &Alignment { abi: 32, pref: 32 });
        assert_eq!(data_layout.alignments.int_alignment(123456), &Alignment { abi: 64, pref: 64 });
        assert_eq!(data_layout.alignments.fp_alignment(FPType::Double), &Alignment { abi: 64, pref: 64 });
        assert_eq!(data_layout.alignments.fp_alignment(FPType::X86_FP80), &Alignment { abi: 128, pref: 128 });
        assert_eq!(data_layout.native_int_widths.as_ref().unwrap().iter().copied().sorted().collect::<Vec<_>>(), vec![8, 16, 32, 64]);
        assert_eq!(data_layout.stack_alignment, Some(128));
    }
    #[cfg(feature="llvm-9-or-lower")] {
        assert_eq!(&data_layout.layout_str, "e-m:o-i64:64-f80:128-n8:16:32:64-S128");
        assert_eq!(&data_layout.endianness, &Endianness::LittleEndian);
        assert_eq!(&data_layout.mangling, &Some(Mangling::MachO));
        assert_eq!(data_layout.alignments.ptr_alignment(0), &PointerLayout { size: 64, alignment: Alignment { abi: 64, pref: 64 }, index_size: 64 });
        assert_eq!(data_layout.alignments.ptr_alignment(33), &PointerLayout { size: 64, alignment: Alignment { abi: 64, pref: 64 }, index_size: 64 });
        assert_eq!(data_layout.alignments.int_alignment(64), &Alignment { abi: 64, pref: 64 });
        assert_eq!(data_layout.alignments.int_alignment(7), &Alignment { abi: 8, pref: 8 });
        assert_eq!(data_layout.alignments.int_alignment(26), &Alignment { abi: 32, pref: 32 });
        assert_eq!(data_layout.alignments.int_alignment(123456), &Alignment { abi: 64, pref: 64 });
        assert_eq!(data_layout.alignments.fp_alignment(FPType::Double), &Alignment { abi: 64, pref: 64 });
        assert_eq!(data_layout.alignments.fp_alignment(FPType::X86_FP80), &Alignment { abi: 128, pref: 128 });
        assert_eq!(data_layout.native_int_widths.as_ref().unwrap().iter().copied().sorted().collect::<Vec<_>>(), vec![8, 16, 32, 64]);
        assert_eq!(data_layout.stack_alignment, Some(128));
    }

    assert_eq!(
        data_layout.alignments.type_alignment(&module.types.int(26)),
        &Alignment { abi: 32, pref: 32 }
    );
    assert_eq!(
        data_layout.alignments.type_alignment(&module.types.pointer_in_addr_space(module.types.int(32), 2)),
        &Alignment { abi: 64, pref: 64 }
    );
    assert_eq!(
        data_layout.alignments.type_alignment(&module.types.pointer_to(module.types.func_type(module.types.void(), vec![], false))),
        &Alignment { abi: 64, pref: 64 }
    )
}
