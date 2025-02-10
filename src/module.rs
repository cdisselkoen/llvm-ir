use crate::constant::ConstantRef;
use crate::debugloc::*;
use crate::function::{Function, FunctionAttribute, FunctionDeclaration, GroupID};
use crate::llvm_sys::*;
use crate::name::Name;
use crate::types::{FPType, Type, TypeRef, Typed, Types, TypesBuilder};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

/// See [LLVM 14 docs on Module Structure](https://releases.llvm.org/14.0.0/docs/LangRef.html#module-structure)
#[derive(Clone)]
pub struct Module {
    /// The name of the module
    pub name: String,
    /// See [LLVM 14 docs on Source Filename](https://releases.llvm.org/14.0.0/docs/LangRef.html#source-filename)
    pub source_file_name: String,
    /// See [LLVM 14 docs on Data Layout](https://releases.llvm.org/14.0.0/docs/LangRef.html#data-layout)
    pub data_layout: DataLayout,
    /// See [LLVM 14 docs on Target Triple](https://releases.llvm.org/14.0.0/docs/LangRef.html#target-triple)
    pub target_triple: Option<String>,
    /// Functions which are defined (not just declared) in this `Module`.
    /// See [LLVM 14 docs on Functions](https://releases.llvm.org/14.0.0/docs/LangRef.html#functions)
    pub functions: Vec<Function>,
    /// Functions which are declared (but not defined) in this `Module`.
    /// See [LLVM 14 docs on Functions](https://releases.llvm.org/14.0.0/docs/LangRef.html#functions)
    pub func_declarations: Vec<FunctionDeclaration>,
    /// See [LLVM 14 docs on Global Variables](https://releases.llvm.org/14.0.0/docs/LangRef.html#global-variables)
    pub global_vars: Vec<GlobalVariable>,
    /// See [LLVM 14 docs on Global Aliases](https://releases.llvm.org/14.0.0/docs/LangRef.html#aliases)
    pub global_aliases: Vec<GlobalAlias>,
    /// See [LLVM 14 docs on Global IFuncs](https://releases.llvm.org/14.0.0/docs/LangRef.html#ifuncs)
    pub global_ifuncs: Vec<GlobalIFunc>,
    // --TODO not yet implemented-- pub function_attribute_groups: Vec<FunctionAttributeGroup>,
    /// See [LLVM 14 docs on Module-Level Inline Assembly](https://releases.llvm.org/14.0.0/docs/LangRef.html#moduleasm)
    pub inline_assembly: String,
    // --TODO not yet implemented-- pub metadata_nodes: Vec<(MetadataNodeID, MetadataNode)>,
    // --TODO not yet implemented-- pub named_metadatas: Vec<NamedMetadata>,
    // --TODO not yet implemented-- pub comdats: Vec<Comdat>,
    /// Holds a reference to all of the `Type`s used in the `Module`, and
    /// facilitates lookups so you can get a `TypeRef` to the `Type` you want.
    pub types: Types,
}

impl Module {
    /// Get the type of anything that is `Typed`.
    pub fn type_of<T: Typed + ?Sized>(&self, t: &T) -> TypeRef {
        self.types.type_of(t)
    }

