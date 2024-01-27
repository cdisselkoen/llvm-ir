use crate::module::AddrSpace;
use either::Either;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

/// See [LLVM 14 docs on Type System](https://releases.llvm.org/14.0.0/docs/LangRef.html#type-system)
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum Type {
    /// See [LLVM 14 docs on Void Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#void-type)
    VoidType,
    /// See [LLVM 14 docs on Integer Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#integer-type)
    IntegerType { bits: u32 },
    /// See [LLVM 14 docs on Pointer Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#pointer-type)
    #[cfg(feature = "llvm-14-or-lower")]
    PointerType {
        pointee_type: TypeRef,
        addr_space: AddrSpace,
    },
    /// See [LLVM 15 docs on Pointer Type](https://releases.llvm.org/15.0.0/docs/LangRef.html#pointer-type)
    /// and [this documentation on Opaque Pointers, introduced in LLVM 15](https://releases.llvm.org/15.0.0/docs/OpaquePointers.html)
    #[cfg(feature = "llvm-15-or-greater")]
    PointerType { addr_space: AddrSpace },
    /// See [LLVM 14 docs on Floating-Point Types](https://releases.llvm.org/14.0.0/docs/LangRef.html#floating-point-types)
    FPType(FPType),
    /// See [LLVM 14 docs on Function Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#function-type)
    FuncType {
        result_type: TypeRef,
        param_types: Vec<TypeRef>,
        is_var_arg: bool,
    },
    /// Vector types (along with integer, FP, pointer, X86_MMX, and X86_AMX types) are "first class types",
    /// which means they can be produced by instructions (see [LLVM 14 docs on First Class Types](https://releases.llvm.org/14.0.0/docs/LangRef.html#first-class-types)).
    /// See [LLVM 14 docs on Vector Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#vector-type)
    VectorType {
        element_type: TypeRef,
        num_elements: usize,
        #[cfg(feature = "llvm-11-or-greater")]
        scalable: bool,
    },
    /// Struct and Array types (but not vector types) are "aggregate types" and cannot be produced by
    /// a single instruction (see [LLVM 14 docs on Aggregate Types](https://releases.llvm.org/14.0.0/docs/LangRef.html#aggregate-types)).
    /// See [LLVM 14 docs on Array Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#array-type)
    ArrayType {
        element_type: TypeRef,
        num_elements: usize,
    },
    /// The `StructType` variant is used for a "literal" (i.e., anonymous) structure type.
    /// See [LLVM 14 docs on Structure Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#structure-type)
    StructType {
        element_types: Vec<TypeRef>,
        is_packed: bool,
    },
    /// Named structure types. Note that these may be self-referential (i.e., recursive).
    /// See [LLVM 14 docs on Structure Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#structure-type)
    /// To get the actual definition of a named structure type, use `module.types.named_struct_def()`.
    NamedStructType {
        /// Name of the struct type
        name: String, // llvm-hs-pure has Name rather than String
    },
    /// See [LLVM 14 docs on X86_MMX Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#x86-mmx-type)
    X86_MMXType,
    // As of this writing, although X86_AMX type definitely exists in LLVM 12+,
    // it doesn't appear to be documented in the LangRef
    #[cfg(feature = "llvm-12-or-greater")]
    X86_AMXType,
    /// See [LLVM 14 docs on Metadata Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#metadata-type)
    MetadataType,
    /// `LabelType` is the type of [`BasicBlock`](../struct.BasicBlock.html) labels.
    /// See [LLVM 14 docs on Label Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#label-type)
    LabelType,
    /// See [LLVM 14 docs on Token Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#token-type)
    TokenType,
    /// See [LLVM 16 docs on Target Extension Type](https://releases.llvm.org/16.0.0/docs/LangRef.html#target-extension-type).
    ///
    /// `TargetExtType` needs more fields, but the necessary getter functions
    /// are apparently not exposed in the LLVM C API (only the C++ API).
    /// See discussion in #39.
    #[cfg(feature = "llvm-16-or-greater")]
    TargetExtType, // TODO ideally we want something like TargetExtType { name: String, contained_types: Vec<TypeRef>, contained_ints: Vec<u32> }
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::VoidType => write!(f, "void"),
            Type::IntegerType { bits } => write!(f, "i{}", bits),
            #[cfg(feature = "llvm-14-or-lower")]
            Type::PointerType { pointee_type, .. } => write!(f, "{}*", pointee_type),
            #[cfg(feature = "llvm-15-or-greater")]
            Type::PointerType { .. } => write!(f, "ptr"),
            Type::FPType(fpt) => write!(f, "{}", fpt),
            Type::FuncType {
                result_type,
                param_types,
                is_var_arg,
            } => {
                write!(f, "{} (", result_type)?;
                for (i, param_ty) in param_types.iter().enumerate() {
                    if i == param_types.len() - 1 {
                        write!(f, "{}", param_ty)?;
                    } else {
                        write!(f, "{}, ", param_ty)?;
                    }
                }
                if *is_var_arg {
                    write!(f, ", ...")?;
                }
                write!(f, ")")?;
                Ok(())
            },
            Type::VectorType {
                element_type,
                num_elements,
                #[cfg(feature = "llvm-11-or-greater")]
                scalable,
            } => {
                #[cfg(feature = "llvm-11-or-greater")]
                if *scalable {
                    write!(f, "<vscale x {} x {}>", num_elements, element_type)
                } else {
                    write!(f, "<{} x {}>", num_elements, element_type)
                }
                #[cfg(feature = "llvm-10-or-lower")]
                write!(f, "<{} x {}>", num_elements, element_type)
            },
            Type::ArrayType {
                element_type,
                num_elements,
            } => write!(f, "[{} x {}]", num_elements, element_type),
            Type::StructType {
                element_types,
                is_packed,
            } => {
                if *is_packed {
                    write!(f, "<")?;
                }
                write!(f, "{{ ")?;
                for (i, element_ty) in element_types.iter().enumerate() {
                    if i == element_types.len() - 1 {
                        write!(f, "{}", element_ty)?;
                    } else {
                        write!(f, "{}, ", element_ty)?;
                    }
                }
                write!(f, " }}")?;
                if *is_packed {
                    write!(f, ">")?;
                }
                Ok(())
            },
            Type::NamedStructType { name } => write!(f, "%{}", name),
            Type::X86_MMXType => write!(f, "x86_mmx"),
            #[cfg(feature = "llvm-12-or-greater")]
            Type::X86_AMXType => write!(f, "x86_amx"),
            Type::MetadataType => write!(f, "metadata"),
            Type::LabelType => write!(f, "label"),
            Type::TokenType => write!(f, "token"),
            #[cfg(feature = "llvm-16-or-greater")]
            Type::TargetExtType => write!(f, "target()"),
                // someday if/when TargetExtType contains other fields, we need something like the below:
                /*
                // Name, then type parameters first then integer parameters.
                let members = [name]
                    .iter()
                    .map(|name| format!("\"{name}\""))
                    .chain(contained_types.iter().map(ToString::to_string))
                    .chain(contained_ints.iter().map(ToString::to_string))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "target({members})")?;

                Ok(())
                */
        }
    }
}

