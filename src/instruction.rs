use crate::constant::ConstantRef;
#[cfg(LLVM_VERSION_9_OR_GREATER)]
use crate::debugloc::{DebugLoc, HasDebugLoc};
use crate::function::{CallingConvention, FunctionAttribute, ParameterAttribute};
use crate::name::Name;
use crate::operand::Operand;
use crate::predicates::*;
use crate::types::{NamedStructDef, Type, TypeRef, Typed, Types};
use either::Either;
use std::convert::TryFrom;
use std::fmt::Debug;

/// Non-terminator instructions.
#[derive(PartialEq, Clone, Debug)]
pub enum Instruction {
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

    // Floating-point ops
    FAdd(FAdd),
    FSub(FSub),
    FMul(FMul),
    FDiv(FDiv),
    FRem(FRem),
    FNeg(FNeg),

    // Vector ops
    ExtractElement(ExtractElement),
    InsertElement(InsertElement),
    ShuffleVector(ShuffleVector),

    // Aggregate ops
    ExtractValue(ExtractValue),
    InsertValue(InsertValue),

    // Memory-related ops
    Alloca(Alloca),
    Load(Load),
    Store(Store),
    Fence(Fence),
    CmpXchg(CmpXchg),
    AtomicRMW(AtomicRMW),
    GetElementPtr(GetElementPtr),

    // Conversion ops
    Trunc(Trunc),
    ZExt(ZExt),
    SExt(SExt),
    FPTrunc(FPTrunc),
    FPExt(FPExt),
    FPToUI(FPToUI),
    FPToSI(FPToSI),
    UIToFP(UIToFP),
    SIToFP(SIToFP),
    PtrToInt(PtrToInt),
    IntToPtr(IntToPtr),
    BitCast(BitCast),
    AddrSpaceCast(AddrSpaceCast),

    // LLVM's "other operations" category
    ICmp(ICmp),
    FCmp(FCmp),
    Phi(Phi),
    Select(Select),
    #[cfg(LLVM_VERSION_10_OR_GREATER)]
    Freeze(Freeze),
    Call(Call),
    VAArg(VAArg),
    LandingPad(LandingPad),
    CatchPad(CatchPad),
    CleanupPad(CleanupPad),
}

/// The [`Type`](../enum.Type.html) of an `Instruction` (or any subtype of `Instruction`) is its result type.
impl Typed for Instruction {
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            Instruction::Add(i) => types.type_of(i),
            Instruction::Sub(i) => types.type_of(i),
            Instruction::Mul(i) => types.type_of(i),
            Instruction::UDiv(i) => types.type_of(i),
            Instruction::SDiv(i) => types.type_of(i),
            Instruction::URem(i) => types.type_of(i),
            Instruction::SRem(i) => types.type_of(i),
            Instruction::And(i) => types.type_of(i),
            Instruction::Or(i) => types.type_of(i),
            Instruction::Xor(i) => types.type_of(i),
            Instruction::Shl(i) => types.type_of(i),
            Instruction::LShr(i) => types.type_of(i),
            Instruction::AShr(i) => types.type_of(i),
            Instruction::FAdd(i) => types.type_of(i),
            Instruction::FSub(i) => types.type_of(i),
            Instruction::FMul(i) => types.type_of(i),
            Instruction::FDiv(i) => types.type_of(i),
            Instruction::FRem(i) => types.type_of(i),
            Instruction::FNeg(i) => types.type_of(i),
            Instruction::ExtractElement(i) => types.type_of(i),
            Instruction::InsertElement(i) => types.type_of(i),
            Instruction::ShuffleVector(i) => types.type_of(i),
            Instruction::ExtractValue(i) => types.type_of(i),
            Instruction::InsertValue(i) => types.type_of(i),
            Instruction::Alloca(i) => types.type_of(i),
            Instruction::Load(i) => types.type_of(i),
            Instruction::Store(i) => types.type_of(i),
            Instruction::Fence(i) => types.type_of(i),
            Instruction::CmpXchg(i) => types.type_of(i),
            Instruction::AtomicRMW(i) => types.type_of(i),
            Instruction::GetElementPtr(i) => types.type_of(i),
            Instruction::Trunc(i) => types.type_of(i),
            Instruction::ZExt(i) => types.type_of(i),
            Instruction::SExt(i) => types.type_of(i),
            Instruction::FPTrunc(i) => types.type_of(i),
            Instruction::FPExt(i) => types.type_of(i),
            Instruction::FPToUI(i) => types.type_of(i),
            Instruction::FPToSI(i) => types.type_of(i),
            Instruction::UIToFP(i) => types.type_of(i),
            Instruction::SIToFP(i) => types.type_of(i),
            Instruction::PtrToInt(i) => types.type_of(i),
            Instruction::IntToPtr(i) => types.type_of(i),
            Instruction::BitCast(i) => types.type_of(i),
            Instruction::AddrSpaceCast(i) => types.type_of(i),
            Instruction::ICmp(i) => types.type_of(i),
            Instruction::FCmp(i) => types.type_of(i),
            Instruction::Phi(i) => types.type_of(i),
            Instruction::Select(i) => types.type_of(i),
            #[cfg(LLVM_VERSION_10_OR_GREATER)]
            Instruction::Freeze(i) => types.type_of(i),
            Instruction::Call(i) => types.type_of(i),
            Instruction::VAArg(i) => types.type_of(i),
            Instruction::LandingPad(i) => types.type_of(i),
            Instruction::CatchPad(i) => types.type_of(i),
            Instruction::CleanupPad(i) => types.type_of(i),
        }
    }
}

#[cfg(LLVM_VERSION_9_OR_GREATER)]
impl HasDebugLoc for Instruction {
    fn get_debug_loc(&self) -> &Option<DebugLoc> {
        match self {
            Instruction::Add(i) => i.get_debug_loc(),
            Instruction::Sub(i) => i.get_debug_loc(),
            Instruction::Mul(i) => i.get_debug_loc(),
            Instruction::UDiv(i) => i.get_debug_loc(),
            Instruction::SDiv(i) => i.get_debug_loc(),
            Instruction::URem(i) => i.get_debug_loc(),
            Instruction::SRem(i) => i.get_debug_loc(),
            Instruction::And(i) => i.get_debug_loc(),
            Instruction::Or(i) => i.get_debug_loc(),
            Instruction::Xor(i) => i.get_debug_loc(),
            Instruction::Shl(i) => i.get_debug_loc(),
            Instruction::LShr(i) => i.get_debug_loc(),
            Instruction::AShr(i) => i.get_debug_loc(),
            Instruction::FAdd(i) => i.get_debug_loc(),
            Instruction::FSub(i) => i.get_debug_loc(),
            Instruction::FMul(i) => i.get_debug_loc(),
            Instruction::FDiv(i) => i.get_debug_loc(),
            Instruction::FRem(i) => i.get_debug_loc(),
            Instruction::FNeg(i) => i.get_debug_loc(),
            Instruction::ExtractElement(i) => i.get_debug_loc(),
            Instruction::InsertElement(i) => i.get_debug_loc(),
            Instruction::ShuffleVector(i) => i.get_debug_loc(),
            Instruction::ExtractValue(i) => i.get_debug_loc(),
            Instruction::InsertValue(i) => i.get_debug_loc(),
            Instruction::Alloca(i) => i.get_debug_loc(),
            Instruction::Load(i) => i.get_debug_loc(),
            Instruction::Store(i) => i.get_debug_loc(),
            Instruction::Fence(i) => i.get_debug_loc(),
            Instruction::CmpXchg(i) => i.get_debug_loc(),
            Instruction::AtomicRMW(i) => i.get_debug_loc(),
            Instruction::GetElementPtr(i) => i.get_debug_loc(),
            Instruction::Trunc(i) => i.get_debug_loc(),
            Instruction::ZExt(i) => i.get_debug_loc(),
            Instruction::SExt(i) => i.get_debug_loc(),
            Instruction::FPTrunc(i) => i.get_debug_loc(),
            Instruction::FPExt(i) => i.get_debug_loc(),
            Instruction::FPToUI(i) => i.get_debug_loc(),
            Instruction::FPToSI(i) => i.get_debug_loc(),
            Instruction::UIToFP(i) => i.get_debug_loc(),
            Instruction::SIToFP(i) => i.get_debug_loc(),
            Instruction::PtrToInt(i) => i.get_debug_loc(),
            Instruction::IntToPtr(i) => i.get_debug_loc(),
            Instruction::BitCast(i) => i.get_debug_loc(),
            Instruction::AddrSpaceCast(i) => i.get_debug_loc(),
            Instruction::ICmp(i) => i.get_debug_loc(),
            Instruction::FCmp(i) => i.get_debug_loc(),
            Instruction::Phi(i) => i.get_debug_loc(),
            Instruction::Select(i) => i.get_debug_loc(),
            #[cfg(LLVM_VERSION_10_OR_GREATER)]
            Instruction::Freeze(i) => i.get_debug_loc(),
            Instruction::Call(i) => i.get_debug_loc(),
            Instruction::VAArg(i) => i.get_debug_loc(),
            Instruction::LandingPad(i) => i.get_debug_loc(),
            Instruction::CatchPad(i) => i.get_debug_loc(),
            Instruction::CleanupPad(i) => i.get_debug_loc(),
        }
    }
}

