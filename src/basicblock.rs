use crate::instruction::Instruction;
use crate::name::Name;
use crate::terminator::Terminator;

/// A `BasicBlock` is a sequence of zero or more non-terminator instructions
/// followed by a single terminator instruction which ends the block.
/// Basic blocks are discussed in the [LLVM 8 docs on Functions](https://releases.llvm.org/8.0.0/docs/LangRef.html#functionstructure)
#[derive(PartialEq, Clone, Debug)]
pub struct BasicBlock {
    pub name: Name,
    pub instrs: Vec<Instruction>,
    pub term: Terminator,
}

impl BasicBlock {
    /// A `BasicBlock` instance with no instructions and an `Unreachable` terminator
    pub fn new(name: Name) -> Self {
        use crate::terminator::Unreachable;
        Self {
            name,
            instrs: vec![],
            term: Terminator::Unreachable(Unreachable {} ),
        }
    }
}

// ********* //
// from_llvm //
// ********* //

use crate::from_llvm::*;
use crate::constant::GlobalNameMap;
use crate::operand::ValToNameMap;
use crate::types::TyNameMap;
use llvm_sys::LLVMOpcode;
use llvm_sys::LLVMTypeKind::LLVMVoidTypeKind;
use std::collections::HashMap;
use log::debug;

pub(crate) type BBMap = HashMap<LLVMBasicBlockRef, Name>;

impl BasicBlock {
    pub(crate) fn from_llvm_ref(bb: LLVMBasicBlockRef, ctr: &mut usize, vnmap: &ValToNameMap, bbmap: &BBMap, gnmap: &GlobalNameMap, tnmap: &mut TyNameMap) -> Self {
        let name = Name::name_or_num( unsafe { get_bb_name(bb) }, ctr);
        assert_eq!(name, bbmap.get(&bb).expect("Expected to find bb in bbmap").clone());
        debug!("Processing a basic block named {:?}", name);
        Self {
            name,
            instrs: all_but_last(get_instructions(bb)).map(|i| Instruction::from_llvm_ref(i, ctr, vnmap, bbmap, gnmap, tnmap)).collect(),
            term: Terminator::from_llvm_ref( unsafe { LLVMGetBasicBlockTerminator(bb) }, ctr, vnmap, bbmap, gnmap, tnmap ),
        }
    }

    // Returns the name of the basic block and a vec of (instruction/terminator, name) pairs
    pub(crate) fn first_pass_names(bb: LLVMBasicBlockRef, ctr: &mut usize) -> (Name, Vec<(LLVMValueRef, Name)>) {
        let bbname = Name::name_or_num( unsafe { get_bb_name(bb) }, ctr);
        let mut instnames = vec![];
        for inst in all_but_last(get_instructions(bb)).filter(|&i| needs_name(i)) {
            instnames.push((inst, Name::name_or_num( unsafe { get_value_name(inst) }, ctr)));
        }
        let term = unsafe { LLVMGetBasicBlockTerminator(bb) };
        if term_needs_name(term) {
            instnames.push((term, Name::name_or_num( unsafe { get_value_name(term) }, ctr)));
        }
        (bbname, instnames)
    }
}

// Given only the LLVMValueRef for an Instruction, determine whether it needs a name
fn needs_name(inst: LLVMValueRef) -> bool {
    if unsafe { get_value_name(inst) != "" } {
        return true;  // has a string name
    }
    match unsafe { LLVMGetInstructionOpcode(inst) } {
        LLVMOpcode::LLVMStore => false,
        LLVMOpcode::LLVMFence => false,
        LLVMOpcode::LLVMAtomicRMW => false,
        LLVMOpcode::LLVMCall => {
            let kind = unsafe { LLVMGetTypeKind(LLVMGetReturnType(LLVMGetCalledFunctionType(inst))) };
            kind != LLVMVoidTypeKind
        },
        _ => true,  // all other instructions have results (destinations) and thus will need names
    }
}

// Given only the LLVMValueRef for a Terminator, determine whether it needs a name
fn term_needs_name(term: LLVMValueRef) -> bool {
    if unsafe { get_value_name(term) != "" } {
        return true;  // has a string name
    }
    match unsafe { LLVMGetInstructionOpcode(term) } {
        LLVMOpcode::LLVMInvoke => true,
        LLVMOpcode::LLVMCatchSwitch => true,
        _ => false,  // all other terminators have no result (destination) and thus don't need names
    }
}
