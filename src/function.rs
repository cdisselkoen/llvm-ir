use crate::debugloc::{DebugLoc, HasDebugLoc};
use crate::module::{Comdat, DLLStorageClass, Linkage, Visibility};
use crate::types::{TypeRef, Typed, Types};
use crate::{BasicBlock, ConstantRef, Name};

/// See [LLVM 14 docs on Functions](https://releases.llvm.org/14.0.0/docs/LangRef.html#functions)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub is_var_arg: bool,
    pub return_type: TypeRef,
    pub basic_blocks: Vec<BasicBlock>,
    pub function_attributes: Vec<FunctionAttribute>, // llvm-hs-pure has Vec<Either<GroupID, FunctionAttribute>>, but I'm not sure how the GroupID ones come about
    pub return_attributes: Vec<ParameterAttribute>,
    pub linkage: Linkage,
    pub visibility: Visibility,
    pub dll_storage_class: DLLStorageClass, // llvm-hs-pure has Option<DLLStorageClass>, but the llvm_sys api doesn't look like it can fail
    pub calling_convention: CallingConvention,
    pub section: Option<String>,
    pub comdat: Option<Comdat>, // llvm-hs-pure has Option<String>, I'm not sure why
    pub alignment: u32,
    /// See [LLVM 14 docs on Garbage Collector Strategy Names](https://releases.llvm.org/14.0.0/docs/LangRef.html#gc)
    pub garbage_collector_name: Option<String>,
    // pub prefix: Option<ConstantRef>,  // appears to not be exposed in the LLVM C API, only the C++ API
    /// Personalities are used for exception handling. See [LLVM 14 docs on Personality Function](https://releases.llvm.org/14.0.0/docs/LangRef.html#personalityfn)
    pub personality_function: Option<ConstantRef>,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: Vec<(String, MetadataRef<MetadataNode>)>,
}

impl Typed for Function {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.func_type(
            self.return_type.clone(),
            self.parameters.iter().map(|p| types.type_of(p)).collect(),
            self.is_var_arg,
        )
    }
}

impl HasDebugLoc for Function {
    fn get_debug_loc(&self) -> &Option<DebugLoc> {
        &self.debugloc
    }
}

impl Function {
    /// Get the `BasicBlock` having the given `Name` (if any).
    pub fn get_bb_by_name(&self, name: &Name) -> Option<&BasicBlock> {
        self.basic_blocks.iter().find(|bb| &bb.name == name)
    }

    /// A Function instance as empty as possible, using defaults
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parameters: vec![],
            is_var_arg: false,
            return_type: Types::blank_for_testing().void(),
            basic_blocks: vec![],
            function_attributes: vec![],
            return_attributes: vec![],
            linkage: Linkage::Private,
            visibility: Visibility::Default,
            dll_storage_class: DLLStorageClass::Default,
            calling_convention: CallingConvention::C,
            section: None,
            comdat: None,
            alignment: 4,
            garbage_collector_name: None,
            personality_function: None,
            debugloc: None,
        }
    }
}

/// See [LLVM 14 docs on Functions](https://releases.llvm.org/14.0.0/docs/LangRef.html#functions)
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub is_var_arg: bool,
    pub return_type: TypeRef,
    pub return_attributes: Vec<ParameterAttribute>,
    pub linkage: Linkage,
    pub visibility: Visibility,
    pub dll_storage_class: DLLStorageClass,
    pub calling_convention: CallingConvention,
    pub alignment: u32,
    /// See [LLVM 14 docs on Garbage Collector Strategy Names](https://releases.llvm.org/14.0.0/docs/LangRef.html#gc)
    pub garbage_collector_name: Option<String>,
    pub debugloc: Option<DebugLoc>,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Parameter {
    pub name: Name,
    pub ty: TypeRef,
    pub attributes: Vec<ParameterAttribute>,
}

impl Typed for Parameter {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.ty.clone()
    }
}

/// See [LLVM 14 docs on Calling Conventions](https://releases.llvm.org/14.0.0/docs/LangRef.html#callingconv)
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum CallingConvention {
    C,
    Fast,
    Cold,
    GHC,
    HiPE,
    WebKit_JS,
    AnyReg,
    PreserveMost,
    PreserveAll,
    Swift,
    CXX_FastTLS,
    X86_StdCall,
    X86_FastCall,
    X86_RegCall,
    X86_ThisCall,
    X86_VectorCall,
    X86_Intr,
    X86_64_SysV,
    ARM_APCS,
    ARM_AAPCS,
    ARM_AAPCS_VFP,
    MSP430_INTR,
    MSP430_Builtin,
    PTX_Kernel,
    PTX_Device,
    SPIR_FUNC,
    SPIR_KERNEL,
    Intel_OCL_BI,
    Win64,
    HHVM,
    HHVM_C,
    AVR_Intr,
    AVR_Signal,
    AVR_Builtin,
    AMDGPU_CS,
    AMDGPU_ES,
    AMDGPU_GS,
    AMDGPU_HS,
    AMDGPU_LS,
    AMDGPU_PS,
    AMDGPU_VS,
    AMDGPU_Kernel,
    /// This is used if LLVM returns a calling convention not in `LLVMCallConv`.
    /// E.g., perhaps a calling convention was added to LLVM and this enum hasn't been updated yet.
    Numbered(u32),
}

