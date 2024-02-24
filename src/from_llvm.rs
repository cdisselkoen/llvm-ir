pub use crate::iterators::*;
use crate::llvm_sys::*;
use std::ffi::CStr;
use std::os::raw::c_char;

// We convert all LLVM strings to owned Strings (which involves a copy)
// partly because we intend to serialize/deserialize our ASTs eventually
pub unsafe fn raw_to_string(raw: *const c_char) -> String {
    let cstr = CStr::from_ptr(raw);
    cstr.to_str().expect("Failed to convert CStr").into()
}

macro_rules! wrap {
    ($llvmFunc:ident, $argty:ty, $wrapperFunc:ident) => {
        pub unsafe fn $wrapperFunc(arg: $argty) -> String {
            debug_assert!(!arg.is_null());
            let ptr = $llvmFunc(arg);
            raw_to_string(ptr)
        }
    };
}

macro_rules! wrap_maybe_null {
    ($llvmFunc: ident, $argty:ty, $wrapperFunc:ident) => {
        pub unsafe fn $wrapperFunc(arg: $argty) -> Option<String> {
            debug_assert!(!arg.is_null());
            let ptr = $llvmFunc(arg);
            if ptr.is_null() {
                None
            } else {
                Some(raw_to_string(ptr))
            }
        }
    };
}

macro_rules! wrap_with_len {
    ($llvmFunc:ident, $argty:ty, $wrapperFunc:ident) => {
        pub unsafe fn $wrapperFunc(arg: $argty) -> String {
            debug_assert!(!arg.is_null());
            let mut len = 0;
            let ptr = $llvmFunc(arg, &mut len);
            raw_to_string(ptr)
        }
    };
}

macro_rules! wrap_with_len_maybe_null {
    ($llvmFunc:ident, $argty:ty, $wrapperFunc:ident) => {
        pub unsafe fn $wrapperFunc(arg: $argty) -> Option<String> {
            debug_assert!(!arg.is_null());
            let mut len = 0;
            let ptr = $llvmFunc(arg, &mut len);
            if ptr.is_null() {
                None
            } else {
                Some(raw_to_string(ptr))
            }
        }
    };
}

wrap_with_len!(LLVMGetModuleInlineAsm, LLVMModuleRef, get_module_inline_asm);
wrap_with_len!(
    LLVMGetModuleIdentifier,
    LLVMModuleRef,
    get_module_identifier
);
wrap_with_len!(LLVMGetSourceFileName, LLVMModuleRef, get_source_file_name);
wrap!(LLVMGetDataLayoutStr, LLVMModuleRef, get_data_layout_str);
wrap_maybe_null!(LLVMGetTarget, LLVMModuleRef, get_target);
wrap_with_len!(LLVMGetValueName2, LLVMValueRef, get_value_name);
wrap_maybe_null!(LLVMGetStructName, LLVMTypeRef, get_struct_name);
wrap_maybe_null!(LLVMGetSection, LLVMValueRef, get_section);
wrap_maybe_null!(LLVMGetGC, LLVMValueRef, get_gc);
wrap!(LLVMGetBasicBlockName, LLVMBasicBlockRef, get_bb_name);
wrap!(LLVMPrintValueToString, LLVMValueRef, print_to_string);
// wrap!(LLVMPrintTypeToString, LLVMTypeRef, print_type_to_string);
wrap_with_len!(
    LLVMGetStringAttributeKind,
    LLVMAttributeRef,
    get_string_attribute_kind
);
wrap_with_len!(
    LLVMGetStringAttributeValue,
    LLVMAttributeRef,
    get_string_attribute_value
);
wrap_with_len_maybe_null!(LLVMGetDebugLocFilename, LLVMValueRef, get_debugloc_filename);
wrap_with_len_maybe_null!(
    LLVMGetDebugLocDirectory,
    LLVMValueRef,
    get_debugloc_directory
);

// Panics if the LLVMValueRef is not a basic block
pub unsafe fn op_to_bb(op: LLVMValueRef) -> LLVMBasicBlockRef {
    assert!(LLVMValueIsBasicBlock(op) != 0);
    LLVMValueAsBasicBlock(op)
}

macro_rules! debug {
    ($($arg:expr),+) => {
        if log::log_enabled!(log::Level::Debug) {
            log::debug!($($arg),+)
        }
    };
}

/// LLVM Context wrapper that frees the underlying context when the wrapper is dropped
pub struct Context {
    pub ctx: LLVMContextRef,
}

impl Context {
    pub fn new() -> Self {
        Self {
            ctx: unsafe { LLVMContextCreate() },
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.ctx);
        }
    }
}
