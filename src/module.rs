use crate::constant::ConstantRef;
use crate::debugloc::*;
use crate::function::{Function, FunctionAttribute, GroupID};
use crate::name::Name;
use crate::types::{Type, TypeRef, Typed, Types, TypesBuilder};
use std::path::Path;

/// See [LLVM 10 docs on Module Structure](https://releases.llvm.org/10.0.0/docs/LangRef.html#module-structure)
#[derive(Clone)]
pub struct Module {
    /// The name of the module
    pub name: String,
    /// See [LLVM 10 docs on Source Filename](https://releases.llvm.org/10.0.0/docs/LangRef.html#source-filename)
    pub source_file_name: String,
    /// See [LLVM 10 docs on Data Layout](https://releases.llvm.org/10.0.0/docs/LangRef.html#data-layout)
    pub data_layout: String, // llvm-hs parses this String into Option<DataLayout> with a custom parser
    /// See [LLVM 10 docs on Target Triple](https://releases.llvm.org/10.0.0/docs/LangRef.html#target-triple)
    pub target_triple: Option<String>,
    /// Functions which are defined (not just declared) in this `Module`.
    /// See [LLVM 10 docs on Functions](https://releases.llvm.org/10.0.0/docs/LangRef.html#functions)
    pub functions: Vec<Function>,
    /// See [LLVM 10 docs on Global Variables](https://releases.llvm.org/10.0.0/docs/LangRef.html#global-variables)
    pub global_vars: Vec<GlobalVariable>,
    /// See [LLVM 10 docs on Global Aliases](https://releases.llvm.org/10.0.0/docs/LangRef.html#aliases)
    pub global_aliases: Vec<GlobalAlias>,
    // --TODO not yet implemented-- pub function_attribute_groups: Vec<FunctionAttributeGroup>,
    /// See [LLVM 10 docs on Module-Level Inline Assembly](https://releases.llvm.org/10.0.0/docs/LangRef.html#moduleasm)
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

    /// Get the `Function` having the given `Name` (if any).
    /// Note that `Function`s are named with `String`s and not `Name`s.
    pub fn get_func_by_name(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|func| func.name == name)
    }

    /// Parse the LLVM bitcode (.bc) file at the given path to create a `Module`
    pub fn from_bc_path(path: impl AsRef<Path>) -> Result<Self, String> {
        // implementation here inspired by the `inkwell` crate's `Module::parse_bitcode_from_path`
        use std::ffi::{CStr, CString};
        use std::mem;

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

        let context = crate::from_llvm::Context::new();

        use llvm_sys::bit_reader::LLVMParseBitcodeInContext2;
        let module = unsafe {
            let mut module: mem::MaybeUninit<LLVMModuleRef> = mem::MaybeUninit::uninit();
            let return_code =
                LLVMParseBitcodeInContext2(context.ctx, memory_buffer, module.as_mut_ptr());
            LLVMDisposeMemoryBuffer(memory_buffer);
            if return_code != 0 {
                return Err("Failed to parse bitcode".to_string());
            }
            module.assume_init()
        };
        debug!("Parsed bitcode to llvm_sys module");
        Ok(Self::from_llvm_ref(module))
    }
}

/// See [LLVM 10 docs on Global Variables](https://releases.llvm.org/10.0.0/docs/LangRef.html#global-variables)
#[derive(PartialEq, Clone, Debug)]
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

/// See [LLVM 10 docs on Global Aliases](https://releases.llvm.org/10.0.0/docs/LangRef.html#aliases)
#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum UnnamedAddr {
    Local,
    Global,
}

/// See [LLVM 10 docs on Linkage Types](https://releases.llvm.org/10.0.0/docs/LangRef.html#linkage)
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

/// See [LLVM 10 docs on Visibility Styles](https://releases.llvm.org/10.0.0/docs/LangRef.html#visibility-styles)
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Visibility {
    Default,
    Hidden,
    Protected,
}

/// See [LLVM 10 docs on DLL Storage Classes](https://releases.llvm.org/10.0.0/docs/LangRef.html#dllstorageclass)
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DLLStorageClass {
    Default,
    Import,
    Export,
}