/// Describes how a given location in memory can be accessed.
/// See [LLVM 16 docs on FunctionAttributes](https://releases.llvm.org/16.0.0/docs/LangRef.html#fnattrs),
/// the section on memory(...)
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum MemoryEffect {
    None,
    Read,
    Write,
    ReadWrite
}

impl MemoryEffect {
    // See https://github.com/llvm/llvm-project/blob/7cbf1a2591520c2491aa35339f227775f4d3adf6/llvm/include/llvm/Support/ModRef.h#L27
    pub(crate) fn from_llvm_bits(val : u64) -> Self {
        match val {
            0b00 => Self::None,
            0b01 => Self::Read,
            0b10 => Self::Write,
            0b11 => Self::ReadWrite,
            _ => panic!("Memory effect given unexpected bits {}", val)
        }
    }
}

/// See [LLVM 14 docs on Function Attributes](https://releases.llvm.org/14.0.0/docs/LangRef.html#fnattrs)
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum FunctionAttribute {
    AlignStack(u64),
    AllocSize {
        elt_size: u32,
        num_elts: Option<u32>,
    },
    AlwaysInline,
    Builtin,
    Cold,
    Convergent,
    InaccessibleMemOnly,
    InaccessibleMemOrArgMemOnly,
    InlineHint,
    JumpTable,
    MinimizeSize,
    Naked,
    NoBuiltin,
    NoCFCheck,
    NoDuplicate,
    NoFree,
    NoImplicitFloat,
    NoInline,
    #[cfg(feature = "llvm-11-or-greater")]
    NoMerge,
    NonLazyBind,
    NoRedZone,
    NoReturn,
    NoRecurse,
    WillReturn,
    ReturnsTwice,
    NoSync,
    NoUnwind,
    #[cfg(feature = "llvm-11-or-greater")]
    NullPointerIsValid,
    OptForFuzzing,
    OptNone,
    OptSize,
    ReadNone,
    ReadOnly,
    WriteOnly,
    ArgMemOnly,
    SafeStack,
    SanitizeAddress,
    SanitizeMemory,
    SanitizeThread,
    SanitizeHWAddress,
    SanitizeMemTag,
    ShadowCallStack,
    SpeculativeLoadHardening,
    Speculatable,
    StackProtect,
    StackProtectReq,
    StackProtectStrong,
    StrictFP,
    UWTable,
    #[cfg(feature = "llvm-16-or-greater")]
    Memory {
        default: MemoryEffect,
        argmem: MemoryEffect,
        inaccessible_mem: MemoryEffect
    },
    StringAttribute {
        kind: String,
        value: String, // for no value, use ""
    },
    UnknownAttribute, // this is used if we get a value not in the above list
}

/// `ParameterAttribute`s can apply to function parameters as well as function return types.
/// See [LLVM 14 docs on Parameter Attributes](https://releases.llvm.org/14.0.0/docs/LangRef.html#paramattrs)
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum ParameterAttribute {
    ZeroExt,
    SignExt,
    InReg,
    #[cfg(feature = "llvm-11-or-lower")]
    ByVal,
    #[cfg(feature = "llvm-12-or-greater")]
    ByVal(TypeRef),
    #[cfg(feature = "llvm-11")]
    Preallocated,
    #[cfg(feature = "llvm-12-or-greater")]
    Preallocated(TypeRef),
    #[cfg(feature = "llvm-12-or-lower")]
    InAlloca,
    #[cfg(feature = "llvm-13-or-greater")]
    InAlloca(TypeRef),
    #[cfg(feature = "llvm-11-or-lower")]
    SRet,
    #[cfg(feature = "llvm-12-or-greater")]
    SRet(TypeRef),
    Alignment(u64),
    NoAlias,
    NoCapture,
    NoFree,
    Nest,
    Returned,
    NonNull,
    Dereferenceable(u64),
    DereferenceableOrNull(u64),
    SwiftSelf,
    SwiftError,
    ImmArg,
    #[cfg(feature = "llvm-11-or-greater")]
    NoUndef,
    StringAttribute {
        kind: String,
        value: String, // for no value, use ""
    },
    UnknownAttribute, // this is used if we get an EnumAttribute not in the above list; or, for LLVM 11 or lower, also for some TypeAttributes (due to C API limitations)
    #[cfg(feature = "llvm-12-or-greater")]
    UnknownTypeAttribute(TypeRef), // this is used if we get a TypeAttribute not in the above list
}