    /// Get the `Function` having the given `name` (if any).
    /// Note that functions are named with `String`s and not `Name`s.
    ///
    /// Note also that this will only find _fully defined_ functions, not
    /// `FunctionDeclaration`s.
    pub fn get_func_by_name(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|func| func.name == name)
    }

    /// Get the `FunctionDeclaration` having the given `name` (if any).
    /// Note that functions are named with `String`s and not `Name`s.
    ///
    /// Note also that this will only find function _declarations_, and will not
    /// find defined functions (use `get_func_by_name()` for that).
    pub fn get_func_decl_by_name(&self, name: &str) -> Option<&FunctionDeclaration> {
        self.func_declarations.iter().find(|decl| decl.name == name)
    }

    /// Get the `GlobalVariable` having the given `Name` (if any).
    pub fn get_global_var_by_name(&self, name: &Name) -> Option<&GlobalVariable> {
        self.global_vars.iter().find(|global| global.name == *name)
    }

    /// Get the `GlobalAlias` having the given `Name` (if any).
    pub fn get_global_alias_by_name(&self, name: &Name) -> Option<&GlobalAlias> {
        self.global_aliases.iter().find(|global| global.name == *name)
    }

    /// Get the `GlobalIFunc` having the given `Name` (if any).
    pub fn get_global_ifunc_by_name(&self, name: &Name) -> Option<&GlobalIFunc> {
        self.global_ifuncs.iter().find(|global| global.name == *name)
    }

    /// Parse the LLVM bitcode (.bc) file at the given path to create a `Module`
    pub fn from_bc_path(path: impl AsRef<Path>) -> Result<Self, String> {
        unsafe fn parse_bc(
            context_ref: LLVMContextRef,
            mem_buf: LLVMMemoryBufferRef,
            out_module: *mut LLVMModuleRef,
        ) -> Result<(), String> {
            let result =
                llvm_sys::bit_reader::LLVMParseBitcodeInContext2(context_ref, mem_buf, out_module);
            LLVMDisposeMemoryBuffer(mem_buf);
            match result {
                0 => Ok(()),
                _ => Err("Failed to parse bitcode".to_owned())
            }
        }
        Self::from_path(path, parse_bc)
    }

    /// Parse the LLVM text IR (.ll) file at the given path to create a `Module`
    pub fn from_ir_path(path: impl AsRef<Path>) -> Result<Self, String> {
        Self::from_path(path, Self::parse_ir)
    }

    /// Parse the given string as LLVM text IR to create a `Module`
    pub fn from_ir_str(str: &str) -> Result<Self, String> {
        let memory_buffer = unsafe {
            LLVMCreateMemoryBufferWithMemoryRange(
                str.as_ptr() as *const _,
                str.len(),
                std::ffi::CString::new("str").unwrap().as_ptr(),
                true.into(),
            )
        };
        Self::from_buffer(memory_buffer, Self::parse_ir)
    }

    unsafe fn parse_ir(
        context_ref: LLVMContextRef,
        mem_buf: LLVMMemoryBufferRef,
        out_module: *mut LLVMModuleRef,
    ) -> Result<(), String> {
        use std::ffi::CStr;
        let mut err_string = std::mem::zeroed();
        // This call takes ownership of the buffer, so we don't free it.
        match llvm_sys::ir_reader::LLVMParseIRInContext(context_ref, mem_buf, out_module, &mut err_string) {
            0 => Ok(()),
            _ => Err(format!("Failed to parse bitcode: {}",
                             CStr::from_ptr(err_string).to_str().expect("Failed to convert CStr")))
        }
    }

    fn from_path(
        path: impl AsRef<Path>,
        parse: unsafe fn(
            context_ref: LLVMContextRef,
            mem_buf: LLVMMemoryBufferRef,
            out_module: *mut LLVMModuleRef,
        ) -> Result<(), String>,
    ) -> Result<Self, String> {
        // implementation here inspired by the `inkwell` crate's `Module::parse_bitcode_from_path`
        use std::ffi::{CStr, CString};

        let path = CString::new(
            path.as_ref()
                .to_str()
                .expect("Did not find a valid Unicode path string"),
        )
        .expect("Failed to convert to CString");
        debug!("Creating a Module from path {:?}", path);

        let memory_buffer = unsafe {
            let mut memory_buffer = std::ptr::null_mut();
            let mut err_string = std::mem::zeroed();
            let return_code = LLVMCreateMemoryBufferWithContentsOfFile(
                path.as_ptr() as *const _,
                &mut memory_buffer,
                &mut err_string,
            );
            if return_code != 0 {
                return Err(CStr::from_ptr(err_string)
                    .to_str()
                    .expect("Failed to convert CStr")
                    .to_owned());
            }
            memory_buffer
        };
        debug!("Created a MemoryBuffer");

        Self::from_buffer(memory_buffer, parse)
    }

    fn from_buffer(
        memory_buffer: LLVMMemoryBufferRef,
        parse: unsafe fn(
            context_ref: LLVMContextRef,
            mem_buf: LLVMMemoryBufferRef,
            out_module: *mut LLVMModuleRef,
        ) -> Result<(), String>,
    ) -> Result<Self, String> {
        let context = crate::from_llvm::Context::new();

        let module = unsafe {
            let mut module: std::mem::MaybeUninit<LLVMModuleRef> = std::mem::MaybeUninit::uninit();
            parse(context.ctx, memory_buffer, module.as_mut_ptr())?;
            module.assume_init()
        };
        debug!("Parsed bitcode to llvm_sys module");
        Ok(Self::from_llvm_ref(module))
    }
}

/// See [LLVM 14 docs on Global Variables](https://releases.llvm.org/14.0.0/docs/LangRef.html#global-variables)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct GlobalVariable {
    pub name: Name,
    pub linkage: Linkage,
    pub visibility: Visibility,
    pub is_constant: bool,
    pub ty: TypeRef,
    pub addr_space: AddrSpace,
    pub dll_storage_class: DLLStorageClass,
    pub thread_local_mode: ThreadLocalMode,
    pub unnamed_addr: Option<UnnamedAddr>,
    pub initializer: Option<ConstantRef>,
    pub section: Option<String>,
    pub comdat: Option<Comdat>, // llvm-hs-pure has Option<String> for some reason
    pub alignment: u32,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: Vec<(String, MetadataRef<MetadataNode>)>,
}

impl Typed for GlobalVariable {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.ty.clone()
    }
}

impl HasDebugLoc for GlobalVariable {
    fn get_debug_loc(&self) -> &Option<DebugLoc> {
        &self.debugloc
    }
}

/// See [LLVM 14 docs on Global Aliases](https://releases.llvm.org/14.0.0/docs/LangRef.html#aliases)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct GlobalAlias {
    pub name: Name,
    pub aliasee: ConstantRef,
    pub linkage: Linkage,
    pub visibility: Visibility,
    pub ty: TypeRef,
    pub addr_space: AddrSpace,
    pub dll_storage_class: DLLStorageClass,
    pub thread_local_mode: ThreadLocalMode,
    pub unnamed_addr: Option<UnnamedAddr>,
}

impl Typed for GlobalAlias {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.ty.clone()
    }
}

/// See [LLVM 14 docs on Global IFuncs](https://releases.llvm.org/14.0.0/docs/LangRef.html#ifuncs)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct GlobalIFunc {
    pub name: Name,
    pub linkage: Linkage,
    pub visibility: Visibility,
    pub ty: TypeRef,
    pub resolver_fn: ConstantRef,
}