/// See [LLVM 14 docs on Floating-Point Types](https://releases.llvm.org/14.0.0/docs/LangRef.html#floating-point-types)
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum FPType {
    Half,
    #[cfg(feature = "llvm-11-or-greater")]
    BFloat,
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

impl Display for FPType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FPType::Half => write!(f, "half"),
            #[cfg(feature = "llvm-11-or-greater")]
            FPType::BFloat => write!(f, "bfloat"),
            FPType::Single => write!(f, "float"),
            FPType::Double => write!(f, "double"),
            FPType::FP128 => write!(f, "fp128"),
            FPType::X86_FP80 => write!(f, "x86_fp80"),
            FPType::PPC_FP128 => write!(f, "ppc_fp128"),
        }
    }
}

/// A `TypeRef` is a reference to a [`Type`](enum.Type.html).
/// Most importantly, it implements `AsRef<Type>` and `Deref<Target = Type>`.
/// It also has a cheap `Clone` -- only the reference is cloned, not the
/// underlying `Type`.
//
// `Arc` is used rather than `Rc` so that `Module` can remain `Sync`.
// This is important because it allows multiple threads to simultaneously access
// a single (immutable) `Module`.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct TypeRef(Arc<Type>);

impl AsRef<Type> for TypeRef {
    fn as_ref(&self) -> &Type {
        self.0.as_ref()
    }
}

impl Deref for TypeRef {
    type Target = Type;

    fn deref(&self) -> &Type {
        self.0.deref()
    }
}

impl Display for TypeRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl TypeRef {
    /// For use only in this module: construct a `TypeRef` by consuming the given owned `Type`.
    /// External users should get `TypeRefs` only from the `Types` or `TypesBuilder` objects.
    fn new(ty: Type) -> Self {
        Self(Arc::new(ty))
    }
}

/// The `Typed` trait is used for anything that has a [`Type`](enum.Type.html).
pub trait Typed {
    fn get_type(&self, types: &Types) -> TypeRef;
}

impl Typed for TypeRef {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.clone()
    }
}

impl Typed for Type {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.get_for_type(self)
    }
}

impl Typed for FPType {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.fp(*self)
    }
}

impl<A, B> Typed for Either<A, B>
where
    A: Typed,
    B: Typed,
{
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            Either::Left(x) => types.type_of(x),
            Either::Right(y) => types.type_of(y),
        }
    }
}