pub type GroupID = usize;

// ********* //
// from_llvm //
// ********* //

use crate::constant::Constant;
use crate::from_llvm::*;
use crate::llvm_sys::*;
use crate::module::ModuleContext;
#[cfg(feature = "llvm-12-or-greater")]
use crate::types::TypesBuilder;
use llvm_sys::comdat::*;
use llvm_sys::{LLVMAttributeFunctionIndex, LLVMAttributeReturnIndex};
use std::collections::HashMap;
use std::ffi::CString;

/// This struct contains data used when translating from llvm-sys into our data
/// structures. The data here is local to a particular Function.
pub(crate) struct FunctionContext<'a> {
    /// Map from llvm-sys basic block to its `Name`
    // We use LLVMBasicBlockRef as a *const, even though it's technically a *mut
    #[allow(clippy::mutable_key_type)]
    pub bb_names: &'a HashMap<LLVMBasicBlockRef, Name>,
    /// Map from llvm-sys value to its `Name`
    // We use LLVMValueRef as a *const, even though it's technically a *mut
    #[allow(clippy::mutable_key_type)]
    pub val_names: &'a HashMap<LLVMValueRef, Name>,
    /// this counter is used to number parameters, variables, and basic blocks that aren't named
    pub ctr: usize,
}

impl FunctionDeclaration {
    pub(crate) fn from_llvm_ref(func: LLVMValueRef, ctx: &mut ModuleContext) -> Self {
        let func = unsafe { LLVMIsAFunction(func) };
        assert!(!func.is_null());
        debug!("Processing func {:?}", unsafe { get_value_name(func) });

        let (decl, _) = FunctionDeclaration::from_llvm_ref_internal(func, ctx);
        decl
    }

    /// this helper is shared by `FunctionDeclaration` and `Function`. It
    /// provides the whole `FunctionDeclaration`, and also the value of the
    /// `local_ctr` after parameters are processed (which is needed by
    /// `Function`).
    fn from_llvm_ref_internal(func: LLVMValueRef, ctx: &mut ModuleContext) -> (Self, usize) {
        #[cfg(feature = "llvm-14-or-lower")]
        let functy = unsafe { LLVMGetElementType(LLVMTypeOf(func)) }; // for some reason the TypeOf a function is <pointer to function> and not just <function> so we have to deref it like this
        #[cfg(feature = "llvm-15-or-greater")]
        let functy = unsafe { LLVMGlobalGetValueType(func) };
        let mut local_ctr = 0; // this counter is used to number parameters, variables, and basic blocks that aren't named
        let decl = Self {
            name: unsafe { get_value_name(func) },
            parameters: {
                let parameters: Vec<Parameter> = get_parameters(func)
                    .enumerate()
                    .map(|(i, p)| Parameter {
                        name: Name::name_or_num(unsafe { get_value_name(p) }, &mut local_ctr),
                        ty: ctx.types.type_from_llvm_ref(unsafe { LLVMTypeOf(p) }),
                        attributes: {
                            let param_num = i + 1; // https://docs.rs/llvm-sys/100.0.1/llvm_sys/type.LLVMAttributeIndex.html indicates that parameter numbers are 1-indexed here; see issue #4
                            let num_attrs =
                                unsafe { LLVMGetAttributeCountAtIndex(func, param_num as u32) };
                            let mut attrs: Vec<LLVMAttributeRef> =
                                Vec::with_capacity(num_attrs as usize);
                            unsafe {
                                LLVMGetAttributesAtIndex(
                                    func,
                                    param_num as u32,
                                    attrs.as_mut_ptr(),
                                );
                                attrs.set_len(num_attrs as usize);
                            };
                            attrs
                                .into_iter()
                                .map(|attr| {
                                    ParameterAttribute::from_llvm_ref(
                                        attr,
                                        &ctx.attrsdata,
                                        #[cfg(feature = "llvm-12-or-greater")]
                                        &mut ctx.types,
                                    )
                                })
                                .collect()
                        },
                    })
                    .collect();
                debug!("Collected info on {} parameters", parameters.len());
                parameters
            },
            is_var_arg: unsafe { LLVMIsFunctionVarArg(functy) } != 0,
            return_type: ctx
                .types
                .type_from_llvm_ref(unsafe { LLVMGetReturnType(functy) }),
            return_attributes: {
                let num_attrs =
                    unsafe { LLVMGetAttributeCountAtIndex(func, LLVMAttributeReturnIndex) };
                if num_attrs > 0 {
                    let mut attrs: Vec<LLVMAttributeRef> = Vec::with_capacity(num_attrs as usize);
                    unsafe {
                        LLVMGetAttributesAtIndex(
                            func,
                            LLVMAttributeReturnIndex,
                            attrs.as_mut_ptr(),
                        );
                        attrs.set_len(num_attrs as usize);
                    };
                    attrs
                        .into_iter()
                        .map(|attr| {
                            ParameterAttribute::from_llvm_ref(
                                attr,
                                &ctx.attrsdata,
                                #[cfg(feature = "llvm-12-or-greater")]
                                &mut ctx.types,
                            )
                        })
                        .collect()
                } else {
                    vec![]
                }
            },
            linkage: Linkage::from_llvm(unsafe { LLVMGetLinkage(func) }),
            visibility: Visibility::from_llvm(unsafe { LLVMGetVisibility(func) }),
            dll_storage_class: DLLStorageClass::from_llvm(unsafe { LLVMGetDLLStorageClass(func) }),
            calling_convention: CallingConvention::from_u32(unsafe {
                LLVMGetFunctionCallConv(func)
            }),
            alignment: unsafe { LLVMGetAlignment(func) },
            garbage_collector_name: unsafe { get_gc(func) },
            debugloc: DebugLoc::from_llvm_no_col(func),
        };
        (decl, local_ctr)
    }
}

