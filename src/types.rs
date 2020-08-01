use crate::module::AddrSpace;
//use crate::name::Name;
use either::Either;
use std::sync::{Arc, RwLock, Weak};

/// See [LLVM 9 docs on Type System](https://releases.llvm.org/9.0.0/docs/LangRef.html#type-system)
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Type {
    /// See [LLVM 9 docs on Void Type](https://releases.llvm.org/9.0.0/docs/LangRef.html#void-type)
    VoidType,
    /// See [LLVM 9 docs on Integer Type](https://releases.llvm.org/9.0.0/docs/LangRef.html#integer-type)
    IntegerType { bits: u32 },
    /// See [LLVM 9 docs on Pointer Type](https://releases.llvm.org/9.0.0/docs/LangRef.html#pointer-type)
    PointerType {
        pointee_type: Box<Type>,
        addr_space: AddrSpace,
    },
    /// See [LLVM 9 docs on Floating-Point Types](https://releases.llvm.org/9.0.0/docs/LangRef.html#floating-point-types)
    FPType(FPType),
    /// See [LLVM 9 docs on Function Type](https://releases.llvm.org/9.0.0/docs/LangRef.html#function-type)
    FuncType {
        result_type: Box<Type>,
        param_types: Vec<Type>,
        is_var_arg: bool,
    },
    /// Vector types (along with integer, FP, pointer, and X86_MMX types) are "first class types",
    /// which means they can be produced by instructions (see [LLVM 9 docs on First Class Types](https://releases.llvm.org/9.0.0/docs/LangRef.html#first-class-types)).
    /// See [LLVM 9 docs on Vector Type](https://releases.llvm.org/9.0.0/docs/LangRef.html#vector-type)
    VectorType {
        element_type: Box<Type>,
        num_elements: usize,
    },
    /// Struct and Array types (but not vector types) are "aggregate types" and cannot be produced by
    /// a single instruction (see [LLVM 9 docs on Aggregate Types](https://releases.llvm.org/9.0.0/docs/LangRef.html#aggregate-types)).
    /// See [LLVM 9 docs on Array Type](https://releases.llvm.org/9.0.0/docs/LangRef.html#array-type)
    ArrayType {
        element_type: Box<Type>,
        num_elements: usize,
    },
    /// The `StructType` variant is used for a "literal" (i.e., anonymous) structure type.
    /// See [LLVM 9 docs on Structure Type](https://releases.llvm.org/9.0.0/docs/LangRef.html#structure-type)
    StructType {
        element_types: Vec<Type>,
        is_packed: bool,
    },
    /// Named structure types. Note that these may be self-referential (i.e., recursive).
    /// See [LLVM 9 docs on Structure Type](https://releases.llvm.org/9.0.0/docs/LangRef.html#structure-type)
    NamedStructType {
        /// Name of the struct type
        name: String, // llvm-hs-pure has Name rather than String
        /// The actual struct type, which will be a `StructType` variant.
        /// A `None` here indicates an opaque type; see [LLVM 9 docs on Opaque Structure Types](https://releases.llvm.org/9.0.0/docs/LangRef.html#t-opaque).
        /// The weak reference should remain valid for at least the lifetime of the `Module` in which the named struct type is defined.
        ty: Option<Weak<RwLock<Type>>>,
    },
    /// See [LLVM 9 docs on X86_MMX Type](https://releases.llvm.org/9.0.0/docs/LangRef.html#x86-mmx-type)
    X86_MMXType, // llvm-hs-pure doesn't have this, not sure what they do with LLVM's http://llvm.org/docs/LangRef.html#x86-mmx-type
    /// See [LLVM 9 docs on Metadata Type](https://releases.llvm.org/9.0.0/docs/LangRef.html#metadata-type)
    MetadataType,
    /// `LabelType` is the type of [`BasicBlock`](../struct.BasicBlock.html) labels.
    /// See [LLVM 9 docs on Label Type](https://releases.llvm.org/9.0.0/docs/LangRef.html#label-type)
    LabelType,
    /// See [LLVM 9 docs on Token Type](https://releases.llvm.org/9.0.0/docs/LangRef.html#token-type)
    TokenType,
}