/// Holds a reference to all of the `Type`s used in the `Module`, and facilitates
/// lookups so you can get a `TypeRef` to the `Type` you want.
#[derive(Clone)]
pub(crate) struct TypesBuilder {
    /// `TypeRef` to `Type::VoidType`
    void_type: TypeRef,
    /// Map of integer size to `Type::IntegerType` of that size
    int_types: TypeCache<u32>,
    /// Map of (pointee type, address space) to the corresponding `Type::PointerType`
    #[cfg(feature = "llvm-14-or-lower")]
    pointer_types: TypeCache<(TypeRef, AddrSpace)>,
    /// Map of address space to the corresponding `Type::PointerType`
    #[cfg(feature = "llvm-15-or-greater")]
    pointer_types: TypeCache<AddrSpace>,
    /// Map of `FPType` to the corresponding `Type::FPType`
    fp_types: TypeCache<FPType>,
    /// Map of `(result_type, param_types, is_var_arg)` to the corresponding `Type::FunctionType`
    func_types: TypeCache<(TypeRef, Vec<TypeRef>, bool)>,
    /// Map of (element type, #elements, scalable) to the corresponding `Type::VectorType`
    vec_types: TypeCache<(TypeRef, usize, bool)>,
    /// Map of (element type, #elements) to the corresponding `Type::ArrayType`
    arr_types: TypeCache<(TypeRef, usize)>,
    /// Map of `(element_types, is_packed)` to the corresponding `Type::StructType`
    struct_types: TypeCache<(Vec<TypeRef>, bool)>,
    /// Map of struct name to the corresponding `Type::NamedStructType`
    named_struct_types: TypeCache<String>,
    /// Map of struct name to the corresponding `NamedStructDef`
    named_struct_defs: HashMap<String, NamedStructDef>,
    /// `TypeRef` to `Type::X86_MMXType`
    x86_mmx_type: TypeRef,
    /// `TypeRef` to `Type::X86_AMXType`
    #[cfg(feature = "llvm-12-or-greater")]
    x86_amx_type: TypeRef,
    /// `TypeRef` to `Type::MetadataType`
    metadata_type: TypeRef,
    /// `TypeRef` to `Type::LabelType`
    label_type: TypeRef,
    /// `TypeRef` to `Type::TokenType`
    token_type: TypeRef,
    /// `TypeRef` to `Type::TargetExtType`
    // someday: Map of `(name, contained_types, contained_ints)` to the corresponding `Type::TargetExtType`. See notes on Type::TargetExtType
    #[cfg(feature = "llvm-16-or-greater")]
    target_ext_type: TypeRef,
    /// internal cache of already-seen `LLVMTypeRef`s so we can quickly produce
    /// the corresponding `TypeRef` without re-parsing the type
    llvm_type_map: HashMap<LLVMTypeRef, TypeRef>,
}

impl TypesBuilder {
    pub fn new() -> Self {
        Self {
            void_type: TypeRef::new(Type::VoidType),
            int_types: TypeCache::new(),
            pointer_types: TypeCache::new(),
            fp_types: TypeCache::new(),
            func_types: TypeCache::new(),
            vec_types: TypeCache::new(),
            arr_types: TypeCache::new(),
            struct_types: TypeCache::new(),
            named_struct_types: TypeCache::new(),
            named_struct_defs: HashMap::new(),
            x86_mmx_type: TypeRef::new(Type::X86_MMXType),
            #[cfg(feature = "llvm-12-or-greater")]
            x86_amx_type: TypeRef::new(Type::X86_AMXType),
            metadata_type: TypeRef::new(Type::MetadataType),
            label_type: TypeRef::new(Type::LabelType),
            token_type: TypeRef::new(Type::TokenType),
            #[cfg(feature = "llvm-16-or-greater")]
            target_ext_type: TypeRef::new(Type::TargetExtType),
            llvm_type_map: HashMap::new(),
        }
    }

    /// Consumes the `TypesBuilder`, producing a `Types`.
    /// This should be done when no new types are expected to be added;
    /// and it allows type lookups without &mut self.
    pub fn build(self) -> Types {
        Types {
            void_type: self.void_type,
            int_types: self.int_types,
            pointer_types: self.pointer_types,
            fp_types: self.fp_types,
            func_types: self.func_types,
            vec_types: self.vec_types,
            arr_types: self.arr_types,
            struct_types: self.struct_types,
            named_struct_types: self.named_struct_types,
            named_struct_defs: self.named_struct_defs,
            x86_mmx_type: self.x86_mmx_type,
            #[cfg(feature = "llvm-12-or-greater")]
            x86_amx_type: self.x86_amx_type,
            metadata_type: self.metadata_type,
            label_type: self.label_type,
            token_type: self.token_type,
            #[cfg(feature = "llvm-16-or-greater")]
            target_ext_type: self.target_ext_type,
        }
    }
}

// some of these methods might not currently be used, that's fine
#[allow(dead_code)]
impl TypesBuilder {
    /// Get the void type
    pub fn void(&self) -> TypeRef {
        self.void_type.clone()
    }

    /// Get the integer type of the specified size (in bits)
    pub fn int(&mut self, bits: u32) -> TypeRef {
        self.int_types
            .lookup_or_insert(bits, || Type::IntegerType { bits })
    }

    /// Get the boolean type (`i1`)
    pub fn bool(&mut self) -> TypeRef {
        self.int(1)
    }

    /// Get the 8-bit integer type
    pub fn i8(&mut self) -> TypeRef {
        self.int(8)
    }

    /// Get the 16-bit integer type
    pub fn i16(&mut self) -> TypeRef {
        self.int(16)
    }

    /// Get the 32-bit integer type
    pub fn i32(&mut self) -> TypeRef {
        self.int(32)
    }

    /// Get the 64-bit integer type
    pub fn i64(&mut self) -> TypeRef {
        self.int(64)
    }

    /// Get a pointer type in the default address space (`0`)
    #[cfg(feature = "llvm-14-or-lower")]
    pub fn pointer_to(&mut self, pointee_type: TypeRef) -> TypeRef {
        self.pointer_in_addr_space(pointee_type, 0) // default to address space 0
    }
    /// Get the pointer type for the default address space (`0`)
    #[cfg(feature = "llvm-15-or-greater")]
    pub fn pointer(&mut self) -> TypeRef {
        self.pointer_in_addr_space(0) // default to address space 0
    }

    /// Get a pointer in the specified address space
    #[cfg(feature = "llvm-14-or-lower")]
    pub fn pointer_in_addr_space(
        &mut self,
        pointee_type: TypeRef,
        addr_space: AddrSpace,
    ) -> TypeRef {
        self.pointer_types
            .lookup_or_insert((pointee_type.clone(), addr_space), || Type::PointerType {
                pointee_type,
                addr_space,
            })
    }
    /// Get a pointer in the specified address space
    #[cfg(feature = "llvm-15-or-greater")]
    pub fn pointer_in_addr_space(&mut self, addr_space: AddrSpace) -> TypeRef {
        self.pointer_types
            .lookup_or_insert(addr_space, || Type::PointerType { addr_space })
    }