impl Function {
    pub(crate) fn from_llvm_ref(func: LLVMValueRef, ctx: &mut ModuleContext) -> Self {
        let func = unsafe { LLVMIsAFunction(func) };
        assert!(!func.is_null());
        debug!("Processing func {:?}", unsafe { get_value_name(func) });

        // `Function` is a strict superset of the information in
        // `FunctionDeclaration`, so we start by collecting all of the
        // information shared by `FunctionDeclaration`, reusing that code
        let (decl, ctr_val_after_parameters) =
            FunctionDeclaration::from_llvm_ref_internal(func, ctx);

        // Functions require two passes over their bodies.
        // First we make a pass just to map `LLVMBasicBlockRef`s to `Name`s and `LLVMValueRef`s to `Name`s.
        // Then we do the actual detailed pass.
        // This is necessary because some instructions (e.g., forward branches) will reference
        //   `LLVMBasicBlockRef`s and/or `LLVMValueRef`s which we wouldn't have
        //   seen before if we tried to do everything in one pass, and therefore
        //   we wouldn't necessarily know what `Name` the block or value had yet.
        let mut local_ctr = ctr_val_after_parameters; // this counter is used to number parameters, variables, and basic blocks that aren't named
        let bbresults: Vec<_> = get_basic_blocks(func)
            .map(|bb| (bb, BasicBlock::first_pass_names(bb, &mut local_ctr)))
            .collect();
        // We use LLVMBasicBlockRef as a *const, even though it's technically a *mut
        #[allow(clippy::mutable_key_type)]
        let bb_names: HashMap<LLVMBasicBlockRef, Name> = bbresults
            .iter()
            .map(|(bb, (bbname, _))| (*bb, bbname.clone()))
            .collect();
        debug!("Collected names of {} basic blocks", bb_names.len());
        // We use LLVMValueRef as a *const, even though it's technically a *mut
        #[allow(clippy::mutable_key_type)]
        let val_names: HashMap<LLVMValueRef, Name> = bbresults
            .into_iter()
            .flat_map(|(_, (_, namepairs))| namepairs.into_iter())
            .chain(get_parameters(func).zip(decl.parameters.iter().map(|p| p.name.clone())))
            .collect();
        debug!("Collected names of {} values", val_names.len());
        let mut func_ctx = FunctionContext {
            bb_names: &bb_names,
            val_names: &val_names,
            ctr: ctr_val_after_parameters, // restart the local_ctr; the second pass should number everything exactly the same though
        };

        Self {
            name: decl.name,
            parameters: decl.parameters,
            is_var_arg: decl.is_var_arg,
            return_type: decl.return_type,
            basic_blocks: {
                get_basic_blocks(func)
                    .map(|bb| BasicBlock::from_llvm_ref(bb, ctx, &mut func_ctx))
                    .collect()
            },
            function_attributes: {
                let num_attrs =
                    unsafe { LLVMGetAttributeCountAtIndex(func, LLVMAttributeFunctionIndex) };
                if num_attrs > 0 {
                    let mut attrs: Vec<LLVMAttributeRef> = Vec::with_capacity(num_attrs as usize);
                    unsafe {
                        LLVMGetAttributesAtIndex(
                            func,
                            LLVMAttributeFunctionIndex,
                            attrs.as_mut_ptr(),
                        );
                        attrs.set_len(num_attrs as usize);
                    };
                    attrs
                        .into_iter()
                        .map(|attr| FunctionAttribute::from_llvm_ref(attr, &ctx.attrsdata))
                        .collect()
                } else {
                    vec![]
                }
            },
            return_attributes: decl.return_attributes,
            linkage: decl.linkage,
            visibility: decl.visibility,
            dll_storage_class: decl.dll_storage_class,
            calling_convention: decl.calling_convention,
            section: unsafe { get_section(func) },
            comdat: {
                let comdat = unsafe { LLVMGetComdat(func) };
                if comdat.is_null() {
                    None
                } else {
                    Some(Comdat::from_llvm_ref(comdat))
                }
            },
            alignment: decl.alignment,
            garbage_collector_name: decl.garbage_collector_name,
            personality_function: {
                if unsafe { LLVMHasPersonalityFn(func) } != 0 {
                    Some(Constant::from_llvm_ref(
                        unsafe { LLVMGetPersonalityFn(func) },
                        ctx,
                    ))
                } else {
                    None
                }
            },
            debugloc: decl.debugloc,
            // metadata: unimplemented!("Function.metadata"),
        }
    }
}

