use std::convert::TryFrom;
use either::Either;
use crate::constant::Constant;
use crate::function::{CallingConvention, FunctionAttribute, ParameterAttribute};
use crate::instruction::{InlineAssembly, HasResult};
use crate::name::Name;
use crate::operand::Operand;
use crate::types::{Type, Typed};

/// Terminator instructions end a basic block.
/// See [LLVM docs on Terminator Instructions](https://releases.llvm.org/8.0.0/docs/LangRef.html#terminator-instructions)
#[derive(PartialEq, Clone, Debug)]
pub enum Terminator {
    Ret(Ret),
    Br(Br),
    CondBr(CondBr),
    Switch(Switch),
    IndirectBr(IndirectBr),
    Invoke(Invoke),
    Resume(Resume),
    Unreachable(Unreachable),
    CleanupRet(CleanupRet),
    CatchRet(CatchRet),
    CatchSwitch(CatchSwitch),
}

/// The [`Type`](../enum.Type.html) of a `Terminator` is its result type.
/// For most terminators, this is `VoidType`.
/// For instance, a [`Ret`](struct.Ret.html) instruction has void type even if
/// the function returns a non-void value; we do not store the result of a `Ret`
/// instruction using something like `%3 = ret i32 %2`.
/// See [LLVM docs on Terminator Instructions](https://releases.llvm.org/8.0.0/docs/LangRef.html#terminator-instructions)
impl Typed for Terminator {
    fn get_type(&self) -> Type {
        match self {
            Terminator::Ret(t) => t.get_type(),
            Terminator::Br(t) => t.get_type(),
            Terminator::CondBr(t) => t.get_type(),
            Terminator::Switch(t) => t.get_type(),
            Terminator::IndirectBr(t) => t.get_type(),
            Terminator::Invoke(t) => t.get_type(),
            Terminator::Resume(t) => t.get_type(),
            Terminator::Unreachable(t) => t.get_type(),
            Terminator::CleanupRet(t) => t.get_type(),
            Terminator::CatchRet(t) => t.get_type(),
            Terminator::CatchSwitch(t) => t.get_type(),
        }
    }
}

/* --TODO not yet implemented: metadata
impl Terminator {
    pub fn get_metadata(&self) -> &InstructionMetadata {
        match self {
            Terminator::Ret(t) => &t.metadata,
            Terminator::Br(t) => &t.metadata,
            Terminator::CondBr(t) => &t.metadata,
            Terminator::Switch(t) => &t.metadata,
            Terminator::IndirectBr(t) => &t.metadata,
            Terminator::Invoke(t) => &t.metadata,
            Terminator::Resume(t) => &t.metadata,
            Terminator::Unreachable(t) => &t.metadata,
            Terminator::CleanupRet(t) => &t.metadata,
            Terminator::CatchRet(t) => &t.metadata,
            Terminator::CatchSwitch(t) => &t.metadata,
        }
    }
}
*/

macro_rules! impl_term {
    ($term:ty, $id:ident) => {
        impl From<$term> for Terminator {
            fn from(term: $term) -> Terminator {
                Terminator::$id(term)
            }
        }

        impl TryFrom<Terminator> for $term {
            type Error = &'static str;
            fn try_from(term: Terminator) -> Result<Self, Self::Error> {
                match term {
                    Terminator::$id(term) => Ok(term),
                    _ => Err("Terminator is not of requested type"),
                }
            }
        }

        /* --TODO not yet implemented: metadata
        impl HasMetadata for $term {
            fn get_metadata(&self) -> &InstructionMetadata {
                &self.metadata
            }
        }
        */
    }
}

macro_rules! impl_hasresult {
    ($term:ty) => {
        impl HasResult for $term {
            fn get_result(&self) -> &Name {
                &self.result
            }
        }
    }
}

macro_rules! void_typed {
    ($term:ty) => {
        impl Typed for $term {
            fn get_type(&self) -> Type {
                Type::VoidType
            }
        }
    };
}

/// See [LLVM 8 docs on the 'ret' instruction](https://releases.llvm.org/8.0.0/docs/LangRef.html#ret-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Ret {
    /// The value being returned, or `None` if returning void.
    pub return_operand: Option<Operand>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(Ret, Ret);
void_typed!(Ret);  // technically the instruction has void type, even though the function may not

