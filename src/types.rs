use crate::module::AddrSpace;
//use crate::name::Name;
use either::Either;

/// See [LLVM 8 docs on Type System](https://releases.llvm.org/8.0.0/docs/LangRef.html#type-system)
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum Type {
    /// See [LLVM 8 docs on Void Type](https://releases.llvm.org/8.0.0/docs/LangRef.html#void-type)
    VoidType,
    /// See [LLVM 8 docs on Integer Type](https://releases.llvm.org/8.0.0/docs/LangRef.html#integer-type)
    IntegerType { bits: u32 },
    /// See [LLVM 8 docs on Pointer Type](https://releases.llvm.org/8.0.0/docs/LangRef.html#pointer-type)
    PointerType { pointee_type: Box<Type>, addr_space: AddrSpace },
    /// See [LLVM 8 docs on Floating-Point Types](https://releases.llvm.org/8.0.0/docs/LangRef.html#floating-point-types)
    FPType(FPType),
    /// See [LLVM 8 docs on Function Type](https://releases.llvm.org/8.0.0/docs/LangRef.html#function-type)
    FuncType { result_type: Box<Type>, param_types: Vec<Type>, is_var_arg: bool },
    /// Vector types (along with integer, FP, pointer, and X86_MMX types) are "first class types",
    /// which means they can be produced by instructions (see [LLVM 8 docs on First Class Types](https://releases.llvm.org/8.0.0/docs/LangRef.html#first-class-types)).
    /// See [LLVM 8 docs on Vector Type](https://releases.llvm.org/8.0.0/docs/LangRef.html#vector-type)
    VectorType { element_type: Box<Type>, num_elements: usize },
    /// Struct and Array types (but not vector types) are "aggregate types" and cannot be produced by
    /// a single instruction (see [LLVM 8 docs on Aggregate Types](https://releases.llvm.org/8.0.0/docs/LangRef.html#aggregate-types)).
    /// See [LLVM 8 docs on Structure Type](https://releases.llvm.org/8.0.0/docs/LangRef.html#structure-type)
    StructType { element_types: Vec<Type>, is_packed: bool },
    /// See [LLVM 8 docs on Array Type](https://releases.llvm.org/8.0.0/docs/LangRef.html#array-type)
    ArrayType { element_type: Box<Type>, num_elements: usize },
    /// Used for self-referential (i.e., recursive) structure types, and also opaque structure types.
    /// May also be used for any named structure type, even if it is not self-referential or opaque.
    /// You can look up the actual `StructType` in the [`Module.named_struct_types`](../module/struct.Module.html#structfield.named_struct_types) map.
    /// See LLVM 8 docs on [Structure Type](https://releases.llvm.org/8.0.0/docs/LangRef.html#structure-type)
    /// and [Opaque Structure Types](http://llvm.org/docs/LangRef.html#opaque-structure-types)
    NamedStructTypeReference(String),  // llvm-hs-pure has Name rather than String
    /// See [LLVM 8 docs on X86_MMX Type](https://releases.llvm.org/8.0.0/docs/LangRef.html#x86-mmx-type)
    X86_MMXType,  // llvm-hs-pure doesn't have this, not sure what they do with LLVM's http://llvm.org/docs/LangRef.html#x86-mmx-type
    /// See [LLVM 8 docs on Metadata Type](https://releases.llvm.org/8.0.0/docs/LangRef.html#metadata-type)
    MetadataType,
    /// `LabelType` is the type of [`BasicBlock`](../struct.BasicBlock.html) labels.
    /// See [LLVM 8 docs on Label Type](https://releases.llvm.org/8.0.0/docs/LangRef.html#label-type)
    LabelType,
    /// See [LLVM 8 docs on Token Type](https://releases.llvm.org/8.0.0/docs/LangRef.html#token-type)
    TokenType,
}

impl Type {
    pub fn bool() -> Type {
        Type::IntegerType { bits: 1 }
    }

    pub fn i8() -> Type {
        Type::IntegerType { bits: 8 }
    }

    pub fn i16() -> Type {
        Type::IntegerType { bits: 16 }
    }

    pub fn i32() -> Type {
        Type::IntegerType { bits: 32 }
    }

    pub fn i64() -> Type {
        Type::IntegerType { bits: 64 }
    }

    pub fn half() -> Type {
        Type::FPType(FPType::Half)
    }

    pub fn single() -> Type {
        Type::FPType(FPType::Single)
    }

    pub fn double() -> Type {
        Type::FPType(FPType::Double)
    }

    pub fn pointer_to(ty: Type) -> Type {
        Type::PointerType { pointee_type: Box::new(ty), addr_space: 0 }  // default to addr_space 0
    }
}

/// See [LLVM 8 docs on Floating-Point Types](https://releases.llvm.org/8.0.0/docs/LangRef.html#floating-point-types)
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum FPType {
    Half,
    Single,
    Double,
    FP128,
    X86_FP80,
    PPC_FP128,
}

impl From<FPType> for Type {
    fn from(fpt: FPType) -> Type {
        Type::FPType(fpt)
    }
}

/// The `Typed` trait is used for anything that has a [`Type`](../enum.Type.html).
pub trait Typed {
    fn get_type(&self) -> Type;
}

impl Typed for Type {
    fn get_type(&self) -> Type {
        self.clone()
    }
}

