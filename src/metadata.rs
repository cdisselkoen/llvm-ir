//! See [LLVM 14 docs on Metadata](https://releases.llvm.org/14.0.0/docs/LangRef.html#metadata)

use either::Either;
use std::fmt::Debug;

#[cfg(feature = "llvm-20-or-greater")]
use crate::from_llvm::print_to_string;
#[cfg(feature = "llvm-20-or-greater")]
use crate::llvm_sys::*;
#[cfg(feature = "llvm-20-or-greater")]
use crate::llvm_sys::llvm_sys::LLVMValueKind;
#[cfg(feature = "llvm-20-or-greater")]
use crate::llvm_sys::llvm_sys::debuginfo::{LLVMGetMetadataKind, LLVMMetadataKind};

use crate::operand::Operand;
use crate::types::{TypeRef, Typed, Types};

#[cfg(feature = "llvm-20-or-greater")]
#[derive(PartialEq, Clone, Debug, Hash)]
pub enum MetadataValue {
    String(String),
    Node(Vec<Option<MetadataValue>>),
    Value(String),
}

#[cfg(feature = "llvm-20-or-greater")]
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct MetadataAttachment {
    pub kind_id: u32,
    pub value: MetadataValue,
}

#[cfg(feature = "llvm-20-or-greater")]
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct NamedMetadata {
    pub name: String,
    pub operands: Vec<MetadataValue>,
}

#[cfg(feature = "llvm-20-or-greater")]
#[derive(PartialEq, Clone, Debug, Hash, Default)]
pub struct InstructionMetadata(pub Vec<MetadataAttachment>);

#[cfg(feature = "llvm-20-or-greater")]
impl InstructionMetadata {
    pub(crate) fn from_llvm_inst(inst: LLVMValueRef) -> Self {
        unsafe {
            let ctx = metadata_context_from_inst(inst);
            let mut count: ::libc::size_t = 0;
            let entries = LLVMInstructionGetAllMetadataOtherThanDebugLoc(inst, &mut count);
            if entries.is_null() || count == 0 {
                return InstructionMetadata(Vec::new());
            }
            let mut metadata = Vec::with_capacity(count as usize);
            for idx in 0..count {
                let kind_id = LLVMValueMetadataEntriesGetKind(entries, idx as u32);
                let md = LLVMValueMetadataEntriesGetMetadata(entries, idx as u32);
                if md.is_null() {
                    continue;
                }
                let value = metadata_value_from_metadata(ctx, md);
                metadata.push(MetadataAttachment { kind_id, value });
            }
            LLVMDisposeValueMetadataEntries(entries);
            InstructionMetadata(metadata)
        }
    }
}

#[cfg(feature = "llvm-20-or-greater")]
pub fn md_kind_id(name: &str) -> u32 {
    let cstr = std::ffi::CString::new(name)
        .expect("Failed to build CString for metadata kind name");
    unsafe { LLVMGetMDKindID(cstr.as_ptr(), name.len() as ::libc::c_uint) }
}

#[cfg(feature = "llvm-20-or-greater")]
pub(crate) unsafe fn named_metadata_from_module(module: LLVMModuleRef) -> Vec<NamedMetadata> {
    let ctx = LLVMGetModuleContext(module);
    let mut named = Vec::new();
    let mut node = LLVMGetFirstNamedMetadata(module);
    while !node.is_null() {
        let mut name_len: ::libc::size_t = 0;
        let name_ptr = LLVMGetNamedMetadataName(node, &mut name_len);
        let name = metadata_string_from_ptr(name_ptr, name_len);
        let name_cstr = std::ffi::CString::new(name.clone())
            .expect("Failed to build CString for named metadata");
        let count = LLVMGetNamedMetadataNumOperands(module, name_cstr.as_ptr());
        let mut values: Vec<LLVMValueRef> = vec![std::ptr::null_mut(); count as usize];
        if count > 0 {
            LLVMGetNamedMetadataOperands(module, name_cstr.as_ptr(), values.as_mut_ptr());
        }
        let operands = values
            .into_iter()
            .filter(|val| !val.is_null())
            .map(|val| metadata_value_from_value(ctx, val))
            .collect();
        named.push(NamedMetadata { name, operands });
        node = LLVMGetNextNamedMetadata(node);
    }
    named
}