impl CallingConvention {
    #[allow(clippy::cognitive_complexity)]
    #[rustfmt::skip] // each calling convention on one line, even if lines get a little long
    pub(crate) fn from_u32(u: u32) -> Self {
        use llvm_sys::LLVMCallConv;
        match u {
            _ if u == LLVMCallConv::LLVMCCallConv as u32 => CallingConvention::C,
            _ if u == LLVMCallConv::LLVMFastCallConv as u32 => CallingConvention::Fast,
            _ if u == LLVMCallConv::LLVMColdCallConv as u32 => CallingConvention::Cold,
            _ if u == LLVMCallConv::LLVMGHCCallConv as u32 => CallingConvention::GHC,
            _ if u == LLVMCallConv::LLVMHiPECallConv as u32 => CallingConvention::HiPE,
            #[cfg(feature = "llvm-17-or-lower")]
            _ if u == LLVMCallConv::LLVMWebKitJSCallConv as u32 => CallingConvention::WebKit_JS,
            _ if u == LLVMCallConv::LLVMAnyRegCallConv as u32 => CallingConvention::AnyReg,
            _ if u == LLVMCallConv::LLVMPreserveMostCallConv as u32 => CallingConvention::PreserveMost,
            _ if u == LLVMCallConv::LLVMPreserveAllCallConv as u32 => CallingConvention::PreserveAll,
            _ if u == LLVMCallConv::LLVMSwiftCallConv as u32 => CallingConvention::Swift,
            _ if u == LLVMCallConv::LLVMCXXFASTTLSCallConv as u32 => CallingConvention::CXX_FastTLS,
            _ if u == LLVMCallConv::LLVMX86StdcallCallConv as u32 => CallingConvention::X86_StdCall,
            _ if u == LLVMCallConv::LLVMX86FastcallCallConv as u32 => CallingConvention::X86_FastCall,
            _ if u == LLVMCallConv::LLVMX86RegCallCallConv as u32 => CallingConvention::X86_RegCall,
            _ if u == LLVMCallConv::LLVMX86ThisCallCallConv as u32 => CallingConvention::X86_ThisCall,
            _ if u == LLVMCallConv::LLVMX86VectorCallCallConv as u32 => CallingConvention::X86_VectorCall,
            _ if u == LLVMCallConv::LLVMX86INTRCallConv as u32 => CallingConvention::X86_Intr,
            _ if u == LLVMCallConv::LLVMX8664SysVCallConv as u32 => CallingConvention::X86_64_SysV,
            _ if u == LLVMCallConv::LLVMARMAPCSCallConv as u32 => CallingConvention::ARM_APCS,
            _ if u == LLVMCallConv::LLVMARMAAPCSCallConv as u32 => CallingConvention::ARM_AAPCS,
            _ if u == LLVMCallConv::LLVMARMAAPCSVFPCallConv as u32 => CallingConvention::ARM_AAPCS_VFP,
            _ if u == LLVMCallConv::LLVMMSP430INTRCallConv as u32 => CallingConvention::MSP430_INTR,
            _ if u == LLVMCallConv::LLVMMSP430BUILTINCallConv as u32 => CallingConvention::MSP430_Builtin,
            _ if u == LLVMCallConv::LLVMPTXKernelCallConv as u32 => CallingConvention::PTX_Kernel,
            _ if u == LLVMCallConv::LLVMPTXDeviceCallConv as u32 => CallingConvention::PTX_Device,
            _ if u == LLVMCallConv::LLVMSPIRFUNCCallConv as u32 => CallingConvention::SPIR_FUNC,
            _ if u == LLVMCallConv::LLVMSPIRKERNELCallConv as u32 => CallingConvention::SPIR_KERNEL,
            _ if u == LLVMCallConv::LLVMIntelOCLBICallConv as u32 => CallingConvention::Intel_OCL_BI,
            _ if u == LLVMCallConv::LLVMWin64CallConv as u32 => CallingConvention::Win64,
            _ if u == LLVMCallConv::LLVMHHVMCallConv as u32 => CallingConvention::HHVM,
            _ if u == LLVMCallConv::LLVMHHVMCCallConv as u32 => CallingConvention::HHVM_C,
            _ if u == LLVMCallConv::LLVMAVRINTRCallConv as u32 => CallingConvention::AVR_Intr,
            _ if u == LLVMCallConv::LLVMAVRSIGNALCallConv as u32 => CallingConvention::AVR_Signal,
            _ if u == LLVMCallConv::LLVMAVRBUILTINCallConv as u32 => CallingConvention::AVR_Builtin,
            _ if u == LLVMCallConv::LLVMAMDGPUCSCallConv as u32 => CallingConvention::AMDGPU_CS,
            _ if u == LLVMCallConv::LLVMAMDGPUESCallConv as u32 => CallingConvention::AMDGPU_ES,
            _ if u == LLVMCallConv::LLVMAMDGPUGSCallConv as u32 => CallingConvention::AMDGPU_GS,
            _ if u == LLVMCallConv::LLVMAMDGPUHSCallConv as u32 => CallingConvention::AMDGPU_HS,
            _ if u == LLVMCallConv::LLVMAMDGPULSCallConv as u32 => CallingConvention::AMDGPU_LS,
            _ if u == LLVMCallConv::LLVMAMDGPUPSCallConv as u32 => CallingConvention::AMDGPU_PS,
            _ if u == LLVMCallConv::LLVMAMDGPUVSCallConv as u32 => CallingConvention::AMDGPU_VS,
            _ if u == LLVMCallConv::LLVMAMDGPUKERNELCallConv as u32 => CallingConvention::AMDGPU_Kernel,
            _ => CallingConvention::Numbered(u),
        }
    }
}

