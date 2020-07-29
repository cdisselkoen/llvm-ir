use crate::name::Name;
use crate::predicates::*;
use crate::types::{Type, Typed};
use std::convert::TryFrom;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

/// See [LLVM 10 docs on Constants](https://releases.llvm.org/10.0.0/docs/LangRef.html#constants).
/// Constants can be either values, or expressions involving other constants (see [LLVM 10 docs on Constant Expressions](https://releases.llvm.org/10.0.0/docs/LangRef.html#constant-expressions)).
#[derive(PartialEq, Clone, Debug)]
pub enum Constant {
    Int {
        bits: u32,
        value: u64, // If the Int is less than 64 bits, the value will be zero-extended to create the Rust `u64` `value` (so if `bits` is 8, the lowest 8 bits of `value` are the relevant bits, and the others are all zeroes). Note that LLVM integers aren't signed or unsigned; each individual instruction indicates whether it's treating the integer as signed or unsigned if necessary (e.g., UDiv vs SDiv).
    },
    Float(Float),
    /// The `Type` here must be a `PointerType`. See [LLVM 10 docs on Simple Constants](https://releases.llvm.org/10.0.0/docs/LangRef.html#simple-constants)
    Null(Type),
    /// A zero-initialized array or struct (or scalar).
    AggregateZero(Type),
    Struct {
        name: Option<String>, // llvm-hs-pure has Option<Name> here, but I don't think struct types can be numbered
        values: Vec<Constant>,
        is_packed: bool,
    },
    Array {
        element_type: Type,
        elements: Vec<Constant>,
    },
    Vector(Vec<Constant>),
    /// `Undef` can be used anywhere a constant is expected. See [LLVM 10 docs on Undefined Values](https://releases.llvm.org/10.0.0/docs/LangRef.html#undefined-values)
    Undef(Type),
    /// The address of the given (non-entry) [`BasicBlock`](../struct.BasicBlock.html). See [LLVM 10 docs on Addresses of Basic Blocks](https://releases.llvm.org/10.0.0/docs/LangRef.html#addresses-of-basic-blocks).
    /// `BlockAddress` needs more fields, but the necessary getter functions are apparently not exposed in the LLVM C API (only the C++ API)
    BlockAddress, // --TODO ideally we want BlockAddress { function: Name, block: Name },
    GlobalReference {
        name: Name,
        ty: Type,
    },
    TokenNone,

    // Constants can also be expressed as operations applied to other constants:

    // Integer binary ops
    Add(Box<Add>),
    Sub(Box<Sub>),
    Mul(Box<Mul>),
    UDiv(Box<UDiv>),
    SDiv(Box<SDiv>),
    URem(Box<URem>),
    SRem(Box<SRem>),

    // Bitwise binary ops
    And(Box<And>),
    Or(Box<Or>),
    Xor(Box<Xor>),
    Shl(Box<Shl>),
    LShr(Box<LShr>),
    AShr(Box<AShr>),

    // Floating-point ops
    FAdd(Box<FAdd>),
    FSub(Box<FSub>),
    FMul(Box<FMul>),
    FDiv(Box<FDiv>),
    FRem(Box<FRem>),

    // Vector ops
    ExtractElement(Box<ExtractElement>),
    InsertElement(Box<InsertElement>),
    ShuffleVector(Box<ShuffleVector>),

    // Aggregate ops
    ExtractValue(Box<ExtractValue>),
    InsertValue(Box<InsertValue>),

    // Memory-related ops
    GetElementPtr(Box<GetElementPtr>),

    // Conversion ops
    Trunc(Box<Trunc>),
    ZExt(Box<ZExt>),
    SExt(Box<SExt>),
    FPTrunc(Box<FPTrunc>),
    FPExt(Box<FPExt>),
    FPToUI(Box<FPToUI>),
    FPToSI(Box<FPToSI>),
    UIToFP(Box<UIToFP>),
    SIToFP(Box<SIToFP>),
    PtrToInt(Box<PtrToInt>),
    IntToPtr(Box<IntToPtr>),
    BitCast(Box<BitCast>),
    AddrSpaceCast(Box<AddrSpaceCast>),

    // Other ops
    ICmp(Box<ICmp>),
    FCmp(Box<FCmp>),
    Select(Box<Select>),
}

/// All of these `Float` variants should have data associated with them, but
/// Rust only has `f32` and `f64` floating-point types, and furthermore,
/// it's not clear how to get 16-, 80-, or 128-bit FP constant values through
/// the LLVM C API (the getters seem to only be exposed in the C++ API?)
#[derive(PartialEq, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Float {
    Half, // TODO perhaps Half(u16)
    Single(f32),
    Double(f64),
    Quadruple, // TODO perhaps Quadruple(u128)
    X86_FP80,  // TODO perhaps X86_FP80((u16, u64)) with the most-significant bits on the left
    PPC_FP128, // TODO perhaps PPC_FP128((u64, u64)) with the most-significant bits on the left
}

