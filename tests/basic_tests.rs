use either::Either;
use llvm_ir::instruction;
use llvm_ir::terminator;
use llvm_ir::Constant;
use llvm_ir::IntPredicate;
use llvm_ir::Module;
use llvm_ir::Name;
use llvm_ir::Operand;
use llvm_ir::Type;
use llvm_ir::Typed;
use std::convert::TryInto;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, RwLock};

fn init_logging() {
    // capture log messages with test harness
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn hellobc() {
    init_logging();
    let path = Path::new("tests/basic_bc/hello.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    assert_eq!(module.name, "tests/basic_bc/hello.bc");
    assert_eq!(module.source_file_name, "hello.c");
    assert_eq!(module.target_triple, Some("x86_64-apple-macosx10.14.0".to_owned()));
    assert_eq!(module.functions.len(), 1);
    let func = &module.functions[0];
    assert_eq!(func.name, "main");
    assert_eq!(func.parameters.len(), 0);
    assert_eq!(func.is_var_arg, false);
    assert_eq!(func.return_type, Type::IntegerType { bits: 32 });
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
        Some(Operand::ConstantOperand(Constant::Int {
            bits: 32,
            value: 0
        }))
    );
}

#[test]
#[allow(clippy::cognitive_complexity)]
fn loopbc() {
    init_logging();
    let path = Path::new("tests/basic_bc/loop.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");

    // get function and check info on it
    assert_eq!(module.functions.len(), 1);
    let func = &module.functions[0];
    assert_eq!(func.name, "loop");
    assert_eq!(func.parameters.len(), 2);
    assert_eq!(func.is_var_arg, false);
    assert_eq!(func.return_type, Type::VoidType);
    assert_eq!(
        func.get_type(),
        Type::FuncType {
            result_type: Box::new(Type::VoidType),
            param_types: vec![Type::i32(), Type::i32()],
            is_var_arg: false,
        }
    );
    assert_eq!(module.get_func_by_name("loop"), Some(func));

    // get parameters and check info on them
    let param0 = &func.parameters[0];
    let param1 = &func.parameters[1];
    assert_eq!(param0.name, Name::Number(0));
    assert_eq!(param1.name, Name::Number(1));
    assert_eq!(param0.ty, Type::i32());
    assert_eq!(param1.ty, Type::i32());
    assert_eq!(param0.get_type(), Type::i32());
    assert_eq!(param1.get_type(), Type::i32());

    // get basic blocks and check their names
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

    // check details about the instructions in basic block %2
    let alloca: &instruction::Alloca = &bb2.instrs[0]
        .clone()
        .try_into()
        .expect("Should be an alloca");
    assert_eq!(alloca.dest, Name::Number(3));
    let allocated_type = Type::ArrayType {
        element_type: Box::new(Type::i32()),
        num_elements: 10,
    };
    assert_eq!(alloca.allocated_type, allocated_type);
    assert_eq!(
        alloca.num_elements,
        Operand::ConstantOperand(Constant::Int { bits: 32, value: 1 }) // One element, which is an array of 10 elements. Not 10 elements, each of which are i32.
    );
    assert_eq!(alloca.alignment, 16);
    assert_eq!(alloca.get_type(), Type::pointer_to(allocated_type.clone()));
    assert_eq!(alloca.num_elements.get_type(), Type::i32());
    let bitcast: &instruction::BitCast = &bb2.instrs[1]
        .clone()
        .try_into()
        .expect("Should be a bitcast");
    assert_eq!(bitcast.dest, Name::Number(4));
    assert_eq!(bitcast.to_type, Type::pointer_to(Type::i8()));
    assert_eq!(
        bitcast.operand,
        Operand::LocalOperand {
            name: Name::Number(3),
            ty: Type::pointer_to(allocated_type.clone())
        }
    );
    assert_eq!(bitcast.get_type(), Type::pointer_to(Type::i8()));
    assert_eq!(bitcast.operand.get_type(), Type::pointer_to(allocated_type.clone()));
    let lifetimestart: &instruction::Call =
        &bb2.instrs[2].clone().try_into().expect("Should be a call");
    if let Either::Right(Operand::ConstantOperand(Constant::GlobalReference { ref name, ref ty } )) = lifetimestart.function {
        assert_eq!(lifetimestart.function.get_type(), Type::pointer_to(ty.clone()));  // lifetimestart.function should be a constant function pointer
        assert_eq!(*name, Name::Name("llvm.lifetime.start.p0i8".to_owned()));
        if let Type::FuncType { ref result_type, ref param_types, ref is_var_arg } = *ty {
            assert_eq!(**result_type, Type::VoidType);
            assert_eq!(*param_types, vec![Type::i64(), Type::pointer_to(Type::i8())]);
            assert_eq!(*is_var_arg, false);
        } else {
            panic!("lifetimestart.function has unexpected type {:?}", ty);
        }
    } else {
        panic!(
            "lifetimestart.function not a GlobalReference as expected; it is actually {:?}",
            &lifetimestart.function
        );
    }
    let arg0 = &lifetimestart.arguments.get(0).expect("Expected an argument 0");
    let arg1 = &lifetimestart.arguments.get(1).expect("Expected an argument 1");
    assert_eq!(arg0.0, Operand::ConstantOperand(Constant::Int { bits: 64, value: 40 } ));
    assert_eq!(arg1.0, Operand::LocalOperand { name: Name::Number(4), ty: Type::pointer_to(Type::i8()) } );
    assert_eq!(arg0.1, vec![]);  // should have no parameter attributes
    assert_eq!(arg1.1.len(), 1);  // should have one parameter attribute
    assert_eq!(lifetimestart.dest, None);
    let memset: &instruction::Call = &bb2.instrs[3].clone().try_into().expect("Should be a call");
    if let Either::Right(Operand::ConstantOperand(Constant::GlobalReference { ref name, ref ty })) = memset.function {
        assert_eq!(*name, Name::Name("llvm.memset.p0i8.i64".to_owned()));
        if let Type::FuncType { ref result_type, ref param_types, ref is_var_arg } = *ty {
            assert_eq!(**result_type, Type::VoidType);
            assert_eq!(*param_types, vec![Type::pointer_to(Type::i8()), Type::i8(), Type::i64(), Type::bool()]);
            assert_eq!(*is_var_arg, false);
        } else {
            panic!("memset.function has unexpected type {:?}", ty);
        }
    } else {
        panic!(
            "memset.function not a GlobalReference as expected; it is actually {:?}",
            memset.function
        );
    }
    assert_eq!(memset.arguments.len(), 4);
    assert_eq!(memset.arguments[0].0, Operand::LocalOperand { name: Name::Number(4), ty: Type::pointer_to(Type::i8()) } );
    assert_eq!(memset.arguments[1].0, Operand::ConstantOperand(Constant::Int { bits: 8, value: 0 } ));
    assert_eq!(memset.arguments[2].0, Operand::ConstantOperand(Constant::Int { bits: 64, value: 40 } ));
    assert_eq!(memset.arguments[3].0, Operand::ConstantOperand(Constant::Int { bits: 1, value: 1 } ));
    assert_eq!(memset.arguments[0].1.len(), 2); // should have two parameter attributes
    let add: &instruction::Add = &bb2.instrs[4].clone().try_into().expect("Should be an add");
    assert_eq!(add.operand0, Operand::LocalOperand { name: Name::Number(1), ty: Type::i32() } );
    assert_eq!(add.operand1, Operand::ConstantOperand(Constant::Int { bits: 32, value: 0x0000_0000_FFFF_FFFF }));
    assert_eq!(add.dest, Name::Number(5));
    assert_eq!(add.get_type(), Type::i32());
    let icmp: &instruction::ICmp = &bb2.instrs[5].clone().try_into().expect("Should be an icmp");
    assert_eq!(icmp.predicate, IntPredicate::ULT);
    assert_eq!(icmp.operand0, Operand::LocalOperand { name: Name::Number(5), ty: Type::i32() } );
    assert_eq!(icmp.operand1, Operand::ConstantOperand(Constant::Int { bits: 32, value: 10 }));
    assert_eq!(icmp.get_type(), Type::bool());
    let condbr: &terminator::CondBr = &bb2.term.clone().try_into().expect("Should be a condbr");
    assert_eq!(condbr.condition, Operand::LocalOperand { name: Name::Number(6), ty: Type::bool() } );
    assert_eq!(condbr.true_dest, Name::Number(7));
    assert_eq!(condbr.false_dest, Name::Number(22));
    assert_eq!(condbr.get_type(), Type::VoidType);

    // check details about certain instructions in basic block %7
    let sext: &instruction::SExt = &bb7.instrs[1].clone().try_into().expect("Should be a SExt");
    assert_eq!(sext.operand, Operand::LocalOperand { name: Name::Number(1), ty: Type::i32() } );
    assert_eq!(sext.to_type, Type::i64());
    assert_eq!(sext.dest, Name::Number(9));
    assert_eq!(sext.get_type(), Type::i64());
    let br: &terminator::Br = &bb7.term.clone().try_into().expect("Should be a Br");
    assert_eq!(br.dest, Name::Number(10));

    // check details about certain instructions in basic block %10
    let phi: &instruction::Phi = &bb10.instrs[0].clone().try_into().expect("Should be a Phi");
    assert_eq!(phi.dest, Name::Number(11));
    assert_eq!(phi.to_type, Type::i64());
    assert_eq!(
        phi.incoming_values,
        vec![
            (
                Operand::ConstantOperand(Constant::Int { bits: 64, value: 0 }),
                Name::Number(7)
            ),
            (
                Operand::LocalOperand { name: Name::Number(20), ty: Type::i64() },
                Name::Number(19)
            ),
        ]
    );
    let gep: &instruction::GetElementPtr =
        &bb10.instrs[1].clone().try_into().expect("Should be a gep");
    assert_eq!(
        gep.address,
        Operand::LocalOperand {
            name: Name::Number(3),
            ty: Type::pointer_to(allocated_type.clone())
        }
    );
    assert_eq!(gep.dest, Name::Number(12));
    assert_eq!(gep.in_bounds, true);
    assert_eq!(
        gep.indices,
        vec![
            Operand::ConstantOperand(Constant::Int { bits: 64, value: 0 }),
            Operand::LocalOperand {
                name: Name::Number(11),
                ty: Type::i64()
            },
        ]
    );
    assert_eq!(gep.get_type(), Type::pointer_to(Type::i32()));
    let store: &instruction::Store = &bb10.instrs[2]
        .clone()
        .try_into()
        .expect("Should be a store");
    assert_eq!(store.address, Operand::LocalOperand { name: Name::Number(12), ty: Type::pointer_to(Type::i32()) });
    assert_eq!(store.value, Operand::LocalOperand { name: Name::Number(8), ty: Type::i32() });
    assert_eq!(store.volatile, true);
    assert_eq!(store.alignment, 4);
    assert_eq!(store.get_type(), Type::VoidType);
    assert_eq!(bb10.instrs[2].is_atomic(), false);

    // and finally other instructions of types we haven't seen yet
    let load: &instruction::Load = &bb14.instrs[2].clone().try_into().expect("Should be a load");
    assert_eq!(load.address, Operand::LocalOperand { name: Name::Number(16), ty: Type::pointer_to(Type::i32()) });
    assert_eq!(load.dest, Name::Number(17));
    assert_eq!(load.volatile, true);
    assert_eq!(load.alignment, 4);
    assert_eq!(load.get_type(), Type::i32());
    assert_eq!(bb14.instrs[2].is_atomic(), false);
    let ret: &terminator::Ret = &bb22.term.clone().try_into().expect("Should be a ret");
    assert_eq!(ret.return_operand, None);
    assert_eq!(ret.get_type(), Type::VoidType);
}

#[test]
fn switchbc() {
    init_logging();
    let path = Path::new("tests/basic_bc/switch.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    assert_eq!(module.functions.len(), 1);
    let func = &module.functions[0];
    assert_eq!(func.name, "has_a_switch");
    let bb = &func.basic_blocks[0];
    let switch: &terminator::Switch = &bb.term.clone().try_into().expect("Should be a switch");
    assert_eq!(switch.operand, Operand::LocalOperand { name: Name::Number(0), ty: Type::i32() });
    assert_eq!(switch.dests.len(), 9);
    assert_eq!(switch.dests[0], (Constant::Int { bits: 32, value: 0 }, Name::Number(12)));
    assert_eq!(switch.dests[1], (Constant::Int { bits: 32, value: 1 }, Name::Number(2)));
    assert_eq!(switch.dests[2], (Constant::Int { bits: 32, value: 13 }, Name::Number(3)));
    assert_eq!(switch.dests[3], (Constant::Int { bits: 32, value: 26 }, Name::Number(4)));
    assert_eq!(switch.dests[4], (Constant::Int { bits: 32, value: 33 }, Name::Number(5)));
    assert_eq!(switch.dests[5], (Constant::Int { bits: 32, value: 142 }, Name::Number(6)));
    assert_eq!(switch.dests[6], (Constant::Int { bits: 32, value: 1678 }, Name::Number(7)));
    assert_eq!(switch.dests[7], (Constant::Int { bits: 32, value: 88 }, Name::Number(8)));
    assert_eq!(switch.dests[8], (Constant::Int { bits: 32, value: 101 }, Name::Number(9)));
    assert_eq!(switch.default_dest, Name::Number(10));

    let phibb = &func
        .get_bb_by_name(&Name::Number(12))
        .expect("Failed to find bb %12");
    let phi: &instruction::Phi = &phibb.instrs[0].clone().try_into().expect("Should be a phi");
    assert_eq!(phi.incoming_values.len(), 10);
}

#[test]
fn variablesbc() {
    init_logging();
    let path = Path::new("tests/basic_bc/variables.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");
    assert_eq!(module.global_vars.len(), 1);
    let var = &module.global_vars[0];
    assert_eq!(var.name, Name::Name("global".to_owned()));
    assert_eq!(var.is_constant, false);
    assert_eq!(var.ty, Type::pointer_to(Type::i32()));
    assert_eq!(var.initializer, Some(Constant::Int { bits: 32, value: 5 }));
    assert_eq!(var.alignment, 4);

    assert_eq!(module.functions.len(), 1);
    let func = &module.functions[0];
    assert_eq!(func.name, "variables");
    let bb = &func.basic_blocks[0];
    let store: &instruction::Store = &bb.instrs[2].clone().try_into().expect("Should be a store");
    assert_eq!(store.address, Operand::LocalOperand { name: Name::Number(3), ty: Type::pointer_to(Type::i32()) });
    assert_eq!(store.get_type(), Type::VoidType);
    let load: &instruction::Load = &bb.instrs[8].clone().try_into().expect("Should be a load");
    assert_eq!(load.address, Operand::LocalOperand { name: Name::Number(4), ty: Type::pointer_to(Type::i32()) });
    assert_eq!(load.get_type(), Type::i32());
    let global_load: &instruction::Load = &bb.instrs[14].clone().try_into().expect("Should be a load");
    assert_eq!(global_load.address, Operand::ConstantOperand(Constant::GlobalReference { name: Name::Name("global".to_owned()), ty: Type::i32() }));
    assert_eq!(global_load.get_type(), Type::i32());
    let global_store: &instruction::Store = &bb.instrs[16].clone().try_into().expect("Should be a store");
    assert_eq!(global_store.address, Operand::ConstantOperand(Constant::GlobalReference { name: Name::Name("global".to_owned()), ty: Type::i32() }));
    assert_eq!(global_store.get_type(), Type::VoidType);
}

#[test]
fn simple_linked_list() {
    init_logging();
    let path = Path::new("tests/basic_bc/linkedlist.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");

    let structty: Arc<RwLock<Type>> = module
        .named_struct_types
        .get("struct.SimpleLinkedList")
        .unwrap_or_else(|| {
            let names: Vec<_> = module.named_struct_types.keys().collect();
            panic!(
                "Failed to find struct.SimpleLinkedList in named_struct_types; have names {:?}",
                names
            )
        })
        .as_ref()
        .expect("SimpleLinkedList should not be an opaque type")
        .clone();
    if let Type::StructType { element_types, .. } = structty.read().unwrap().deref() {
        assert_eq!(element_types.len(), 2);
        assert_eq!(element_types[0], Type::i32());
        if let Type::PointerType { pointee_type, .. } = &element_types[1] {
            if let Type::NamedStructType { ref name, ref ty } = **pointee_type {
                assert_eq!(name, "struct.SimpleLinkedList");
                let ty: Arc<RwLock<Type>> = ty
                    .as_ref()
                    .expect("Inner type should not be opaque")
                    .upgrade()
                    .expect("Failed to upgrade weak ref");
                assert_eq!(ty.read().unwrap().deref(), structty.read().unwrap().deref()); // the type should be truly recursive, in that the pointed-to type should be the same as the original type
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
            "Expected SimpleLinkedList to be a StructType, got {:?}",
            structty
        );
    }

    let func = module
        .get_func_by_name("simple_linked_list")
        .expect("Failed to find function");
    let alloca: &instruction::Alloca = &func.basic_blocks[0].instrs[1]
        .clone()
        .try_into()
        .expect("Should be an alloca");
    if let Type::NamedStructType { ref name, ref ty } = alloca.allocated_type {
        assert_eq!(name, "struct.SimpleLinkedList");
        let inner_ty: Arc<RwLock<Type>> = ty
            .as_ref()
            .expect("Allocated type should not be opaque")
            .upgrade()
            .expect("Failed to upgrade weak ref");
        assert_eq!(inner_ty.read().unwrap().deref(), structty.read().unwrap().deref()); // this should be exactly the same struct type as when we accessed it through the module above
    } else {
        panic!(
            "Expected alloca.allocated_type to be a NamedStructType, got {:?}",
            alloca.allocated_type
        );
    }
}

#[test]
fn indirectly_recursive_type() {
    init_logging();
    let path = Path::new("tests/basic_bc/linkedlist.bc");
    let module = Module::from_bc_path(&path).expect("Failed to parse module");

    let aty: Arc<RwLock<Type>> = module
        .named_struct_types
        .get("struct.NodeA")
        .unwrap_or_else(|| {
            let names: Vec<_> = module.named_struct_types.keys().collect();
            panic!(
                "Failed to find struct.NodeA in named_struct_types; have names {:?}",
                names
            )
        })
        .as_ref()
        .expect("NodeA should not be an opaque type")
        .clone();
    let bty: Arc<RwLock<Type>> = module
        .named_struct_types
        .get("struct.NodeB")
        .unwrap_or_else(|| {
            let names: Vec<_> = module.named_struct_types.keys().collect();
            panic!(
                "Failed to find struct.NodeB in named_struct_types; have names {:?}",
                names
            )
        })
        .as_ref()
        .expect("NodeB should not be an opaque type")
        .clone();
    if let Type::StructType { element_types, .. } = aty.read().unwrap().deref() {
        assert_eq!(element_types.len(), 2);
        assert_eq!(element_types[0], Type::i32());
        if let Type::PointerType { pointee_type, .. } = &element_types[1] {
            if let Type::NamedStructType { ref name, ref ty } = **pointee_type {
                assert_eq!(name, "struct.NodeB");
                let ty: Arc<RwLock<Type>> = ty
                    .as_ref()
                    .expect("Inner type should not be opaque")
                    .upgrade()
                    .expect("Failed to upgrade weak ref");
                assert_eq!(ty.read().unwrap().deref(), bty.read().unwrap().deref());
            } else {
                panic!(
                    "Expected pointee type to be a NamedStructType, got {:?}",
                    **pointee_type
                );
            }
        } else {
            panic!(
                "Expected inner type to be a PointerType, got {:?}",
                element_types[1]
            );
        }
    } else {
        panic!("Expected NodeA to be a StructType, got {:?}", aty);
    }
    if let Type::StructType { element_types, .. } = bty.read().unwrap().deref() {
        assert_eq!(element_types.len(), 2);
        assert_eq!(element_types[0], Type::i32());
        if let Type::PointerType { pointee_type, .. } = &element_types[1] {
            if let Type::NamedStructType { ref name, ref ty } = **pointee_type {
                assert_eq!(name, "struct.NodeA");
                let ty: Arc<RwLock<Type>> = ty
                    .as_ref()
                    .expect("Inner type should not be opaque")
                    .upgrade()
                    .expect("Failed to upgrade weak ref");
                assert_eq!(ty.read().unwrap().deref(), aty.read().unwrap().deref());
            } else {
                panic!(
                    "Expected pointee type to be a NamedStructType, got {:?}",
                    **pointee_type
                );
            }
        } else {
            panic!(
                "Expected inner type to be a PointerType, got {:?}",
                element_types[1]
            );
        }
    } else {
        panic!("Expected NodeB to be a StructType, got {:?}", bty);
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
    if let Type::NamedStructType { ref name, ref ty } = alloca_a.allocated_type {
        assert_eq!(name, "struct.NodeA");
        let inner_ty: Arc<RwLock<Type>> = ty
            .as_ref()
            .expect("Allocated type should not be opaque")
            .upgrade()
            .expect("Failed to upgrade weak ref");
        assert_eq!(inner_ty.read().unwrap().deref(), aty.read().unwrap().deref()); // this should be exactly the same struct type as when we accessed it through the module above
    } else {
        panic!(
            "Expected alloca_a.allocated_type to be a NamedStructType, got {:?}",
            alloca_a.allocated_type
        );
    }
    if let Type::NamedStructType { ref name, ref ty } = alloca_b.allocated_type {
        assert_eq!(name, "struct.NodeB");
        let inner_ty: Arc<RwLock<Type>> = ty
            .as_ref()
            .expect("Allocated type should not be opaque")
            .upgrade()
            .expect("Failed to upgrade weak ref");
        assert_eq!(inner_ty.read().unwrap().deref(), bty.read().unwrap().deref());
    } else {
        panic!(
            "Expected alloca_b.allocated_type to be a NamedStructType, got {:?}",
            alloca_b.allocated_type
        );
    }
}
