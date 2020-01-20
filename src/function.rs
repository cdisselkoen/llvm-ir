use crate::basicblock::BasicBlock;
use crate::constant::Constant;
use crate::debugloc::{DebugLoc, HasDebugLoc};
use crate::module::{Comdat, DLLStorageClass, Linkage, Visibility};
use crate::name::Name;
use crate::types::{Type, Typed};
use std::num;

/// See [LLVM 9 docs on Functions](https://releases.llvm.org/9.0.0/docs/LangRef.html#functions)
#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub is_var_arg: bool,
    pub return_type: Type,
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
    /// See [LLVM 9 docs on Garbage Collector Strategy Names](https://releases.llvm.org/9.0.0/docs/LangRef.html#gc)
    pub garbage_collector_name: Option<String>,
    // pub prefix: Option<Constant>,  // appears to not be exposed in the LLVM C API, only the C++ API
    /// Personalities are used for exception handling. See [LLVM 9 docs on Personality Function](https://releases.llvm.org/9.0.0/docs/LangRef.html#personalityfn)
    pub personality_function: Option<Constant>,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: Vec<(String, MetadataRef<MetadataNode>)>,
}

impl Typed for Function {
    fn get_type(&self) -> Type {
        Type::FuncType {
            result_type: Box::new(self.return_type.clone()),
            param_types: self.parameters.iter().map(|p| p.get_type()).collect(),
            is_var_arg: self.is_var_arg,
        }
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
        for bb in self.basic_blocks.iter() {
            if &bb.name == name {
                return Some(bb);
            }
        }
        None
    }

    /// A Function instance as empty as possible, using defaults
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parameters: vec![],
            is_var_arg: false,
            return_type: Type::VoidType,
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

#[derive(PartialEq, Clone, Debug)]
pub struct Parameter {
    pub name: Name,
    pub ty: Type,
    pub attributes: Vec<ParameterAttribute>,
}

impl Typed for Parameter {
    fn get_type(&self) -> Type {
        self.ty.clone()
    }
}

/// See [LLVM 9 docs on Calling Conventions](https://releases.llvm.org/9.0.0/docs/LangRef.html#callingconv)
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Attribute {
    EnumAttribute {
        kind: u32,
        value: Option<num::NonZeroU64>, // LLVM C API documentation for LLVMGetEnumAttributeValue says "0 is returned if none exists", so it seems to be impossible to distinguish between a value of 0 and no value (if a value of 0 is legal at all)
    },
    StringAttribute {
        kind: String,
        value: String, // for no value, use ""
    },
}

/// See [LLVM 9 docs on Function Attributes](https://releases.llvm.org/9.0.0/docs/LangRef.html#fnattrs)
pub type FunctionAttribute = Attribute;
/* llvm-hs-pure has the following enum here, but the LLVM C API just uses an unsigned for enum attributes
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum FunctionAttribute {
    NoReturn,
    NoUnwind,
    ReadNone,
    ReadOnly,
    NoInline,
    NoRecurse,
    AlwaysInline,
    MinimizeSize,
    OptimizeForSize,
    OptimizeNone,
    StackProtect,
    StackProtectReq,
    StackProtectStrong,
    StrictFP,
    NoRedZone,
    NoImplicitFloat,
    Naked,
    InlineHint,
    StackAlignment(u64),
    ReturnsTwice,
    UWTable,
    NonLazyBind,
    Builtin,
    NoBuiltin,
    Cold,
    JumpTable,
    NoDuplicate,
    SanitizeAddress,
    SanitizeHWAddress,
    SanitizeThread,
    SanitizeMemory,
    Speculatable,
    StringAttribute { kind: String, value: String },  // for no value, use ""
    AllocSize((u32, Option<u32>)),  // if the first is 0, the second cannot be Some(0)
    WriteOnly,
    ArgMemOnly,
    Convergent,
    InaccessibleMemOnly,
    InaccessibleMemOrArgMemOnly,
    SafeStack,
}
*/

/// `ParameterAttribute`s can apply to function parameters as well as function return types.
/// See [LLVM 9 docs on Parameter Attributes](https://releases.llvm.org/9.0.0/docs/LangRef.html#paramattrs)
pub type ParameterAttribute = Attribute;
/* llvm-hs-pure has the following enum here, but the LLVM C API just uses an unsigned for enum attributes
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ParameterAttribute {
    ZeroExt,
    SignExt,
    InReg,
    SRet,
    Alignment(u64),
    NoAlias,
    ByVal,
    NoCapture,
    Nest,
    ReadNone,
    ReadOnly,
    WriteOnly,
    InAlloca,
    NonNull,
    Dereferenceable(u64),
    DereferenceableOrNull(u64),
    Returned,
    SwiftSelf,
    SwiftError,
    StringAttribute { kind: String, value: String },  // for no value, use ""
}
*/

pub type GroupID = usize;

// ********* //
// from_llvm //
// ********* //

use crate::basicblock::BBMap;
use crate::constant::GlobalNameMap;
use crate::from_llvm::*;
use crate::operand::ValToNameMap;
use crate::types::TyNameMap;
use llvm_sys::comdat::*;
use llvm_sys::{LLVMAttributeFunctionIndex, LLVMAttributeReturnIndex};

impl Function {
    pub(crate) fn from_llvm_ref(
        func: LLVMValueRef,
        gnmap: &GlobalNameMap,
        tnmap: &mut TyNameMap,
    ) -> Self {
        let func = unsafe { LLVMIsAFunction(func) };
        assert!(!func.is_null());
        debug!("Processing func {:?}", unsafe { get_value_name(func) });
        let mut local_ctr = 0; // this ctr is used to number parameters, variables, and basic blocks that aren't named

        let parameters: Vec<Parameter> = {
            get_parameters(func)
                .enumerate()
                .map(|(i, p)| Parameter {
                    name: Name::name_or_num(unsafe { get_value_name(p) }, &mut local_ctr),
                    ty: Type::from_llvm_ref(unsafe { LLVMTypeOf(p) }, tnmap),
                    attributes: {
                        let num_attrs = unsafe { LLVMGetAttributeCountAtIndex(func, i as u32) };
                        let mut attrs: Vec<LLVMAttributeRef> =
                            Vec::with_capacity(num_attrs as usize);
                        unsafe {
                            LLVMGetAttributesAtIndex(func, i as u32, attrs.as_mut_ptr());
                            attrs.set_len(num_attrs as usize);
                        };
                        attrs
                            .into_iter()
                            .filter_map(ParameterAttribute::from_llvm_ref)
                            .collect()
                    },
                })
                .collect()
        };

        let ctr_val_after_parameters = local_ctr;

        // Functions require two passes over their bodies.
        // First we make a pass just to map `LLVMBasicBlockRef`s to `Name`s and `LLVMValueRef`s to `Name`s.
        // Then we do the actual detailed pass.
        // This is necessary because some instructions (e.g., forward branches) will reference
        //   `LLVMBasicBlockRef`s and/or `LLVMValueRef`s which we wouldn't have
        //   seen before if we tried to do everything in one pass, and therefore
        //   we wouldn't necessarily know what `Name` the block or value had yet.
        let bbresults: Vec<_> = get_basic_blocks(func)
            .map(|bb| (bb, BasicBlock::first_pass_names(bb, &mut local_ctr)))
            .collect();
        let bbmap: BBMap = bbresults
            .iter()
            .map(|(bb, (bbname, _))| (*bb, bbname.clone()))
            .collect();
        debug!("Collected a BBMap with {} entries", bbmap.len());
        let vnmap: ValToNameMap = bbresults
            .into_iter()
            .flat_map(|(_, (_, namepairs))| namepairs.into_iter())
            .chain(get_parameters(func).zip(parameters.iter().map(|p| p.name.clone())))
            .collect();
        debug!("Collected a ValToNameMap with {} entries", vnmap.len());

        local_ctr = ctr_val_after_parameters; // reset the local_ctr; the second pass should number everything exactly the same though
        let functy = unsafe { LLVMGetElementType(LLVMTypeOf(func)) }; // for some reason the TypeOf a function is <pointer to function> and not just <function> so we have to deref it like this
        Self {
            name: unsafe { get_value_name(func) },
            parameters,
            is_var_arg: unsafe { LLVMIsFunctionVarArg(functy) } != 0,
            return_type: Type::from_llvm_ref(unsafe { LLVMGetReturnType(functy) }, tnmap),
            basic_blocks: {
                get_basic_blocks(func)
                    .map(|bb| {
                        BasicBlock::from_llvm_ref(bb, &mut local_ctr, &vnmap, &bbmap, gnmap, tnmap)
                    })
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
                        .filter_map(FunctionAttribute::from_llvm_ref)
                        .collect()
                } else {
                    vec![]
                }
            },
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
                        .filter_map(ParameterAttribute::from_llvm_ref)
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
            section: unsafe { get_section(func) },
            comdat: {
                let comdat = unsafe { LLVMGetComdat(func) };
                if comdat.is_null() {
                    None
                } else {
                    Some(Comdat::from_llvm_ref(comdat))
                }
            },
            alignment: unsafe { LLVMGetAlignment(func) },
            garbage_collector_name: unsafe { get_gc(func) },
            personality_function: {
                if unsafe { LLVMHasPersonalityFn(func) } != 0 {
                    Some(Constant::from_llvm_ref(
                        unsafe { LLVMGetPersonalityFn(func) },
                        gnmap,
                        tnmap,
                    ))
                } else {
                    None
                }
            },
            debugloc: DebugLoc::from_llvm_no_col(func),
            // metadata: unimplemented!("Function.metadata"),
        }
    }
}