impl Typed for Float {
    fn get_type(&self) -> Type {
        match self {
            Float::Half => Type::half(),
            Float::Single(_) => Type::single(),
            Float::Double(_) => Type::double(),
            Float::Quadruple => Type::FPType(FPType::FP128),
            Float::X86_FP80 => Type::FPType(FPType::X86_FP80),
            Float::PPC_FP128 => Type::FPType(FPType::PPC_FP128),
        }
    }
}

impl Typed for Constant {
    fn get_type(&self) -> Type {
        match self {
            Constant::Int { bits, .. } => Type::IntegerType { bits: *bits },
            Constant::Float(f) => f.get_type(),
            Constant::Null(t) => t.clone(),
            Constant::AggregateZero(t) => t.clone(),
            Constant::Struct { values, is_packed, .. } => Type::StructType {
                element_types: values.iter().map(|v| v.get_type()).collect(),
                is_packed: *is_packed,
            },
            Constant::Array { element_type, elements } => Type::ArrayType {
                element_type: Box::new(element_type.clone()),
                num_elements: elements.len(),
            },
            Constant::Vector(v) => Type::VectorType {
                element_type: Box::new(v[0].get_type()),
                num_elements: v.len(),
            },
            Constant::Undef(t) => t.clone(),
            Constant::BlockAddress { .. } => Type::LabelType,
            Constant::GlobalReference { ty, .. } => Type::pointer_to(ty.clone()),
            Constant::TokenNone => Type::TokenType,
            Constant::Add(a) => a.get_type(),
            Constant::Sub(s) => s.get_type(),
            Constant::Mul(m) => m.get_type(),
            Constant::UDiv(d) => d.get_type(),
            Constant::SDiv(d) => d.get_type(),
            Constant::URem(r) => r.get_type(),
            Constant::SRem(r) => r.get_type(),
            Constant::And(a) => a.get_type(),
            Constant::Or(o) => o.get_type(),
            Constant::Xor(x) => x.get_type(),
            Constant::Shl(s) => s.get_type(),
            Constant::LShr(l) => l.get_type(),
            Constant::AShr(a) => a.get_type(),
            Constant::FAdd(f) => f.get_type(),
            Constant::FSub(f) => f.get_type(),
            Constant::FMul(f) => f.get_type(),
            Constant::FDiv(f) => f.get_type(),
            Constant::FRem(f) => f.get_type(),
            Constant::ExtractElement(e) => e.get_type(),
            Constant::InsertElement(i) => i.get_type(),
            Constant::ShuffleVector(s) => s.get_type(),
            Constant::ExtractValue(e) => e.get_type(),
            Constant::InsertValue(i) => i.get_type(),
            Constant::GetElementPtr(g) => g.get_type(),
            Constant::Trunc(t) => t.get_type(),
            Constant::ZExt(z) => z.get_type(),
            Constant::SExt(s) => s.get_type(),
            Constant::FPTrunc(f) => f.get_type(),
            Constant::FPExt(f) => f.get_type(),
            Constant::FPToUI(f) => f.get_type(),
            Constant::FPToSI(f) => f.get_type(),
            Constant::UIToFP(u) => u.get_type(),
            Constant::SIToFP(s) => s.get_type(),
            Constant::PtrToInt(p) => p.get_type(),
            Constant::IntToPtr(i) => i.get_type(),
            Constant::BitCast(b) => b.get_type(),
            Constant::AddrSpaceCast(a) => a.get_type(),
            Constant::ICmp(i) => i.get_type(),
            Constant::FCmp(f) => f.get_type(),
            Constant::Select(s) => s.get_type(),
        }
    }
}

pub trait ConstUnaryOp {
    fn get_operand(&self) -> &Constant;
}

pub trait ConstBinaryOp {
    fn get_operand0(&self) -> &Constant;
    fn get_operand1(&self) -> &Constant;
}

macro_rules! impl_constexpr {
    ($expr:ty, $id:ident) => {
        impl From<$expr> for Constant {
            fn from(expr: $expr) -> Constant {
                Constant::$id(Box::new(expr))
            }
        }

        impl TryFrom<Constant> for $expr {
            type Error = &'static str;
            fn try_from(constant: Constant) -> Result<Self, Self::Error> {
                match constant {
                    Constant::$id(expr) => Ok(*expr),
                    _ => Err("Constant is not of requested type"),
                }
            }
        }
    };
}

macro_rules! impl_unop {
    ($expr:ty) => {
        impl ConstUnaryOp for $expr {
            fn get_operand(&self) -> &Constant {
                &self.operand
            }
        }
    };
}

macro_rules! impl_binop {
    ($expr:ty) => {
        impl ConstBinaryOp for $expr {
            fn get_operand0(&self) -> &Constant {
                &self.operand0
            }
            fn get_operand1(&self) -> &Constant {
                &self.operand1
            }
        }
    };
}

// Use on binops where the result type is the same as both operand types
macro_rules! binop_same_type {
    ($expr:ty) => {
        impl Typed for $expr {
            fn get_type(&self) -> Type {
                let t = self.get_operand0().get_type();
                assert_eq!(t, self.get_operand1().get_type());
                t
            }
        }
    };
}