// `PartialEq` cannot be automatically derived for `Type` because of the `Weak` in `NamedStructType`
impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::VoidType, Type::VoidType) => true,
            (Type::IntegerType { bits: bits_a },
             Type::IntegerType { bits: bits_b })
            => bits_a == bits_b,
            (Type::PointerType { pointee_type: pt_a, addr_space: as_a },
             Type::PointerType { pointee_type: pt_b, addr_space: as_b })
            => pt_a == pt_b && as_a == as_b,
            (Type::FPType(fp_a),
             Type::FPType(fp_b))
            => fp_a == fp_b,
            (Type::FuncType { result_type: rt_a, param_types: pt_a, is_var_arg: iva_a },
             Type::FuncType { result_type: rt_b, param_types: pt_b, is_var_arg: iva_b })
            => rt_a == rt_b && pt_a == pt_b && iva_a == iva_b,
            (Type::VectorType { element_type: et_a, num_elements: num_a },
             Type::VectorType { element_type: et_b, num_elements: num_b })
            => et_a == et_b && num_a == num_b,
            (Type::ArrayType { element_type: et_a, num_elements: num_a },
             Type::ArrayType { element_type: et_b, num_elements: num_b })
            => et_a == et_b && num_a == num_b,
            (Type::StructType { element_types: et_a, is_packed: ip_a },
             Type::StructType { element_types: et_b, is_packed: ip_b })
            => et_a == et_b && ip_a == ip_b,
            // Named structs are equal if their names are equal, disregarding their weak refs.
            // This means even an opaque definition is considered equal to a
            //   non-opaque definition (they still refer to the same type).
            (Type::NamedStructType { name: name_a, .. },
             Type::NamedStructType { name: name_b, .. })
            => name_a == name_b,
            (Type::X86_MMXType, Type::X86_MMXType) => true,
            (Type::MetadataType, Type::MetadataType) => true,
            (Type::LabelType, Type::LabelType) => true,
            (Type::TokenType, Type::TokenType) => true,
            _ => false,
        }
    }
}
// Our `PartialEq` still satisfies the required properties of `Eq`
impl Eq for Type {}

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
        Type::PointerType {
            pointee_type: Box::new(ty),
            addr_space: 0, // default to addr_space 0
        }
    }
}

/// See [LLVM 9 docs on Floating-Point Types](https://releases.llvm.org/9.0.0/docs/LangRef.html#floating-point-types)
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