impl Typed for FPType {
    fn get_type(&self) -> Type {
        self.clone().into()
    }
}

impl<A,B> Typed for Either<A,B> where A: Typed, B:Typed {
    fn get_type(&self) -> Type {
        match self {
            Either::Left(x) => x.get_type(),
            Either::Right(y) => y.get_type(),
        }
    }
}

// ********* //
// from_llvm //
// ********* //

use crate::from_llvm::*;
use llvm_sys::LLVMTypeKind;
use std::collections::HashMap;

pub(crate) type TyNameMap = HashMap<String, Option<Type>>;

impl Type {
    pub(crate) fn from_llvm_ref(ty: LLVMTypeRef, tynamemap: &mut TyNameMap) -> Self {
        let kind = unsafe { LLVMGetTypeKind(ty) };
        match kind {
            LLVMTypeKind::LLVMVoidTypeKind => Type::VoidType,
            LLVMTypeKind::LLVMIntegerTypeKind => Type::IntegerType {
                bits: unsafe { LLVMGetIntTypeWidth(ty) },
            },
            LLVMTypeKind::LLVMPointerTypeKind => Type::PointerType {
                pointee_type: Box::new(Type::from_llvm_ref(unsafe { LLVMGetElementType(ty) }, tynamemap)),
                addr_space: unsafe { LLVMGetPointerAddressSpace(ty) },
            },
            LLVMTypeKind::LLVMArrayTypeKind => Type::ArrayType {
                element_type: Box::new(Type::from_llvm_ref(unsafe { LLVMGetElementType(ty) }, tynamemap)),
                num_elements: unsafe { LLVMGetArrayLength(ty) as usize },
            },
            LLVMTypeKind::LLVMVectorTypeKind => Type::VectorType {
                element_type: Box::new(Type::from_llvm_ref(unsafe { LLVMGetElementType(ty) }, tynamemap)),
                num_elements: unsafe { LLVMGetVectorSize(ty) as usize },
            },
            LLVMTypeKind::LLVMStructTypeKind => {
                let name = if unsafe { LLVMIsLiteralStruct(ty) } != 0 {
                    None
                } else {
                    unsafe { get_struct_name(ty) }
                };

                match name {
                    Some(ref s) if !s.is_empty() => {
                        if !tynamemap.contains_key(s) {
                            tynamemap.insert(s.clone(), None);  // put it in as opaque for now, so that the next call will terminate
                            let actual_type = Type::struct_type_from_llvm_ref(ty, tynamemap);
                            tynamemap.insert(s.clone(), Some(actual_type));  // now put in the actual correct type
                        }
                        Type::NamedStructTypeReference(s.clone())
                    },
                    _ => Type::struct_type_from_llvm_ref(ty, tynamemap),
                }
            },
            LLVMTypeKind::LLVMFunctionTypeKind => Type::FuncType {
                result_type: Box::new(Type::from_llvm_ref(unsafe { LLVMGetReturnType(ty) }, tynamemap)),
                param_types: {
                    let num_types = unsafe { LLVMCountParamTypes(ty) };
                    let mut types: Vec<LLVMTypeRef> = Vec::with_capacity(num_types as usize);
                    unsafe {
                        LLVMGetParamTypes(ty, types.as_mut_ptr());
                        types.set_len(num_types as usize);
                    };
                    types.into_iter().map(|t| Type::from_llvm_ref(t, tynamemap)).collect()
                },
                is_var_arg: unsafe { LLVMIsFunctionVarArg(ty) } != 0,
            },
            LLVMTypeKind::LLVMHalfTypeKind => Type::FPType(FPType::Half),
            LLVMTypeKind::LLVMFloatTypeKind => Type::FPType(FPType::Single),
            LLVMTypeKind::LLVMDoubleTypeKind => Type::FPType(FPType::Double),
            LLVMTypeKind::LLVMFP128TypeKind => Type::FPType(FPType::FP128),
            LLVMTypeKind::LLVMX86_FP80TypeKind => Type::FPType(FPType::X86_FP80),
            LLVMTypeKind::LLVMPPC_FP128TypeKind => Type::FPType(FPType::PPC_FP128),
            LLVMTypeKind::LLVMX86_MMXTypeKind => Type::X86_MMXType,
            LLVMTypeKind::LLVMMetadataTypeKind => Type::MetadataType,
            LLVMTypeKind::LLVMLabelTypeKind => Type::LabelType,
            LLVMTypeKind::LLVMTokenTypeKind => Type::TokenType,
        }
    }

    /// creates an actual `StructType`, regardless of whether the struct is named or not
    fn struct_type_from_llvm_ref(ty: LLVMTypeRef, tynamemap: &mut TyNameMap) -> Self {
        Type::StructType {
            element_types: {
                let num_types = unsafe { LLVMCountStructElementTypes(ty) };
                let mut types: Vec<LLVMTypeRef> = Vec::with_capacity(num_types as usize);
                unsafe {
                    LLVMGetStructElementTypes(ty, types.as_mut_ptr());
                    types.set_len(num_types as usize);
                };
                types.into_iter().map(|t| Type::from_llvm_ref(t, tynamemap)).collect()
            },
            is_packed: unsafe { LLVMIsPackedStruct(ty) } != 0,
        }
    }
}