impl Instruction {
    /// Get the result (destination) of the `Instruction`, or `None` if the
    /// `Instruction` doesn't have a result (has void type).
    pub fn try_get_result(&self) -> Option<&Name> {
        match self {
            Instruction::Add(i) => Some(&i.dest),
            Instruction::Sub(i) => Some(&i.dest),
            Instruction::Mul(i) => Some(&i.dest),
            Instruction::UDiv(i) => Some(&i.dest),
            Instruction::SDiv(i) => Some(&i.dest),
            Instruction::URem(i) => Some(&i.dest),
            Instruction::SRem(i) => Some(&i.dest),
            Instruction::And(i) => Some(&i.dest),
            Instruction::Or(i) => Some(&i.dest),
            Instruction::Xor(i) => Some(&i.dest),
            Instruction::Shl(i) => Some(&i.dest),
            Instruction::LShr(i) => Some(&i.dest),
            Instruction::AShr(i) => Some(&i.dest),
            Instruction::FAdd(i) => Some(&i.dest),
            Instruction::FSub(i) => Some(&i.dest),
            Instruction::FMul(i) => Some(&i.dest),
            Instruction::FDiv(i) => Some(&i.dest),
            Instruction::FRem(i) => Some(&i.dest),
            Instruction::FNeg(i) => Some(&i.dest),
            Instruction::ExtractElement(i) => Some(&i.dest),
            Instruction::InsertElement(i) => Some(&i.dest),
            Instruction::ShuffleVector(i) => Some(&i.dest),
            Instruction::ExtractValue(i) => Some(&i.dest),
            Instruction::InsertValue(i) => Some(&i.dest),
            Instruction::Alloca(i) => Some(&i.dest),
            Instruction::Load(i) => Some(&i.dest),
            Instruction::Store(_) => None,
            Instruction::Fence(_) => None,
            Instruction::CmpXchg(i) => Some(&i.dest),
            Instruction::AtomicRMW(i) => Some(&i.dest),
            Instruction::GetElementPtr(i) => Some(&i.dest),
            Instruction::Trunc(i) => Some(&i.dest),
            Instruction::ZExt(i) => Some(&i.dest),
            Instruction::SExt(i) => Some(&i.dest),
            Instruction::FPTrunc(i) => Some(&i.dest),
            Instruction::FPExt(i) => Some(&i.dest),
            Instruction::FPToUI(i) => Some(&i.dest),
            Instruction::FPToSI(i) => Some(&i.dest),
            Instruction::UIToFP(i) => Some(&i.dest),
            Instruction::SIToFP(i) => Some(&i.dest),
            Instruction::PtrToInt(i) => Some(&i.dest),
            Instruction::IntToPtr(i) => Some(&i.dest),
            Instruction::BitCast(i) => Some(&i.dest),
            Instruction::AddrSpaceCast(i) => Some(&i.dest),
            Instruction::ICmp(i) => Some(&i.dest),
            Instruction::FCmp(i) => Some(&i.dest),
            Instruction::Phi(i) => Some(&i.dest),
            Instruction::Select(i) => Some(&i.dest),
            #[cfg(LLVM_VERSION_10_OR_GREATER)]
            Instruction::Freeze(i) => Some(&i.dest),
            Instruction::Call(i) => i.dest.as_ref(),
            Instruction::VAArg(i) => Some(&i.dest),
            Instruction::LandingPad(i) => Some(&i.dest),
            Instruction::CatchPad(i) => Some(&i.dest),
            Instruction::CleanupPad(i) => Some(&i.dest),
        }
    }

    /// Whether the `Instruction` is atomic
    pub fn is_atomic(&self) -> bool {
        match self {
            Instruction::Add(_) => false,
            Instruction::Sub(_) => false,
            Instruction::Mul(_) => false,
            Instruction::UDiv(_) => false,
            Instruction::SDiv(_) => false,
            Instruction::URem(_) => false,
            Instruction::SRem(_) => false,
            Instruction::And(_) => false,
            Instruction::Or(_) => false,
            Instruction::Xor(_) => false,
            Instruction::Shl(_) => false,
            Instruction::LShr(_) => false,
            Instruction::AShr(_) => false,
            Instruction::FAdd(_) => false,
            Instruction::FSub(_) => false,
            Instruction::FMul(_) => false,
            Instruction::FDiv(_) => false,
            Instruction::FRem(_) => false,
            Instruction::FNeg(_) => false,
            Instruction::ExtractElement(_) => false,
            Instruction::InsertElement(_) => false,
            Instruction::ShuffleVector(_) => false,
            Instruction::ExtractValue(_) => false,
            Instruction::InsertValue(_) => false,
            Instruction::Alloca(_) => false,
            Instruction::Load(i) => i.atomicity.is_some(),
            Instruction::Store(i) => i.atomicity.is_some(),
            Instruction::Fence(_) => true,
            Instruction::CmpXchg(_) => true,
            Instruction::AtomicRMW(_) => true,
            Instruction::GetElementPtr(_) => false,
            Instruction::Trunc(_) => false,
            Instruction::ZExt(_) => false,
            Instruction::SExt(_) => false,
            Instruction::FPTrunc(_) => false,
            Instruction::FPExt(_) => false,
            Instruction::FPToUI(_) => false,
            Instruction::FPToSI(_) => false,
            Instruction::UIToFP(_) => false,
            Instruction::SIToFP(_) => false,
            Instruction::PtrToInt(_) => false,
            Instruction::IntToPtr(_) => false,
            Instruction::BitCast(_) => false,
            Instruction::AddrSpaceCast(_) => false,
            Instruction::ICmp(_) => false,
            Instruction::FCmp(_) => false,
            Instruction::Phi(_) => false,
            Instruction::Select(_) => false,
            #[cfg(LLVM_VERSION_10_OR_GREATER)]
            Instruction::Freeze(_) => false,
            Instruction::Call(_) => false,
            Instruction::VAArg(_) => false,
            Instruction::LandingPad(_) => false,
            Instruction::CatchPad(_) => false,
            Instruction::CleanupPad(_) => false,
        }
    }
}

/* --TODO not yet implemented: metadata
pub trait HasMetadata {
    fn get_metadata(&self) -> &InstructionMetadata;
}

impl HasMetadata for Instruction {
    fn get_metadata(&self) -> &InstructionMetadata {
        match self {
            Instruction::Add(i) => &i.metadata,
            Instruction::Sub(i) => &i.metadata,
            Instruction::Mul(i) => &i.metadata,
            Instruction::UDiv(i) => &i.metadata,
            Instruction::SDiv(i) => &i.metadata,
            Instruction::URem(i) => &i.metadata,
            Instruction::SRem(i) => &i.metadata,
            Instruction::And(i) => &i.metadata,
            Instruction::Or(i) => &i.metadata,
            Instruction::Xor(i) => &i.metadata,
            Instruction::Shl(i) => &i.metadata,
            Instruction::LShr(i) => &i.metadata,
            Instruction::AShr(i) => &i.metadata,
            Instruction::FAdd(i) => &i.metadata,
            Instruction::FSub(i) => &i.metadata,
            Instruction::FMul(i) => &i.metadata,
            Instruction::FDiv(i) => &i.metadata,
            Instruction::FRem(i) => &i.metadata,
            Instruction::FNeg(i) => &i.metadata,
            Instruction::ExtractElement(i) => &i.metadata,
            Instruction::InsertElement(i) => &i.metadata,
            Instruction::ShuffleVector(i) => &i.metadata,
            Instruction::ExtractValue(i) => &i.metadata,
            Instruction::InsertValue(i) => &i.metadata,
            Instruction::Alloca(i) => &i.metadata,
            Instruction::Load(i) => &i.metadata,
            Instruction::Store(i) => &i.metadata,
            Instruction::Fence(i) => &i.metadata,
            Instruction::CmpXchg(i) => &i.metadata,
            Instruction::AtomicRMW(i) => &i.metadata,
            Instruction::GetElementPtr(i) => &i.metadata,
            Instruction::Trunc(i) => &i.metadata,
            Instruction::ZExt(i) => &i.metadata,
            Instruction::SExt(i) => &i.metadata,
            Instruction::FPTrunc(i) => &i.metadata,
            Instruction::FPExt(i) => &i.metadata,
            Instruction::FPToUI(i) => &i.metadata,
            Instruction::FPToSI(i) => &i.metadata,
            Instruction::UIToFP(i) => &i.metadata,
            Instruction::SIToFP(i) => &i.metadata,
            Instruction::PtrToInt(i) => &i.metadata,
            Instruction::IntToPtr(i) => &i.metadata,
            Instruction::BitCast(i) => &i.metadata,
            Instruction::AddrSpaceCast(i) => &i.metadata,
            Instruction::ICmp(i) => &i.metadata,
            Instruction::FCmp(i) => &i.metadata,
            Instruction::Phi(i) => &i.metadata,
            Instruction::Select(i) => &i.metadata,
            #[cfg(LLVM_VERSION_10_OR_GREATER)]
            Instruction::Freeze(i) => &i.metadata,
            Instruction::Call(i) => &i.metadata,
            Instruction::VAArg(i) => &i.metadata,
            Instruction::LandingPad(i) => &i.metadata,
            Instruction::CatchPad(i) => &i.metadata,
            Instruction::CleanupPad(i) => &i.metadata,
        }
    }
}
*/