pub(crate) struct AttributesData {
    function_attribute_names: HashMap<u32, String>,
    param_attribute_names: HashMap<u32, String>,
}

impl AttributesData {
    pub fn create() -> Self {
        let function_attribute_names = [
            "alignstack",
            "allocsize",
            "alwaysinline",
            "builtin",
            "cold",
            "convergent",
            #[cfg(feature = "llvm-15-or-lower")]
            "inaccessiblememonly",
            #[cfg(feature = "llvm-15-or-lower")]
            "inaccessiblemem_or_argmemonly",
            "inlinehint",
            "jumptable",
            "minsize",
            "naked",
            "nobuiltin",
            "nocf_check",
            "noduplicate",
            "nofree",
            "noimplicitfloat",
            "noinline",
            #[cfg(feature = "llvm-11-or-greater")]
            "nomerge",
            "nonlazybind",
            "noredzone",
            "noreturn",
            "norecurse",
            "willreturn",
            "returns_twice",
            "nosync",
            "nounwind",
            #[cfg(feature = "llvm-11-or-greater")]
            "null_pointer_is_valid",
            "optforfuzzing",
            "optnone",
            "optsize",
            "readnone",
            "readonly",
            "writeonly",
            #[cfg(feature = "llvm-15-or-lower")]
            "argmemonly",
            "safestack",
            "sanitize_address",
            "sanitize_memory",
            "sanitize_thread",
            "sanitize_hwaddress",
            "sanitize_memtag",
            "shadowcallstack",
            "speculative_load_hardening",
            "speculatable",
            "ssp",
            "sspreq",
            "sspstrong",
            "strictfp",
            "uwtable",
            #[cfg(feature = "llvm-16-or-greater")]
            "memory"
        ]
        .iter()
        .map(|&attrname| {
            let cstr = CString::new(attrname).unwrap();
            let kind = unsafe { LLVMGetEnumAttributeKindForName(cstr.as_ptr(), attrname.len()) };
            assert_ne!(kind, 0, "Function attribute {:?} not found", attrname);
            (kind, attrname.into())
        })
        .collect();
        let param_attribute_names = [
            "zeroext",
            "signext",
            "inreg",
            "byval",
            #[cfg(feature = "llvm-11-or-greater")]
            "preallocated",
            "inalloca",
            "sret",
            "align",
            "noalias",
            "nocapture",
            "nofree",
            "nest",
            "returned",
            "nonnull",
            "dereferenceable",
            "dereferenceable_or_null",
            "swiftself",
            "swifterror",
            "immarg",
            #[cfg(feature = "llvm-11-or-greater")]
            "noundef",
        ]
        .iter()
        .map(|&attrname| {
            let cstr = CString::new(attrname).unwrap();
            let kind = unsafe { LLVMGetEnumAttributeKindForName(cstr.as_ptr(), attrname.len()) };
            assert_ne!(kind, 0, "Parameter attribute {:?} not found", attrname);
            (kind, attrname.into())
        })
        .collect();
        Self {
            function_attribute_names,
            param_attribute_names,
        }
    }

    /// Get the string name of an enum-style function attribute, or `None` if
    /// it's not one that we know about
    pub fn lookup_function_attr(&self, kind: u32) -> Option<&str> {
        self.function_attribute_names.get(&kind).map(|s| s.as_str())
    }