/// See [LLVM 8 docs on the 'br' instruction](https://releases.llvm.org/8.0.0/docs/LangRef.html#br-instruction).
/// The LLVM 'br' instruction has both conditional and unconditional variants, which we separate -- this is
/// the unconditional variant, while the conditional variant is [`CondBr`](struct.CondBr.html).
#[derive(PartialEq, Clone, Debug)]
pub struct Br {
    /// The [`Name`](../enum.Name.html) of the [`BasicBlock`](../struct.BasicBlock.html) destination.
    pub dest: Name,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(Br, Br);
void_typed!(Br);

/// See [LLVM 8 docs on the 'br' instruction](https://releases.llvm.org/8.0.0/docs/LangRef.html#br-instruction).
/// The LLVM 'br' instruction has both conditional and unconditional variants, which we separate -- this is
/// the conditional variant, while the unconditional variant is [`Br`](struct.Br.html).
#[derive(PartialEq, Clone, Debug)]
pub struct CondBr {
    /// The branch condition.
    pub condition: Operand,
    /// The [`Name`](../enum.Name.html) of the [`BasicBlock`](../struct.BasicBlock.html) destination if the `condition` is true.
    pub true_dest: Name,
    /// The [`Name`](../enum.Name.html) of the [`BasicBlock`](../struct.BasicBlock.html) destination if the `condition` is false.
    pub false_dest: Name,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(CondBr, CondBr);
void_typed!(CondBr);

/// See [LLVM 8 docs on the 'switch' instruction](https://releases.llvm.org/8.0.0/docs/LangRef.html#switch-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Switch {
    pub operand: Operand,
    pub dests: Vec<(Constant, Name)>,
    pub default_dest: Name,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(Switch, Switch);
void_typed!(Switch);

/// See [LLVM 8 docs on the 'indirectbr' instruction](https://releases.llvm.org/8.0.0/docs/LangRef.html#indirectbr-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct IndirectBr {
    /// Address to jump to (must be derived from a [`Constant::BlockAddress`](../enum.Constant.html))
    pub operand: Operand,
    /// The "full set of possible destinations" which the `IndirectBr` could jump to.
    /// These are [`Name`](../enum.Name.html)s of
    /// [`BasicBlock`](../struct.BasicBlock.html)s in the current function;
    /// `IndirectBr` cannot be used to jump between functions.
    pub possible_dests: Vec<Name>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(IndirectBr, IndirectBr);
void_typed!(IndirectBr);

/// See [LLVM 8 docs on the 'invoke' instruction](https://releases.llvm.org/8.0.0/docs/LangRef.html#invoke-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Invoke {
    pub function: Either<InlineAssembly, Operand>,
    pub arguments: Vec<(Operand, Vec<ParameterAttribute>)>,
    pub return_attributes: Vec<ParameterAttribute>,
    pub result: Name,  // The name of the variable that will get the result of the call (if the callee returns with 'ret')
    pub return_label: Name,  // Should be the name of a basic block. If the callee returns normally (i.e., with 'ret'), control flow resumes here.
    pub exception_label: Name,  // Should be the name of a basic block. If the callee returns with 'resume' or another exception-handling mechanism, control flow resumes here.
    pub function_attributes: Vec<FunctionAttribute>,  // llvm-hs has the equivalent of Vec<Either<GroupID, FunctionAttribute>>, but I'm not sure how the GroupID option comes up
    pub calling_convention: CallingConvention,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(Invoke, Invoke);
impl_hasresult!(Invoke);

impl Typed for Invoke {
    fn get_type(&self) -> Type {
        if let Type::FuncType { result_type, .. } = self.function.get_type() {
            *result_type
        } else {
            panic!("self.function has a type that's not FuncType");
        }
    }
}

/// See [LLVM 8 docs on the 'resume' instruction](https://releases.llvm.org/8.0.0/docs/LangRef.html#resume-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Resume {
    pub operand: Operand,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(Resume, Resume);
void_typed!(Resume);

/// See [LLVM 8 docs on the 'unreachable' instruction](https://releases.llvm.org/8.0.0/docs/LangRef.html#unreachable-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Unreachable {
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(Unreachable, Unreachable);
void_typed!(Unreachable);

/// See [LLVM 8 docs on the 'cleanupret' instruction](https://releases.llvm.org/8.0.0/docs/LangRef.html#cleanupret-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct CleanupRet {
    pub cleanup_pad: Operand,
    /// `None` here indicates 'unwind to caller'
    pub unwind_dest: Option<Name>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(CleanupRet, CleanupRet);
void_typed!(CleanupRet);

/// See [LLVM 8 docs on the 'catchret' instruction](https://releases.llvm.org/8.0.0/docs/LangRef.html#catchret-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct CatchRet {
    pub catch_pad: Operand,
    pub successor: Name,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(CatchRet, CatchRet);
void_typed!(CatchRet);

/// See [LLVM 8 docs on the 'catchswitch' instruction](https://releases.llvm.org/8.0.0/docs/LangRef.html#catchswitch-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct CatchSwitch {
    pub parent_pad: Operand,
    /// Cannot be empty
    pub catch_handlers: Vec<Name>,
    /// `None` here indicates 'unwind to caller'
    pub default_unwind_dest: Option<Name>,
    pub result: Name,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(CatchSwitch, CatchSwitch);
impl_hasresult!(CatchSwitch);

impl Typed for CatchSwitch {
    fn get_type(&self) -> Type {
        unimplemented!("Typed for CatchSwitch")
        // It's clear that there is a result of this instruction, but the documentation doesn't appear to clearly describe what its type is
    }
}

// ********* //
// from_llvm //
// ********* //

use crate::from_llvm::*;
use crate::basicblock::BBMap;
use crate::constant::GlobalNameMap;
use crate::operand::ValToNameMap;
use crate::types::TyNameMap;
use llvm_sys::LLVMOpcode;
use log::debug;

impl Terminator {
    pub(crate) fn from_llvm_ref(term: LLVMValueRef, ctr: &mut usize, vnmap: &ValToNameMap, bbmap: &BBMap, gnmap: &GlobalNameMap, tnmap: &mut TyNameMap) -> Self {
        debug!("Processing terminator {:?}", unsafe { print_to_string(term) });
        match unsafe { LLVMGetInstructionOpcode(term) } {
            LLVMOpcode::LLVMRet => Terminator::Ret(Ret::from_llvm_ref(term, vnmap, gnmap, tnmap)),
            LLVMOpcode::LLVMBr => match unsafe { LLVMGetNumOperands(term) } {
                1 => Terminator::Br(Br::from_llvm_ref(term, bbmap)),
                3 => Terminator::CondBr(CondBr::from_llvm_ref(term, vnmap, bbmap, gnmap, tnmap)),
                n => panic!("LLVMBr with {} operands, expected 1 or 3", n),
            },
            LLVMOpcode::LLVMSwitch => Terminator::Switch(Switch::from_llvm_ref(term, vnmap, bbmap, gnmap, tnmap)),
            LLVMOpcode::LLVMIndirectBr => Terminator::IndirectBr(IndirectBr::from_llvm_ref(term, vnmap, bbmap, gnmap, tnmap)),
            LLVMOpcode::LLVMInvoke => Terminator::Invoke(Invoke::from_llvm_ref(term, ctr, vnmap, bbmap, gnmap, tnmap)),
            LLVMOpcode::LLVMResume => Terminator::Resume(Resume::from_llvm_ref(term, vnmap, gnmap, tnmap)),
            LLVMOpcode::LLVMUnreachable => Terminator::Unreachable(Unreachable::from_llvm_ref(term)),
            LLVMOpcode::LLVMCleanupRet => Terminator::CleanupRet(CleanupRet::from_llvm_ref(term, vnmap, bbmap, gnmap, tnmap)),
            LLVMOpcode::LLVMCatchRet => Terminator::CatchRet(CatchRet::from_llvm_ref(term, vnmap, bbmap, gnmap, tnmap)),
            LLVMOpcode::LLVMCatchSwitch => Terminator::CatchSwitch(CatchSwitch::from_llvm_ref(term, ctr, vnmap, bbmap, gnmap, tnmap)),
            opcode => panic!("Terminator::from_llvm_ref called with a non-terminator instruction (opcode {:?})", opcode),
        }
    }
}

impl Ret {
    pub(crate) fn from_llvm_ref(term: LLVMValueRef, vnmap: &ValToNameMap, gnmap: &GlobalNameMap, tnmap: &mut TyNameMap) -> Self {
        Self {
            return_operand: match unsafe { LLVMGetNumOperands(term) } {
                0 => None,
                1 => Some(Operand::from_llvm_ref( unsafe { LLVMGetOperand(term, 0) }, vnmap, gnmap, tnmap )),
                n => panic!("Ret instruction with {} operands", n),
            },
            // metadata: InstructionMetadata::from_llvm_inst(term),
        }
    }
}

impl Br {
    pub(crate) fn from_llvm_ref(term: LLVMValueRef, bbmap: &BBMap) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(term) }, 1);
        Self {
            dest: bbmap.get( unsafe { &op_to_bb(LLVMGetOperand(term, 0)) } )
                .expect("Failed to find destination bb in map")
                .clone(),
            // metadata: InstructionMetadata::from_llvm_inst(term),
        }
    }
}