// Use on binops where the result type is the same as the first operand type
macro_rules! binop_left_type {
    ($expr:ty) => {
        impl Typed for $expr {
            fn get_type(&self) -> Type {
                self.get_operand0().get_type()
            }
        }
    };
}

// Use on anything that has a 'to_type' field which indicates its result type
macro_rules! explicitly_typed {
    ($expr:ty) => {
        impl Typed for $expr {
            fn get_type(&self) -> Type {
                self.to_type.clone()
            }
        }
    };
}

#[derive(PartialEq, Clone, Debug)]
pub struct Add {
    pub operand0: Constant,
    pub operand1: Constant,
    // pub nsw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    // pub nuw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
}

impl_constexpr!(Add, Add);
impl_binop!(Add);
binop_same_type!(Add);

#[derive(PartialEq, Clone, Debug)]
pub struct Sub {
    pub operand0: Constant,
    pub operand1: Constant,
    // pub nsw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    // pub nuw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
}

impl_constexpr!(Sub, Sub);
impl_binop!(Sub);
binop_same_type!(Sub);

#[derive(PartialEq, Clone, Debug)]
pub struct Mul {
    pub operand0: Constant,
    pub operand1: Constant,
    // pub nsw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    // pub nuw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
}

impl_constexpr!(Mul, Mul);
impl_binop!(Mul);
binop_same_type!(Mul);

#[derive(PartialEq, Clone, Debug)]
pub struct UDiv {
    pub operand0: Constant,
    pub operand1: Constant,
    // pub exact: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
}

impl_constexpr!(UDiv, UDiv);
impl_binop!(UDiv);
binop_same_type!(UDiv);

#[derive(PartialEq, Clone, Debug)]
pub struct SDiv {
    pub operand0: Constant,
    pub operand1: Constant,
    // pub exact: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
}

impl_constexpr!(SDiv, SDiv);
impl_binop!(SDiv);
binop_same_type!(SDiv);

#[derive(PartialEq, Clone, Debug)]
pub struct URem {
    pub operand0: Constant,
    pub operand1: Constant,
}

impl_constexpr!(URem, URem);
impl_binop!(URem);
binop_same_type!(URem);

#[derive(PartialEq, Clone, Debug)]
pub struct SRem {
    pub operand0: Constant,
    pub operand1: Constant,
}

impl_constexpr!(SRem, SRem);
impl_binop!(SRem);
binop_same_type!(SRem);

#[derive(PartialEq, Clone, Debug)]
pub struct And {
    pub operand0: Constant,
    pub operand1: Constant,
}

impl_constexpr!(And, And);
impl_binop!(And);
binop_same_type!(And);

#[derive(PartialEq, Clone, Debug)]
pub struct Or {
    pub operand0: Constant,
    pub operand1: Constant,
}

impl_constexpr!(Or, Or);
impl_binop!(Or);
binop_same_type!(Or);

#[derive(PartialEq, Clone, Debug)]
pub struct Xor {
    pub operand0: Constant,
    pub operand1: Constant,
}

impl_constexpr!(Xor, Xor);
impl_binop!(Xor);
binop_same_type!(Xor);

#[derive(PartialEq, Clone, Debug)]
pub struct Shl {
    pub operand0: Constant,
    pub operand1: Constant,
    // pub nsw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
    // pub nuw: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
}

impl_constexpr!(Shl, Shl);
impl_binop!(Shl);
binop_left_type!(Shl);

#[derive(PartialEq, Clone, Debug)]
pub struct LShr {
    pub operand0: Constant,
    pub operand1: Constant,
    // pub exact: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
}

impl_constexpr!(LShr, LShr);
impl_binop!(LShr);
binop_left_type!(LShr);

#[derive(PartialEq, Clone, Debug)]
pub struct AShr {
    pub operand0: Constant,
    pub operand1: Constant,
    // pub exact: bool,  // getters for these seem to not be exposed in the LLVM C API, only in the C++ one
}

impl_constexpr!(AShr, AShr);
impl_binop!(AShr);
binop_left_type!(AShr);

#[derive(PartialEq, Clone, Debug)]
pub struct FAdd {
    pub operand0: Constant,
    pub operand1: Constant,
}

impl_constexpr!(FAdd, FAdd);
impl_binop!(FAdd);
binop_same_type!(FAdd);

#[derive(PartialEq, Clone, Debug)]
pub struct FSub {
    pub operand0: Constant,
    pub operand1: Constant,
}

impl_constexpr!(FSub, FSub);
impl_binop!(FSub);
binop_same_type!(FSub);

#[derive(PartialEq, Clone, Debug)]
pub struct FMul {
    pub operand0: Constant,
    pub operand1: Constant,
}

impl_constexpr!(FMul, FMul);
impl_binop!(FMul);
binop_same_type!(FMul);

#[derive(PartialEq, Clone, Debug)]
pub struct FDiv {
    pub operand0: Constant,
    pub operand1: Constant,
}

