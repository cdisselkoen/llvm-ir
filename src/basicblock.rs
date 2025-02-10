use crate::instruction::Instruction;
use crate::name::Name;
use crate::terminator::Terminator;

/// A `BasicBlock` is a sequence of zero or more non-terminator instructions
/// followed by a single terminator instruction which ends the block.
/// Basic blocks are discussed in the [LLVM 14 docs on Functions](https://releases.llvm.org/14.0.0/docs/LangRef.html#functionstructure)
#[derive(PartialEq, Clone, Debug, Hash)]
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
            term: Terminator::Unreachable(Unreachable {
                debugloc: None,
            }),
        }
    }
}

// ********* //
// from_llvm //
// ********* //

use crate::from_llvm::*;
use crate::function::FunctionContext;
use crate::llvm_sys::*;
use crate::module::ModuleContext;
use llvm_sys::LLVMOpcode;
use llvm_sys::LLVMTypeKind::LLVMVoidTypeKind;

impl BasicBlock {
    pub(crate) fn from_llvm_ref(
        bb: LLVMBasicBlockRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        let name = Name::name_or_num(unsafe { get_bb_name(bb) }, &mut func_ctx.ctr);
        debug_assert_eq!(
            &name,
            func_ctx
                .bb_names
                .get(&bb)
                .expect("Expected to find bb in func_ctx.bb_names"),
        );
        debug!("Processing a basic block named {:?}", name);
        Self {
            name,
            instrs: all_but_last(get_instructions(bb))
                .map(|i| Instruction::from_llvm_ref(i, ctx, func_ctx))
                .collect(),
            term: Terminator::from_llvm_ref(
                unsafe { LLVMGetBasicBlockTerminator(bb) },
                ctx,
                func_ctx,
            ),
        }
    }

    // Returns the name of the basic block and a vec of (instruction/terminator, name) pairs
    pub(crate) fn first_pass_names(
        bb: LLVMBasicBlockRef,
        ctr: &mut usize,
    ) -> (Name, Vec<(LLVMValueRef, Name)>) {
        let bbname = Name::name_or_num(unsafe { get_bb_name(bb) }, ctr);
        let mut instnames = vec![];
        for inst in all_but_last(get_instructions(bb)).filter(|&i| needs_name(i)) {
            instnames.push((
                inst,
                Name::name_or_num(unsafe { get_value_name(inst) }, ctr),
            ));
        }
        let term = unsafe { LLVMGetBasicBlockTerminator(bb) };
        if term_needs_name(term) {
            instnames.push((
                term,
                Name::name_or_num(unsafe { get_value_name(term) }, ctr),
            ));
        }
        (bbname, instnames)
    }
}

// Given only the LLVMValueRef for an Instruction, determine whether it needs a name
fn needs_name(inst: LLVMValueRef) -> bool {
    if unsafe { !get_value_name(inst).is_empty() } {
        return true; // has a string name
    }
    match unsafe { LLVMGetInstructionOpcode(inst) } {
        LLVMOpcode::LLVMStore => false,
        LLVMOpcode::LLVMFence => false,
        LLVMOpcode::LLVMCall => {
            // needs a name unless we're calling a void function
            let kind =
                unsafe { LLVMGetTypeKind(LLVMGetReturnType(LLVMGetCalledFunctionType(inst))) };
            kind != LLVMVoidTypeKind
        },
        _ => true, // all other instructions have results (destinations) and thus will need names
    }
}

// Given only the LLVMValueRef for a Terminator, determine whether it needs a name
fn term_needs_name(term: LLVMValueRef) -> bool {
    if unsafe { !get_value_name(term).is_empty() } {
        return true; // has a string name
    }
    match unsafe { LLVMGetInstructionOpcode(term) } {
        LLVMOpcode::LLVMInvoke => true,
        LLVMOpcode::LLVMCatchSwitch => true,
        LLVMOpcode::LLVMCallBr => true,
        _ => false, // all other terminators have no result (destination) and thus don't need names
    }
}