    /// Get a floating-point type
    pub fn fp(&mut self, fpt: FPType) -> TypeRef {
        self.fp_types.lookup_or_insert(fpt, || Type::FPType(fpt))
    }

    /// Get the single-precision floating-point type
    pub fn single(&mut self) -> TypeRef {
        self.fp(FPType::Single)
    }

    /// Get the double-precision floating-point type
    pub fn double(&mut self) -> TypeRef {
        self.fp(FPType::Double)
    }

    /// Get a function type
    pub fn func_type(
        &mut self,
        result_type: TypeRef,
        param_types: Vec<TypeRef>,
        is_var_arg: bool,
    ) -> TypeRef {
        self.func_types.lookup_or_insert(
            (result_type.clone(), param_types.clone(), is_var_arg),
            || Type::FuncType {
                result_type,
                param_types,
                is_var_arg,
            },
        )
    }

    /// Get a vector type
    #[cfg(feature = "llvm-11-or-greater")]
    pub fn vector_of(
        &mut self,
        element_type: TypeRef,
        num_elements: usize,
        scalable: bool,
    ) -> TypeRef {
        self.vec_types
            .lookup_or_insert((element_type.clone(), num_elements, scalable), || {
                Type::VectorType {
                    element_type,
                    num_elements,
                    scalable,
                }
            })
    }
    /// Get a vector type
    #[cfg(feature = "llvm-10-or-lower")]
    pub fn vector_of(&mut self, element_type: TypeRef, num_elements: usize) -> TypeRef {
        self.vec_types
            .lookup_or_insert((element_type.clone(), num_elements, false), || {
                Type::VectorType {
                    element_type,
                    num_elements,
                }
            })
    }

    /// Get an array type
    pub fn array_of(&mut self, element_type: TypeRef, num_elements: usize) -> TypeRef {
        self.arr_types
            .lookup_or_insert((element_type.clone(), num_elements), || Type::ArrayType {
                element_type,
                num_elements,
            })
    }

    /// Get a struct type
    pub fn struct_of(&mut self, element_types: Vec<TypeRef>, is_packed: bool) -> TypeRef {
        self.struct_types
            .lookup_or_insert((element_types.clone(), is_packed), || Type::StructType {
                element_types,
                is_packed,
            })
    }

    /// Get the `TypeRef` for the struct with the given name.
    ///
    /// Note that this gives a `NamedStructType`.
    /// To get the actual _definition_ of a named struct (the `NamedStructDef`),
    /// use `named_struct_def()`.
    pub fn named_struct(&mut self, name: String) -> TypeRef {
        self.named_struct_types
            .lookup_or_insert(name.clone(), || Type::NamedStructType { name })
    }

    /// Get the `NamedStructDef` for the struct with the given `name`.
    ///
    /// Panics if no definition has been added for that struct name.
    ///
    /// Note that this gives a `NamedStructDef`.
    /// To get the `NamedStructType` for a `name`, use `named_struct()`.
    pub fn named_struct_def(&self, name: &str) -> &NamedStructDef {
        self.named_struct_defs
            .get(name)
            .expect("Named struct has not been defined")
    }

    /// Add the given `NamedStructDef` as the definition of the struct with the given `name`.
    ///
    /// Panics if that name already had a definition.
    pub fn add_named_struct_def(&mut self, name: String, def: NamedStructDef) {
        match self.named_struct_defs.entry(name) {
            Entry::Occupied(_) => {
                panic!("Trying to redefine named struct");
            },
            Entry::Vacant(ventry) => {
                ventry.insert(def);
            },
        }
    }

    /// Get the X86_MMX type
    pub fn x86_mmx(&self) -> TypeRef {
        self.x86_mmx_type.clone()
    }

    /// Get the X86_AMX type
    #[cfg(feature = "llvm-12-or-greater")]
    pub fn x86_amx(&self) -> TypeRef {
        self.x86_amx_type.clone()
    }

    /// Get the metadata type
    pub fn metadata_type(&self) -> TypeRef {
        self.metadata_type.clone()
    }

    /// Get the label type
    pub fn label_type(&self) -> TypeRef {
        self.label_type.clone()
    }

    /// Get the token type
    pub fn token_type(&self) -> TypeRef {
        self.token_type.clone()
    }

    /// Get the target extension type
    #[cfg(feature = "llvm-16-or-greater")]
    pub fn target_ext_type(
        &mut self,
        // name: String, // TODO not exposed in the LLVM C API; see notes on Type::TargetExtType
        // contained_types: Vec<TypeRef>, // TODO not exposed in the LLVM C API; see notes on Type::TargetExtType
        // contained_ints: Vec<u32>, // TODO not exposed in the LLVM C API; see notes on Type::TargetExtType
    ) -> TypeRef {
        self.target_ext_type.clone()
    }
}

#[derive(Clone, Debug, Hash)]
pub enum NamedStructDef {
    /// An opaque struct type; see [LLVM 14 docs on Opaque Structure Types](https://releases.llvm.org/14.0.0/docs/LangRef.html#t-opaque).
    Opaque,
    /// A struct type with a definition. The `TypeRef` here is guaranteed to be to a `StructType` variant.
    Defined(TypeRef),
}