pub trait HasResult: Debug + Typed {
    fn get_result(&self) -> &Name;
}

pub trait UnaryOp: HasResult {
    fn get_operand(&self) -> &Operand;
}

pub trait BinaryOp: HasResult {
    fn get_operand0(&self) -> &Operand;
    fn get_operand1(&self) -> &Operand;
}

pub mod groups;

impl Instruction {
    /// Determine if the `Instruction` is one of the ones in
    /// [`groups::BinaryOp`](groups/enum.BinaryOp.html), without actually using
    /// `try_into()` (which would consume it)
    pub fn is_binary_op(&self) -> bool {
        match self {
            Instruction::Add(_) => true,
            Instruction::Sub(_) => true,
            Instruction::Mul(_) => true,
            Instruction::UDiv(_) => true,
            Instruction::SDiv(_) => true,
            Instruction::URem(_) => true,
            Instruction::SRem(_) => true,
            Instruction::And(_) => true,
            Instruction::Or(_) => true,
            Instruction::Xor(_) => true,
            Instruction::Shl(_) => true,
            Instruction::LShr(_) => true,
            Instruction::AShr(_) => true,
            Instruction::FAdd(_) => true,
            Instruction::FSub(_) => true,
            Instruction::FMul(_) => true,
            Instruction::FDiv(_) => true,
            Instruction::FRem(_) => true,
            _ => false,
        }
    }

    /// Determine if the `Instruction` is one of the ones in
    /// [`groups::UnaryOp`](groups/enum.UnaryOp.html), without actually using
    /// `try_into()` (which would consume it)
    pub fn is_unary_op(&self) -> bool {
        match self {
            Instruction::AddrSpaceCast(_) => true,
            Instruction::BitCast(_) => true,
            Instruction::FNeg(_) => true,
            Instruction::FPExt(_) => true,
            Instruction::FPToSI(_) => true,
            Instruction::FPToUI(_) => true,
            Instruction::FPTrunc(_) => true,
            #[cfg(LLVM_VERSION_10_OR_GREATER)]
            Instruction::Freeze(_) => true,
            Instruction::IntToPtr(_) => true,
            Instruction::PtrToInt(_) => true,
            Instruction::SExt(_) => true,
            Instruction::SIToFP(_) => true,
            Instruction::Trunc(_) => true,
            Instruction::UIToFP(_) => true,
            Instruction::ZExt(_) => true,
            _ => false,
        }
    }
}

macro_rules! impl_inst {
    ($inst:ty, $id:ident) => {
        impl From<$inst> for Instruction {
            fn from(inst: $inst) -> Instruction {
                Instruction::$id(inst)
            }
        }

        impl TryFrom<Instruction> for $inst {
            type Error = &'static str;
            fn try_from(inst: Instruction) -> Result<Self, Self::Error> {
                match inst {
                    Instruction::$id(inst) => Ok(inst),
                    _ => Err("Instruction is not of requested type"),
                }
            }
        }

        #[cfg(LLVM_VERSION_9_OR_GREATER)]
        impl HasDebugLoc for $inst {
            fn get_debug_loc(&self) -> &Option<DebugLoc> {
                &self.debugloc
            }
        }

        /* --TODO not yet implemented: metadata
        impl HasMetadata for $inst {
            fn get_metadata(&self) -> &InstructionMetadata {
                &self.metadata
            }
        }
        */
    };
}

macro_rules! impl_hasresult {
    ($inst:ty) => {
        impl HasResult for $inst {
            fn get_result(&self) -> &Name {
                &self.dest
            }
        }
    };
}

macro_rules! impl_unop {
    ($inst:ty) => {
        impl_hasresult!($inst);

        impl UnaryOp for $inst {
            fn get_operand(&self) -> &Operand {
                &self.operand
            }
        }
    };
}

macro_rules! impl_binop {
    ($inst:ty, $id:ident) => {
        impl_hasresult!($inst);

        impl BinaryOp for $inst {
            fn get_operand0(&self) -> &Operand {
                &self.operand0
            }
            fn get_operand1(&self) -> &Operand {
                &self.operand1
            }
        }

        impl From<$inst> for groups::BinaryOp {
            fn from(inst: $inst) -> Self {
                groups::BinaryOp::$id(inst)
            }
        }

        impl TryFrom<groups::BinaryOp> for $inst {
            type Error = &'static str;
            fn try_from(bo: groups::BinaryOp) -> Result<Self, Self::Error> {
                match bo {
                    groups::BinaryOp::$id(i) => Ok(i),
                    _ => Err("BinaryOp is not of requested type"),
                }
            }
        }
    };
}

// Use on unops where the result type is the same as the operand type
macro_rules! unop_same_type {
    ($inst:ty) => {
        impl Typed for $inst {
            fn get_type(&self, types: &Types) -> TypeRef {
                types.type_of(self.get_operand())
            }
        }
    };
}

// Use on binops where the result type is the same as both operand types
macro_rules! binop_same_type {
    ($inst:ty) => {
        impl Typed for $inst {
            fn get_type(&self, types: &Types) -> TypeRef {
                let ty = types.type_of(self.get_operand0());
                assert_eq!(ty, types.type_of(self.get_operand1()));
                ty
            }
        }
    };
}

// Use on binops where the result type is the same as the first operand type
macro_rules! binop_left_type {
    ($inst:ty) => {
        impl Typed for $inst {
            fn get_type(&self, types: &Types) -> TypeRef {
                types.type_of(self.get_operand0())
            }
        }
    };
}

// Use on anything that has a 'to_type' field which indicates its result type
macro_rules! explicitly_typed {
    ($inst:ty) => {
        impl Typed for $inst {
            fn get_type(&self, _types: &Types) -> TypeRef {
                self.to_type.clone()
            }
        }
    };
}

macro_rules! void_typed {
    ($inst:ty) => {
        impl Typed for $inst {
            fn get_type(&self, types: &Types) -> TypeRef {
                types.void()
            }
        }
    };
}

/// See [LLVM 10 docs on the 'add' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#add-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Add {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub nsw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    // pub nuw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Add, Add);
impl_binop!(Add, Add);
binop_same_type!(Add);

/// See [LLVM 10 docs on the 'sub' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#sub-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Sub {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub nsw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    // pub nuw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Sub, Sub);
impl_binop!(Sub, Sub);
binop_same_type!(Sub);

/// See [LLVM 10 docs on the 'mul' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#mul-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Mul {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub nsw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    // pub nuw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Mul, Mul);
impl_binop!(Mul, Mul);
binop_same_type!(Mul);

/// See [LLVM 10 docs on the 'udiv' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#udiv-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct UDiv {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub exact: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(UDiv, UDiv);
impl_binop!(UDiv, UDiv);
binop_same_type!(UDiv);

/// See [LLVM 10 docs on the 'sdiv' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#sdiv-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct SDiv {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub exact: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(SDiv, SDiv);
impl_binop!(SDiv, SDiv);
binop_same_type!(SDiv);

/// See [LLVM 10 docs on the 'urem' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#urem-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct URem {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(URem, URem);
impl_binop!(URem, URem);
binop_same_type!(URem);

/// See [LLVM 10 docs on the 'srem' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#srem-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct SRem {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(SRem, SRem);
impl_binop!(SRem, SRem);
binop_same_type!(SRem);

/// Bitwise logical and.
/// See [LLVM 10 docs on the 'and' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#and-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct And {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(And, And);
impl_binop!(And, And);
binop_same_type!(And);

/// Bitwise logical inclusive or.
/// See [LLVM 10 docs on the 'or' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#or-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Or {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Or, Or);
impl_binop!(Or, Or);
binop_same_type!(Or);

/// Bitwise logical exclusive or.
/// See [LLVM 10 docs on the 'xor' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#xor-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Xor {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Xor, Xor);
impl_binop!(Xor, Xor);
binop_same_type!(Xor);

/// Shift left.
/// See [LLVM 10 docs on the 'shl' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#shl-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Shl {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub nsw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    // pub nuw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Shl, Shl);
impl_binop!(Shl, Shl);
binop_left_type!(Shl);

/// Logical shift right.
/// See [LLVM 10 docs on the 'lshr' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#lshr-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct LShr {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub exact: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(LShr, LShr);
impl_binop!(LShr, LShr);
binop_left_type!(LShr);

/// Arithmetic shift right.
/// See [LLVM 10 docs on the 'ashr' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#ashr-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct AShr {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub exact: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(AShr, AShr);
impl_binop!(AShr, AShr);
binop_left_type!(AShr);

/// Floating-point add.
/// See [LLVM 10 docs on the 'fadd' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#fadd-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct FAdd {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub fast_math_flags: FastMathFlags,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(FAdd, FAdd);
impl_binop!(FAdd, FAdd);
binop_same_type!(FAdd);

/// Floating-point sub.
/// See [LLVM 10 docs on the 'fsub' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#fsub-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct FSub {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub fast_math_flags: FastMathFlags,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(FSub, FSub);
impl_binop!(FSub, FSub);
binop_same_type!(FSub);