impl CondBr {
    pub(crate) fn from_llvm_ref(term: LLVMValueRef, vnmap: &ValToNameMap, bbmap: &BBMap, gnmap: &GlobalNameMap, tnmap: &mut TyNameMap) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(term) }, 3);
        Self {
            condition: Operand::from_llvm_ref( unsafe { LLVMGetOperand(term, 0) }, vnmap, gnmap, tnmap ),
            true_dest: bbmap.get( unsafe { &op_to_bb(LLVMGetOperand(term, 2)) } )
                .expect("Failed to find true-destination bb in map")
                .clone(),
            false_dest: bbmap.get( unsafe { &op_to_bb(LLVMGetOperand(term, 1)) } )
                .expect("Failed to find false-destination in bb map")
                .clone(),
            // metadata: InstructionMetadata::from_llvm_inst(term),
        }
    }
}

impl Switch {
    pub(crate) fn from_llvm_ref(term: LLVMValueRef, vnmap: &ValToNameMap, bbmap: &BBMap, gnmap: &GlobalNameMap, tnmap: &mut TyNameMap) -> Self {
        Self {
            operand: Operand::from_llvm_ref( unsafe { LLVMGetOperand(term, 0) }, vnmap, gnmap, tnmap ),
            dests: {
                let num_dests = unsafe { LLVMGetNumSuccessors(term) };
                let dest_bbs = (1 ..= num_dests).map(|i| {  // LLVMGetSuccessor(0) apparently gives the default dest
                    bbmap.get( unsafe { &LLVMGetSuccessor(term, i) } )
                        .expect("Failed to find switch destination in map")
                        .clone()
                });
                let dest_vals = (1 .. num_dests).map(|i| {
                    Constant::from_llvm_ref( unsafe { LLVMGetOperand(term, 2*i) }, gnmap, tnmap )  // 2*i because empirically, operand 1 is the default dest, and operands 3/5/7/etc are the successor blocks
                });
                Iterator::zip(dest_vals, dest_bbs).collect()
            },
            default_dest: bbmap.get( unsafe { &LLVMGetSwitchDefaultDest(term) } )
                .expect("Failed to find switch default destination in map")
                .clone(),
            // metadata: InstructionMetadata::from_llvm_inst(term),
        }
    }
}