/// Holds a reference to all of the `Type`s used in the `Module`, and facilitates
/// lookups so you can get a `TypeRef` to the `Type` you want.
//
// Unlike `TypesBuilder`, this is intended to be immutable, and performs type
// lookups without &mut self.
// It should be created from `TypesBuilder::build()`, and once it is built,
// it should contain all types ever used in the `Module`.
//
// That said, if you happen to want a type which wasn't encountered when parsing
// the `Module` (e.g., a pointer to some type in the `Module`, even if the
// `Module` doesn't itself create pointers to that type), it will still
// construct that `Type` and give you a `TypeRef`; you'll just be the sole owner
// of that `Type` object.
#[derive(Clone)]
pub struct Types {
    /// `TypeRef` to `Type::VoidType`
    void_type: TypeRef,
    /// Map of integer size to `Type::IntegerType` of that size
    int_types: TypeCache<u32>,
    /// Map of (pointee type, address space) to the corresponding `Type::PointerType`
    #[cfg(feature = "llvm-14-or-lower")]
    pointer_types: TypeCache<(TypeRef, AddrSpace)>,
    /// Map of address space to the corresponding `Type::PointerType`
    #[cfg(feature = "llvm-15-or-greater")]
    pointer_types: TypeCache<AddrSpace>,
    /// Map of `FPType` to the corresponding `Type::FPType`
    fp_types: TypeCache<FPType>,
    /// Map of `(result_type, param_types, is_var_arg)` to the corresponding `Type::FunctionType`
    func_types: TypeCache<(TypeRef, Vec<TypeRef>, bool)>,
    /// Map of (element type, #elements, scalable) to the corresponding `Type::VectorType`.
    /// For LLVM 10 and lower, `scalable` is always `false`.
    vec_types: TypeCache<(TypeRef, usize, bool)>,
    /// Map of (element type, #elements) to the corresponding `Type::ArrayType`
    arr_types: TypeCache<(TypeRef, usize)>,
    /// Map of `(element_types, is_packed)` to the corresponding `Type::StructType`
    struct_types: TypeCache<(Vec<TypeRef>, bool)>,
    /// Map of struct name to the corresponding `Type::NamedStructType`
    named_struct_types: TypeCache<String>,
    /// Map of struct name to the corresponding `NamedStructDef`
    named_struct_defs: HashMap<String, NamedStructDef>,
    /// `TypeRef` to `Type::X86_MMXType`
    x86_mmx_type: TypeRef,
    /// `TypeRef` to `Type::X86_AMXType`
    #[cfg(feature = "llvm-12-or-greater")]
    x86_amx_type: TypeRef,
    /// `TypeRef` to `Type::MetadataType`
    metadata_type: TypeRef,
    /// `TypeRef` to `Type::LabelType`
    label_type: TypeRef,
    /// `TypeRef` to `Type::TokenType`
    token_type: TypeRef,
    /// `TypeRef` to `Type::TargetExtType`
    #[cfg(feature = "llvm-16-or-greater")]
    target_ext_type: TypeRef,
}

impl Types {
    /// Get the type of anything that is `Typed`
    pub fn type_of<T: Typed + ?Sized>(&self, t: &T) -> TypeRef {
        t.get_type(self)
    }

    /// Get the void type
    pub fn void(&self) -> TypeRef {
        self.void_type.clone()
    }

    /// Get the integer type of the specified size (in bits)
    pub fn int(&self, bits: u32) -> TypeRef {
        self.int_types
            .lookup(&bits)
            .unwrap_or_else(|| TypeRef::new(Type::IntegerType { bits }))
    }

    /// Get the boolean type (`i1`)
    pub fn bool(&self) -> TypeRef {
        self.int(1)
    }

    /// Get the 8-bit integer type
    pub fn i8(&self) -> TypeRef {
        self.int(8)
    }

    /// Get the 16-bit integer type
    pub fn i16(&self) -> TypeRef {
        self.int(16)
    }

    /// Get the 32-bit integer type
    pub fn i32(&self) -> TypeRef {
        self.int(32)
    }

    /// Get the 64-bit integer type
    pub fn i64(&self) -> TypeRef {
        self.int(64)
    }

    /// Get a pointer type in the default address space (`0`)
    #[cfg(feature = "llvm-14-or-lower")]
    pub fn pointer_to(&self, pointee_type: TypeRef) -> TypeRef {
        self.pointer_in_addr_space(pointee_type, 0)
    }
    /// Get the pointer type for the default address space (`0`)
    #[cfg(feature = "llvm-15-or-greater")]
    pub fn pointer(&self) -> TypeRef {
        self.pointer_in_addr_space(0)
    }

    /// Get a pointer type in the specified address space
    #[cfg(feature = "llvm-14-or-lower")]
    pub fn pointer_in_addr_space(&self, pointee_type: TypeRef, addr_space: AddrSpace) -> TypeRef {
        self.pointer_types
            .lookup(&(pointee_type.clone(), addr_space))
            .unwrap_or_else(|| {
                TypeRef::new(Type::PointerType {
                    pointee_type,
                    addr_space,
                })
            })
    }
    /// Get a pointer type in the specified address space
    #[cfg(feature = "llvm-15-or-greater")]
    pub fn pointer_in_addr_space(&self, addr_space: AddrSpace) -> TypeRef {
        self.pointer_types
            .lookup(&addr_space)
            .unwrap_or_else(|| TypeRef::new(Type::PointerType { addr_space }))
    }