impl<A, B> Typed for Either<A, B>
where
    A: Typed,
    B: Typed,
{
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
use std::collections::{HashMap, HashSet};

pub(crate) type TyNameMap = HashMap<String, Option<Arc<RwLock<Type>>>>;

impl Type {
    pub(crate) fn from_llvm_ref(ty: LLVMTypeRef, tynamemap: &mut TyNameMap) -> Self {
        let kind = unsafe { LLVMGetTypeKind(ty) };
        match kind {
            LLVMTypeKind::LLVMVoidTypeKind => Type::VoidType,
            LLVMTypeKind::LLVMIntegerTypeKind => Type::IntegerType {
                bits: unsafe { LLVMGetIntTypeWidth(ty) },
            },
            LLVMTypeKind::LLVMPointerTypeKind => Type::PointerType {
                pointee_type: Box::new(Type::from_llvm_ref(
                    unsafe { LLVMGetElementType(ty) },
                    tynamemap,
                )),
                addr_space: unsafe { LLVMGetPointerAddressSpace(ty) },
            },
            LLVMTypeKind::LLVMArrayTypeKind => Type::ArrayType {
                element_type: Box::new(Type::from_llvm_ref(
                    unsafe { LLVMGetElementType(ty) },
                    tynamemap,
                )),
                num_elements: unsafe { LLVMGetArrayLength(ty) as usize },
            },
            LLVMTypeKind::LLVMVectorTypeKind => Type::VectorType {
                element_type: Box::new(Type::from_llvm_ref(
                    unsafe { LLVMGetElementType(ty) },
                    tynamemap,
                )),
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
                        let actual_type: Option<Arc<RwLock<Type>>> = if tynamemap.contains_key(s) {
                            tynamemap.get(s).unwrap().clone()
                        } else if unsafe { LLVMIsOpaqueStruct(ty) } != 0 {
                            tynamemap.insert(s.clone(), None);
                            None
                        } else {
                            // first fill in the entry as opaque for now, so that the call to struct_type_from_llvm_ref will terminate
                            tynamemap.insert(s.clone(), None);
                            // now compute the actual correct type. Any self-references will be opaqued
                            let type_with_opaqued_self_refs =
                                Type::struct_type_from_llvm_ref(ty, tynamemap);
                            // recursively replace any opaqued self-references with weak refs to self
                            let arc = Arc::new(RwLock::new(type_with_opaqued_self_refs.clone()));
                            let actual_type =
                                Type::replace_in_type(type_with_opaqued_self_refs, s, &arc);
                            *arc.write().unwrap() = actual_type;
                            // and finally, put the completed type in the map
                            tynamemap.insert(s.clone(), Some(arc.clone()));
                            Some(arc)
                        };
                        Type::NamedStructType {
                            name: s.clone(),
                            ty: actual_type.map(|arc| Arc::downgrade(&arc)),
                        }
                    },
                    _ => Type::struct_type_from_llvm_ref(ty, tynamemap),
                }
            },
            LLVMTypeKind::LLVMFunctionTypeKind => Type::FuncType {
                result_type: Box::new(Type::from_llvm_ref(
                    unsafe { LLVMGetReturnType(ty) },
                    tynamemap,
                )),
                param_types: {
                    let num_types = unsafe { LLVMCountParamTypes(ty) };
                    let mut types: Vec<LLVMTypeRef> = Vec::with_capacity(num_types as usize);
                    unsafe {
                        LLVMGetParamTypes(ty, types.as_mut_ptr());
                        types.set_len(num_types as usize);
                    };
                    types
                        .into_iter()
                        .map(|t| Type::from_llvm_ref(t, tynamemap))
                        .collect()
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

    /// Replace any opaqued named structs with the given `name`, with weak references to the given `Arc<RwLock<Type>>`
    fn replace_in_type(ty: Type, target_name: &str, replacement: &Arc<RwLock<Type>>) -> Type {
        Type::_replace_in_type(ty, target_name, replacement, &mut HashSet::new())
    }

    // `seen_names` is here to prevent infinite recursion on self-referential
    // struct types; we don't need to continue into any recursive reference (as
    // we've been there already and done any necessary replacements)
    fn _replace_in_type(
        ty: Type,
        target_name: &str,
        replacement: &Arc<RwLock<Type>>,
        seen_names: &mut HashSet<String>,
    ) -> Type {
        match ty {
            Type::NamedStructType { ref name, ty: None } if name == target_name => {
                Type::NamedStructType {
                    name: name.clone(),
                    ty: Some(Arc::downgrade(replacement)),
                }
            },
            // confused on the proper syntax here; neither `{ ref name, ref ty: Some(weak) }` nor
            // `{ ref name, ty: ref Some(weak) }` parse as valid, and just `ty: Some(weak)` errors due to
            // attempting to move the value. For now we have this hack instead.
            Type::NamedStructType { ref name, ref ty }
                if ty.is_some() && !seen_names.contains(name) =>
            {
                seen_names.insert(name.clone());
                let weak = ty
                    .as_ref()
                    .expect("we checked that ty.is_some() in the pattern guard");
                let arc: Arc<RwLock<Type>> =
                    weak.upgrade().expect("Failed to upgrade weak reference");
                let inner_ty = arc.read().unwrap().clone();
                *arc.write().unwrap() =
                    Type::_replace_in_type(inner_ty, target_name, replacement, seen_names);
                Type::NamedStructType {
                    name: name.clone(),
                    ty: Some(Arc::downgrade(&arc)),
                }
            }
            Type::PointerType {
                pointee_type,
                addr_space,
            } => Type::PointerType {
                pointee_type: Box::new(Type::_replace_in_type(
                    *pointee_type,
                    target_name,
                    replacement,
                    seen_names,
                )),
                addr_space,
            },
            Type::FuncType {
                result_type,
                param_types,
                is_var_arg,
            } => Type::FuncType {
                // we don't mind that one recursive call here might add things
                // to `seen_names` that will affect the processing of other
                // recursive calls.
                // All `NamedStructType`s with the same name should be refs to
                // the same `StructType`, so once we've done the replacement in
                // that `StructType` once, we don't need to do it again, even on
                // another "branch" of the recursion
                result_type: Box::new(Type::_replace_in_type(
                    *result_type,
                    target_name,
                    replacement,
                    seen_names,
                )),
                param_types: param_types
                    .into_iter()
                    .map(|t| Type::_replace_in_type(t, target_name, replacement, seen_names))
                    .collect(),
                is_var_arg,
            },
            Type::VectorType {
                element_type,
                num_elements,
            } => Type::VectorType {
                element_type: Box::new(Type::_replace_in_type(
                    *element_type,
                    target_name,
                    replacement,
                    seen_names,
                )),
                num_elements,
            },
            Type::ArrayType {
                element_type,
                num_elements,
            } => Type::ArrayType {
                element_type: Box::new(Type::_replace_in_type(
                    *element_type,
                    target_name,
                    replacement,
                    seen_names,
                )),
                num_elements,
            },
            Type::StructType {
                element_types,
                is_packed,
            } => Type::StructType {
                // we don't mind that one recursive call here might add things
                // to `seen_names` that will affect the processing of other
                // recursive calls.
                // All `NamedStructType`s with the same name should be refs to
                // the same `StructType`, so once we've done the replacement in
                // that `StructType` once, we don't need to do it again, even on
                // another "branch" of the recursion
                element_types: element_types
                    .into_iter()
                    .map(|t| Type::_replace_in_type(t, target_name, replacement, seen_names))
                    .collect(),
                is_packed,
            },
            _ => ty, // nothing to replace. In particular we don't need to replace anything in opaque named structs when name != target_name.
        }
    }

    /// creates an actual `StructType`, regardless of whether the struct is named or not
    ///
    /// Caller is responsible for ensuring that `ty` is not an opaque struct type
    fn struct_type_from_llvm_ref(ty: LLVMTypeRef, tynamemap: &mut TyNameMap) -> Self {
        if unsafe { LLVMIsOpaqueStruct(ty) } != 0 {
            panic!("struct_type_from_llvm_ref: shouldn't pass an opaque struct type to this function");
        }
        Type::StructType {
            element_types: {
                let num_types = unsafe { LLVMCountStructElementTypes(ty) };
                let mut types: Vec<LLVMTypeRef> = Vec::with_capacity(num_types as usize);
                unsafe {
                    LLVMGetStructElementTypes(ty, types.as_mut_ptr());
                    types.set_len(num_types as usize);
                };
                types
                    .into_iter()
                    .map(|t| Type::from_llvm_ref(t, tynamemap))
                    .collect()
            },
            is_packed: unsafe { LLVMIsPackedStruct(ty) } != 0,
        }
    }
}