#[cfg(feature = "llvm-20-or-greater")]
pub(crate) unsafe fn metadata_value_from_metadata(
    ctx: LLVMContextRef,
    md: LLVMMetadataRef,
) -> MetadataValue {
    let val = LLVMMetadataAsValue(ctx, md);
    metadata_value_from_value(ctx, val)
}

#[cfg(feature = "llvm-20-or-greater")]
pub(crate) unsafe fn metadata_value_from_value(
    ctx: LLVMContextRef,
    val: LLVMValueRef,
) -> MetadataValue {
    if val.is_null() {
        return MetadataValue::Value("<null>".into());
    }
    if LLVMGetValueKind(val) != LLVMValueKind::LLVMMetadataAsValueValueKind {
        return MetadataValue::Value(print_to_string(val));
    }
    let md = LLVMValueAsMetadata(val);
    if md.is_null() {
        return MetadataValue::Value(print_to_string(val));
    }
    match LLVMGetMetadataKind(md) {
        LLVMMetadataKind::LLVMMDStringMetadataKind => MetadataValue::String(md_string_from_value(val)),
        LLVMMetadataKind::LLVMMDTupleMetadataKind => {
            let count = LLVMGetMDNodeNumOperands(val);
            let mut values: Vec<LLVMValueRef> = vec![std::ptr::null_mut(); count as usize];
            if count > 0 {
                LLVMGetMDNodeOperands(val, values.as_mut_ptr());
            }
            let elems = values
                .into_iter()
                .map(|op| {
                    if op.is_null() {
                        None
                    } else {
                        Some(metadata_value_from_value(ctx, op))
                    }
                })
                .collect();
            MetadataValue::Node(elems)
        },
        _ => MetadataValue::Value(print_to_string(val)),
    }
}

#[cfg(feature = "llvm-20-or-greater")]
unsafe fn md_string_from_value(val: LLVMValueRef) -> String {
    let mut len: ::libc::c_uint = 0;
    let ptr = LLVMGetMDString(val, &mut len);
    metadata_string_from_ptr(ptr, len as ::libc::size_t)
}

#[cfg(feature = "llvm-20-or-greater")]
unsafe fn metadata_string_from_ptr(ptr: *const ::libc::c_char, len: ::libc::size_t) -> String {
    if ptr.is_null() || len == 0 {
        return String::new();
    }
    let bytes = std::slice::from_raw_parts(ptr as *const u8, len as usize);
    String::from_utf8_lossy(bytes).into_owned()
}

#[cfg(feature = "llvm-20-or-greater")]
pub(crate) unsafe fn metadata_context_from_inst(inst: LLVMValueRef) -> LLVMContextRef {
    let bb = LLVMGetInstructionParent(inst);
    let func = LLVMGetBasicBlockParent(bb);
    let module = LLVMGetGlobalParent(func);
    LLVMGetModuleContext(module)
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum MetadataRef<T> where T: PartialEq + Clone + Debug {
    Ref(MetadataNodeID),
    Inline(Box<T>),
}

pub type MetadataNodeID = usize;

/// See [LLVM 14 docs on Metadata Nodes and Metadata Strings](https://releases.llvm.org/14.0.0/docs/LangRef.html#metadata-nodes-and-metadata-strings)
#[derive(PartialEq, Clone, Debug, Hash)]
pub enum Metadata {
    String(String),
    Node(MetadataRef<MetadataNode>),
    Value(Operand),
}

impl Typed for Metadata {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.metadata_type()
    }
}