    /// Get a floating-point type
    pub fn fp(&self, fpt: FPType) -> TypeRef {
        self.fp_types
            .lookup(&fpt)
            .unwrap_or_else(|| TypeRef::new(Type::FPType(fpt)))
    }

    /// Get the single-precision floating-point type
    pub fn single(&self) -> TypeRef {
        self.fp(FPType::Single)
    }

    /// Get the double-precision floating-point type
    pub fn double(&self) -> TypeRef {
        self.fp(FPType::Double)
    }

    /// Get a function type
    pub fn func_type(
        &self,
        result_type: TypeRef,
        param_types: Vec<TypeRef>,
        is_var_arg: bool,
    ) -> TypeRef {
        self.func_types
            .lookup(&(result_type.clone(), param_types.clone(), is_var_arg))
            .unwrap_or_else(|| {
                TypeRef::new(Type::FuncType {
                    result_type,
                    param_types,
                    is_var_arg,
                })
            })
    }

    /// Get a vector type
    #[cfg(feature = "llvm-11-or-greater")]
    pub fn vector_of(&self, element_type: TypeRef, num_elements: usize, scalable: bool) -> TypeRef {
        self.vec_types
            .lookup(&(element_type.clone(), num_elements, scalable))
            .unwrap_or_else(|| {
                TypeRef::new(Type::VectorType {
                    element_type,
                    num_elements,
                    scalable,
                })
            })
    }
    #[cfg(feature = "llvm-10-or-lower")]
    pub fn vector_of(&self, element_type: TypeRef, num_elements: usize) -> TypeRef {
        self.vec_types
            .lookup(&(element_type.clone(), num_elements, false))
            .unwrap_or_else(|| {
                TypeRef::new(Type::VectorType {
                    element_type,
                    num_elements,
                })
            })
    }

    /// Get an array type
    pub fn array_of(&self, element_type: TypeRef, num_elements: usize) -> TypeRef {
        self.arr_types
            .lookup(&(element_type.clone(), num_elements))
            .unwrap_or_else(|| {
                TypeRef::new(Type::ArrayType {
                    element_type,
                    num_elements,
                })
            })
    }

    /// Get a struct type
    pub fn struct_of(&self, element_types: Vec<TypeRef>, is_packed: bool) -> TypeRef {
        self.struct_types
            .lookup(&(element_types.clone(), is_packed))
            .unwrap_or_else(|| {
                TypeRef::new(Type::StructType {
                    element_types,
                    is_packed,
                })
            })
    }

    /// Get the `TypeRef` for the struct with the given `name`.
    ///
    /// Note that this gives a `NamedStructType`.
    /// To get the actual _definition_ of a named struct (the `NamedStructDef`),
    /// use `named_struct_def()`.
    pub fn named_struct(&self, name: &str) -> TypeRef {
        self.named_struct_types
            .lookup(name)
            .unwrap_or_else(|| TypeRef::new(Type::NamedStructType { name: name.into() }))
    }

    /// Get the `NamedStructDef` for the struct with the given `name`, or
    /// `None` if there is no struct by that name.
    ///
    /// Note that this gives a `NamedStructDef`.
    /// To get the `NamedStructType` for a `name`, use `named_struct()`.
    pub fn named_struct_def(&self, name: &str) -> Option<&NamedStructDef> {
        self.named_struct_defs.get(name)
    }

    /// Get the names of all the named structs
    pub fn all_struct_names(&self) -> impl Iterator<Item = &String> {
        self.named_struct_defs.keys()
    }

    /// Add the given `NamedStructDef` as the definition of the struct with the given `name`.
    ///
    /// Panics if that name already had a definition.
    pub fn add_named_struct_def(&mut self, name: String, def: NamedStructDef) {
        match self.named_struct_defs.entry(name) {
            Entry::Occupied(_) => {
                panic!("Trying to redefine named struct");
            },
            Entry::Vacant(ventry) => {
                ventry.insert(def);
            },
        }
    }

    /// Remove the definition of the struct with the given `name`.
    ///
    /// Returns `true` if the definition was removed, or `false` if no definition
    /// existed.
    pub fn remove_named_struct_def(&mut self, name: &str) -> bool {
        self.named_struct_defs.remove(name).is_some()
    }

    /// Get the X86_MMX type
    pub fn x86_mmx(&self) -> TypeRef {
        self.x86_mmx_type.clone()
    }

    /// Get the X86_AMX type
    #[cfg(feature = "llvm-12-or-greater")]
    pub fn x86_amx(&self) -> TypeRef {
        self.x86_amx_type.clone()
    }

    /// Get the metadata type
    pub fn metadata_type(&self) -> TypeRef {
        self.metadata_type.clone()
    }

    /// Get the label type
    pub fn label_type(&self) -> TypeRef {
        self.label_type.clone()
    }

    /// Get the token type
    pub fn token_type(&self) -> TypeRef {
        self.token_type.clone()
    }

    /// Get the `TypeRef` for target extension type with the given
    /// name, contained types, and contained ints.
    #[cfg(feature = "llvm-16-or-greater")]
    pub fn target_ext_type(
        &self,
        // name: String, // TODO not exposed in the LLVM C API; see notes on Type::TargetExtType
        // contained_types: Vec<TypeRef>, // TODO not exposed in the LLVM C API; see notes on Type::TargetExtType
        // contained_ints: Vec<u32>, // TODO not exposed in the LLVM C API; see notes on Type::TargetExtType
    ) -> TypeRef {
        self.target_ext_type.clone()
    }