impl CallingConvention {
    #[allow(clippy::cognitive_complexity)]
    pub(crate) fn from_u32(u: u32) -> Self {
        use llvm_sys::LLVMCallConv;
        match u {
            _ if u == LLVMCallConv::LLVMCCallConv as u32 => CallingConvention::C,
            _ if u == LLVMCallConv::LLVMFastCallConv as u32 => CallingConvention::Fast,
            _ if u == LLVMCallConv::LLVMColdCallConv as u32 => CallingConvention::Cold,
            _ if u == LLVMCallConv::LLVMGHCCallConv as u32 => CallingConvention::GHC,
            _ if u == LLVMCallConv::LLVMHiPECallConv as u32 => CallingConvention::HiPE,
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

impl Attribute {
    /// Returns `None` if we encounter an attribute we don't know about or aren't
    /// equipped to handle
    pub(crate) fn from_llvm_ref(a: LLVMAttributeRef) -> Option<Self> {
        if unsafe { LLVMIsEnumAttribute(a) } != 0 {
            Some(Attribute::EnumAttribute {
                kind: unsafe { LLVMGetEnumAttributeKind(a) },
                value: num::NonZeroU64::new(unsafe { LLVMGetEnumAttributeValue(a) }),
            })
        } else if unsafe { LLVMIsStringAttribute(a) } != 0 {
            Some(Attribute::StringAttribute {
                kind: unsafe { get_string_attribute_kind(a) },
                value: unsafe { get_string_attribute_value(a) },
            })
        } else {
            debug!("Encountered an unknown attribute: neither enum nor string");
            None
        }
    }
}