    /// Get the string name of an enum-style parameter attribute, or `None` if
    /// it's not one that we know about
    pub fn lookup_param_attr(&self, kind: u32) -> Option<&str> {
        self.param_attribute_names.get(&kind).map(|s| s.as_str())
    }
}

impl FunctionAttribute {
    pub(crate) fn from_llvm_ref(a: LLVMAttributeRef, attrsdata: &AttributesData) -> Self {
        if unsafe { LLVMIsEnumAttribute(a) } != 0 {
            let kind = unsafe { LLVMGetEnumAttributeKind(a) };
            match attrsdata.lookup_function_attr(kind) {
                Some("alignstack") => Self::AlignStack(unsafe { LLVMGetEnumAttributeValue(a) }),
                Some("allocsize") => {
                    let value = unsafe { LLVMGetEnumAttributeValue(a) };
                    // looking at the LLVM implementation as of this writing
                    // (near the top of Attributes.cpp),
                    // the elt_size value is the upper 32 bits, and the num_elts value
                    // is the lower 32 bits, or the sentinel -1 for None
                    let elt_size = (value >> 32) as u32;
                    let num_elts = match (value & 0xFFFF_FFFF) as u32 {
                        0xFFFF_FFFF => None,
                        val => Some(val),
                    };
                    Self::AllocSize { elt_size, num_elts }
                },
                Some("alwaysinline") => Self::AlwaysInline,
                Some("builtin") => Self::Builtin,
                Some("cold") => Self::Cold,
                Some("convergent") => Self::Convergent,
                Some("inaccessiblememonly") => Self::InaccessibleMemOnly,
                Some("inaccessiblemem_or_argmemonly") => Self::InaccessibleMemOrArgMemOnly,
                Some("inlinehint") => Self::InlineHint,
                Some("jumptable") => Self::JumpTable,
                Some("minsize") => Self::MinimizeSize,
                Some("naked") => Self::Naked,
                Some("nobuiltin") => Self::NoBuiltin,
                Some("nocf_check") => Self::NoCFCheck,
                Some("noduplicate") => Self::NoDuplicate,
                Some("nofree") => Self::NoFree,
                Some("noimplicitfloat") => Self::NoImplicitFloat,
                Some("noinline") => Self::NoInline,
                #[cfg(feature = "llvm-11-or-greater")]
                Some("nomerge") => Self::NoMerge,
                Some("nonlazybind") => Self::NonLazyBind,
                Some("noredzone") => Self::NoRedZone,
                Some("noreturn") => Self::NoReturn,
                Some("norecurse") => Self::NoRecurse,
                Some("willreturn") => Self::WillReturn,
                Some("returns_twice") => Self::ReturnsTwice,
                Some("nosync") => Self::NoSync,
                Some("nounwind") => Self::NoUnwind,
                #[cfg(feature = "llvm-11-or-greater")]
                Some("null_pointer_is_valid") => Self::NullPointerIsValid,
                Some("optforfuzzing") => Self::OptForFuzzing,
                Some("optnone") => Self::OptNone,
                Some("optsize") => Self::OptSize,
                Some("readnone") => Self::ReadNone,
                Some("readonly") => Self::ReadOnly,
                Some("writeonly") => Self::WriteOnly,
                Some("argmemonly") => Self::ArgMemOnly,
                Some("safestack") => Self::SafeStack,
                Some("sanitize_address") => Self::SanitizeAddress,
                Some("sanitize_memory") => Self::SanitizeMemory,
                Some("sanitize_thread") => Self::SanitizeThread,
                Some("sanitize_hwaddress") => Self::SanitizeHWAddress,
                Some("sanitize_memtag") => Self::SanitizeMemTag,
                Some("shadowcallstack") => Self::ShadowCallStack,
                Some("speculative_load_hardening") => Self::SpeculativeLoadHardening,
                Some("speculatable") => Self::Speculatable,
                Some("ssp") => Self::StackProtect,
                Some("sspreq") => Self::StackProtectReq,
                Some("sspstrong") => Self::StackProtectStrong,
                Some("strictfp") => Self::StrictFP,
                Some("uwtable") => Self::UWTable,
                #[cfg(feature = "llvm-16-or-greater")]
                Some("memory") => {
                    let value = unsafe { LLVMGetEnumAttributeValue(a) };

                    // The value is encoded as a bitmask for the possible effects, shifted for each location kind,
                    // and merged together
                    // See https://github.com/llvm/llvm-project/blob/7cbf1a2591520c2491aa35339f227775f4d3adf6/llvm/include/llvm/Support/ModRef.h#L63
                    // for the breakdown of the encoding logic

                    let encoded_argmem           = (value >> 0) & 0b11;
                    let encoded_inaccessible_mem = (value >> 2) & 0b11;
                    let encoded_default_mem      = (value >> 4) & 0b11;

                    Self::Memory {
                        default: MemoryEffect::from_llvm_bits(encoded_default_mem),
                        argmem: MemoryEffect::from_llvm_bits(encoded_argmem),
                        inaccessible_mem: MemoryEffect::from_llvm_bits(encoded_inaccessible_mem)
                    }
                },
                Some(s) => panic!("Unhandled value from lookup_function_attr: {:?}", s),
                None => {
                    debug!("unknown enum function attr {}", kind);
                    Self::UnknownAttribute
                },
            }
        } else if unsafe { LLVMIsStringAttribute(a) } != 0 {
            Self::StringAttribute {
                kind: unsafe { get_string_attribute_kind(a) },
                value: unsafe { get_string_attribute_value(a) },
            }
        } else {
            debug!("Encountered an unknown function attribute: neither enum nor string");
            Self::UnknownAttribute
        }
    }
}

