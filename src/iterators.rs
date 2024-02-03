//! Iterators over various module-level objects

use crate::llvm_sys::*;
use std::iter::Peekable;

pub fn get_defined_functions(module: LLVMModuleRef) -> impl Iterator<Item = LLVMValueRef> {
    FunctionIterator::new(module).filter(|&f| is_defined(f))
}

pub fn get_declared_functions(module: LLVMModuleRef) -> impl Iterator<Item = LLVMValueRef> {
    FunctionIterator::new(module).filter(|&f| !is_defined(f))
}

pub fn get_globals(module: LLVMModuleRef) -> impl Iterator<Item = LLVMValueRef> {
    GlobalIterator::new(module)
}

pub fn get_global_aliases(module: LLVMModuleRef) -> impl Iterator<Item = LLVMValueRef> {
    GlobalAliasIterator::new(module)
}

pub fn get_global_ifuncs(module: LLVMModuleRef) -> impl Iterator<Item = LLVMValueRef> {
    GlobalIFuncIterator::new(module)
}

pub fn get_parameters(func: LLVMValueRef) -> impl Iterator<Item = LLVMValueRef> {
    ParamIterator::new(func)
}

pub fn get_basic_blocks(func: LLVMValueRef) -> impl Iterator<Item = LLVMBasicBlockRef> {
    BasicBlockIterator::new(func)
}

pub fn get_instructions(bb: LLVMBasicBlockRef) -> impl Iterator<Item = LLVMValueRef> {
    InstructionIterator::new(bb)
}

macro_rules! iterator {
    ($struct_name:ident, $parent:ty, $item:ty, $init:ident, $next:ident) => {
        struct $struct_name {
            current: $item,
        }

        impl $struct_name {
            fn new(parent: $parent) -> Self {
                Self {
                    current: unsafe { $init(parent) },
                }
            }
        }

        impl Iterator for $struct_name {
            type Item = $item;

            fn next(&mut self) -> Option<Self::Item> {
                if self.current.is_null() {
                    None
                } else {
                    let rval = self.current;
                    self.current = unsafe { $next(self.current) };
                    Some(rval)
                }
            }
        }
    };
}

iterator!(
    FunctionIterator,
    LLVMModuleRef,
    LLVMValueRef,
    LLVMGetFirstFunction,
    LLVMGetNextFunction
);
iterator!(
    GlobalIterator,
    LLVMModuleRef,
    LLVMValueRef,
    LLVMGetFirstGlobal,
    LLVMGetNextGlobal
);
iterator!(
    GlobalAliasIterator,
    LLVMModuleRef,
    LLVMValueRef,
    LLVMGetFirstGlobalAlias,
    LLVMGetNextGlobalAlias
);
iterator!(
    GlobalIFuncIterator,
    LLVMModuleRef,
    LLVMValueRef,
    LLVMGetFirstGlobalIFunc,
    LLVMGetNextGlobalIFunc
);
iterator!(
    ParamIterator,
    LLVMValueRef,
    LLVMValueRef,
    LLVMGetFirstParam,
    LLVMGetNextParam
);
iterator!(
    BasicBlockIterator,
    LLVMValueRef,
    LLVMBasicBlockRef,
    LLVMGetFirstBasicBlock,
    LLVMGetNextBasicBlock
);
iterator!(
    InstructionIterator,
    LLVMBasicBlockRef,
    LLVMValueRef,
    LLVMGetFirstInstruction,
    LLVMGetNextInstruction
);

pub fn all_but_last<I, T>(i: I) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T>,
{
    let rval: AllButLastIterator<I> = AllButLastIterator::new(i);
    rval
}

struct AllButLastIterator<I: Iterator> {
    p: Peekable<I>,
}

impl<I: Iterator> AllButLastIterator<I> {
    fn new(i: I) -> Self {
        Self { p: i.peekable() }
    }
}

impl<I: Iterator> Iterator for AllButLastIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.p.next() {
            None => None,
            Some(item) => {
                if self.p.peek().is_some() {
                    Some(item)
                } else {
                    None
                }
            },
        }
    }
}

/// Is the function actually defined in this module (as opposed to just declared)
fn is_defined(func: LLVMValueRef) -> bool {
    unsafe { LLVMIsDeclaration(func) == 0 } // note that we inverted the logic: if it IsDeclaration then we return false
}