impl_constexpr!(FDiv, FDiv);
impl_binop!(FDiv);
binop_same_type!(FDiv);

#[derive(PartialEq, Clone, Debug)]
pub struct FRem {
    pub operand0: Constant,
    pub operand1: Constant,
}

impl_constexpr!(FRem, FRem);
impl_binop!(FRem);
binop_same_type!(FRem);

#[derive(PartialEq, Clone, Debug)]
pub struct ExtractElement {
    pub vector: Constant,
    pub index: Constant,
}

impl_constexpr!(ExtractElement, ExtractElement);

impl Typed for ExtractElement {
    fn get_type(&self) -> Type {
        match self.vector.get_type() {
            Type::VectorType { element_type, .. } => *element_type,
            ty => panic!("Expected an ExtractElement vector to be VectorType, got {:?}", ty),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct InsertElement {
    pub vector: Constant,
    pub element: Constant,
    pub index: Constant,
}

impl_constexpr!(InsertElement, InsertElement);

impl Typed for InsertElement {
    fn get_type(&self) -> Type {
        self.vector.get_type()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ShuffleVector {
    pub operand0: Constant,
    pub operand1: Constant,
    pub mask: Constant,
}

impl_constexpr!(ShuffleVector, ShuffleVector);
impl_binop!(ShuffleVector);

impl Typed for ShuffleVector {
    fn get_type(&self) -> Type {
        let ty = self.operand0.get_type();
        assert_eq!(ty, self.operand1.get_type());
        match ty {
            Type::VectorType { element_type, .. } => match self.mask.get_type() {
                Type::VectorType { num_elements, .. } => Type::VectorType { element_type, num_elements },
                ty => panic!("Expected a ShuffleVector mask to be VectorType, got {:?}", ty),
            },
            _ => panic!("Expected a ShuffleVector operand to be VectorType, got {:?}", ty),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ExtractValue {
    pub aggregate: Constant,
    pub indices: Vec<u32>,
}

impl_constexpr!(ExtractValue, ExtractValue);

impl Typed for ExtractValue {
    fn get_type(&self) -> Type {
        ev_type(self.aggregate.get_type(), self.indices.iter().copied())
    }
}

fn ev_type(cur_type: Type, mut indices: impl Iterator<Item = u32>) -> Type {
    match indices.next() {
        None => cur_type,
        Some(index) => match cur_type {
            Type::ArrayType { element_type, .. } => ev_type(*element_type, indices),
            Type::StructType { element_types, .. } => ev_type(
                element_types
                    .get(index as usize)
                    .expect("ExtractValue index out of range")
                    .clone(),
                indices,
            ),
            _ => panic!("ExtractValue from something that's not ArrayType or StructType; its type is {:?}", cur_type),
        },
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct InsertValue {
    pub aggregate: Constant,
    pub element: Constant,
    pub indices: Vec<u32>,
}

impl_constexpr!(InsertValue, InsertValue);

impl Typed for InsertValue {
    fn get_type(&self) -> Type {
        self.aggregate.get_type()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct GetElementPtr {
    pub address: Constant,
    pub indices: Vec<Constant>,
    pub in_bounds: bool,
}

impl_constexpr!(GetElementPtr, GetElementPtr);

impl Typed for GetElementPtr {
    fn get_type(&self) -> Type {
        gep_type(&self.address.get_type(), self.indices.iter())
    }
}

fn gep_type<'a, 'b>(cur_type: &'a Type, mut indices: impl Iterator<Item = &'b Constant>) -> Type {
    match indices.next() {
        None => Type::pointer_to(cur_type.clone()), // iterator is done
        Some(index) => match cur_type {
            Type::PointerType { pointee_type, .. } => gep_type(pointee_type, indices),
            Type::VectorType { element_type, .. } => gep_type(element_type, indices),
            Type::ArrayType { element_type, .. } => gep_type(element_type, indices),
            Type::StructType { element_types, .. } => {
                if let Constant::Int { value, .. } = index {
                    gep_type(
                        element_types.get(*value as usize).expect("GEP index out of range"),
                        indices,
                    )
                } else {
                    panic!("Expected GEP index on a constant struct to be a Constant::Int; got {:?}", index)
                }
            },
            Type::NamedStructType { ty, .. } => match ty {
                None => panic!("GEP on an opaque struct type"),
                Some(weak) => match weak.upgrade().expect("Weak reference disappeared").read().unwrap().deref() {
                    Type::StructType { element_types, .. } => {
                        if let Constant::Int { value, .. } = index {
                            gep_type(element_types.get(*value as usize).expect("GEP index out of range"), indices)
                        } else {
                            panic!("Expected GEP index on a struct to be a Constant::Int; got {:?}", index)
                        }
                    },
                    ty => panic!("Expected NamedStructType inner type to be a StructType; got {:?}", ty),
                },
            }
            _ => panic!("Expected GEP base type to be a PointerType, VectorType, ArrayType, StructType, or NamedStructType; got {:?}", cur_type),
        },
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Trunc {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(Trunc, Trunc);
impl_unop!(Trunc);
explicitly_typed!(Trunc);

#[derive(PartialEq, Clone, Debug)]
pub struct ZExt {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(ZExt, ZExt);
impl_unop!(ZExt);
explicitly_typed!(ZExt);

#[derive(PartialEq, Clone, Debug)]
pub struct SExt {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(SExt, SExt);
impl_unop!(SExt);
explicitly_typed!(SExt);

#[derive(PartialEq, Clone, Debug)]
pub struct FPTrunc {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(FPTrunc, FPTrunc);
impl_unop!(FPTrunc);
explicitly_typed!(FPTrunc);

#[derive(PartialEq, Clone, Debug)]
pub struct FPExt {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(FPExt, FPExt);
impl_unop!(FPExt);
explicitly_typed!(FPExt);

#[derive(PartialEq, Clone, Debug)]
pub struct FPToUI {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(FPToUI, FPToUI);
impl_unop!(FPToUI);
explicitly_typed!(FPToUI);

#[derive(PartialEq, Clone, Debug)]
pub struct FPToSI {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(FPToSI, FPToSI);
impl_unop!(FPToSI);
explicitly_typed!(FPToSI);

#[derive(PartialEq, Clone, Debug)]
pub struct UIToFP {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(UIToFP, UIToFP);
impl_unop!(UIToFP);
explicitly_typed!(UIToFP);

#[derive(PartialEq, Clone, Debug)]
pub struct SIToFP {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(SIToFP, SIToFP);
impl_unop!(SIToFP);
explicitly_typed!(SIToFP);

#[derive(PartialEq, Clone, Debug)]
pub struct PtrToInt {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(PtrToInt, PtrToInt);
impl_unop!(PtrToInt);
explicitly_typed!(PtrToInt);

#[derive(PartialEq, Clone, Debug)]
pub struct IntToPtr {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(IntToPtr, IntToPtr);
impl_unop!(IntToPtr);
explicitly_typed!(IntToPtr);

#[derive(PartialEq, Clone, Debug)]
pub struct BitCast {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(BitCast, BitCast);
impl_unop!(BitCast);
explicitly_typed!(BitCast);

#[derive(PartialEq, Clone, Debug)]
pub struct AddrSpaceCast {
    pub operand: Constant,
    pub to_type: Type,
}

impl_constexpr!(AddrSpaceCast, AddrSpaceCast);
impl_unop!(AddrSpaceCast);
explicitly_typed!(AddrSpaceCast);

#[derive(PartialEq, Clone, Debug)]
pub struct ICmp {
    pub predicate: IntPredicate,
    pub operand0: Constant,
    pub operand1: Constant,
}

impl_constexpr!(ICmp, ICmp);
impl_binop!(ICmp);

impl Typed for ICmp {
    fn get_type(&self) -> Type {
        let t = self.operand0.get_type();
        assert_eq!(t, self.operand1.get_type());
        match t {
            Type::VectorType { num_elements, .. } => Type::VectorType {
                element_type: Box::new(Type::bool()),
                num_elements,
            },
            _ => Type::bool(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct FCmp {
    pub predicate: FPPredicate,
    pub operand0: Constant,
    pub operand1: Constant,
}

impl_constexpr!(FCmp, FCmp);
impl_binop!(FCmp);

impl Typed for FCmp {
    fn get_type(&self) -> Type {
        let t = self.operand0.get_type();
        assert_eq!(t, self.operand1.get_type());
        match t {
            Type::VectorType { num_elements, .. } => Type::VectorType {
                element_type: Box::new(Type::bool()),
                num_elements,
            },
            _ => Type::bool(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Select {
    pub condition: Constant,
    pub true_value: Constant,
    pub false_value: Constant,
}

impl_constexpr!(Select, Select);

impl Typed for Select {
    fn get_type(&self) -> Type {
        let t = self.true_value.get_type();
        assert_eq!(t, self.false_value.get_type());
        t
    }
}

// ********* //
// from_llvm //
// ********* //

use crate::from_llvm::*;
use crate::types::FPType;
use crate::types::TyNameMap;
use std::collections::HashMap;

pub(crate) type GlobalNameMap = HashMap<LLVMValueRef, Name>;

impl Constant {
    pub(crate) fn from_llvm_ref(
        constant: LLVMValueRef,
        gnmap: &GlobalNameMap,
        tnmap: &mut TyNameMap,
    ) -> Self {
        use llvm_sys::LLVMValueKind;
        if unsafe { LLVMIsAConstant(constant).is_null() } {
            panic!(
                "Constant::from_llvm_ref: argument wasn't a constant; ValueKind {:?}",
                unsafe { LLVMGetValueKind(constant) }
            )
        }
        match unsafe { LLVMGetValueKind(constant) } {
            LLVMValueKind::LLVMConstantIntValueKind => {
                match Type::from_llvm_ref( unsafe { LLVMTypeOf(constant) }, tnmap ) {
                    Type::IntegerType { bits } => Constant::Int {
                        bits,
                        value: unsafe { LLVMConstIntGetZExtValue(constant) } as u64,
                    },
                    ty => panic!("Expected Constant::Int to have type Type::IntegerType; got {:?}", ty),
                }
            },
            LLVMValueKind::LLVMConstantFPValueKind => {
                match Type::from_llvm_ref( unsafe { LLVMTypeOf(constant) }, tnmap ) {
                    Type::FPType(fptype) => Constant::Float(match fptype {
                        FPType::Half => Float::Half,
                        FPType::Single => Float::Single( unsafe {
                            let mut b = 0;
                            let b_ptr: *mut std::os::raw::c_int = &mut b;
                            LLVMConstRealGetDouble(constant, b_ptr)
                        } as f32),
                        FPType::Double => Float::Double( unsafe {
                            let mut b = 0;
                            let b_ptr: *mut std::os::raw::c_int = &mut b;
                            LLVMConstRealGetDouble(constant, b_ptr)
                        } ),
                        FPType::FP128 => Float::Quadruple,
                        FPType::X86_FP80 => Float::X86_FP80,
                        FPType::PPC_FP128 => Float::PPC_FP128,
                    }),
                    ty => panic!("Expected Constant::Float to have type Type::FPType; got {:?}", ty),
                }
            },
            LLVMValueKind::LLVMConstantStructValueKind => {
                let (num_elements, is_packed) = match Type::from_llvm_ref( unsafe { LLVMTypeOf(constant) }, tnmap ) {
                    Type::StructType { element_types, is_packed } => (element_types.len(), is_packed),
                    Type::NamedStructType { ref ty, .. } => {
                        let arc: Arc<RwLock<Type>> = ty.as_ref()
                            .expect("Constant of opaque struct type")
                            .upgrade()
                            .expect("Weak reference should be valid for at least the lifetime of tnmap");
                        let innerty: &Type = &arc.read().unwrap();
                        if let Type::StructType { element_types, is_packed } = innerty {
                            (element_types.len(), *is_packed)
                        } else {
                            panic!("Expected NamedStructType inner type to be a StructType, but it actually is a {:?}", innerty)
                        }
                    },
                    ty => panic!("Expected Constant::Struct to have type StructType or NamedStructType; got {:?}", ty),
                };
                Constant::Struct {
                    name: None,  // --TODO not yet implemented: Constant::Struct name
                    values: {
                        (0 .. num_elements).map(|i| {
                            Constant::from_llvm_ref( unsafe { LLVMGetOperand(constant, i as u32) }, gnmap, tnmap)
                        }).collect()
                    },
                    is_packed,
                }
            },
            LLVMValueKind::LLVMConstantArrayValueKind => {
                match Type::from_llvm_ref( unsafe { LLVMTypeOf(constant) }, tnmap ) {
                    Type::ArrayType { element_type, num_elements } => Constant::Array {
                        element_type: *element_type,
                        elements: {
                            (0 .. num_elements).map(|i| Constant::from_llvm_ref( unsafe { LLVMGetOperand(constant, i as u32) }, gnmap, tnmap)).collect()
                        },
                    },
                    ty => panic!("Expected Constant::Array to have type Type::ArrayType; got {:?}", ty),
                }
            },
            LLVMValueKind::LLVMConstantVectorValueKind => {
                let num_elements = unsafe { LLVMGetNumOperands(constant) };
                Constant::Vector(
                    (0 .. num_elements).map(|i| Constant::from_llvm_ref( unsafe { LLVMGetOperand(constant, i as u32) }, gnmap, tnmap)).collect()
                )
            },
            LLVMValueKind::LLVMConstantDataArrayValueKind => {
                match Type::from_llvm_ref( unsafe { LLVMTypeOf(constant) }, tnmap ) {
                    Type::ArrayType { element_type, num_elements } => Constant::Array {
                        element_type: *element_type,
                        elements: {
                            (0 .. num_elements).map(|i| Constant::from_llvm_ref( unsafe { LLVMGetElementAsConstant(constant, i as u32) }, gnmap, tnmap)).collect()
                        },
                    },
                    ty => panic!("Expected ConstantDataArray to have type Type::ArrayType; got {:?}", ty),
                }
            },
            LLVMValueKind::LLVMConstantDataVectorValueKind => {
                match Type::from_llvm_ref( unsafe { LLVMTypeOf(constant) }, tnmap ) {
                    Type::VectorType { num_elements, .. } => Constant::Vector(
                        (0 .. num_elements).map(|i| Constant::from_llvm_ref( unsafe { LLVMGetElementAsConstant(constant, i as u32) }, gnmap, tnmap)).collect()
                    ),
                    ty => panic!("Expected ConstantDataVector to have type Type::VectorType; got {:?}", ty),
                }
            },
            LLVMValueKind::LLVMConstantPointerNullValueKind => {
                Constant::Null(Type::from_llvm_ref( unsafe { LLVMTypeOf(constant) }, tnmap ))
            },
            LLVMValueKind::LLVMConstantAggregateZeroValueKind => {
                Constant::AggregateZero(Type::from_llvm_ref( unsafe { LLVMTypeOf(constant) }, tnmap ))
            },
            LLVMValueKind::LLVMUndefValueValueKind => {
                Constant::Undef(Type::from_llvm_ref( unsafe { LLVMTypeOf(constant) }, tnmap ))
            },
            LLVMValueKind::LLVMConstantTokenNoneValueKind => {
                Constant::TokenNone
            },
            LLVMValueKind::LLVMBlockAddressValueKind => {
                Constant::BlockAddress
            },
            LLVMValueKind::LLVMConstantExprValueKind => {
                use llvm_sys::LLVMOpcode;
                match unsafe { LLVMGetConstOpcode(constant) } {
                    LLVMOpcode::LLVMAdd => Constant::Add(Box::new(Add::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMSub => Constant::Sub(Box::new(Sub::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMMul => Constant::Mul(Box::new(Mul::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMUDiv => Constant::UDiv(Box::new(UDiv::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMSDiv => Constant::SDiv(Box::new(SDiv::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMURem => Constant::URem(Box::new(URem::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMSRem => Constant::SRem(Box::new(SRem::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMAnd => Constant::And(Box::new(And::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMOr => Constant::Or(Box::new(Or::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMXor => Constant::Xor(Box::new(Xor::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMShl => Constant::Shl(Box::new(Shl::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMLShr => Constant::LShr(Box::new(LShr::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMAShr => Constant::AShr(Box::new(AShr::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMFAdd => Constant::FAdd(Box::new(FAdd::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMFSub => Constant::FSub(Box::new(FSub::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMFMul => Constant::FMul(Box::new(FMul::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMFDiv => Constant::FDiv(Box::new(FDiv::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMFRem => Constant::FRem(Box::new(FRem::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMExtractElement => Constant::ExtractElement(Box::new(ExtractElement::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMInsertElement => Constant::InsertElement(Box::new(InsertElement::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMShuffleVector => Constant::ShuffleVector(Box::new(ShuffleVector::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMExtractValue => Constant::ExtractValue(Box::new(ExtractValue::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMInsertValue => Constant::InsertValue(Box::new(InsertValue::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMGetElementPtr => Constant::GetElementPtr(Box::new(GetElementPtr::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMTrunc => Constant::Trunc(Box::new(Trunc::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMZExt => Constant::ZExt(Box::new(ZExt::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMSExt => Constant::SExt(Box::new(SExt::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMFPTrunc => Constant::FPTrunc(Box::new(FPTrunc::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMFPExt => Constant::FPExt(Box::new(FPExt::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMFPToUI => Constant::FPToUI(Box::new(FPToUI::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMFPToSI => Constant::FPToSI(Box::new(FPToSI::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMUIToFP => Constant::UIToFP(Box::new(UIToFP::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMSIToFP => Constant::SIToFP(Box::new(SIToFP::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMPtrToInt => Constant::PtrToInt(Box::new(PtrToInt::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMIntToPtr => Constant::IntToPtr(Box::new(IntToPtr::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMBitCast => Constant::BitCast(Box::new(BitCast::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMAddrSpaceCast => Constant::AddrSpaceCast(Box::new(AddrSpaceCast::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMICmp => Constant::ICmp(Box::new(ICmp::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMFCmp => Constant::FCmp(Box::new(FCmp::from_llvm_ref(constant, gnmap, tnmap))),
                    LLVMOpcode::LLVMSelect => Constant::Select(Box::new(Select::from_llvm_ref(constant, gnmap, tnmap))),
                    opcode => panic!("ConstantExpr has unexpected opcode {:?}", opcode),
                }
            },
            _ if unsafe { !LLVMIsAGlobalValue(constant).is_null() } => {
                Constant::GlobalReference {
                    name: gnmap.get(&constant)
                        .unwrap_or_else(|| { let names: Vec<_> = gnmap.values().collect(); panic!("Global not found in GlobalNameMap; have names {:?}", names) })
                        .clone(),
                    ty: Type::from_llvm_ref( unsafe { LLVMGlobalGetValueType(constant) }, tnmap ),
                }
            },
            k => panic!("Constant::from_llvm_ref: don't know how to handle this Constant with ValueKind {:?}", k),
        }
    }
}

macro_rules! binop_from_llvm {
    ($expr:ident) => {
        impl $expr {
            pub(crate) fn from_llvm_ref(
                expr: LLVMValueRef,
                gnmap: &GlobalNameMap,
                tnmap: &mut TyNameMap,
            ) -> Self {
                assert_eq!(unsafe { LLVMGetNumOperands(expr) }, 2);
                Self {
                    operand0: Constant::from_llvm_ref(
                        unsafe { LLVMGetOperand(expr, 0) },
                        gnmap,
                        tnmap,
                    ),
                    operand1: Constant::from_llvm_ref(
                        unsafe { LLVMGetOperand(expr, 1) },
                        gnmap,
                        tnmap,
                    ),
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

impl ExtractElement {
    pub(crate) fn from_llvm_ref(
        expr: LLVMValueRef,
        gnmap: &GlobalNameMap,
        tnmap: &mut TyNameMap,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(expr) }, 2);
        Self {
            vector: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 0) }, gnmap, tnmap),
            index: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 1) }, gnmap, tnmap),
        }
    }
}

impl InsertElement {
    pub(crate) fn from_llvm_ref(
        expr: LLVMValueRef,
        gnmap: &GlobalNameMap,
        tnmap: &mut TyNameMap,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(expr) }, 3);
        Self {
            vector: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 0) }, gnmap, tnmap),
            element: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 1) }, gnmap, tnmap),
            index: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 2) }, gnmap, tnmap),
        }
    }
}

impl ShuffleVector {
    pub(crate) fn from_llvm_ref(
        expr: LLVMValueRef,
        gnmap: &GlobalNameMap,
        tnmap: &mut TyNameMap,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(expr) }, 3);
        Self {
            operand0: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 0) }, gnmap, tnmap),
            operand1: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 1) }, gnmap, tnmap),
            mask: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 2) }, gnmap, tnmap),
        }
    }
}

impl ExtractValue {
    pub(crate) fn from_llvm_ref(
        expr: LLVMValueRef,
        gnmap: &GlobalNameMap,
        tnmap: &mut TyNameMap,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(expr) }, 2);
        Self {
            aggregate: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 0) }, gnmap, tnmap),
            indices: unsafe {
                let num_indices = LLVMGetNumIndices(expr);
                let ptr = LLVMGetIndices(expr);
                std::slice::from_raw_parts(ptr, num_indices as usize).to_vec()
            },
        }
    }
}

impl InsertValue {
    pub(crate) fn from_llvm_ref(
        expr: LLVMValueRef,
        gnmap: &GlobalNameMap,
        tnmap: &mut TyNameMap,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(expr) }, 3);
        Self {
            aggregate: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 0) }, gnmap, tnmap),
            element: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 1) }, gnmap, tnmap),
            indices: unsafe {
                let num_indices = LLVMGetNumIndices(expr);
                let ptr = LLVMGetIndices(expr);
                std::slice::from_raw_parts(ptr, num_indices as usize).to_vec()
            },
        }
    }
}

impl GetElementPtr {
    pub(crate) fn from_llvm_ref(
        expr: LLVMValueRef,
        gnmap: &GlobalNameMap,
        tnmap: &mut TyNameMap,
    ) -> Self {
        Self {
            address: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 0) }, gnmap, tnmap),
            indices: {
                let num_indices = unsafe { LLVMGetNumOperands(expr) as u32 } - 1; // LLVMGetNumIndices(), which we use for instruction::GetElementPtr, appears empirically to not work for constant::GetElementPtr
                (1..=num_indices)
                    .map(|i| {
                        Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, i) }, gnmap, tnmap)
                    })
                    .collect()
            },
            in_bounds: unsafe { LLVMIsInBounds(expr) } != 0,
        }
    }
}

// These constexprs have the property that their result type is ambiguous from
//   knowing only their operands.
macro_rules! typed_unop_from_llvm {
    ($expr:ident) => {
        impl $expr {
            pub(crate) fn from_llvm_ref(
                expr: LLVMValueRef,
                gnmap: &GlobalNameMap,
                tnmap: &mut TyNameMap,
            ) -> Self {
                assert_eq!(unsafe { LLVMGetNumOperands(expr) }, 1);
                Self {
                    operand: Constant::from_llvm_ref(
                        unsafe { LLVMGetOperand(expr, 0) },
                        gnmap,
                        tnmap,
                    ),
                    to_type: Type::from_llvm_ref(unsafe { LLVMTypeOf(expr) }, tnmap),
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
        expr: LLVMValueRef,
        gnmap: &GlobalNameMap,
        tnmap: &mut TyNameMap,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(expr) }, 2);
        Self {
            predicate: IntPredicate::from_llvm(unsafe { LLVMGetICmpPredicate(expr) }),
            operand0: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 0) }, gnmap, tnmap),
            operand1: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 1) }, gnmap, tnmap),
        }
    }
}

impl FCmp {
    pub(crate) fn from_llvm_ref(
        expr: LLVMValueRef,
        gnmap: &GlobalNameMap,
        tnmap: &mut TyNameMap,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(expr) }, 2);
        Self {
            predicate: FPPredicate::from_llvm(unsafe { LLVMGetFCmpPredicate(expr) }),
            operand0: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 0) }, gnmap, tnmap),
            operand1: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 1) }, gnmap, tnmap),
        }
    }
}

impl Select {
    pub(crate) fn from_llvm_ref(
        expr: LLVMValueRef,
        gnmap: &GlobalNameMap,
        tnmap: &mut TyNameMap,
    ) -> Self {
        assert_eq!(unsafe { LLVMGetNumOperands(expr) }, 3);
        Self {
            condition: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 0) }, gnmap, tnmap),
            true_value: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 1) }, gnmap, tnmap),
            false_value: Constant::from_llvm_ref(unsafe { LLVMGetOperand(expr, 2) }, gnmap, tnmap),
        }
    }
}