/// Floating-point multiply.
/// See [LLVM 10 docs on the 'fmul' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#fmul-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct FMul {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub fast_math_flags: FastMathFlags,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(FMul, FMul);
impl_binop!(FMul, FMul);
binop_same_type!(FMul);

/// Floating-point divide.
/// See [LLVM 10 docs on the 'fdiv' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#fdiv-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct FDiv {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub fast_math_flags: FastMathFlags,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(FDiv, FDiv);
impl_binop!(FDiv, FDiv);
binop_same_type!(FDiv);

/// Floating-point remainder.
/// See [LLVM 10 docs on the 'frem' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#frem-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct FRem {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub fast_math_flags: FastMathFlags,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(FRem, FRem);
impl_binop!(FRem, FRem);
binop_same_type!(FRem);

/// Floating-point unary negation.
/// See [LLVM 10 docs on the 'fneg' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#fneg-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct FNeg {
    pub operand: Operand,
    pub dest: Name,
    // pub fast_math_flags: FastMathFlags,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(FNeg, FNeg);
impl_unop!(FNeg);
unop_same_type!(FNeg);

/// Get an element from a vector at a specified index.
/// See [LLVM 10 docs on the 'extractelement' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#extractelement-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct ExtractElement {
    pub vector: Operand,
    pub index: Operand,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(ExtractElement, ExtractElement);
impl_hasresult!(ExtractElement);

impl Typed for ExtractElement {
    fn get_type(&self, types: &Types) -> TypeRef {
        match types.type_of(&self.vector).as_ref() {
            Type::VectorType { element_type, .. } => element_type.clone(),
            ty => panic!(
                "Expected an ExtractElement vector to be VectorType, got {:?}",
                ty
            ),
        }
    }
}

/// Insert an element into a vector at a specified index.
/// See [LLVM 10 docs on the 'insertelement' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#insertelement-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct InsertElement {
    pub vector: Operand,
    pub element: Operand,
    pub index: Operand,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(InsertElement, InsertElement);
impl_hasresult!(InsertElement);

impl Typed for InsertElement {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.type_of(&self.vector)
    }
}

/// See [LLVM 10 docs on the 'shufflevector' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#shufflevector-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct ShuffleVector {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub mask: ConstantRef,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(ShuffleVector, ShuffleVector);
impl_hasresult!(ShuffleVector);

impl Typed for ShuffleVector {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        assert_eq!(ty, types.type_of(&self.operand1));
        match ty.as_ref() {
            Type::VectorType { element_type, .. } => match types.type_of(&self.mask).as_ref() {
                Type::VectorType { num_elements, .. } => {
                    types.vector_of(element_type.clone(), *num_elements)
                },
                ty => panic!(
                    "Expected a ShuffleVector mask to be VectorType, got {:?}",
                    ty
                ),
            },
            _ => panic!(
                "Expected a ShuffleVector operand to be VectorType, got {:?}",
                ty
            ),
        }
    }
}

/// Extract the value of a member field from an aggregate (struct or array) type.
/// See [LLVM 10 docs on the 'extractvalue' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#extractvalue-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct ExtractValue {
    pub aggregate: Operand,
    pub indices: Vec<u32>,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(ExtractValue, ExtractValue);
impl_hasresult!(ExtractValue);

impl Typed for ExtractValue {
    fn get_type(&self, types: &Types) -> TypeRef {
        ev_type(types.type_of(&self.aggregate), self.indices.iter().copied())
    }
}

fn ev_type(cur_type: TypeRef, mut indices: impl Iterator<Item = u32>) -> TypeRef {
    match indices.next() {
        None => cur_type,
        Some(index) => match cur_type.as_ref() {
            Type::ArrayType { element_type, .. } => ev_type(element_type.clone(), indices),
            Type::StructType { element_types, .. } => ev_type(
                element_types
                    .get(index as usize)
                    .expect("ExtractValue index out of range")
                    .clone(),
                indices,
            ),
            _ => panic!(
                "ExtractValue from something that's not ArrayType or StructType; its type is {:?}",
                cur_type
            ),
        },
    }
}

/// Insert a value into a member field of an aggregate (struct or array) type.
/// See [LLVM 10 docs on the 'insertvalue' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#insertvalue-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct InsertValue {
    pub aggregate: Operand,
    pub element: Operand,
    pub indices: Vec<u32>,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(InsertValue, InsertValue);
impl_hasresult!(InsertValue);

impl Typed for InsertValue {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.type_of(&self.aggregate)
    }
}

/// Allocate memory on the stack.
/// See [LLVM 10 docs on the 'alloca' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#alloca-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Alloca {
    pub allocated_type: TypeRef,
    pub num_elements: Operand, // llvm-hs-pure has Option<Operand>
    pub dest: Name,
    pub alignment: u32,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Alloca, Alloca);
impl_hasresult!(Alloca);

impl Typed for Alloca {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.pointer_to(self.allocated_type.clone())
    }
}

/// Load a value from memory.
/// See [LLVM 10 docs on the 'load' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#load-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Load {
    pub address: Operand,
    pub dest: Name,
    pub volatile: bool,
    pub atomicity: Option<Atomicity>,
    pub alignment: u32,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Load, Load);
impl_hasresult!(Load);

impl Typed for Load {
    fn get_type(&self, types: &Types) -> TypeRef {
        match types.type_of(&self.address).as_ref() {
            Type::PointerType { pointee_type, .. } => pointee_type.clone(),
            ty => panic!("Expected a load address to be PointerType, got {:?}", ty),
        }
    }
}

/// Store a value to memory.
/// See [LLVM 10 docs on the 'store' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#store-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Store {
    pub address: Operand,
    pub value: Operand,
    pub volatile: bool,
    pub atomicity: Option<Atomicity>,
    pub alignment: u32,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Store, Store);
void_typed!(Store);

/// See [LLVM 10 docs on the 'fence' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#fence-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Fence {
    pub atomicity: Atomicity,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Fence, Fence);
void_typed!(Fence);

/// Atomic compare and exchange.
/// See [LLVM 10 docs on the 'cmpxchg' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#cmpxchg-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct CmpXchg {
    pub address: Operand,
    pub expected: Operand,
    pub replacement: Operand,
    pub dest: Name,
    pub volatile: bool,
    /// This includes the "success" `MemoryOrdering`
    pub atomicity: Atomicity,
    /// This is the "failure" `MemoryOrdering`
    pub failure_memory_ordering: MemoryOrdering,
    #[cfg(LLVM_VERSION_10_OR_GREATER)]
    pub weak: bool,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(CmpXchg, CmpXchg);
impl_hasresult!(CmpXchg);

impl Typed for CmpXchg {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.expected);
        assert_eq!(ty, types.type_of(&self.replacement));
        types.struct_of(vec![ty, types.bool()], false)
    }
}

/// Atomic read-modify-write.
/// See [LLVM 10 docs on the 'atomicrmw' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#atomicrmw-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct AtomicRMW {
    #[cfg(LLVM_VERSION_10_OR_GREATER)] // the binop-getter was added to the LLVM C API in LLVM 10
    pub operation: RMWBinOp,
    pub address: Operand,
    pub value: Operand,
    pub dest: Name,
    pub volatile: bool,
    pub atomicity: Atomicity,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(AtomicRMW, AtomicRMW);
impl_hasresult!(AtomicRMW);

impl Typed for AtomicRMW {
    fn get_type(&self, types: &Types) -> TypeRef {
        match types.type_of(&self.address).as_ref() {
            Type::PointerType { pointee_type, .. } => pointee_type.clone(),
            ty => panic!(
                "Expected an AtomicRMW address to be PointerType, got {:?}",
                ty
            ),
        }
    }
}

/// Get the address of a subelement of an aggregate data structure.
/// Only performs address calculation, does not actually access memory.
/// See [LLVM 10 docs on the 'getelementptr' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#getelementptr-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct GetElementPtr {
    pub address: Operand,
    pub indices: Vec<Operand>,
    pub dest: Name,
    pub in_bounds: bool,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(GetElementPtr, GetElementPtr);
impl_hasresult!(GetElementPtr);

impl Typed for GetElementPtr {
    fn get_type(&self, types: &Types) -> TypeRef {
        gep_type(types.type_of(&self.address), self.indices.iter(), types)
    }
}