impl IndirectBr {
    pub(crate) fn from_llvm_ref(term: LLVMValueRef, vnmap: &ValToNameMap, bbmap: &BBMap, gnmap: &GlobalNameMap, tnmap: &mut TyNameMap) -> Self {
        Self {
            operand: Operand::from_llvm_ref( unsafe { LLVMGetOperand(term, 0) }, vnmap, gnmap, tnmap ),
            possible_dests: {
                let num_dests = unsafe { LLVMGetNumSuccessors(term) };
                (0 .. num_dests).map(|i| {
                    bbmap.get( unsafe { &LLVMGetSuccessor(term, i) } )
                        .expect("Failed to find indirect branch destination in map")
                        .clone()
                }).collect()
            },
            // metadata: InstructionMetadata::from_llvm_inst(term),
        }
    }
}

impl Invoke {
    pub(crate) fn from_llvm_ref(term: LLVMValueRef, ctr: &mut usize, vnmap: &ValToNameMap, bbmap: &BBMap, gnmap: &GlobalNameMap, tnmap: &mut TyNameMap) -> Self {
        use crate::instruction::CallInfo;
        let callinfo = CallInfo::from_llvm_ref(term, vnmap, gnmap, tnmap);
        Self {
            function: callinfo.function,
            arguments: callinfo.arguments,
            return_attributes: callinfo.return_attributes,
            result: Name::name_or_num( unsafe { get_value_name(term) }, ctr),
            return_label: bbmap.get( unsafe { &LLVMGetNormalDest(term) } )
                .expect("Failed to find invoke return destination in map")
                .clone(),
            exception_label: bbmap.get( unsafe { &LLVMGetUnwindDest(term) } )
                .expect("Failed to find invoke exception destination in map")
                .clone(),
            function_attributes: callinfo.function_attributes,
            calling_convention: callinfo.calling_convention,
            // metadata: InstructionMetadata::from_llvm_inst(term),
        }
    }
}