/// See [LLVM 10 docs on Thread Local Storage Models](https://releases.llvm.org/10.0.0/docs/LangRef.html#thread-local-storage-models)
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ThreadLocalMode {
    NotThreadLocal,
    GeneralDynamic,
    LocalDynamic,
    InitialExec,
    LocalExec,
}

/// For discussion of address spaces, see [LLVM 10 docs on Pointer Type](https://releases.llvm.org/10.0.0/docs/LangRef.html#pointer-type)
pub type AddrSpace = u32;

/// See [LLVM 10 docs on Attribute Groups](https://releases.llvm.org/10.0.0/docs/LangRef.html#attribute-groups)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FunctionAttributeGroup {
    pub group_id: GroupID,
    pub attrs: Vec<FunctionAttribute>,
}

/* --TODO not yet implemented: metadata
/// See [LLVM 10 docs on Named Metadata](https://releases.llvm.org/10.0.0/docs/LangRef.html#named-metadata)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct NamedMetadata {
    pub name: String,
    pub node_ids: Vec<MetadataNodeID>,
}
*/

/// See [LLVM 10 docs on Comdats](https://releases.llvm.org/10.0.0/docs/LangRef.html#langref-comdats)
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Comdat {
    pub name: String,
    pub selection_kind: SelectionKind,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SelectionKind {
    Any,
    ExactMatch,
    Largest,
    NoDuplicates,
    SameSize,
}

/* llvm-hs parses the data_layout into basically this structure

#[derive(Clone, Debug)]
pub struct DataLayout {
    pub endianness: Endianness,
    pub mangling: Option<Mangling>,
    pub stack_alignment: Option<u32>,
    pub pointer_layouts: HashMap<AddrSpace, (u32, AlignmentInfo)>,
    pub type_layouts: HashMap<(AlignType, u32), AlignmentInfo>,
    pub aggregate_layout: AlignmentInfo,
    pub native_sizes: Option<HashSet<u32>>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Mangling {
    ELF,
    MIPS,
    MachO,
    WindowsCOFF,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct AlignmentInfo {
    pub abi_alignment: u32,
    pub preferred_alignment: u32,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum AlignType {
    Integer,
    Vector,
    Float,
}

*/

// ********* //
// from_llvm //
// ********* //

use crate::constant::Constant;
use crate::function::AttributesData;
use crate::from_llvm::*;
use llvm_sys::comdat::*;
use llvm_sys::{
    LLVMDLLStorageClass,
    LLVMLinkage,
    LLVMThreadLocalMode,
    LLVMUnnamedAddr,
    LLVMVisibility,
};
use std::collections::HashMap;

/// This struct contains data used when translating llvm-sys objects into our
/// data structures
pub(crate) struct ModuleContext<'a> {
    pub types: TypesBuilder,
    pub attrsdata: AttributesData,
    /// Map from an llvm-sys constant to the corresponding llvm-ir `ConstantRef`
    pub constants: HashMap<LLVMValueRef, ConstantRef>,
    /// Map from an llvm-sys global to its `Name`
    pub global_names: &'a HashMap<LLVMValueRef, Name>,
}

impl<'a> ModuleContext<'a> {
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
        // First we make a pass just to map global objects -- in particular, Functions,
        //   GlobalVariables, and GlobalAliases -- to Names; then we do the actual
        //   detailed pass.
        // This is necessary because these structures may reference each other in a
        //   circular fashion, and we need to be able to fill in the Name of the
        //   referenced object from having only its `LLVMValueRef`.
        let global_names: HashMap<LLVMValueRef, Name> = get_defined_functions(module)
            .chain(get_declared_functions(module))
            .chain(get_globals(module))
            .chain(get_global_aliases(module))
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
            data_layout: unsafe { get_data_layout_str(module) },
            target_triple: unsafe { get_target(module) },
            functions: get_defined_functions(module)
                .map(|f| Function::from_llvm_ref(f, &mut ctx))
                .collect(),
            global_vars: get_globals(module)
                .map(|g| GlobalVariable::from_llvm_ref(g, &mut global_ctr, &mut ctx))
                .collect(),
            global_aliases: get_global_aliases(module)
                .map(|g| GlobalAlias::from_llvm_ref(g, &mut global_ctr, &mut ctx))
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