    /// Get a `TypeRef` for the given `Type`
    #[rustfmt::skip] // so we can keep each of the match arms more consistent with each other
    pub fn get_for_type(&self, ty: &Type) -> TypeRef {
        match ty {
            Type::VoidType => self.void(),
            Type::IntegerType{ bits } => self.int(*bits),
            #[cfg(feature = "llvm-14-or-lower")]
            Type::PointerType { pointee_type, addr_space } => {
                self.pointer_in_addr_space(pointee_type.clone(), *addr_space)
            },
            #[cfg(feature = "llvm-15-or-greater")]
            Type::PointerType { addr_space } => {
                self.pointer_in_addr_space(*addr_space)
            },
            Type::FPType(fpt) => self.fp(*fpt),
            Type::FuncType { result_type, param_types, is_var_arg } => {
                self.func_type(result_type.clone(), param_types.clone(), *is_var_arg)
            },
            #[cfg(feature="llvm-11-or-greater")]
            Type::VectorType { element_type, num_elements, scalable } => {
                self.vector_of(element_type.clone(), *num_elements, *scalable)
            },
            #[cfg(feature="llvm-10-or-lower")]
            Type::VectorType { element_type, num_elements } => {
                self.vector_of(element_type.clone(), *num_elements)
            },
            Type::ArrayType { element_type, num_elements } => {
                self.array_of(element_type.clone(), *num_elements)
            },
            Type::StructType { element_types, is_packed } => {
                self.struct_of(element_types.clone(), *is_packed)
            },
            Type::NamedStructType { name  } => self.named_struct(name),
            Type::X86_MMXType => self.x86_mmx(),
            #[cfg(feature="llvm-12-or-greater")]
            Type::X86_AMXType => self.x86_amx(),
            Type::MetadataType => self.metadata_type(),
            Type::LabelType => self.label_type(),
            Type::TokenType => self.token_type(),
            #[cfg(feature="llvm-16-or-greater")]
            Type::TargetExtType => self.target_ext_type(),
        }
    }
}

impl Types {
    /// Get a blank `Types` containing essentially no types.
    /// This function is intended only for use in testing;
    /// it's probably not useful otherwise.
    pub fn blank_for_testing() -> Self {
        TypesBuilder::new().build()
    }
}

#[derive(Clone, Debug)]
struct TypeCache<K: Eq + Hash + Clone> {
    map: HashMap<K, TypeRef>,
}

#[allow(dead_code)]
impl<K: Eq + Hash + Clone> TypeCache<K> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Get a `TypeRef` to the `Type` with the given key,
    /// or `None` if the `Type` is not present.
    fn lookup<Q: ?Sized>(&self, key: &Q) -> Option<TypeRef>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get(key).cloned()
    }

    /// Get a `TypeRef` to the `Type` with the given key.
    /// The `if_missing` function or closure will be called to create that `Type`
    /// if it hasn't been created yet.
    fn lookup_or_insert(&mut self, key: K, if_missing: impl FnOnce() -> Type) -> TypeRef {
        self.map
            .entry(key)
            .or_insert_with(|| TypeRef::new(if_missing()))
            .clone()
    }

    /// Is a `Type` for the given key currently in the cache?
    fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
}

// ********* //
// from_llvm //
// ********* //

use crate::from_llvm::*;
use crate::llvm_sys::*;
use llvm_sys::LLVMTypeKind;
use std::collections::hash_map::Entry;

impl TypesBuilder {
    pub(crate) fn type_from_llvm_ref(&mut self, ty: LLVMTypeRef) -> TypeRef {
        if let Some(typeref) = self.llvm_type_map.get(&ty) {
            return typeref.clone();
        }
        let typeref = self.parse_type_from_llvm_ref(ty);
        self.llvm_type_map.insert(ty, typeref.clone());
        typeref
    }

