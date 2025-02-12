use std::fmt::{self, Display};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum IntPredicate {
    EQ,
    NE,
    UGT,
    UGE,
    ULT,
    ULE,
    SGT,
    SGE,
    SLT,
    SLE,
}

impl Display for IntPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntPredicate::EQ => write!(f, "eq"),
            IntPredicate::NE => write!(f, "ne"),
            IntPredicate::UGT => write!(f, "ugt"),
            IntPredicate::UGE => write!(f, "uge"),
            IntPredicate::ULT => write!(f, "ult"),
            IntPredicate::ULE => write!(f, "ule"),
            IntPredicate::SGT => write!(f, "sgt"),
            IntPredicate::SGE => write!(f, "sge"),
            IntPredicate::SLT => write!(f, "slt"),
            IntPredicate::SLE => write!(f, "sle"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum FPPredicate {
    False,
    OEQ,
    OGT,
    OGE,
    OLT,
    OLE,
    ONE,
    ORD,
    UNO,
    UEQ,
    UGT,
    UGE,
    ULT,
    ULE,
    UNE,
    True,
}

impl Display for FPPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FPPredicate::False => write!(f, "false"),
            FPPredicate::OEQ => write!(f, "oeq"),
            FPPredicate::OGT => write!(f, "ogt"),
            FPPredicate::OGE => write!(f, "oge"),
            FPPredicate::OLT => write!(f, "olt"),
            FPPredicate::OLE => write!(f, "ole"),
            FPPredicate::ONE => write!(f, "one"),
            FPPredicate::ORD => write!(f, "ord"),
            FPPredicate::UNO => write!(f, "uno"),
            FPPredicate::UEQ => write!(f, "ueq"),
            FPPredicate::UGT => write!(f, "ugt"),
            FPPredicate::UGE => write!(f, "uge"),
            FPPredicate::ULT => write!(f, "ult"),
            FPPredicate::ULE => write!(f, "ule"),
            FPPredicate::UNE => write!(f, "une"),
            FPPredicate::True => write!(f, "true"),
        }
    }
}

// ********* //
// from_llvm //
// ********* //

use crate::llvm_sys::*;
use llvm_sys::LLVMIntPredicate;
use llvm_sys::LLVMRealPredicate;

impl IntPredicate {
    pub(crate) fn from_llvm(pred: LLVMIntPredicate) -> Self {
        match pred {
            LLVMIntPredicate::LLVMIntEQ => IntPredicate::EQ,
            LLVMIntPredicate::LLVMIntNE => IntPredicate::NE,
            LLVMIntPredicate::LLVMIntUGT => IntPredicate::UGT,
            LLVMIntPredicate::LLVMIntUGE => IntPredicate::UGE,
            LLVMIntPredicate::LLVMIntULT => IntPredicate::ULT,
            LLVMIntPredicate::LLVMIntULE => IntPredicate::ULE,
            LLVMIntPredicate::LLVMIntSGT => IntPredicate::SGT,
            LLVMIntPredicate::LLVMIntSGE => IntPredicate::SGE,
            LLVMIntPredicate::LLVMIntSLT => IntPredicate::SLT,
            LLVMIntPredicate::LLVMIntSLE => IntPredicate::SLE,
        }
    }
}

impl FPPredicate {
    pub(crate) fn from_llvm(pred: LLVMRealPredicate) -> Self {
        match pred {
            LLVMRealPredicate::LLVMRealPredicateFalse => FPPredicate::False,
            LLVMRealPredicate::LLVMRealOEQ => FPPredicate::OEQ,
            LLVMRealPredicate::LLVMRealOGT => FPPredicate::OGT,
            LLVMRealPredicate::LLVMRealOGE => FPPredicate::OGE,
            LLVMRealPredicate::LLVMRealOLT => FPPredicate::OLT,
            LLVMRealPredicate::LLVMRealOLE => FPPredicate::OLE,
            LLVMRealPredicate::LLVMRealONE => FPPredicate::ONE,
            LLVMRealPredicate::LLVMRealORD => FPPredicate::ORD,
            LLVMRealPredicate::LLVMRealUNO => FPPredicate::UNO,
            LLVMRealPredicate::LLVMRealUEQ => FPPredicate::UEQ,
            LLVMRealPredicate::LLVMRealUGT => FPPredicate::UGT,
            LLVMRealPredicate::LLVMRealUGE => FPPredicate::UGE,
            LLVMRealPredicate::LLVMRealULT => FPPredicate::ULT,
            LLVMRealPredicate::LLVMRealULE => FPPredicate::ULE,
            LLVMRealPredicate::LLVMRealUNE => FPPredicate::UNE,
            LLVMRealPredicate::LLVMRealPredicateTrue => FPPredicate::True,
        }
    }
}