fn gep_type<'o>(
    cur_type: TypeRef,
    mut indices: impl Iterator<Item = &'o Operand>,
    types: &Types,
) -> TypeRef {
    if let Type::NamedStructType { name } = cur_type.as_ref() {
        match types.named_struct_def(name) {
            None => panic!("Named struct without a definition (name {:?})", name),
            Some(NamedStructDef::Opaque) => {
                panic!("GEP on an opaque struct type (name {:?})", name)
            },
            Some(NamedStructDef::Defined(ty)) => {
                return gep_type(ty.clone(), indices, types);
            },
        }
    }
    match indices.next() {
        None => types.pointer_to(cur_type),  // iterator is done
        Some(index) => match cur_type.as_ref() {
            Type::PointerType { pointee_type, .. } => gep_type(pointee_type.clone(), indices, types),
            Type::VectorType { element_type, .. } => gep_type(element_type.clone(), indices, types),
            Type::ArrayType { element_type, .. } => gep_type(element_type.clone(), indices, types),
            Type::StructType { element_types, .. } => {
                if let Operand::ConstantOperand(cref) = index {
                    if let Constant::Int { value, .. } = cref.as_ref() {
                        gep_type(element_types.get(*value as usize).cloned().expect("GEP index out of range"), indices, types)
                    } else {
                        panic!("Expected GEP index on a struct to be a Constant::Int; got {:?}", cref)
                    }
                } else {
                    panic!("Expected GEP index on a struct to be a Operand::ConstantOperand(Constant::Int); got {:?}", index)
                }
            },
            Type::NamedStructType { .. } => panic!("This case should have been handled above"),
            _ => panic!("Expected GEP base type to be a PointerType, VectorType, ArrayType, StructType, or NamedStructType; got {:?}", cur_type),
        }
    }
}

/// Truncate.
/// See [LLVM 10 docs on the 'trunc' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#trunc-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Trunc {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Trunc, Trunc);
impl_unop!(Trunc);
explicitly_typed!(Trunc);

/// Zero-extend.
/// See [LLVM 10 docs on the 'zext' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#zext-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct ZExt {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(ZExt, ZExt);
impl_unop!(ZExt);
explicitly_typed!(ZExt);

/// Sign-extend.
/// See [LLVM 10 docs on the 'sext' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#sext-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct SExt {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(SExt, SExt);
impl_unop!(SExt);
explicitly_typed!(SExt);

/// Truncate a floating-point value.
/// See [LLVM 10 docs on the 'fptrunc' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#fptrunc-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct FPTrunc {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(FPTrunc, FPTrunc);
impl_unop!(FPTrunc);
explicitly_typed!(FPTrunc);

/// Extend a floating-point value.
/// See [LLVM 10 docs on the 'fpext' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#fpext-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct FPExt {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(FPExt, FPExt);
impl_unop!(FPExt);
explicitly_typed!(FPExt);

/// Convert floating-point to unsigned integer.
/// See [LLVM 10 docs on the 'fptoui' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#fptoui-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct FPToUI {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(FPToUI, FPToUI);
impl_unop!(FPToUI);
explicitly_typed!(FPToUI);

/// Convert floating-point to signed integer.
/// See [LLVM 10 docs on the 'fptosi' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#fptosi-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct FPToSI {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(FPToSI, FPToSI);
impl_unop!(FPToSI);
explicitly_typed!(FPToSI);

/// Convert unsigned integer to floating-point.
/// See [LLVM 10 docs on the 'uitofp' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#uitofp-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct UIToFP {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(UIToFP, UIToFP);
impl_unop!(UIToFP);
explicitly_typed!(UIToFP);

/// Convert signed integer to floating-point.
/// See [LLVM 10 docs on the 'sitofp' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#sitofp-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct SIToFP {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(SIToFP, SIToFP);
impl_unop!(SIToFP);
explicitly_typed!(SIToFP);

/// Convert pointer to integer.
/// See [LLVM 10 docs on the 'ptrtoint' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#ptrtoint-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct PtrToInt {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(PtrToInt, PtrToInt);
impl_unop!(PtrToInt);
explicitly_typed!(PtrToInt);

/// Convert integer to pointer.
/// See [LLVM 10 docs on the 'inttoptr' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#inttoptr-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct IntToPtr {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(IntToPtr, IntToPtr);
impl_unop!(IntToPtr);
explicitly_typed!(IntToPtr);

/// Convert between types without changing any bits.
/// See [LLVM 10 docs on the 'bitcast' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#bitcast-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct BitCast {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(BitCast, BitCast);
impl_unop!(BitCast);
explicitly_typed!(BitCast);

/// See [LLVM 10 docs on the 'addrspacecast' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#addrspacecast-to-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct AddrSpaceCast {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(AddrSpaceCast, AddrSpaceCast);
impl_unop!(AddrSpaceCast);
explicitly_typed!(AddrSpaceCast);

/// Compare integers, pointers, or vectors of integers or pointers.
/// See [LLVM 10 docs on the 'icmp' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#icmp-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct ICmp {
    pub predicate: IntPredicate,
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(ICmp, ICmp);
impl_hasresult!(ICmp);

impl Typed for ICmp {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        assert_eq!(ty, types.type_of(&self.operand1));
        match ty.as_ref() {
            Type::VectorType { num_elements, .. } => types.vector_of(types.bool(), *num_elements),
            _ => types.bool(),
        }
    }
}

/// Compare floating-point values or vectors of floating-point values.
/// See [LLVM 10 docs on the 'fcmp' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#fcmp-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct FCmp {
    pub predicate: FPPredicate,
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(FCmp, FCmp);
impl_hasresult!(FCmp);

impl Typed for FCmp {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        assert_eq!(ty, types.type_of(&self.operand1));
        match ty.as_ref() {
            Type::VectorType { num_elements, .. } => types.vector_of(types.bool(), *num_elements),
            _ => types.bool(),
        }
    }
}

/// See [LLVM 10 docs on the 'phi' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#phi-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Phi {
    pub incoming_values: Vec<(Operand, Name)>,
    pub dest: Name,
    pub to_type: TypeRef,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Phi, Phi);
impl_hasresult!(Phi);
explicitly_typed!(Phi);

/// See [LLVM 10 docs on the 'select' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#select-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Select {
    pub condition: Operand,
    pub true_value: Operand,
    pub false_value: Operand,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Select, Select);
impl_hasresult!(Select);

impl Typed for Select {
    fn get_type(&self, types: &Types) -> TypeRef {
        let t = types.type_of(&self.true_value);
        assert_eq!(t, types.type_of(&self.false_value));
        t
    }
}

/// See [LLVM 10 docs on the `freeze` instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#freeze-instruction)
#[cfg(LLVM_VERSION_10_OR_GREATER)]
#[derive(PartialEq, Clone, Debug)]
pub struct Freeze {
    pub operand: Operand,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

#[cfg(LLVM_VERSION_10_OR_GREATER)]
impl_inst!(Freeze, Freeze);
#[cfg(LLVM_VERSION_10_OR_GREATER)]
impl_unop!(Freeze);
#[cfg(LLVM_VERSION_10_OR_GREATER)]
unop_same_type!(Freeze);

/// Function call.
/// See [LLVM 10 docs on the 'call' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#call-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Call {
    pub function: Either<InlineAssembly, Operand>,
    pub arguments: Vec<(Operand, Vec<ParameterAttribute>)>,
    pub return_attributes: Vec<ParameterAttribute>,
    pub dest: Option<Name>, // will be None if the `function` returns void
    pub function_attributes: Vec<FunctionAttribute>, // llvm-hs has the equivalent of Vec<Either<GroupID, FunctionAttribute>>, but I'm not sure how the GroupID option comes up
    pub is_tail_call: bool, // llvm-hs has the more sophisticated structure Option<TailCallKind>, but the LLVM C API just gives us true/false
    pub calling_convention: CallingConvention,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(Call, Call);

impl Typed for Call {
    fn get_type(&self, types: &Types) -> TypeRef {
        match types.type_of(&self.function).as_ref() {
            Type::PointerType { pointee_type, .. } => match pointee_type.as_ref() {
                Type::FuncType { result_type, .. } => result_type.clone(),
                ty => panic!("Expected Call's function argument to be of type pointer-to-function, got pointer-to-{:?}", ty),
            },
            ty => panic!("Expected Call's function argument to be of type pointer-to-function, got {:?}", ty),
        }
    }
}

/// See [LLVM 10 docs on the 'va_arg' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#va-arg-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct VAArg {
    pub arg_list: Operand,
    pub cur_type: TypeRef,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(VAArg, VAArg);
impl_hasresult!(VAArg);

impl Typed for VAArg {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.cur_type.clone()
    }
}

/// Used for exception handling.
/// See [LLVM 10 docs on the 'landingpad' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#landingpad-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct LandingPad {
    pub result_type: TypeRef,
    pub clauses: Vec<LandingPadClause>,
    pub dest: Name,
    pub cleanup: bool,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(LandingPad, LandingPad);
impl_hasresult!(LandingPad);

impl Typed for LandingPad {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.result_type.clone()
    }
}

/// Used for exception handling.
/// See [LLVM 10 docs on the 'catchpad' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#catchpad-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct CatchPad {
    pub catch_switch: Operand,
    pub args: Vec<Operand>,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(CatchPad, CatchPad);
impl_hasresult!(CatchPad);

impl Typed for CatchPad {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.token_type()
    }
}

/// Used for exception handling.
/// See [LLVM 10 docs on the 'cleanuppad' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#cleanuppad-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct CleanupPad {
    pub parent_pad: Operand,
    pub args: Vec<Operand>,
    pub dest: Name,
    #[cfg(LLVM_VERSION_9_OR_GREATER)]
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(CleanupPad, CleanupPad);
impl_hasresult!(CleanupPad);

