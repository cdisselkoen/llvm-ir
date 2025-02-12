use crate::types::{TypeRef, Typed, Types};
use crate::{ConstantRef, Name};
use std::fmt::{self, Display};

#[derive(PartialEq, Clone, Debug, Hash)]
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

impl Operand {
    /// Get a reference to the `Constant`, if the operand is a constant;
    /// otherwise, returns `None`.
    ///
    /// This allows nested matching on `Operand`. Instead of the following code
    /// (which doesn't compile because you can't directly match on `ConstantRef`)
    /// ```ignore
    /// if let Operand::ConstantOperand(Constant::Float(Float::Double(val))) = op
    /// ```
    /// you can write this:
    /// ```ignore
    /// if let Some(Constant::Float(Float::Double(val))) = op.as_constant()
    /// ```
    pub fn as_constant(&self) -> Option<&Constant> {
        match self {
            Operand::ConstantOperand(cref) => Some(cref),
            _ => None,
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::LocalOperand { name, ty } => write!(f, "{} {}", ty, name),
            Operand::ConstantOperand(cref) => write!(f, "{}", &cref),
            Operand::MetadataOperand => write!(f, "<metadata>"),
        }
    }
}

// ********* //
// from_llvm //
// ********* //

use crate::constant::Constant;
use crate::function::FunctionContext;
use crate::llvm_sys::*;
use crate::module::ModuleContext;
use llvm_sys::LLVMValueKind;

impl Operand {
    pub(crate) fn from_llvm_ref(
        operand: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &FunctionContext,
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
                name: func_ctx.val_names
                    .get(&operand)
                    .unwrap_or_else(|| {
                        let names: Vec<_> = func_ctx.val_names.values().collect();
                        let kind = unsafe { LLVMGetValueKind(operand) };
                        panic!(
                            "Failed to find operand with kind {:?} in func_ctx.val_names; have names {:?}",
                            kind, names
                        )
                    })
                    .clone(),
                ty: ctx.types.type_from_llvm_ref(unsafe { LLVMTypeOf(operand) }),
            }
        }
    }
}