impl ParameterAttribute {
    pub(crate) fn from_llvm_ref(
        a: LLVMAttributeRef,
        attrsdata: &AttributesData,
        #[cfg(feature = "llvm-12-or-greater")] types: &mut TypesBuilder,
    ) -> Self {
        if unsafe { LLVMIsEnumAttribute(a) } != 0 {
            let kind = unsafe { LLVMGetEnumAttributeKind(a) };
            match attrsdata.lookup_param_attr(kind) {
                Some("zeroext") => Self::ZeroExt,
                Some("signext") => Self::SignExt,
                Some("inreg") => Self::InReg,
                #[cfg(feature = "llvm-11-or-lower")]
                Some("byval") => Self::ByVal,
                #[cfg(feature = "llvm-11")]
                Some("preallocated") => Self::Preallocated,
                #[cfg(feature = "llvm-12-or-lower")]
                Some("inalloca") => Self::InAlloca,
                #[cfg(feature = "llvm-11-or-lower")]
                Some("sret") => Self::SRet,
                Some("align") => Self::Alignment(unsafe { LLVMGetEnumAttributeValue(a) }),
                Some("noalias") => Self::NoAlias,
                Some("nocapture") => Self::NoCapture,
                Some("nofree") => Self::NoFree,
                Some("nest") => Self::Nest,
                Some("returned") => Self::Returned,
                Some("nonnull") => Self::NonNull,
                Some("dereferenceable") => {
                    Self::Dereferenceable(unsafe { LLVMGetEnumAttributeValue(a) })
                },
                Some("dereferenceable_or_null") => {
                    Self::DereferenceableOrNull(unsafe { LLVMGetEnumAttributeValue(a) })
                },
                Some("swiftself") => Self::SwiftSelf,
                Some("swifterror") => Self::SwiftError,
                Some("immarg") => Self::ImmArg,
                #[cfg(feature = "llvm-11-or-greater")]
                Some("noundef") => Self::NoUndef,
                Some(s) => panic!("Unhandled value from lookup_param_attr: {:?}", s),
                None => {
                    debug!("unknown enum param attr {}", kind);
                    Self::UnknownAttribute
                },
            }
        } else if unsafe { LLVMIsStringAttribute(a) } != 0 {
            Self::StringAttribute {
                kind: unsafe { get_string_attribute_kind(a) },
                value: unsafe { get_string_attribute_value(a) },
            }
        } else if Self::is_type_attr(a) {
            #[cfg(feature = "llvm-11-or-lower")]
            {
                debug!("Encountered a type attr, which shouldn't happen on LLVM 11 or lower");
                Self::UnknownAttribute
            }
            #[cfg(feature = "llvm-12-or-greater")]
            {
                let kind = unsafe { LLVMGetEnumAttributeKind(a) };
                let ty = types.type_from_llvm_ref(unsafe { LLVMGetTypeAttributeValue(a) });
                match attrsdata.lookup_param_attr(kind) {
                    Some("byval") => Self::ByVal(ty),
                    Some("preallocated") => Self::Preallocated(ty),
                    #[cfg(feature = "llvm-13-or-greater")]
                    Some("inalloca") => Self::InAlloca(ty),
                    Some("sret") => Self::SRet(ty),
                    Some(s) => panic!("Unhandled value from lookup_param_attr: {:?}", s),
                    None => {
                        debug!("unknown type param attr {}", kind);
                        Self::UnknownTypeAttribute(ty)
                    },
                }
            }
        } else {
            debug!("Encountered an unknown parameter attribute: neither enum, string, nor type");
            Self::UnknownAttribute
        }
    }

    #[cfg(feature = "llvm-11-or-lower")]
    fn is_type_attr(_a: LLVMAttributeRef) -> bool {
        false
    }
    #[cfg(feature = "llvm-12-or-greater")]
    fn is_type_attr(a: LLVMAttributeRef) -> bool {
        unsafe { LLVMIsTypeAttribute(a) != 0 }
    }
}