impl Typed for CleanupPad {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.token_type()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TailCallKind {
    Tail,
    MustTail,
    NoTail,
}

/// See [LLVM 10 docs on Fast-Math Flags](https://releases.llvm.org/10.0.0/docs/LangRef.html#fastmath)
#[derive(PartialEq, Eq, Clone, Debug)]
#[allow(non_snake_case)]
pub struct FastMathFlags {
    pub allow_reassoc: bool,
    pub no_NaNs: bool,
    pub no_Infs: bool,
    pub no_signed_zeros: bool,
    pub allow_reciprocal: bool,
    pub allow_contract: bool,
    pub approx_func: bool,
}

/// See [LLVM 10 docs on Atomic Memory Ordering Constraints](https://releases.llvm.org/10.0.0/docs/LangRef.html#ordering)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Atomicity {
    pub synch_scope: SynchronizationScope,
    pub mem_ordering: MemoryOrdering,
}

/// See [LLVM 10 docs on Atomic Memory Ordering Constraints](https://releases.llvm.org/10.0.0/docs/LangRef.html#ordering)
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SynchronizationScope {
    SingleThread,
    System,
}

/// See [LLVM 10 docs on Atomic Memory Ordering Constraints](https://releases.llvm.org/10.0.0/docs/LangRef.html#ordering)
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MemoryOrdering {
    Unordered,
    Monotonic,
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent,
    NotAtomic, // since we only have a `MemoryOrdering` on atomic instructions, we should never need this. But empirically, some atomic instructions -- e.g. the first 'atomicrmw' instruction in our 'atomic_no_syncscope' test -- have this `MemoryOrdering`
}

/// See [LLVM 10 docs on Inline Assembler Expressions](https://releases.llvm.org/10.0.0/docs/LangRef.html#inline-assembler-expressions).
// --TODO this seems to be the data structure we want. But see notes on
// InlineAssembly::from_llvm_ref()
/*
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct InlineAssembly {
    pub assembly: String,
    pub ty: TypeRef,
    pub constraints: String,
    pub has_side_effects: bool,
    pub align_stack: bool,
    pub dialect: AssemblyDialect,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum AssemblyDialect {
    ATT,
    Intel,
}
*/
// Instead we have this for now
/// `InlineAssembly` needs more fields, but the necessary getter functions are apparently not exposed in the LLVM C API (only the C++ API)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct InlineAssembly {
    pub ty: TypeRef,
}

impl Typed for InlineAssembly {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.ty.clone()
    }
}

/// See [LLVM 10 docs on the 'atomicrmw' instruction](https://releases.llvm.org/10.0.0/docs/LangRef.html#i-atomicrmw)
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RMWBinOp {
    Xchg,
    Add,
    Sub,
    And,
    Nand,
    Or,
    Xor,
    Max,
    Min,
    UMax,
    UMin,
    #[cfg(LLVM_VERSION_10_OR_GREATER)]
    FAdd,
    #[cfg(LLVM_VERSION_10_OR_GREATER)]
    FSub,
}

// --TODO this seems to be the data structure we want. But see notes on
// LandingPadClause::from_llvm_ref()
/*
#[derive(PartialEq, Clone, Debug)]
pub enum LandingPadClause {
    Catch(Constant),
    Filter(Constant),
}
*/
// Instead we have this for now
/// `LandingPadClause` needs more fields, but the necessary getter functions are apparently not exposed in the LLVM C API (only the C++ API)
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct LandingPadClause {}

// ********* //
// from_llvm //
// ********* //

use crate::constant::Constant;
use crate::from_llvm::*;
use crate::function::FunctionContext;
use crate::module::ModuleContext;
use crate::types::TypesBuilder;
use llvm_sys::LLVMAtomicOrdering;
#[cfg(LLVM_VERSION_10_OR_GREATER)]
use llvm_sys::LLVMAtomicRMWBinOp;
use llvm_sys::LLVMOpcode;
use llvm_sys::LLVMTypeKind::LLVMVoidTypeKind;

