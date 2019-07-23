use super::Instruction;
use super::{AShr, Add, And, FAdd, FDiv, FMul, FRem, FSub, LShr, Mul, Or, SDiv, SRem, Shl, Sub, UDiv, URem, Xor};
use super::{HasResult, Name, Operand, Type, Typed};
use std::convert::TryFrom;

/// Just the BinaryOps.  This ends up being better than a `&dyn `[`BinaryOp`](../trait.BinaryOp.html) for various reasons.
#[derive(PartialEq, Clone, Debug)]
pub enum BinaryOp {
    // Integer binary ops
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    UDiv(UDiv),
    SDiv(SDiv),
    URem(URem),
    SRem(SRem),

    // Bitwise binary ops
    And(And),
    Or(Or),
    Xor(Xor),
    Shl(Shl),
    LShr(LShr),
    AShr(AShr),

    // Floating-point binary ops
    FAdd(FAdd),
    FSub(FSub),
    FMul(FMul),
    FDiv(FDiv),
    FRem(FRem),
}

impl From<BinaryOp> for Instruction {
    fn from(bo: BinaryOp) -> Instruction {
        match bo {
            BinaryOp::Add(i) => i.into(),
            BinaryOp::Sub(i) => i.into(),
            BinaryOp::Mul(i) => i.into(),
            BinaryOp::UDiv(i) => i.into(),
            BinaryOp::SDiv(i) => i.into(),
            BinaryOp::URem(i) => i.into(),
            BinaryOp::SRem(i) => i.into(),
            BinaryOp::And(i) => i.into(),
            BinaryOp::Or(i) => i.into(),
            BinaryOp::Xor(i) => i.into(),
            BinaryOp::Shl(i) => i.into(),
            BinaryOp::LShr(i) => i.into(),
            BinaryOp::AShr(i) => i.into(),
            BinaryOp::FAdd(i) => i.into(),
            BinaryOp::FSub(i) => i.into(),
            BinaryOp::FMul(i) => i.into(),
            BinaryOp::FDiv(i) => i.into(),
            BinaryOp::FRem(i) => i.into(),
        }
    }
}

impl TryFrom<Instruction> for BinaryOp {
    type Error = &'static str;
    fn try_from(inst: Instruction) -> Result<Self, Self::Error> {
        match inst {
            Instruction::Add(i) => Ok(BinaryOp::Add(i)),
            Instruction::Sub(i) => Ok(BinaryOp::Sub(i)),
            Instruction::Mul(i) => Ok(BinaryOp::Mul(i)),
            Instruction::UDiv(i) => Ok(BinaryOp::UDiv(i)),
            Instruction::SDiv(i) => Ok(BinaryOp::SDiv(i)),
            Instruction::URem(i) => Ok(BinaryOp::URem(i)),
            Instruction::SRem(i) => Ok(BinaryOp::SRem(i)),
            Instruction::And(i) => Ok(BinaryOp::And(i)),
            Instruction::Or(i) => Ok(BinaryOp::Or(i)),
            Instruction::Xor(i) => Ok(BinaryOp::Xor(i)),
            Instruction::Shl(i) => Ok(BinaryOp::Shl(i)),
            Instruction::LShr(i) => Ok(BinaryOp::LShr(i)),
            Instruction::AShr(i) => Ok(BinaryOp::AShr(i)),
            Instruction::FAdd(i) => Ok(BinaryOp::FAdd(i)),
            Instruction::FSub(i) => Ok(BinaryOp::FSub(i)),
            Instruction::FMul(i) => Ok(BinaryOp::FMul(i)),
            Instruction::FDiv(i) => Ok(BinaryOp::FDiv(i)),
            Instruction::FRem(i) => Ok(BinaryOp::FRem(i)),
            _ => Err("Not a binary op"),
        }
    }
}

impl Typed for BinaryOp {
    fn get_type(&self) -> Type {
        match self {
            BinaryOp::Add(i) => i.get_type(),
            BinaryOp::Sub(i) => i.get_type(),
            BinaryOp::Mul(i) => i.get_type(),
            BinaryOp::UDiv(i) => i.get_type(),
            BinaryOp::SDiv(i) => i.get_type(),
            BinaryOp::URem(i) => i.get_type(),
            BinaryOp::SRem(i) => i.get_type(),
            BinaryOp::And(i) => i.get_type(),
            BinaryOp::Or(i) => i.get_type(),
            BinaryOp::Xor(i) => i.get_type(),
            BinaryOp::Shl(i) => i.get_type(),
            BinaryOp::LShr(i) => i.get_type(),
            BinaryOp::AShr(i) => i.get_type(),
            BinaryOp::FAdd(i) => i.get_type(),
            BinaryOp::FSub(i) => i.get_type(),
            BinaryOp::FMul(i) => i.get_type(),
            BinaryOp::FDiv(i) => i.get_type(),
            BinaryOp::FRem(i) => i.get_type(),
        }
    }
}

