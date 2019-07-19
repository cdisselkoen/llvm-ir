use crate::constant::Constant;
use crate::name::Name;
use crate::types::{Type, Typed};
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub enum Operand {
    LocalOperand { name: Name, ty: Type },  // e.g., i32 %foo
    ConstantOperand(Constant),  // includes GlobalReference for things like @foo
    MetadataOperand,  // --TODO not yet implemented-- MetadataOperand(Box<Metadata>),
}

impl Typed for Operand {
    fn get_type(&self) -> Type {
        match self {
            Operand::LocalOperand { ty, .. } => ty.clone(),
            Operand::ConstantOperand(c) => c.get_type(),
            Operand::MetadataOperand => Type::MetadataType,
        }
    }
}

// ********* //
// from_llvm //
// ********* //

use crate::from_llvm::*;
use crate::constant::GlobalNameMap;
use crate::types::TyNameMap;
use llvm_sys::LLVMValueKind;

pub(crate) type ValToNameMap = HashMap<LLVMValueRef, Name>;

impl Operand {
    pub(crate) fn from_llvm_ref(operand: LLVMValueRef, vnmap: &ValToNameMap, gnmap: &GlobalNameMap, tnmap: &mut TyNameMap) -> Self {
        let constant = unsafe { LLVMIsAConstant(operand) };
        if !constant.is_null() {
            Operand::ConstantOperand(Constant::from_llvm_ref(constant, gnmap, tnmap))
        } else if unsafe { LLVMGetValueKind(operand) } == LLVMValueKind::LLVMMetadataAsValueValueKind {
            Operand::MetadataOperand
        } else {
            Operand::LocalOperand {
                name: vnmap.get(&operand)
                    .unwrap_or_else(|| {
                        let names: Vec<_> = vnmap.values().collect();
                        let kind = unsafe { LLVMGetValueKind(operand) };
                        panic!("Failed to find operand with kind {:?} in vnmap; have names {:?}", kind, names) })
                    .clone(),
                ty: Type::from_llvm_ref( unsafe { LLVMTypeOf(operand) }, tnmap ),
            }
        }
    }
}