impl Instruction {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        debug!("Processing instruction {:?}", unsafe {
            print_to_string(inst)
        });
        match unsafe { LLVMGetInstructionOpcode(inst) } {
            LLVMOpcode::LLVMAdd => Instruction::Add(Add::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMSub => Instruction::Sub(Sub::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMMul => Instruction::Mul(Mul::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMUDiv => Instruction::UDiv(UDiv::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMSDiv => Instruction::SDiv(SDiv::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMURem => Instruction::URem(URem::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMSRem => Instruction::SRem(SRem::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMAnd => Instruction::And(And::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMOr => Instruction::Or(Or::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMXor => Instruction::Xor(Xor::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMShl => Instruction::Shl(Shl::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMLShr => Instruction::LShr(LShr::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMAShr => Instruction::AShr(AShr::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMFAdd => Instruction::FAdd(FAdd::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMFSub => Instruction::FSub(FSub::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMFMul => Instruction::FMul(FMul::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMFDiv => Instruction::FDiv(FDiv::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMFRem => Instruction::FRem(FRem::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMFNeg => Instruction::FNeg(FNeg::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMExtractElement => {
                Instruction::ExtractElement(ExtractElement::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMInsertElement => {
                Instruction::InsertElement(InsertElement::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMShuffleVector => {
                Instruction::ShuffleVector(ShuffleVector::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMExtractValue => {
                Instruction::ExtractValue(ExtractValue::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMInsertValue => {
                Instruction::InsertValue(InsertValue::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMAlloca => {
                Instruction::Alloca(Alloca::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMLoad => Instruction::Load(Load::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMStore => Instruction::Store(Store::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMFence => Instruction::Fence(Fence::from_llvm_ref(inst)),
            LLVMOpcode::LLVMAtomicCmpXchg => {
                Instruction::CmpXchg(CmpXchg::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMAtomicRMW => {
                Instruction::AtomicRMW(AtomicRMW::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMGetElementPtr => {
                Instruction::GetElementPtr(GetElementPtr::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMTrunc => Instruction::Trunc(Trunc::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMZExt => Instruction::ZExt(ZExt::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMSExt => Instruction::SExt(SExt::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMFPTrunc => {
                Instruction::FPTrunc(FPTrunc::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMFPExt => Instruction::FPExt(FPExt::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMFPToUI => {
                Instruction::FPToUI(FPToUI::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMFPToSI => {
                Instruction::FPToSI(FPToSI::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMUIToFP => {
                Instruction::UIToFP(UIToFP::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMSIToFP => {
                Instruction::SIToFP(SIToFP::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMPtrToInt => {
                Instruction::PtrToInt(PtrToInt::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMIntToPtr => {
                Instruction::IntToPtr(IntToPtr::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMBitCast => {
                Instruction::BitCast(BitCast::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMAddrSpaceCast => {
                Instruction::AddrSpaceCast(AddrSpaceCast::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMICmp => Instruction::ICmp(ICmp::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMFCmp => Instruction::FCmp(FCmp::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMPHI => Instruction::Phi(Phi::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMSelect => {
                Instruction::Select(Select::from_llvm_ref(inst, ctx, func_ctx))
            },
            #[cfg(LLVM_VERSION_10_OR_GREATER)]
            LLVMOpcode::LLVMFreeze => {
                Instruction::Freeze(Freeze::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMCall => Instruction::Call(Call::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMVAArg => Instruction::VAArg(VAArg::from_llvm_ref(inst, ctx, func_ctx)),
            LLVMOpcode::LLVMLandingPad => {
                Instruction::LandingPad(LandingPad::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMCatchPad => {
                Instruction::CatchPad(CatchPad::from_llvm_ref(inst, ctx, func_ctx))
            },
            LLVMOpcode::LLVMCleanupPad => {
                Instruction::CleanupPad(CleanupPad::from_llvm_ref(inst, ctx, func_ctx))
            },
            opcode => panic!(
                "Instruction::from_llvm_ref called with a terminator instruction (opcode {:?})",
                opcode
            ),
        }
    }
}

macro_rules! unop_from_llvm {
    ($inst:ident) => {
        impl $inst {
            pub(crate) fn from_llvm_ref(
                inst: LLVMValueRef,
                ctx: &mut ModuleContext,
                func_ctx: &mut FunctionContext,
            ) -> Self {
                assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 1);
                Self {
                    operand: Operand::from_llvm_ref(
                        unsafe { LLVMGetOperand(inst, 0) },
                        ctx,
                        func_ctx,
                    ),
                    dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
                    #[cfg(LLVM_VERSION_9_OR_GREATER)]
                    debugloc: DebugLoc::from_llvm_with_col(inst),
                    // metadata: InstructionMetadata::from_llvm_inst(inst),
                }
            }
        }
    };
}

macro_rules! binop_from_llvm {
    ($inst:ident) => {
        impl $inst {
            pub(crate) fn from_llvm_ref(
                inst: LLVMValueRef,
                ctx: &mut ModuleContext,
                func_ctx: &mut FunctionContext,
            ) -> Self {
                assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 2);
                Self {
                    operand0: Operand::from_llvm_ref(
                        unsafe { LLVMGetOperand(inst, 0) },
                        ctx,
                        func_ctx,
                    ),
                    operand1: Operand::from_llvm_ref(
                        unsafe { LLVMGetOperand(inst, 1) },
                        ctx,
                        func_ctx,
                    ),
                    dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
                    #[cfg(LLVM_VERSION_9_OR_GREATER)]
                    debugloc: DebugLoc::from_llvm_with_col(inst),
                    // metadata: InstructionMetadata::from_llvm_inst(inst),
                }
            }
        }
    };
}

binop_from_llvm!(Add);
binop_from_llvm!(Sub);
binop_from_llvm!(Mul);
binop_from_llvm!(UDiv);
binop_from_llvm!(SDiv);
binop_from_llvm!(URem);
binop_from_llvm!(SRem);
binop_from_llvm!(And);
binop_from_llvm!(Or);
binop_from_llvm!(Xor);
binop_from_llvm!(Shl);
binop_from_llvm!(LShr);
binop_from_llvm!(AShr);
binop_from_llvm!(FAdd);
binop_from_llvm!(FSub);
binop_from_llvm!(FMul);
binop_from_llvm!(FDiv);
binop_from_llvm!(FRem);
unop_from_llvm!(FNeg);
#[cfg(LLVM_VERSION_10_OR_GREATER)]
unop_from_llvm!(Freeze);

impl ExtractElement {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 2);
        Self {
            vector: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            index: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 1) }, ctx, func_ctx),
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl InsertElement {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 3);
        Self {
            vector: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            element: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 1) }, ctx, func_ctx),
            index: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 2) }, ctx, func_ctx),
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl ShuffleVector {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 3);
        Self {
            operand0: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            operand1: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 1) }, ctx, func_ctx),
            mask: Constant::from_llvm_ref(unsafe { LLVMGetOperand(inst, 2) }, ctx),
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl ExtractValue {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 1);
        Self {
            aggregate: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            indices: unsafe {
                let num_indices = LLVMGetNumIndices(inst);
                let ptr = LLVMGetIndices(inst);
                std::slice::from_raw_parts(ptr, num_indices as usize).to_vec()
            },
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl InsertValue {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 2);
        Self {
            aggregate: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            element: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 1) }, ctx, func_ctx),
            indices: unsafe {
                let num_indices = LLVMGetNumIndices(inst);
                let ptr = LLVMGetIndices(inst);
                std::slice::from_raw_parts(ptr, num_indices as usize).to_vec()
            },
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl Alloca {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 1);
        Self {
            allocated_type: ctx.types.type_from_llvm_ref(unsafe { LLVMGetAllocatedType(inst) }),
            num_elements: Operand::from_llvm_ref(
                unsafe { LLVMGetOperand(inst, 0) }, // This is a guess. or maybe num_elements is included in allocated_type?
                ctx,
                func_ctx,
            ),
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            alignment: unsafe { LLVMGetAlignment(inst) },
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl Load {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 1);
        Self {
            address: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            volatile: unsafe { LLVMGetVolatile(inst) } != 0,
            atomicity: {
                let ordering = unsafe { LLVMGetOrdering(inst) };
                if ordering == LLVMAtomicOrdering::LLVMAtomicOrderingNotAtomic {
                    None
                } else {
                    Some(Atomicity {
                        synch_scope: SynchronizationScope::from_llvm_ref(inst),
                        mem_ordering: MemoryOrdering::from_llvm(ordering),
                    })
                }
            },
            alignment: unsafe { LLVMGetAlignment(inst) },
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl Store {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 2);
        Self {
            address: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 1) }, ctx, func_ctx),
            value: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            volatile: unsafe { LLVMGetVolatile(inst) } != 0,
            atomicity: {
                let ordering = unsafe { LLVMGetOrdering(inst) };
                if ordering == LLVMAtomicOrdering::LLVMAtomicOrderingNotAtomic {
                    None
                } else {
                    Some(Atomicity {
                        synch_scope: SynchronizationScope::from_llvm_ref(inst),
                        mem_ordering: MemoryOrdering::from_llvm(ordering),
                    })
                }
            },
            alignment: unsafe { LLVMGetAlignment(inst) },
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl Fence {
    pub(crate) fn from_llvm_ref(inst: LLVMValueRef) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 0);
        Self {
            atomicity: Atomicity {
                synch_scope: SynchronizationScope::from_llvm_ref(inst),
                mem_ordering: MemoryOrdering::from_llvm(unsafe { LLVMGetOrdering(inst) }),
            },
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl CmpXchg {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 3);
        Self {
            address: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            expected: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 1) }, ctx, func_ctx),
            replacement: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 2) }, ctx, func_ctx),
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            volatile: unsafe { LLVMGetVolatile(inst) } != 0,
            atomicity: Atomicity {
                synch_scope: SynchronizationScope::from_llvm_ref(inst),
                mem_ordering: MemoryOrdering::from_llvm(unsafe {
                    LLVMGetCmpXchgSuccessOrdering(inst)
                }),
            },
            failure_memory_ordering: MemoryOrdering::from_llvm(unsafe {
                LLVMGetCmpXchgFailureOrdering(inst)
            }),
            #[cfg(LLVM_VERSION_10_OR_GREATER)]
            weak: unsafe { LLVMGetWeak(inst) } != 0,
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl AtomicRMW {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 2);
        Self {
            #[cfg(LLVM_VERSION_10_OR_GREATER)] // the binop-getter was added to the LLVM C API in LLVM 10
            operation: RMWBinOp::from_llvm(unsafe { LLVMGetAtomicRMWBinOp(inst) }),
            address: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            value: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 1) }, ctx, func_ctx),
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            volatile: unsafe { LLVMGetVolatile(inst) } != 0,
            atomicity: Atomicity {
                synch_scope: SynchronizationScope::from_llvm_ref(inst),
                mem_ordering: MemoryOrdering::from_llvm(unsafe { LLVMGetOrdering(inst) }),
            },
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl GetElementPtr {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        Self {
            address: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            indices: {
                let num_indices = unsafe { LLVMGetNumIndices(inst) };
                (1 ..= num_indices)
                    .map(|i| {
                        Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, i) }, ctx, func_ctx)
                    })
                    .collect()
            },
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            in_bounds: unsafe { LLVMIsInBounds(inst) } != 0,
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

// These instructions have the property that their result type is ambiguous from
//   knowing only their operands.
macro_rules! typed_unop_from_llvm {
    ($inst:ident) => {
        impl $inst {
            pub(crate) fn from_llvm_ref(
                inst: LLVMValueRef,
                ctx: &mut ModuleContext,
                func_ctx: &mut FunctionContext,
            ) -> Self {
                assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 1);
                Self {
                    operand: Operand::from_llvm_ref(
                        unsafe { LLVMGetOperand(inst, 0) },
                        ctx,
                        func_ctx,
                    ),
                    to_type: ctx.types.type_from_llvm_ref(unsafe { LLVMTypeOf(inst) }),
                    dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
                    #[cfg(LLVM_VERSION_9_OR_GREATER)]
                    debugloc: DebugLoc::from_llvm_with_col(inst),
                    // metadata: InstructionMetadata::from_llvm_inst(inst),
                }
            }
        }
    };
}

typed_unop_from_llvm!(Trunc);
typed_unop_from_llvm!(ZExt);
typed_unop_from_llvm!(SExt);
typed_unop_from_llvm!(FPTrunc);
typed_unop_from_llvm!(FPExt);
typed_unop_from_llvm!(FPToUI);
typed_unop_from_llvm!(FPToSI);
typed_unop_from_llvm!(UIToFP);
typed_unop_from_llvm!(SIToFP);
typed_unop_from_llvm!(PtrToInt);
typed_unop_from_llvm!(IntToPtr);
typed_unop_from_llvm!(BitCast);
typed_unop_from_llvm!(AddrSpaceCast);

impl ICmp {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 2);
        Self {
            predicate: IntPredicate::from_llvm(unsafe { LLVMGetICmpPredicate(inst) }),
            operand0: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            operand1: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 1) }, ctx, func_ctx),
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl FCmp {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 2);
        Self {
            predicate: FPPredicate::from_llvm(unsafe { LLVMGetFCmpPredicate(inst) }),
            operand0: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            operand1: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 1) }, ctx, func_ctx),
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl Phi {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        Self {
            incoming_values: {
                let num_incoming = unsafe { LLVMCountIncoming(inst) };
                (0 .. num_incoming)
                    .map(|i| {
                        let operand = Operand::from_llvm_ref(
                            unsafe { LLVMGetIncomingValue(inst, i) },
                            ctx,
                            func_ctx,
                        );
                        let name = func_ctx
                            .bb_names
                            .get(unsafe { &LLVMGetIncomingBlock(inst, i) })
                            .expect("Failed to find incoming block in the map")
                            .clone();
                        (operand, name)
                    })
                    .collect()
            },
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            to_type: ctx.types.type_from_llvm_ref(unsafe { LLVMTypeOf(inst) }),
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl Select {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 3);
        Self {
            condition: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            true_value: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 1) }, ctx, func_ctx),
            false_value: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 2) }, ctx, func_ctx),
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

// just the logic shared by Call and Invoke. Not a public struct, just an implementation convenience.
pub(crate) struct CallInfo {
    pub function: Either<InlineAssembly, Operand>,
    pub arguments: Vec<(Operand, Vec<ParameterAttribute>)>,
    pub return_attributes: Vec<ParameterAttribute>,
    pub function_attributes: Vec<FunctionAttribute>,
    pub calling_convention: CallingConvention,
}

impl CallInfo {
    // Call this function only an a Call instruction or Invoke terminator
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        use llvm_sys::{LLVMAttributeFunctionIndex, LLVMAttributeReturnIndex};
        Self {
            function: {
                let called_val = unsafe { LLVMGetCalledValue(inst) };
                let asm = unsafe { LLVMIsAInlineAsm(called_val) };
                if !asm.is_null() {
                    Either::Left(InlineAssembly::from_llvm_ref(asm, &mut ctx.types))
                } else {
                    Either::Right(Operand::from_llvm_ref(called_val, ctx, func_ctx))
                }
            },
            arguments: {
                let num_args: u32 = unsafe { LLVMGetNumArgOperands(inst) } as u32;
                (0 .. num_args) // arguments are (0 .. num_args); other operands (such as the called function) are after that
                    .map(|i| {
                        let operand = Operand::from_llvm_ref(
                            unsafe { LLVMGetOperand(inst, i) },
                            ctx,
                            func_ctx,
                        );
                        let attrs = {
                            let num_attrs =
                                unsafe { LLVMGetCallSiteAttributeCount(inst, (i + 1) as u32) }; // see LLVM C API (Core.h) comments on `LLVMAttributeReturnIndex` and `LLVMAttributeFunctionIndex`
                            let mut attrs: Vec<LLVMAttributeRef> =
                                Vec::with_capacity(num_attrs as usize);
                            unsafe {
                                LLVMGetCallSiteAttributes(inst, (i + 1) as u32, attrs.as_mut_ptr());
                                attrs.set_len(num_attrs as usize);
                            };
                            attrs
                                .into_iter()
                                .map(|attr| ParameterAttribute::from_llvm_ref(attr, &ctx.attrsdata))
                                .collect()
                        };
                        (operand, attrs)
                    })
                    .collect()
            },
            return_attributes: {
                let num_attrs =
                    unsafe { LLVMGetCallSiteAttributeCount(inst, LLVMAttributeReturnIndex) };
                let mut attrs: Vec<LLVMAttributeRef> = Vec::with_capacity(num_attrs as usize);
                unsafe {
                    LLVMGetCallSiteAttributes(inst, LLVMAttributeReturnIndex, attrs.as_mut_ptr());
                    attrs.set_len(num_attrs as usize);
                };
                attrs
                    .into_iter()
                    .map(|attr| ParameterAttribute::from_llvm_ref(attr, &ctx.attrsdata))
                    .collect()
            },
            function_attributes: {
                let num_attrs =
                    unsafe { LLVMGetCallSiteAttributeCount(inst, LLVMAttributeFunctionIndex) };
                let mut attrs: Vec<LLVMAttributeRef> = Vec::with_capacity(num_attrs as usize);
                unsafe {
                    LLVMGetCallSiteAttributes(inst, LLVMAttributeFunctionIndex, attrs.as_mut_ptr());
                    attrs.set_len(num_attrs as usize);
                };
                attrs
                    .into_iter()
                    .map(|attr| FunctionAttribute::from_llvm_ref(attr, &ctx.attrsdata))
                    .collect()
            },
            calling_convention: CallingConvention::from_u32(unsafe {
                LLVMGetInstructionCallConv(inst)
            }),
        }
    }
}

impl Call {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        let callinfo = CallInfo::from_llvm_ref(inst, ctx, func_ctx);
        Self {
            function: callinfo.function,
            arguments: callinfo.arguments,
            return_attributes: callinfo.return_attributes,
            dest: if unsafe {
                LLVMGetTypeKind(LLVMGetReturnType(LLVMGetCalledFunctionType(inst)))
                    == LLVMVoidTypeKind
            } {
                None
            } else {
                Some(Name::name_or_num(
                    unsafe { get_value_name(inst) },
                    &mut func_ctx.ctr,
                ))
            },
            function_attributes: callinfo.function_attributes,
            is_tail_call: unsafe { LLVMIsTailCall(inst) } != 0,
            calling_convention: callinfo.calling_convention,
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl VAArg {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(inst) }, 1);
        Self {
            arg_list: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            cur_type: ctx.types.type_from_llvm_ref(unsafe { LLVMTypeOf(inst) }),
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl LandingPad {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        Self {
            result_type: ctx.types.type_from_llvm_ref(unsafe { LLVMTypeOf(inst) }),
            clauses: {
                let num_clauses = unsafe { LLVMGetNumClauses(inst) };
                (0 .. num_clauses)
                    .map(|i| LandingPadClause::from_llvm_ref(unsafe { LLVMGetClause(inst, i) }))
                    .collect()
            },
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            cleanup: unsafe { LLVMIsCleanup(inst) } != 0,
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl CatchPad {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        Self {
            catch_switch: Operand::from_llvm_ref(
                unsafe { LLVMGetParentCatchSwitch(inst) },
                ctx,
                func_ctx,
            ),
            args: {
                let num_args = unsafe { LLVMGetNumArgOperands(inst) };
                (0 .. num_args)
                    .map(|i| {
                        Operand::from_llvm_ref(unsafe { LLVMGetArgOperand(inst, i) }, ctx, func_ctx)
                    })
                    .collect()
            },
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl CleanupPad {
    pub(crate) fn from_llvm_ref(
        inst: LLVMValueRef,
        ctx: &mut ModuleContext,
        func_ctx: &mut FunctionContext,
    ) -> Self {
        Self {
            parent_pad: Operand::from_llvm_ref(unsafe { LLVMGetOperand(inst, 0) }, ctx, func_ctx),
            args: {
                let num_args = unsafe { LLVMGetNumArgOperands(inst) };
                (0 .. num_args)
                    .map(|i| {
                        Operand::from_llvm_ref(unsafe { LLVMGetArgOperand(inst, i) }, ctx, func_ctx)
                    })
                    .collect()
            },
            dest: Name::name_or_num(unsafe { get_value_name(inst) }, &mut func_ctx.ctr),
            #[cfg(LLVM_VERSION_9_OR_GREATER)]
            debugloc: DebugLoc::from_llvm_with_col(inst),
            // metadata: InstructionMetadata::from_llvm_inst(inst),
        }
    }
}

impl SynchronizationScope {
    pub(crate) fn from_llvm_ref(inst: LLVMValueRef) -> Self {
        if unsafe { LLVMIsAtomicSingleThread(inst) } != 0 {
            SynchronizationScope::SingleThread
        } else {
            SynchronizationScope::System
        }
    }
}

impl MemoryOrdering {
    #[rustfmt::skip] // each one on one line, even if lines get a little long
    pub(crate) fn from_llvm(ao: LLVMAtomicOrdering) -> Self {
        match ao {
            LLVMAtomicOrdering::LLVMAtomicOrderingUnordered => MemoryOrdering::Unordered,
            LLVMAtomicOrdering::LLVMAtomicOrderingMonotonic => MemoryOrdering::Monotonic,
            LLVMAtomicOrdering::LLVMAtomicOrderingAcquire => MemoryOrdering::Acquire,
            LLVMAtomicOrdering::LLVMAtomicOrderingRelease => MemoryOrdering::Release,
            LLVMAtomicOrdering::LLVMAtomicOrderingAcquireRelease => MemoryOrdering::AcquireRelease,
            LLVMAtomicOrdering::LLVMAtomicOrderingSequentiallyConsistent => MemoryOrdering::SequentiallyConsistent,
            LLVMAtomicOrdering::LLVMAtomicOrderingNotAtomic => MemoryOrdering::NotAtomic,
        }
    }
}

#[cfg(LLVM_VERSION_10_OR_GREATER)]
impl RMWBinOp {
    pub(crate) fn from_llvm(rmwbo: LLVMAtomicRMWBinOp) -> Self {
        match rmwbo {
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpXchg => Self::Xchg,
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpAdd => Self::Add,
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpSub => Self::Sub,
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpAnd => Self::And,
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpNand => Self::Nand,
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpOr => Self::Or,
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpXor => Self::Xor,
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpMax => Self::Max,
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpMin => Self::Min,
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpUMax => Self::UMax,
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpUMin => Self::UMin,
            #[cfg(LLVM_VERSION_10_OR_GREATER)]
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpFAdd => Self::FAdd,
            #[cfg(LLVM_VERSION_10_OR_GREATER)]
            LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpFSub => Self::FSub,
        }
    }
}

impl InlineAssembly {
    pub(crate) fn from_llvm_ref(asm: LLVMValueRef, types: &mut TypesBuilder) -> Self {
        // The LLVM C API appears to have no way to get any information about an
        // `InlineAssembly`? You can tell whether an `LLVMValueRef` is an
        // `InlineAssembly`, but once you know it is one, there seem to be no
        // other related methods
        Self {
            ty: types.type_from_llvm_ref(unsafe { LLVMTypeOf(asm) }),
        }
    }
}

impl LandingPadClause {
    pub(crate) fn from_llvm_ref(_lpc: LLVMValueRef) -> Self {
        // The LLVM C API has an enum `LLVMLandingPadClauseTy`, but appears not
        // to reference it. In particular, it's unclear how to tell whether a
        // given clause is a `Catch` or a `Filter`.
        Self {}
    }
}