/* --TODO not yet implemented: metadata
impl HasMetadata for BinaryOp {
    fn get_metadata(&self) -> &InstructionMetadata {
        match self {
            BinaryOp::Add(i) => i.get_metadata(),
            BinaryOp::Sub(i) => i.get_metadata(),
            BinaryOp::Mul(i) => i.get_metadata(),
            BinaryOp::UDiv(i) => i.get_metadata(),
            BinaryOp::SDiv(i) => i.get_metadata(),
            BinaryOp::URem(i) => i.get_metadata(),
            BinaryOp::SRem(i) => i.get_metadata(),
            BinaryOp::And(i) => i.get_metadata(),
            BinaryOp::Or(i) => i.get_metadata(),
            BinaryOp::Xor(i) => i.get_metadata(),
            BinaryOp::Shl(i) => i.get_metadata(),
            BinaryOp::LShr(i) => i.get_metadata(),
            BinaryOp::AShr(i) => i.get_metadata(),
            BinaryOp::FAdd(i) => i.get_metadata(),
            BinaryOp::FSub(i) => i.get_metadata(),
            BinaryOp::FMul(i) => i.get_metadata(),
            BinaryOp::FDiv(i) => i.get_metadata(),
            BinaryOp::FRem(i) => i.get_metadata(),
        }
    }
}
*/

impl HasResult for BinaryOp {
    fn get_result(&self) -> &Name {
        match self {
            BinaryOp::Add(i) => i.get_result(),
            BinaryOp::Sub(i) => i.get_result(),
            BinaryOp::Mul(i) => i.get_result(),
            BinaryOp::UDiv(i) => i.get_result(),
            BinaryOp::SDiv(i) => i.get_result(),
            BinaryOp::URem(i) => i.get_result(),
            BinaryOp::SRem(i) => i.get_result(),
            BinaryOp::And(i) => i.get_result(),
            BinaryOp::Or(i) => i.get_result(),
            BinaryOp::Xor(i) => i.get_result(),
            BinaryOp::Shl(i) => i.get_result(),
            BinaryOp::LShr(i) => i.get_result(),
            BinaryOp::AShr(i) => i.get_result(),
            BinaryOp::FAdd(i) => i.get_result(),
            BinaryOp::FSub(i) => i.get_result(),
            BinaryOp::FMul(i) => i.get_result(),
            BinaryOp::FDiv(i) => i.get_result(),
            BinaryOp::FRem(i) => i.get_result(),
        }
    }
}

impl super::BinaryOp for BinaryOp {
    fn get_operand0(&self) -> &Operand {
        match self {
            BinaryOp::Add(i) => i.get_operand0(),
            BinaryOp::Sub(i) => i.get_operand0(),
            BinaryOp::Mul(i) => i.get_operand0(),
            BinaryOp::UDiv(i) => i.get_operand0(),
            BinaryOp::SDiv(i) => i.get_operand0(),
            BinaryOp::URem(i) => i.get_operand0(),
            BinaryOp::SRem(i) => i.get_operand0(),
            BinaryOp::And(i) => i.get_operand0(),
            BinaryOp::Or(i) => i.get_operand0(),
            BinaryOp::Xor(i) => i.get_operand0(),
            BinaryOp::Shl(i) => i.get_operand0(),
            BinaryOp::LShr(i) => i.get_operand0(),
            BinaryOp::AShr(i) => i.get_operand0(),
            BinaryOp::FAdd(i) => i.get_operand0(),
            BinaryOp::FSub(i) => i.get_operand0(),
            BinaryOp::FMul(i) => i.get_operand0(),
            BinaryOp::FDiv(i) => i.get_operand0(),
            BinaryOp::FRem(i) => i.get_operand0(),
        }
    }

    fn get_operand1(&self) -> &Operand {
        match self {
            BinaryOp::Add(i) => i.get_operand1(),
            BinaryOp::Sub(i) => i.get_operand1(),
            BinaryOp::Mul(i) => i.get_operand1(),
            BinaryOp::UDiv(i) => i.get_operand1(),
            BinaryOp::SDiv(i) => i.get_operand1(),
            BinaryOp::URem(i) => i.get_operand1(),
            BinaryOp::SRem(i) => i.get_operand1(),
            BinaryOp::And(i) => i.get_operand1(),
            BinaryOp::Or(i) => i.get_operand1(),
            BinaryOp::Xor(i) => i.get_operand1(),
            BinaryOp::Shl(i) => i.get_operand1(),
            BinaryOp::LShr(i) => i.get_operand1(),
            BinaryOp::AShr(i) => i.get_operand1(),
            BinaryOp::FAdd(i) => i.get_operand1(),
            BinaryOp::FSub(i) => i.get_operand1(),
            BinaryOp::FMul(i) => i.get_operand1(),
            BinaryOp::FDiv(i) => i.get_operand1(),
            BinaryOp::FRem(i) => i.get_operand1(),
        }
    }
}
