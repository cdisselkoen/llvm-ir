#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