    fn parse_type_from_llvm_ref(&mut self, ty: LLVMTypeRef) -> TypeRef {
        let kind = unsafe { LLVMGetTypeKind(ty) };
        match kind {
            LLVMTypeKind::LLVMVoidTypeKind => self.void(),
            LLVMTypeKind::LLVMIntegerTypeKind => self.int(unsafe { LLVMGetIntTypeWidth(ty) }),
            #[cfg(feature = "llvm-14-or-lower")]
            LLVMTypeKind::LLVMPointerTypeKind => {
                let pointee_type = self.type_from_llvm_ref(unsafe { LLVMGetElementType(ty) });
                self.pointer_in_addr_space(pointee_type, unsafe { LLVMGetPointerAddressSpace(ty) })
            },
            #[cfg(feature = "llvm-15-or-greater")]
            LLVMTypeKind::LLVMPointerTypeKind => {
                self.pointer_in_addr_space(unsafe { LLVMGetPointerAddressSpace(ty) })
            },
            LLVMTypeKind::LLVMArrayTypeKind => {
                let element_type = self.type_from_llvm_ref(unsafe { LLVMGetElementType(ty) });
                
                // LLVMGetArrayLength2 was added in LLVM-17: the old function still exists there,
                // but is deprecated. The parameters are the same, but the return type is changed
                // from c_uint to u64
                #[cfg(feature = "llvm-16-or-lower")]
                let array_len = unsafe { LLVMGetArrayLength(ty) as usize };
                #[cfg(feature = "llvm-17-or-greater")]
                let array_len = unsafe { LLVMGetArrayLength2(ty) as usize };
                
                self.array_of(element_type, array_len)
            },
            LLVMTypeKind::LLVMVectorTypeKind => {
                let element_type = self.type_from_llvm_ref(unsafe { LLVMGetElementType(ty) });
                #[cfg(feature = "llvm-11-or-greater")]
                let ret = self.vector_of(
                    element_type,
                    unsafe { LLVMGetVectorSize(ty) as usize },
                    false,
                );
                #[cfg(feature = "llvm-10-or-lower")]
                let ret = self.vector_of(element_type, unsafe { LLVMGetVectorSize(ty) as usize });
                ret
            },
            #[cfg(feature = "llvm-11-or-greater")]
            LLVMTypeKind::LLVMScalableVectorTypeKind => {
                let element_type = self.type_from_llvm_ref(unsafe { LLVMGetElementType(ty) });
                self.vector_of(
                    element_type,
                    unsafe { LLVMGetVectorSize(ty) as usize },
                    true,
                )
            },
            LLVMTypeKind::LLVMStructTypeKind => {
                let name = if unsafe { LLVMIsLiteralStruct(ty) } != 0 {
                    None
                } else {
                    unsafe { get_struct_name(ty) }
                };

                match name {
                    Some(s) if !s.is_empty() => {
                        if self.named_struct_types.contains_key(&s) {
                            // already defined: return the NamedStructType and don't change the definition
                            self.named_struct(s)
                        } else if unsafe { LLVMIsOpaqueStruct(ty) } != 0 {
                            // add the definition as opaque
                            self.add_named_struct_def(s.clone(), NamedStructDef::Opaque);
                            // return the NamedStructType
                            self.named_struct(s)
                        } else {
                            // add the NamedStructType first, so that the call to struct_type_from_llvm_ref will terminate
                            let named_struct_typeref = self.named_struct(s.clone());
                            // now compute the actual struct type. Any self-references will point to the NamedStructType we just created
                            let actual_struct_type = self.struct_type_from_llvm_ref(ty);
                            // add this definition for the named struct
                            self.add_named_struct_def(
                                s,
                                NamedStructDef::Defined(actual_struct_type),
                            );
                            // And now we return the NamedStructType
                            named_struct_typeref
                        }
                    },
                    _ => self.struct_type_from_llvm_ref(ty),
                }
            },
            LLVMTypeKind::LLVMFunctionTypeKind => {
                let result_type = self.type_from_llvm_ref(unsafe { LLVMGetReturnType(ty) });
                let param_types = {
                    let num_types = unsafe { LLVMCountParamTypes(ty) };
                    let mut types: Vec<LLVMTypeRef> = Vec::with_capacity(num_types as usize);
                    unsafe {
                        LLVMGetParamTypes(ty, types.as_mut_ptr());
                        types.set_len(num_types as usize);
                    };
                    types
                        .into_iter()
                        .map(|t| self.type_from_llvm_ref(t))
                        .collect()
                };
                self.func_type(
                    result_type,
                    param_types,
                    unsafe { LLVMIsFunctionVarArg(ty) } != 0,
                )
            },
            LLVMTypeKind::LLVMHalfTypeKind => self.fp(FPType::Half),
            #[cfg(feature = "llvm-11-or-greater")]
            LLVMTypeKind::LLVMBFloatTypeKind => self.fp(FPType::BFloat),
            LLVMTypeKind::LLVMFloatTypeKind => self.fp(FPType::Single),
            LLVMTypeKind::LLVMDoubleTypeKind => self.fp(FPType::Double),
            LLVMTypeKind::LLVMFP128TypeKind => self.fp(FPType::FP128),
            LLVMTypeKind::LLVMX86_FP80TypeKind => self.fp(FPType::X86_FP80),
            LLVMTypeKind::LLVMPPC_FP128TypeKind => self.fp(FPType::PPC_FP128),
            LLVMTypeKind::LLVMX86_MMXTypeKind => self.x86_mmx(),
            #[cfg(feature = "llvm-12-or-greater")]
            LLVMTypeKind::LLVMX86_AMXTypeKind => self.x86_amx(),
            LLVMTypeKind::LLVMMetadataTypeKind => self.metadata_type(),
            LLVMTypeKind::LLVMLabelTypeKind => self.label_type(),
            LLVMTypeKind::LLVMTokenTypeKind => self.token_type(),
            #[cfg(feature = "llvm-16-or-greater")]
            LLVMTypeKind::LLVMTargetExtTypeKind => self.target_ext_type(),
        }
    }

    /// creates an actual `StructType`, regardless of whether the struct is named or not
    ///
    /// Caller is responsible for ensuring that `ty` is not an opaque struct type
    fn struct_type_from_llvm_ref(&mut self, ty: LLVMTypeRef) -> TypeRef {
        if unsafe { LLVMIsOpaqueStruct(ty) } != 0 {
            panic!(
                "struct_type_from_llvm_ref: shouldn't pass an opaque struct type to this function"
            );
        }
        let element_types = {
            let num_types = unsafe { LLVMCountStructElementTypes(ty) };
            let mut types: Vec<LLVMTypeRef> = Vec::with_capacity(num_types as usize);
            unsafe {
                LLVMGetStructElementTypes(ty, types.as_mut_ptr());
                types.set_len(num_types as usize);
            };
            types
                .into_iter()
                .map(|t| self.type_from_llvm_ref(t))
                .collect()
        };
        self.struct_of(element_types, unsafe { LLVMIsPackedStruct(ty) } != 0)
    }
}
