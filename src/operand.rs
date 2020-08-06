use crate::types::{TypeRef, Typed, Types};
use crate::{ConstantRef, Name};
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub enum Operand {
    /// e.g., `i32 %foo`
    LocalOperand {
        name: Name,
        ty: TypeRef,
    },
    /// includes [`GlobalReference`](../constant/enum.Constant.html#variant.GlobalReference) for things like `@foo`
    ConstantOperand(ConstantRef),
    MetadataOperand, // --TODO not yet implemented-- MetadataOperand(Box<Metadata>),
}

impl Typed for Operand {
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            Operand::LocalOperand { ty, .. } => ty.clone(),
            Operand::ConstantOperand(c) => types.type_of(c),
            Operand::MetadataOperand => types.metadata_type(),
        }
    }
}

// ********* //
// from_llvm //
// ********* //

use crate::constant::Constant;
use crate::from_llvm::*;
use crate::module::FromLLVMContext;
use llvm_sys::LLVMValueKind;

pub(crate) type ValToNameMap = HashMap<LLVMValueRef, Name>;

impl Operand {
    pub(crate) fn from_llvm_ref(
        operand: LLVMValueRef,
        vnmap: &ValToNameMap,
        ctx: &mut FromLLVMContext,
    ) -> Self {
        let constant = unsafe { LLVMIsAConstant(operand) };
        if !constant.is_null() {
            Operand::ConstantOperand(Constant::from_llvm_ref(constant, ctx))
        } else if unsafe {
            LLVMGetValueKind(operand) == LLVMValueKind::LLVMMetadataAsValueValueKind
        } {
            Operand::MetadataOperand
        } else {
            Operand::LocalOperand {
                name: vnmap
                    .get(&operand)
                    .unwrap_or_else(|| {
                        let names: Vec<_> = vnmap.values().collect();
                        let kind = unsafe { LLVMGetValueKind(operand) };
                        panic!(
                            "Failed to find operand with kind {:?} in vnmap; have names {:?}",
                            kind, names
                        )
                    })
                    .clone(),
                ty: ctx.types.type_from_llvm_ref(unsafe { LLVMTypeOf(operand) }),
            }
        }
    }
}