impl Typed for GlobalIFunc {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.ty.clone()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum UnnamedAddr {
    Local,
    Global,
}

/// See [LLVM 14 docs on Linkage Types](https://releases.llvm.org/14.0.0/docs/LangRef.html#linkage)
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Linkage {
    Private,
    Internal,
    External,
    ExternalWeak,
    AvailableExternally,
    LinkOnceAny,
    LinkOnceODR,
    LinkOnceODRAutoHide,
    WeakAny,
    WeakODR,
    Common,
    Appending,
    DLLImport,
    DLLExport,
    Ghost,
    LinkerPrivate,
    LinkerPrivateWeak,
}

/// See [LLVM 14 docs on Visibility Styles](https://releases.llvm.org/14.0.0/docs/LangRef.html#visibility-styles)
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Visibility {
    Default,
    Hidden,
    Protected,
}

/// See [LLVM 14 docs on DLL Storage Classes](https://releases.llvm.org/14.0.0/docs/LangRef.html#dllstorageclass)
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum DLLStorageClass {
    Default,
    Import,
    Export,
}

/// See [LLVM 14 docs on Thread Local Storage Models](https://releases.llvm.org/14.0.0/docs/LangRef.html#thread-local-storage-models)
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum ThreadLocalMode {
    NotThreadLocal,
    GeneralDynamic,
    LocalDynamic,
    InitialExec,
    LocalExec,
}

/// For discussion of address spaces, see [LLVM 14 docs on Pointer Type](https://releases.llvm.org/14.0.0/docs/LangRef.html#pointer-type)
pub type AddrSpace = u32;

/// See [LLVM 14 docs on Attribute Groups](https://releases.llvm.org/14.0.0/docs/LangRef.html#attribute-groups)
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct FunctionAttributeGroup {
    pub group_id: GroupID,
    pub attrs: Vec<FunctionAttribute>,
}

/* --TODO not yet implemented: metadata
/// See [LLVM 14 docs on Named Metadata](https://releases.llvm.org/14.0.0/docs/LangRef.html#named-metadata)
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct NamedMetadata {
    pub name: String,
    pub node_ids: Vec<MetadataNodeID>,
}
*/

/// See [LLVM 14 docs on Comdats](https://releases.llvm.org/14.0.0/docs/LangRef.html#langref-comdats)
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Comdat {
    pub name: String,
    pub selection_kind: SelectionKind,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum SelectionKind {
    Any,
    ExactMatch,
    Largest,
    NoDuplicates,
    SameSize,
}

/// See [LLVM 14 docs on Data Layout](https://releases.llvm.org/14.0.0/docs/LangRef.html#data-layout)
#[derive(Clone, Debug)]
pub struct DataLayout {
    /// The data layout in string form, as described in the Data Layout docs linked above
    pub layout_str: String,
    /// Little-endian or big-endian?
    pub endianness: Endianness,
    /// Natural alignment of the stack, in bits. For more, see the Data Layout docs linked above
    pub stack_alignment: Option<u32>,
    /// Address space for program memory
    pub program_address_space: AddrSpace,
    /// Address space for objects created by `alloca`
    pub alloca_address_space: AddrSpace,
    /// Alignment for various types in memory
    pub alignments: Alignments,
    /// What mangling will be applied when the LLVM module is compiled to machine code
    pub mangling: Option<Mangling>,
    /// Native integer width(s) for the target CPU
    pub native_int_widths: Option<HashSet<u32>>,
    /// Address spaces with non-integral pointer types
    pub non_integral_ptr_types: HashSet<AddrSpace>,
}

impl PartialEq for DataLayout {
    fn eq(&self, other: &Self) -> bool {
        // The layout string fully specifies all the other information
        self.layout_str == other.layout_str
    }
}

impl Eq for DataLayout {}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Endianness {
    /// Least-significant bits are stored in the lowest address location
    LittleEndian,
    /// Most-significant bits are stored in the lowest address location
    BigEndian,
}

/// Alignment details for a type.
/// See [LLVM 14 docs on Data Layout](https://releases.llvm.org/14.0.0/docs/LangRef.html#data-layout)
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Alignment {
    /// Minimum alignment (in bits) per the ABI
    pub abi: u32,
    /// Preferred alignment (in bits)
    pub pref: u32,
}

/// Alignment details for function pointers.
/// See [LLVM 14 docs on Data Layout](https://releases.llvm.org/14.0.0/docs/LangRef.html#data-layout)
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FunctionPtrAlignment {
    /// If `true`, function pointer alignment is independent of function alignment.
    /// If `false`, function pointer alignment is a multiple of function alignment.
    pub independent: bool,
    /// Minimum alignment (in bits) per the ABI
    pub abi: u32,
}

/// Layout details for pointers (other than function pointers).
/// See [LLVM 14 docs on Data Layout](https://releases.llvm.org/14.0.0/docs/LangRef.html#data-layout)
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct PointerLayout {
    /// Size of a pointer in bits
    pub size: u32,
    /// Alignment of a pointer
    pub alignment: Alignment,
    /// Size of an index used for address calculation, in bits
    pub index_size: u32,
}

/// Alignment for various types in memory.
/// See [LLVM 14 docs on Data Layout](https://releases.llvm.org/14.0.0/docs/LangRef.html#data-layout)
#[derive(Clone, Debug)]
pub struct Alignments {
    /// Explicit alignments for various sizes of integers (in bits). Sizes not
    /// specified here are determined according to the rules described in the
    /// Data Layout docs.
    int_alignments: BTreeMap<u32, Alignment>,
    /// Explicit alignments for various sizes of vectors (in bits). Sizes not
    /// specified here are determined according to the rules described in the
    /// Data Layout docs.
    vec_alignments: BTreeMap<u32, Alignment>,
    /// Alignment for floating-point types, by size (in bits)
    fp_alignments: HashMap<u32, Alignment>,
    /// Alignment for aggregate types (structs, arrays)
    agg_alignment: Alignment,
    /// Alignment for function pointers
    fptr_alignment: FunctionPtrAlignment,
    /// Alignment for function pointers, as an `Alignment`
    fptr_alignment_as_alignment: Alignment,
    /// Layout details for (non-function-pointer) pointers, by address space
    pointer_layouts: HashMap<AddrSpace, PointerLayout>,
}

impl Alignments {
    /// Alignment of the given type (in bits)
    pub fn type_alignment(&self, ty: &Type) -> &Alignment {
        match ty {
            Type::IntegerType { bits } => self.int_alignment(*bits),
            Type::VectorType {
                element_type,
                num_elements,
                ..
            } => {
                let element_size_bits = match element_type.as_ref() {
                    Type::IntegerType { bits } => *bits,
                    Type::FPType(fpt) => Self::fpt_size(*fpt),
                    ty => panic!("Didn't expect a vector with element type {:?}", ty),
                };
                self.vec_alignment(element_size_bits * (*num_elements as u32))
            },
            Type::FPType(fpt) => self.fp_alignment(*fpt),
            Type::StructType { .. } | Type::NamedStructType { .. } | Type::ArrayType { .. } => {
                self.agg_alignment()
            },
            #[cfg(feature = "llvm-14-or-lower")]
            Type::PointerType {
                pointee_type,
                addr_space,
            } => match pointee_type.as_ref() {
                Type::FuncType { .. } => &self.fptr_alignment_as_alignment,
                _ => &self.ptr_alignment(*addr_space).alignment,
            },
            #[cfg(feature = "llvm-15-or-greater")]
            Type::PointerType { addr_space } => &self.ptr_alignment(*addr_space).alignment,
            _ => panic!("Don't know how to get the alignment of {:?}", ty),
        }
    }

    /// Alignment of the integer type of the given size (in bits)
    pub fn int_alignment(&self, size: u32) -> &Alignment {
        // If we have an explicit entry for this size, use that
        if let Some(alignment) = self.int_alignments.get(&size) {
            return alignment;
        }
        // Find the next largest size that has an explicit entry and use that
        let next_largest_entry = self.int_alignments.iter().find(|(&k, _)| k > size);
        match next_largest_entry {
            Some((_, alignment)) => alignment,
            None => {
                // `size` is larger than any explicit entry: use the largest explicit entry
                self.int_alignments
                    .values()
                    .rev()
                    .next()
                    .expect("Should have at least one explicit entry")
            },
        }
    }

    /// Alignment of the vector type of the given total size (in bits)
    pub fn vec_alignment(&self, size: u32) -> &Alignment {
        // If we have an explicit entry for this size, use that
        if let Some(alignment) = self.vec_alignments.get(&size) {
            return alignment;
        }
        // Find the next smaller size that has an explicit entry and use that
        let next_smaller_entry = self.vec_alignments.iter().find(|(&k, _)| k < size);
        match next_smaller_entry {
            Some((_, alignment)) => alignment,
            None => {
                // `size` is smaller than any explicit entry. LLVM docs seem to
                // be not clear what happens here, I assume we just use the
                // smallest explicit entry
                self.vec_alignments
                    .values()
                    .next()
                    .expect("Should have at least one explicit entry")
            },
        }
    }

    /// Alignment of the given floating-point type
    pub fn fp_alignment(&self, fpt: FPType) -> &Alignment {
        self.fp_alignments
            .get(&Self::fpt_size(fpt))
            .unwrap_or_else(|| {
                panic!(
                    "No alignment information for {:?} - does the target support that type?",
                    fpt
                )
            })
    }

    /// Alignment of aggregate types (structs, arrays)
    pub fn agg_alignment(&self) -> &Alignment {
        &self.agg_alignment
    }

    /// Alignment of function pointers
    pub fn fptr_alignment(&self) -> &FunctionPtrAlignment {
        &self.fptr_alignment
    }

    /// Alignment of (non-function-pointer) pointers in the given address space
    pub fn ptr_alignment(&self, addr_space: AddrSpace) -> &PointerLayout {
        match self.pointer_layouts.get(&addr_space) {
            Some(layout) => layout,
            None => self
                .pointer_layouts
                .get(&0)
                .expect("Should have a pointer layout for address space 0"),
        }
    }

    /// for internal use: size of an `FPType`, in bits
    fn fpt_size(fpt: FPType) -> u32 {
        match fpt {
            FPType::Half => 16,
            #[cfg(feature = "llvm-11-or-greater")]
            FPType::BFloat => 16,
            FPType::Single => 32,
            FPType::Double => 64,
            FPType::FP128 => 128,
            FPType::X86_FP80 => 80,
            FPType::PPC_FP128 => 128,
        }
    }
}

/// See [LLVM 14 docs on Data Layout](https://releases.llvm.org/14.0.0/docs/LangRef.html#data-layout)
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Mangling {
    ELF,
    MIPS,
    MachO,
    WindowsX86COFF,
    WindowsCOFF,
    #[cfg(feature = "llvm-11-or-greater")]
    XCOFF,
}

// ********* //
// from_llvm //
// ********* //

use crate::constant::Constant;
use crate::from_llvm::*;
use crate::function::AttributesData;
use llvm_sys::comdat::*;
use llvm_sys::{
    LLVMDLLStorageClass,
    LLVMLinkage,
    LLVMThreadLocalMode,
    LLVMUnnamedAddr,
    LLVMVisibility,
};

/// This struct contains data used when translating llvm-sys objects into our
/// data structures
pub(crate) struct ModuleContext<'a> {
    pub types: TypesBuilder,
    pub attrsdata: AttributesData,
    /// Map from an llvm-sys constant to the corresponding llvm-ir `ConstantRef`
    // We use LLVMValueRef as a *const, even though it's technically a *mut
    #[allow(clippy::mutable_key_type)]
    pub constants: HashMap<LLVMValueRef, ConstantRef>,
    /// Map from an llvm-sys global to its `Name`
    // We use LLVMValueRef as a *const, even though it's technically a *mut
    #[allow(clippy::mutable_key_type)]
    pub global_names: &'a HashMap<LLVMValueRef, Name>,
}

impl<'a> ModuleContext<'a> {
    // We use LLVMValueRef as a *const, even though it's technically a *mut
    #[allow(clippy::mutable_key_type)]
    fn new(global_names: &'a HashMap<LLVMValueRef, Name>) -> Self {
        Self {
            types: TypesBuilder::new(),
            attrsdata: AttributesData::create(),
            constants: HashMap::new(),
            global_names,
        }
    }
}

impl Module {
    pub(crate) fn from_llvm_ref(module: LLVMModuleRef) -> Self {
        debug!("Creating a Module from an LLVMModuleRef");
        let mut global_ctr = 0; // this ctr is used to number global objects that aren't named

        // Modules require two passes over their contents.
        // First we make a pass just to map global objects -- in particular,
        //   Functions, GlobalVariables, GlobalAliases, and GlobalIFuncs -- to
        //   Names; then we do the actual detailed pass.
        // This is necessary because these structures may reference each other in a
        //   circular fashion, and we need to be able to fill in the Name of the
        //   referenced object from having only its `LLVMValueRef`.
        // We use LLVMValueRef as a *const, even though it's technically a *mut
        #[allow(clippy::mutable_key_type)]
        let global_names: HashMap<LLVMValueRef, Name> = get_defined_functions(module)
            .chain(get_declared_functions(module))
            .chain(get_globals(module))
            .chain(get_global_aliases(module))
            .chain(get_global_ifuncs(module))
            .map(|g| {
                (
                    g,
                    Name::name_or_num(unsafe { get_value_name(g) }, &mut global_ctr),
                )
            })
            .collect();
        global_ctr = 0; // reset the global_ctr; the second pass should number everything exactly the same though

        let mut ctx = ModuleContext::new(&global_names);

        Self {
            name: unsafe { get_module_identifier(module) },
            source_file_name: unsafe { get_source_file_name(module) },
            data_layout: DataLayout::from_module_ref(module),
            target_triple: unsafe { get_target(module) },
            functions: get_defined_functions(module)
                .map(|f| Function::from_llvm_ref(f, &mut ctx))
                .collect(),
            func_declarations: get_declared_functions(module)
                .map(|f| FunctionDeclaration::from_llvm_ref(f, &mut ctx))
                .collect(),
            global_vars: get_globals(module)
                .map(|g| GlobalVariable::from_llvm_ref(g, &mut global_ctr, &mut ctx))
                .collect(),
            global_aliases: get_global_aliases(module)
                .map(|g| GlobalAlias::from_llvm_ref(g, &mut global_ctr, &mut ctx))
                .collect(),
            global_ifuncs: get_global_ifuncs(module)
                .map(|g| GlobalIFunc::from_llvm_ref(g, &mut global_ctr, &mut ctx))
                .collect(),
            // function_attribute_groups: unimplemented!("function_attribute_groups"),  // llvm-hs collects these in the decoder monad or something
            inline_assembly: unsafe { get_module_inline_asm(module) },
            // metadata_nodes: unimplemented!("metadata_nodes"),
            // named_metadatas: unimplemented!("named_metadatas"),
            // comdats: unimplemented!("comdats"),  // I think llvm-hs also collects these along the way
            types: ctx.types.build(),
        }
    }
}

impl GlobalVariable {
    pub(crate) fn from_llvm_ref(
        global: LLVMValueRef,
        ctr: &mut usize,
        ctx: &mut ModuleContext,
    ) -> Self {
        let ty = ctx.types.type_from_llvm_ref(unsafe { LLVMTypeOf(global) });
        let addr_space = match ty.as_ref() {
            Type::PointerType { addr_space, .. } => *addr_space,
            _ => panic!("GlobalVariable has a non-pointer type, {:?}", ty),
        };
        debug!("Processing a GlobalVariable with type {:?}", ty);
        Self {
            name: Name::name_or_num(unsafe { get_value_name(global) }, ctr),
            linkage: Linkage::from_llvm(unsafe { LLVMGetLinkage(global) }),
            visibility: Visibility::from_llvm(unsafe { LLVMGetVisibility(global) }),
            is_constant: unsafe { LLVMIsGlobalConstant(global) } != 0,
            ty,
            addr_space,
            dll_storage_class: DLLStorageClass::from_llvm(unsafe {
                LLVMGetDLLStorageClass(global)
            }),
            thread_local_mode: ThreadLocalMode::from_llvm(unsafe {
                LLVMGetThreadLocalMode(global)
            }),
            unnamed_addr: UnnamedAddr::from_llvm(unsafe { LLVMGetUnnamedAddress(global) }),
            initializer: {
                let it = unsafe { LLVMGetInitializer(global) };
                if it.is_null() {
                    None
                } else {
                    Some(Constant::from_llvm_ref(it, ctx))
                }
            },
            section: unsafe { get_section(global) },
            comdat: {
                let comdat = unsafe { LLVMGetComdat(global) };
                if comdat.is_null() {
                    None
                } else {
                    Some(Comdat::from_llvm_ref(unsafe { LLVMGetComdat(global) }))
                }
            },
            alignment: unsafe { LLVMGetAlignment(global) },
            debugloc: DebugLoc::from_llvm_no_col(global),
            // metadata: unimplemented!("metadata"),
        }
    }
}

impl GlobalAlias {
    pub(crate) fn from_llvm_ref(
        alias: LLVMValueRef,
        ctr: &mut usize,
        ctx: &mut ModuleContext,
    ) -> Self {
        let ty = ctx.types.type_from_llvm_ref(unsafe { LLVMTypeOf(alias) });
        let addr_space = match ty.as_ref() {
            Type::PointerType { addr_space, .. } => *addr_space,
            _ => panic!("GlobalAlias has a non-pointer type, {:?}", ty),
        };
        Self {
            name: Name::name_or_num(unsafe { get_value_name(alias) }, ctr),
            aliasee: Constant::from_llvm_ref(unsafe { LLVMAliasGetAliasee(alias) }, ctx),
            linkage: Linkage::from_llvm(unsafe { LLVMGetLinkage(alias) }),
            visibility: Visibility::from_llvm(unsafe { LLVMGetVisibility(alias) }),
            ty,
            addr_space,
            dll_storage_class: DLLStorageClass::from_llvm(unsafe { LLVMGetDLLStorageClass(alias) }),
            thread_local_mode: ThreadLocalMode::from_llvm(unsafe { LLVMGetThreadLocalMode(alias) }),
            unnamed_addr: UnnamedAddr::from_llvm(unsafe { LLVMGetUnnamedAddress(alias) }),
        }
    }
}

impl GlobalIFunc {
    pub(crate) fn from_llvm_ref(
        ifunc: LLVMValueRef,
        ctr: &mut usize,
        ctx: &mut ModuleContext,
    ) -> Self {
        Self {
            name: Name::name_or_num(unsafe { get_value_name(ifunc) }, ctr),
            linkage: Linkage::from_llvm(unsafe { LLVMGetLinkage(ifunc) }),
            visibility: Visibility::from_llvm(unsafe { LLVMGetVisibility(ifunc) }),
            ty: ctx.types.type_from_llvm_ref(unsafe { LLVMTypeOf(ifunc) }),
            resolver_fn: Constant::from_llvm_ref(unsafe { LLVMGetGlobalIFuncResolver(ifunc) }, ctx),
        }
    }
}

/* --TODO not yet implemented: metadata
impl NamedMetadata {
    pub(crate) fn from_llvm_ref(nm: LLVMNamedMDNodeRef) -> Self {
        unimplemented!("NamedMetadata::from_llvm_ref")
    }
}
*/

impl UnnamedAddr {
    pub(crate) fn from_llvm(ua: LLVMUnnamedAddr) -> Option<Self> {
        use LLVMUnnamedAddr::*;
        match ua {
            LLVMNoUnnamedAddr => None,
            LLVMLocalUnnamedAddr => Some(UnnamedAddr::Local),
            LLVMGlobalUnnamedAddr => Some(UnnamedAddr::Global),
        }
    }
}

impl Linkage {
    pub(crate) fn from_llvm(linkage: LLVMLinkage) -> Self {
        use LLVMLinkage::*;
        match linkage {
            LLVMExternalLinkage => Linkage::External,
            LLVMAvailableExternallyLinkage => Linkage::AvailableExternally,
            LLVMLinkOnceAnyLinkage => Linkage::LinkOnceAny,
            LLVMLinkOnceODRLinkage => Linkage::LinkOnceODR,
            LLVMLinkOnceODRAutoHideLinkage => Linkage::LinkOnceODRAutoHide,
            LLVMWeakAnyLinkage => Linkage::WeakAny,
            LLVMWeakODRLinkage => Linkage::WeakODR,
            LLVMAppendingLinkage => Linkage::Appending,
            LLVMInternalLinkage => Linkage::Internal,
            LLVMPrivateLinkage => Linkage::Private,
            LLVMDLLImportLinkage => Linkage::DLLImport,
            LLVMDLLExportLinkage => Linkage::DLLExport,
            LLVMExternalWeakLinkage => Linkage::ExternalWeak,
            LLVMGhostLinkage => Linkage::Ghost,
            LLVMCommonLinkage => Linkage::Common,
            LLVMLinkerPrivateLinkage => Linkage::LinkerPrivate,
            LLVMLinkerPrivateWeakLinkage => Linkage::LinkerPrivateWeak,
        }
    }
}

impl Visibility {
    pub(crate) fn from_llvm(visibility: LLVMVisibility) -> Self {
        use LLVMVisibility::*;
        match visibility {
            LLVMDefaultVisibility => Visibility::Default,
            LLVMHiddenVisibility => Visibility::Hidden,
            LLVMProtectedVisibility => Visibility::Protected,
        }
    }
}

impl DLLStorageClass {
    pub(crate) fn from_llvm(dllsc: LLVMDLLStorageClass) -> Self {
        use LLVMDLLStorageClass::*;
        match dllsc {
            LLVMDefaultStorageClass => DLLStorageClass::Default,
            LLVMDLLImportStorageClass => DLLStorageClass::Import,
            LLVMDLLExportStorageClass => DLLStorageClass::Export,
        }
    }
}

impl ThreadLocalMode {
    pub(crate) fn from_llvm(tlm: LLVMThreadLocalMode) -> Self {
        use LLVMThreadLocalMode::*;
        match tlm {
            LLVMNotThreadLocal => ThreadLocalMode::NotThreadLocal,
            LLVMGeneralDynamicTLSModel => ThreadLocalMode::GeneralDynamic,
            LLVMLocalDynamicTLSModel => ThreadLocalMode::LocalDynamic,
            LLVMInitialExecTLSModel => ThreadLocalMode::InitialExec,
            LLVMLocalExecTLSModel => ThreadLocalMode::LocalExec,
        }
    }
}

impl Comdat {
    pub(crate) fn from_llvm_ref(comdat: LLVMComdatRef) -> Self {
        Self {
            name: "error: not yet implemented: Comdat.name".to_owned(), // there appears to not be a getter for this in the LLVM C API?  I could be misunderstanding something
            selection_kind: SelectionKind::from_llvm(unsafe { LLVMGetComdatSelectionKind(comdat) }),
        }
    }
}

impl SelectionKind {
    pub(crate) fn from_llvm(sk: LLVMComdatSelectionKind) -> Self {
        use LLVMComdatSelectionKind::*;
        match sk {
            LLVMAnyComdatSelectionKind => SelectionKind::Any,
            LLVMExactMatchComdatSelectionKind => SelectionKind::ExactMatch,
            LLVMLargestComdatSelectionKind => SelectionKind::Largest,
            LLVMNoDuplicatesComdatSelectionKind => SelectionKind::NoDuplicates,
            LLVMSameSizeComdatSelectionKind => SelectionKind::SameSize,
        }
    }
}

impl Default for DataLayout {
    fn default() -> Self {
        Self {
            layout_str: String::new(),
            endianness: Endianness::BigEndian,
            stack_alignment: None,
            program_address_space: 0,
            alloca_address_space: 0,
            alignments: Alignments::default(),
            mangling: None,
            native_int_widths: None,
            non_integral_ptr_types: HashSet::new(),
        }
    }
}

impl DataLayout {
    pub(crate) fn from_module_ref(module: LLVMModuleRef) -> Self {
        let layout_str = unsafe { get_data_layout_str(module) };
        let mut data_layout = DataLayout {
            layout_str,
            ..Default::default()
        };
        for spec in data_layout.layout_str.split('-') {
            if spec == "E" {
                data_layout.endianness = Endianness::BigEndian;
            } else if spec == "e" {
                data_layout.endianness = Endianness::LittleEndian;
            } else if let Some(stripped) = spec.strip_prefix('S') {
                data_layout.stack_alignment =
                    Some(stripped.parse().expect("datalayout 'S': Failed to parse"));
            } else if let Some(stripped) = spec.strip_prefix('P') {
                data_layout.program_address_space =
                    stripped.parse().expect("datalayout 'P': Failed to parse");
            } else if let Some(stripped) = spec.strip_prefix('A') {
                data_layout.alloca_address_space =
                    stripped.parse().expect("datalayout 'A': Failed to parse");
            } else if spec.starts_with('p') {
                let mut chunks = spec.split(':');
                let first_chunk = chunks.next().unwrap();
                let addr_space: AddrSpace = if first_chunk == "p" {
                    0
                } else {
                    first_chunk[1 ..]
                        .parse()
                        .expect("datalayout 'p': Failed to parse address space")
                };
                let second_chunk = chunks
                    .next()
                    .expect("datalayout 'p' spec should have a size chunk");
                let size: u32 = second_chunk
                    .parse()
                    .expect("datalayout 'p': Failed to parse pointer size");
                let third_chunk = chunks
                    .next()
                    .expect("datalayout 'p' spec should have an abi chunk");
                let abi: u32 = third_chunk
                    .parse()
                    .expect("datalayout 'p': Failed to parse abi");
                let pref: u32 = if let Some(fourth_chunk) = chunks.next() {
                    fourth_chunk
                        .parse()
                        .expect("datalayout 'p': Failed to parse pref")
                } else {
                    abi
                };
                let idx: u32 = if let Some(fifth_chunk) = chunks.next() {
                    fifth_chunk
                        .parse()
                        .expect("datalayout 'p': Failed to parse idx")
                } else {
                    size
                };
                assert!(chunks.next().is_none(), "datalayout 'p': Too many chunks");
                data_layout.alignments.pointer_layouts.insert(
                    addr_space,
                    PointerLayout {
                        size,
                        alignment: Alignment { abi, pref },
                        index_size: idx,
                    },
                );
            } else if spec.starts_with('i') {
                let mut chunks = spec.split(':');
                let first_chunk = chunks.next().unwrap();
                let size: u32 = first_chunk[1 ..]
                    .parse()
                    .expect("datalayout 'i': Failed to parse size");
                let second_chunk = chunks
                    .next()
                    .expect("datalayout 'i' spec should have an abi chunk");
                let abi: u32 = second_chunk
                    .parse()
                    .expect("datalayout 'i': Failed to parse abi");
                let pref = if let Some(third_chunk) = chunks.next() {
                    third_chunk
                        .parse()
                        .expect("datalayout 'i': Failed to parse pref")
                } else {
                    abi
                };
                assert!(chunks.next().is_none(), "datalayout 'i': Too many chunks");
                data_layout
                    .alignments
                    .int_alignments
                    .insert(size, Alignment { abi, pref });
            } else if spec.starts_with('v') {
                let mut chunks = spec.split(':');
                let first_chunk = chunks.next().unwrap();
                let size: u32 = first_chunk[1 ..]
                    .parse()
                    .expect("datalayout 'v': Failed to parse size");
                let second_chunk = chunks
                    .next()
                    .expect("datalayout 'v' spec should have an abi chunk");
                let abi: u32 = second_chunk
                    .parse()
                    .expect("datalayout 'v': Failed to parse abi");
                let pref = if let Some(third_chunk) = chunks.next() {
                    third_chunk
                        .parse()
                        .expect("datalayout 'v': Failed to parse pref")
                } else {
                    abi
                };
                assert!(chunks.next().is_none(), "datalayout 'v': Too many chunks");
                data_layout
                    .alignments
                    .vec_alignments
                    .insert(size, Alignment { abi, pref });
            } else if spec.starts_with('f') {
                let mut chunks = spec.split(':');
                let first_chunk = chunks.next().unwrap();
                let size: u32 = first_chunk[1 ..]
                    .parse()
                    .expect("datalayout 'f': Failed to parse size");
                let second_chunk = chunks
                    .next()
                    .expect("datalayout 'f' spec should have an abi chunk");
                let abi: u32 = second_chunk
                    .parse()
                    .expect("datalayout 'f': Failed to parse abi");
                let pref = if let Some(third_chunk) = chunks.next() {
                    third_chunk
                        .parse()
                        .expect("datalayout 'f': Failed to parse pref")
                } else {
                    abi
                };
                assert!(chunks.next().is_none(), "datalayout 'f': Too many chunks");
                data_layout
                    .alignments
                    .fp_alignments
                    .insert(size, Alignment { abi, pref });
            } else if spec.starts_with('a') {
                let mut chunks = spec.split(':');
                let first_chunk = chunks.next().unwrap();
                assert!(first_chunk == "a" || first_chunk == "a0");
                let second_chunk = chunks
                    .next()
                    .expect("datalayout 'a' spec should have an abi chunk");
                let abi: u32 = second_chunk
                    .parse()
                    .expect("datalayout 'a': Failed to parse abi");
                let pref = if let Some(third_chunk) = chunks.next() {
                    third_chunk
                        .parse()
                        .expect("datalayout 'a': Failed to parse pref")
                } else {
                    abi
                };
                assert!(chunks.next().is_none(), "datalayout 'a': Too many chunks");
                data_layout.alignments.agg_alignment = Alignment { abi, pref };
            } else if let Some(stripped) = spec.strip_prefix("Fi") {
                let abi: u32 = stripped
                    .parse()
                    .expect("datalayout 'Fi': Failed to parse abi");
                data_layout.alignments.fptr_alignment = FunctionPtrAlignment {
                    independent: true,
                    abi,
                };
                data_layout.alignments.fptr_alignment_as_alignment =
                    Alignment { abi, pref: abi };
            } else if let Some(stripped) = spec.strip_prefix("Fn") {
                let abi: u32 = stripped
                    .parse()
                    .expect("datalayout 'Fn': Failed to parse abi");
                data_layout.alignments.fptr_alignment = FunctionPtrAlignment {
                    independent: false,
                    abi,
                };
                data_layout.alignments.fptr_alignment_as_alignment =
                    Alignment { abi, pref: abi };
            } else if spec.starts_with('m') {
                let mut chunks = spec.split(':');
                let first_chunk = chunks.next().unwrap();
                assert_eq!(first_chunk, "m");
                let second_chunk = chunks
                    .next()
                    .expect("datalayout 'm' spec should have a mangling chunk");
                let mangling = match second_chunk {
                    "e" => Mangling::ELF,
                    "m" => Mangling::MIPS,
                    "o" => Mangling::MachO,
                    "x" => Mangling::WindowsX86COFF,
                    "w" => Mangling::WindowsCOFF,
                    #[cfg(feature = "llvm-11-or-greater")]
                    "a" => Mangling::XCOFF,
                    _ => panic!("datalayout 'm': Unknown mangling {:?}", second_chunk),
                };
                assert!(chunks.next().is_none(), "datalayout 'm': Too many chunks");
                data_layout.mangling = Some(mangling);
            } else if spec.starts_with("ni") {
                let mut chunks = spec.split(':');
                let first_chunk = chunks.next().unwrap();
                assert_eq!(first_chunk, "ni");
                for chunk in chunks {
                    let addr_space: AddrSpace = chunk
                        .parse()
                        .expect("datalayout 'ni': Failed to parse addr space");
                    assert_ne!(addr_space, 0, "LLVM spec does not allow address space 0 to have non-integral pointer types");
                    data_layout.non_integral_ptr_types.insert(addr_space);
                }
            } else if spec.starts_with('n') {
                let native_int_widths = data_layout
                    .native_int_widths
                    .get_or_insert_with(HashSet::new);
                let mut chunks = spec.split(':');
                let first_chunk = chunks.next().unwrap();
                let size = first_chunk[1 ..]
                    .parse()
                    .expect("datalayout 'n': Failed to parse first size");
                native_int_widths.insert(size);
                for chunk in chunks {
                    let size = chunk.parse().expect("datalayout 'n': Failed to parse size");
                    native_int_widths.insert(size);
                }
            } else if spec.is_empty() {
                // do nothing
            } else {
                panic!("datalayout: Unknown spec {:?}", spec);
            }
        }
        data_layout
    }
}

impl Default for Alignments {
    fn default() -> Self {
        Self {
            // Explicit alignments for various sizes of integers (in bits). Sizes not
            // specified here are determined according to the rules described in the
            // Data Layout docs.
            int_alignments: vec![
                (1, Alignment { abi: 8, pref: 8 }),
                (8, Alignment { abi: 8, pref: 8 }),
                (16, Alignment { abi: 16, pref: 16 }),
                (32, Alignment { abi: 32, pref: 32 }),
                (64, Alignment { abi: 32, pref: 64 }),
            ]
            .into_iter()
            .collect(),
            // Explicit alignments for various sizes of vectors (in bits). Sizes not
            // specified here are determined according to the rules described in the
            // Data Layout docs.
            vec_alignments: vec![
                (64, Alignment { abi: 64, pref: 64 }),
                (
                    128,
                    Alignment {
                        abi: 128,
                        pref: 128,
                    },
                ),
            ]
            .into_iter()
            .collect(),
            // Alignment for floating-point types, by size (in bits)
            fp_alignments: vec![
                (16, Alignment { abi: 16, pref: 16 }),
                (32, Alignment { abi: 32, pref: 32 }),
                (64, Alignment { abi: 64, pref: 64 }),
                (
                    128,
                    Alignment {
                        abi: 128,
                        pref: 128,
                    },
                ),
            ]
            .into_iter()
            .collect(),
            // Alignment for aggregate types (structs, arrays)
            agg_alignment: Alignment { abi: 0, pref: 64 },
            // Alignment for function pointers
            fptr_alignment: FunctionPtrAlignment {
                independent: true,
                abi: 64,
            },
            // Alignment for function pointers, as an `Alignment`
            fptr_alignment_as_alignment: Alignment { abi: 64, pref: 64 },
            // Layout details for (non-function-pointer) pointers, by address space
            pointer_layouts: vec![(
                0,
                PointerLayout {
                    size: 64,
                    alignment: Alignment { abi: 64, pref: 64 },
                    index_size: 64,
                },
            )]
            .into_iter()
            .collect(),
        }
    }
}