impl Resume {
    pub(crate) fn from_llvm_ref(term: LLVMValueRef, vnmap: &ValToNameMap, gnmap: &GlobalNameMap, tnmap: &mut TyNameMap) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(term) }, 1);
        Self {
            operand: Operand::from_llvm_ref( unsafe { LLVMGetOperand(term, 0) }, vnmap, gnmap, tnmap ),
            // metadata: InstructionMetadata::from_llvm_inst(term),
        }
    }
}

impl Unreachable {
    pub(crate) fn from_llvm_ref(term: LLVMValueRef) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(term) }, 0);
        Self {
            // metadata: InstructionMetadata::from_llvm_inst(term),
        }
    }
}

impl CleanupRet {
    pub(crate) fn from_llvm_ref(term: LLVMValueRef, vnmap: &ValToNameMap, bbmap: &BBMap, gnmap: &GlobalNameMap, tnmap: &mut TyNameMap) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(term) }, 1);
        Self {
            cleanup_pad: Operand::from_llvm_ref( unsafe { LLVMGetOperand(term, 0) }, vnmap, gnmap, tnmap ),
            unwind_dest: {
                let dest = unsafe { LLVMGetUnwindDest(term) };
                if dest.is_null() {
                    None
                } else {
                    Some(bbmap.get(&dest)
                        .unwrap_or_else(|| { let names: Vec<_> = bbmap.values().collect(); panic!("Failed to find unwind destination in map; have names {:?}", names) })
                        .clone()
                    )
                }
            },
            // metadata: InstructionMetadata::from_llvm_inst(term),
        }
    }
}

impl CatchRet {
    pub(crate) fn from_llvm_ref(term: LLVMValueRef, vnmap: &ValToNameMap, bbmap: &BBMap, gnmap: &GlobalNameMap, tnmap: &mut TyNameMap) -> Self {
        Self {
            catch_pad: Operand::from_llvm_ref( unsafe { LLVMGetOperand(term, 0) }, vnmap, gnmap, tnmap ),
            successor: bbmap.get( unsafe { &LLVMGetSuccessor(term, 0) } )
                .expect("Failed to find CatchRet successor in map")
                .clone(),
            // metadata: InstructionMetadata::from_llvm_inst(term),
        }
    }
}

impl CatchSwitch {
    pub(crate) fn from_llvm_ref(term: LLVMValueRef, ctr: &mut usize, vnmap: &ValToNameMap, bbmap: &BBMap, gnmap: &GlobalNameMap, tnmap: &mut TyNameMap) -> Self {
        Self {
            parent_pad: Operand::from_llvm_ref( unsafe { LLVMGetOperand(term, 0) }, vnmap, gnmap, tnmap ),
            catch_handlers: {
                let num_handlers = unsafe { LLVMGetNumHandlers(term) };
                let mut handlers: Vec<LLVMBasicBlockRef> = Vec::with_capacity(num_handlers as usize);
                unsafe {
                    LLVMGetHandlers(term, handlers.as_mut_ptr());
                    handlers.set_len(num_handlers as usize);
                };
                handlers.into_iter()
                    .map(|h| bbmap.get(&h).expect("Failed to find catch handler in map").clone())
                    .collect()
            },
            default_unwind_dest: {
                let dest = unsafe { LLVMGetUnwindDest(term) };
                if dest.is_null() {
                    None
                } else {
                    Some(bbmap.get(&dest)
                        .unwrap_or_else(|| { let names: Vec<_> = bbmap.values().collect(); panic!("Failed to find CatchSwitch default unwind destination in map; have names {:?}", names) })
                        .clone()
                    )
                }
            },
            result: Name::name_or_num( unsafe { get_value_name(term) }, ctr),
            // metadata: InstructionMetadata::from_llvm_inst(term),
        }
    }
}