/// See [LLVM 14 docs on Metadata Nodes and Metadata Strings](https://releases.llvm.org/14.0.0/docs/LangRef.html#metadata-nodes-and-metadata-strings)
#[derive(PartialEq, Clone, Debug, Hash)]
pub enum MetadataNode {
    Tuple(Vec<Option<Metadata>>),  // None represents null
    Expression(DIExpression),
    GlobalVariableExpression(DIGlobalVariableExpression),
    Location(DILocation),
    MacroNode(DIMacroNode),
    Node(DINode),
}

// DI* types are in alphabetical order in this file

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum DIAccessibility {
    Private,
    Protected,
    Public,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DIArrayType {
    pub subscripts: Vec<DISubrange>,
    pub element_type: Option<MetadataRef<DIType>>,
    pub size_in_bits: u64,
    pub align_in_bits: u32,
    pub flags: Vec<DIFlag>,
}

/// See [LLVM 14 docs on DIBasicType](https://releases.llvm.org/14.0.0/docs/LangRef.html#dibasictype)
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct DIBasicType {
    pub name: String,
    pub size_in_bits: u64,
    pub align_in_bits: u32,
    pub encoding: Option<Encoding>,
    pub tag: DIBasicTypeTag,
    pub flags: Vec<DIFlag>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum DIBasicTypeTag {
    BaseType,
    UnspecifiedType,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct DIChecksumInfo {
    pub kind: DIChecksumKind,
    pub value: String,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum DIChecksumKind {
    MD5,
    SHA1,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DIClassType {
    pub name: String,
    pub scope: Option<MetadataRef<DIScope>>,
    pub file: Option<MetadataRef<DIFile>>,
    pub line: u32,
    pub derived_from: Option<MetadataRef<DIType>>,
    pub elements: Vec<MetadataRef<Either<DIDerivedType,  DISubprogram>>>,
    pub vtable_holder: Option<MetadataRef<DIType>>,
    pub template_params: Vec<DITemplateParameter>,
    pub identifier: String,
    pub size_in_bits: u64,
    pub align_in_bits: u32,
    pub flags: Vec<DIFlag>,
}

/// See [LLVM 14 docs on DICompileUnit](https://releases.llvm.org/14.0.0/docs/LangRef.html#dicompileunit)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DICompileUnit {
    pub language: u32,
    pub file: MetadataRef<DIFile>,
    pub producer: String,
    pub optimized: bool,
    pub flags: String,
    pub runtime_version: u32,
    pub split_debug_filename: String,
    pub emission_kind: DIDebugEmissionKind,
    pub enums: Vec<MetadataRef<DIEnumerationType>>,
    pub retained_types: Vec<MetadataRef<Either<DIType, DISubprogram>>>,
    pub globals: Vec<MetadataRef<DIGlobalVariableExpression>>,
    pub imports: Vec<MetadataRef<DIImportedEntity>>,
    pub macros: Vec<MetadataRef<DIMacroNode>>,
    pub dwoid: u64,
    pub split_debug_inlining: bool,
    pub debug_info_for_profiling: bool,
    pub name_table_kind: DIDebugNameTableKind,
    pub debug_base_address: bool,
}

/// See [LLVM 14 docs on DICompositeType](https://releases.llvm.org/14.0.0/docs/LangRef.html#dicompositetype)
#[derive(PartialEq, Clone, Debug, Hash)]
pub enum DICompositeType {
    Array(DIArrayType),
    Class(DIClassType),
    Enumeration(DIEnumerationType),
    Structure(DIStructureType),
    Union(DIUnionType),
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum DICount {
    Constant(i64),
    Variable(MetadataRef<DIVariable>),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum DIDebugEmissionKind {
    NoDebug,
    FullDebug,
    LineTablesOnly,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum DIDebugNameTableKind {
    Default,
    GNU,
    None,
}

/// See [LLVM 14 docs on DIDerivedType](https://releases.llvm.org/14.0.0/docs/LangRef.html#diderivedtype)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DIDerivedType {
    pub tag: DIDerivedTypeTag,
    pub name: String,
    pub file: Option<MetadataRef<DIFile>>,
    pub line: u32,
    pub scope: Option<MetadataRef<DIScope>>,
    pub base_type: Option<MetadataRef<DIType>>,  // `None` would represent `void*`
    pub size_in_bits: u64,
    pub align_in_bits: u32,
    pub offset_in_bits: u64,
    pub address_space: Option<u32>,
    pub flags: Vec<DIFlag>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum DIDerivedTypeTag {
    Typedef,
    PointerType,
    PtrToMemberType,
    ReferenceType,
    RValueReferenceType,
    ConstType,
    VolatileType,
    RestrictType,
    AtomicType,
    Member,
    Inheritance,
    Friend,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DIEnumerationType {
    pub name: String,
    pub scope: Option<MetadataRef<DIScope>>,
    pub file: Option<MetadataRef<DIFile>>,
    pub line: u32,
    pub values: Vec<DIEnumerator>,
    pub base_type: Option<MetadataRef<DIType>>,
    pub identifier: String,
    pub size_in_bits: u64,
    pub align_in_bits: u32,
}

/// See [LLVM 14 docs on DIEnumerator](https://releases.llvm.org/14.0.0/docs/LangRef.html#dienumerator)
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct DIEnumerator {
    pub name: String,
    pub value: i64,
    pub is_unsigned: bool,
}

/// See [LLVM 14 docs on DIExpression](https://releases.llvm.org/14.0.0/docs/LangRef.html#diexpression)
pub type DIExpression = Vec<DWOp>;

/// See [LLVM 14 docs on DIFile](https://releases.llvm.org/14.0.0/docs/LangRef.html#difile)
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct DIFile {
    pub filename: String,
    pub directory: String,
    pub checksum: Option<DIChecksumInfo>,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum DIFlag {
    Accessibility(DIAccessibility),
    FwdDecl,
    AppleBlock,
    BlockByrefStruct,
    VirtualFlag,
    Artificial,
    Explicit,
    Prototyped,
    ObjcClassComplete,
    ObjectPointer,
    Vector,
    StaticMember,
    LValueReference,
    RValueReference,
    InheritanceFlag(DIInheritance),
    IntroducedVirtual,
    BitField,
    NoReturn,
    MainSubprogram,
}

/// See [LLVM 14 docs on DIGlobalVariable](https://releases.llvm.org/14.0.0/docs/LangRef.html#diglobalvariable)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DIGlobalVariable {
    pub name: String,
    pub linkage_name: String,
    pub scope: Option<MetadataRef<DIScope>>,
    pub file: Option<MetadataRef<DIFile>>,
    pub line: u32,
    pub ty: Option<MetadataRef<DIType>>,
    pub local: bool,
    pub definition: bool,
    pub static_data_member_declaration: Option<MetadataRef<DIDerivedType>>,
    pub template_params: Vec<MetadataRef<DITemplateParameter>>,
    pub align_in_bits: u32,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DIGlobalVariableExpression {
    pub var: MetadataRef<DIGlobalVariable>,
    pub expr: MetadataRef<DIExpression>,
}

/// See [LLVM 14 docs on DIImportedEntity](https://releases.llvm.org/14.0.0/docs/LangRef.html#diimportedentity)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DIImportedEntity {
    pub tag: DIImportedEntityTag,
    pub name: String,
    pub scope: MetadataRef<DIScope>,
    pub entity: Option<MetadataRef<DINode>>,
    pub file: Option<MetadataRef<DIFile>>,
    pub line: u32,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum DIImportedEntityTag {
    Module,
    Declaration,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum DIInheritance {
    SingleInheritance,
    MultipleInheritance,
    VirtualInheritance,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum DILexicalBlockBase {
    LexicalBlock(DILexicalBlock),
    LexicalBlockFile(DILexicalBlockFile),
}

/// See [LLVM 14 docs on DILexicalBlock](https://releases.llvm.org/14.0.0/docs/LangRef.html#dilexicalblock)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DILexicalBlock {
    pub scope: MetadataRef<DILocalScope>,
    pub file: Option<MetadataRef<DIFile>>,
    pub line: u32,
    pub column: u32,
}

/// See [LLVM 14 docs on DILexicalBlockFile](https://releases.llvm.org/14.0.0/docs/LangRef.html#dilexicalblockfile)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DILexicalBlockFile {
    pub scope: MetadataRef<DILocalScope>,
    pub file: Option<MetadataRef<DIFile>>,
    pub discriminator: u32,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum DILocalScope {
    LexicalBlockBase(DILexicalBlockBase),
    Subprogram(DISubprogram),
}

/// See [LLVM 14 docs on DILocalVariable](https://releases.llvm.org/14.0.0/docs/LangRef.html#dilocalvariable)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DILocalVariable {
    pub name: String,
    pub scope: MetadataRef<DIScope>,
    pub file: Option<MetadataRef<DIFile>>,
    pub line: u32,
    pub ty: Option<MetadataRef<DIType>>,
    pub flags: Vec<DIFlag>,
    pub arg: u16,
    pub align_in_bits: u32,
}

/// See [LLVM 14 docs on DILocation](https://releases.llvm.org/14.0.0/docs/LangRef.html#dilocation)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DILocation {
    pub line: u32,
    pub column: u32,
    pub scope: MetadataRef<DILocalScope>,
}

/// See LLVM 14 docs on [DIMacro](https://releases.llvm.org/14.0.0/docs/LangRef.html#dimacro) and
/// [DIMacroFile](https://releases.llvm.org/14.0.0/docs/LangRef.html#dimacrofile)
#[derive(PartialEq, Clone, Debug, Hash)]
pub enum DIMacroNode {
    Macro { name: String, value: String, info: DIMacroInfo, line: u32 },
    MacroFile { file: MetadataRef<DIFile>, elements: Vec<MetadataRef<DIMacroNode>>, line: u32 },
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum DIMacroInfo {
    Define,
    Undef,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DIModule {
    pub name: String,
    pub scope: Option<MetadataRef<DIScope>>,
    pub configuration_macros: String,
    pub include_path: String,
    pub isys_root: String,
}

/// See [LLVM 14 docs on DINamespace](https://releases.llvm.org/14.0.0/docs/LangRef.html#dinamespace)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DINamespace {
    pub name: String,
    pub scope: Option<MetadataRef<DIScope>>,
    pub export_symbols: bool,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum DINode {
    Enumerator(DIEnumerator),
    ImportedEntity(DIImportedEntity),
    ObjCProperty(DIObjCProperty),
    Scope(DIScope),
    Subrange(DISubrange),
    TemplateParameter(DITemplateParameter),
    Variable(DIVariable),
}

/// See [LLVM 14 docs on DIObjCProperty](https://releases.llvm.org/14.0.0/docs/LangRef.html#diobjcproperty)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DIObjCProperty {
    pub name: String,
    pub file: Option<MetadataRef<DIFile>>,
    pub line: u32,
    pub getter_name: String,
    pub setter_name: String,
    pub attributes: u32,
    pub ty: Option<MetadataRef<DIType>>,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum DIScope {
    CompileUnit(DICompileUnit),
    File(DIFile),
    LocalScope(DILocalScope),
    Module(DIModule),
    Namespace(DINamespace),
    Type(DIType),
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DIStructureType {
    pub name: String,
    pub scope: Option<MetadataRef<DIScope>>,
    pub file: Option<MetadataRef<DIFile>>,
    pub line: u32,
    pub flags: Vec<DIFlag>,
    pub derived_from: Option<MetadataRef<DIType>>,
    pub elements: Vec<MetadataRef<Either<DIDerivedType, DISubprogram>>>,
    pub runtime_lang: u16,
    pub vtable_holder: Option<MetadataRef<DIType>>,
    pub identifier: String,
    pub size_in_bits: u64,
    pub align_in_bits: u32,
}

/// See [LLVM 14 docs on DISubprogram](https://releases.llvm.org/14.0.0/docs/LangRef.html#disubprogram)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DISubprogram {
    pub name: String,
    pub linkage_name: String,
    pub scope: Option<MetadataRef<DIScope>>,
    pub file: Option<MetadataRef<DIFile>>,
    pub line: u32,
    pub subroutine_type: Option<MetadataRef<DISubroutineType>>,
    pub local_to_unit: bool,
    pub definition: bool,
    pub scope_line: u32,
    pub containing_type: Option<MetadataRef<DIType>>,
    pub virtuality: Virtuality,
    pub virtuality_index: u32,
    pub this_adjustment: i32,
    pub flags: Vec<DIFlag>,
    pub optimized: bool,
    pub unit: Option<MetadataRef<DICompileUnit>>,
    pub template_params: Vec<MetadataRef<DITemplateParameter>>,
    pub declaration: Option<MetadataRef<DISubprogram>>,
    pub retained_nodes: Vec<MetadataRef<DILocalVariable>>,
    pub thrown_types: Vec<MetadataRef<DIType>>,
}

/// See [LLVM 14 docs on DISubrange](https://releases.llvm.org/14.0.0/docs/LangRef.html#disubrange)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DISubrange {
    pub count: DICount,
    pub lower_bound: i64,
}

/// See [LLVM 14 docs on DISubroutineType](https://releases.llvm.org/14.0.0/docs/LangRef.html#disubroutinetype)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DISubroutineType {
    /// First the return type, then the operand types. `None` means `void`.
    pub type_array: Vec<Option<MetadataRef<DIType>>>,
    pub cc: u8,
    pub flags: Vec<DIFlag>,
}

/// See LLVM 14 docs on [DITemplateTypeParameter](https://releases.llvm.org/14.0.0/docs/LangRef.html#ditemplatetypeparameter)
/// and [DITemplateValueParameter](https://releases.llvm.org/14.0.0/docs/LangRef.html#ditemplatevalueparameter)
#[derive(PartialEq, Clone, Debug, Hash)]
pub enum DITemplateParameter {
    TypeParameter { name: String, ty: Option<MetadataRef<DIType>> },
    ValueParameter { name: String, ty: Option<MetadataRef<DIType>>, value: Option<Box<Metadata>>, tag: DITemplateValueParameterTag },
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum DITemplateValueParameterTag {
    TemplateValueParameter,
    GNUTemplateTemplateParam,
    GNUTemplateParameterPack,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum DIType {
    Basic(DIBasicType),
    Composite(DICompositeType),
    Derived(DIDerivedType),
    Subroutine(DISubroutineType),
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct DIUnionType {
    pub name: String,
    pub scope: Option<MetadataRef<DIScope>>,
    pub file: Option<MetadataRef<DIFile>>,
    pub line: u32,
    pub flags: Vec<DIFlag>,
    pub elements: Vec<MetadataRef<Either<DIDerivedType, DISubprogram>>>,
    pub runtime_lang: u16,
    pub identifier: String,
    pub size_in_bits: u64,
    pub align_in_bits: u32,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum DIVariable {
    Global(DIGlobalVariable),
    Local(DILocalVariable),
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum DWOp {
    Fragment { offset: u64, size: u64 },  // must be last in the list
    StackValue,  // must be either last or followed by Fragment
    Swap,
    ConstU(u64),
    Lit0,
    PlusUConst(u64),
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Not,
    Or,
    Xor,
    And,
    Shr,
    Shra,
    Shl,
    Dup,
    Deref,
    XDeref,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Encoding {
    AddressEncoding,
    BooleanEncoding,
    FloatEncoding,
    SignedEncoding,
    SignedCharEncoding,
    UnsignedEncoding,
    UnsignedCharEncoding,
    UTFEncoding,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Virtuality {
    NoVirtuality,
    Virtual,
    PureVirtual,
}

// ********* //
// from_llvm //
// ********* //

use crate::from_llvm::*;

impl Metadata {
    pub(crate) fn from_llvm_ref(md: LLVMValueRef) -> Self {
        unimplemented!("Metadata::from_llvm_ref");
    }
}
