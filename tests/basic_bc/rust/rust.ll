; ModuleID = 'rust.3a1fbbbh-cgu.0'
source_filename = "rust.3a1fbbbh-cgu.0"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

%"core::option::Option<usize>::Some" = type { [1 x i64], i64, [0 x i64] }
%"core::ptr::Repr<isize>" = type { [2 x i64] }
%"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>" = type { [0 x i64], %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock", [0 x i64] }
%"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock" = type { [0 x i64], i64, [0 x i64], i64, [0 x i64], i64, [0 x i64], i64, [0 x i64] }
%"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>" = type { [4 x i64] }
%"core::marker::PhantomData<&isize>" = type {}
%"alloc::vec::Vec<isize>" = type { [0 x i64], { i64*, i64 }, [0 x i64], i64, [0 x i64] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@str.0 = internal constant [72 x i8] c"/rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b/src/libcore/ops/arith.rs"
@str.1 = internal constant [28 x i8] c"attempt to add with overflow"
@panic_loc.2 = private unnamed_addr constant { { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 } { { [0 x i8]*, i64 } { [0 x i8]* bitcast ([28 x i8]* @str.1 to [0 x i8]*), i64 28 }, { [0 x i8]*, i64 } { [0 x i8]* bitcast ([72 x i8]* @str.0 to [0 x i8]*), i64 72 }, i32 100, i32 45 }, align 8
@str.3 = internal constant [57 x i8] c"attempt to calculate the remainder with a divisor of zero"
@panic_loc.4 = private unnamed_addr constant { { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 } { { [0 x i8]*, i64 } { [0 x i8]* bitcast ([57 x i8]* @str.3 to [0 x i8]*), i64 57 }, { [0 x i8]*, i64 } { [0 x i8]* bitcast ([72 x i8]* @str.0 to [0 x i8]*), i64 72 }, i32 528, i32 45 }, align 8
@str.5 = internal constant [48 x i8] c"attempt to calculate the remainder with overflow"
@panic_loc.6 = private unnamed_addr constant { { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 } { { [0 x i8]*, i64 } { [0 x i8]* bitcast ([48 x i8]* @str.5 to [0 x i8]*), i64 48 }, { [0 x i8]*, i64 } { [0 x i8]* bitcast ([72 x i8]* @str.0 to [0 x i8]*), i64 72 }, i32 528, i32 45 }, align 8
@str.7 = internal constant [72 x i8] c"/rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b/src/libcore/slice/mod.rs"
@panic_loc.8 = private unnamed_addr constant { { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 } { { [0 x i8]*, i64 } { [0 x i8]* bitcast ([57 x i8]* @str.3 to [0 x i8]*), i64 57 }, { [0 x i8]*, i64 } { [0 x i8]* bitcast ([72 x i8]* @str.7 to [0 x i8]*), i64 72 }, i32 5216, i32 19 }, align 8
@0 = private unnamed_addr constant <{ [33 x i8] }> <{ [33 x i8] c"attempt to create unaligned slice" }>, align 1
@1 = private unnamed_addr constant <{ [24 x i8] }> <{ [24 x i8] c"src/libcore/slice/mod.rs" }>, align 1
@2 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [33 x i8] }>, <{ [33 x i8] }>* @0, i32 0, i32 0, i32 0), [8 x i8] c"!\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [24 x i8] }>, <{ [24 x i8] }>* @1, i32 0, i32 0, i32 0), [16 x i8] c"\18\00\00\00\00\00\00\00`\14\00\00\05\00\00\00" }>, align 8
@3 = private unnamed_addr constant <{ [55 x i8] }> <{ [55 x i8] c"attempt to create slice covering half the address space" }>, align 1
@4 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [55 x i8] }>, <{ [55 x i8] }>* @3, i32 0, i32 0, i32 0), [8 x i8] c"7\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [24 x i8] }>, <{ [24 x i8] }>* @1, i32 0, i32 0, i32 0), [16 x i8] c"\18\00\00\00\00\00\00\00a\14\00\00\05\00\00\00" }>, align 8
@panic_loc.9 = private unnamed_addr constant { { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 } { { [0 x i8]*, i64 } { [0 x i8]* bitcast ([57 x i8]* @str.3 to [0 x i8]*), i64 57 }, { [0 x i8]*, i64 } { [0 x i8]* bitcast ([72 x i8]* @str.7 to [0 x i8]*), i64 72 }, i32 5237, i32 19 }, align 8
@5 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [33 x i8] }>, <{ [33 x i8] }>* @0, i32 0, i32 0, i32 0), [8 x i8] c"!\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [24 x i8] }>, <{ [24 x i8] }>* @1, i32 0, i32 0, i32 0), [16 x i8] c"\18\00\00\00\00\00\00\00u\14\00\00\05\00\00\00" }>, align 8
@6 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [55 x i8] }>, <{ [55 x i8] }>* @3, i32 0, i32 0, i32 0), [8 x i8] c"7\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [24 x i8] }>, <{ [24 x i8] }>* @1, i32 0, i32 0, i32 0), [16 x i8] c"\18\00\00\00\00\00\00\00v\14\00\00\05\00\00\00" }>, align 8
@panic_bounds_check_loc.a = private unnamed_addr constant { { [0 x i8]*, i64 }, i32, i32 } { { [0 x i8]*, i64 } { [0 x i8]* bitcast ([72 x i8]* @str.7 to [0 x i8]*), i64 72 }, i32 2687, i32 14 }, align 8
@str.b = internal constant [7 x i8] c"rust.rs"
@panic_loc.c = private unnamed_addr constant { { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 } { { [0 x i8]*, i64 } { [0 x i8]* bitcast ([28 x i8]* @str.1 to [0 x i8]*), i64 28 }, { [0 x i8]*, i64 } { [0 x i8]* bitcast ([7 x i8]* @str.b to [0 x i8]*), i64 7 }, i32 6, i32 9 }, align 8
@panic_loc.d = private unnamed_addr constant { { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 } { { [0 x i8]*, i64 } { [0 x i8]* bitcast ([28 x i8]* @str.1 to [0 x i8]*), i64 28 }, { [0 x i8]*, i64 } { [0 x i8]* bitcast ([7 x i8]* @str.b to [0 x i8]*), i64 7 }, i32 9, i32 16 }, align 8

; <isize as core::ops::arith::Add>::add
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN47_$LT$isize$u20$as$u20$core..ops..arith..Add$GT$3add17h5b676053fe71193bE"(i64 %self, i64 %other) unnamed_addr #0 {
start:
  %0 = call { i64, i1 } @llvm.sadd.with.overflow.i64(i64 %self, i64 %other)
  %1 = extractvalue { i64, i1 } %0, 0
  %2 = extractvalue { i64, i1 } %0, 1
  %3 = call i1 @llvm.expect.i1(i1 %2, i1 false)
  br i1 %3, label %panic, label %bb1

bb1:                                              ; preds = %start
  ret i64 %1

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h89241d71a860ed98E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast ({ { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 }* @panic_loc.2 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; <isize as core::ops::arith::Rem>::rem
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN47_$LT$isize$u20$as$u20$core..ops..arith..Rem$GT$3rem17hcd48825ab2f2796fE"(i64 %self, i64 %other) unnamed_addr #0 {
start:
  %0 = icmp eq i64 %other, 0
  %1 = call i1 @llvm.expect.i1(i1 %0, i1 false)
  br i1 %1, label %panic, label %bb1

bb1:                                              ; preds = %start
  %2 = icmp eq i64 %other, -1
  %3 = icmp eq i64 %self, -9223372036854775808
  %4 = and i1 %2, %3
  %5 = call i1 @llvm.expect.i1(i1 %4, i1 false)
  br i1 %5, label %panic1, label %bb2

bb2:                                              ; preds = %bb1
  %6 = srem i64 %self, %other
  ret i64 %6

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h89241d71a860ed98E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast ({ { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 }* @panic_loc.4 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

panic1:                                           ; preds = %bb1
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h89241d71a860ed98E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast ({ { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 }* @panic_loc.6 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; <usize as core::iter::range::Step>::add_usize
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @"_ZN49_$LT$usize$u20$as$u20$core..iter..range..Step$GT$9add_usize17h9709d2abb99d1f02E"(i64* noalias readonly align 8 dereferenceable(8) %self, i64 %n) unnamed_addr #0 {
start:
  %_3 = alloca i64, align 8
; call <T as core::convert::TryFrom<U>>::try_from
  %0 = call i64 @"_ZN53_$LT$T$u20$as$u20$core..convert..TryFrom$LT$U$GT$$GT$8try_from17h8206762129109ef0E"(i64 %n)
  store i64 %0, i64* %_3, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %1 = load i64, i64* %_3, align 8
  %2 = load i64, i64* %self, align 8
; call core::num::<impl usize>::checked_add
  %3 = call { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_add17h9d0539ad7ca701ccE"(i64 %2, i64 %1)
  %4 = extractvalue { i64, i64 } %3, 0
  %5 = extractvalue { i64, i64 } %3, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %6 = insertvalue { i64, i64 } undef, i64 %4, 0
  %7 = insertvalue { i64, i64 } %6, i64 %5, 1
  ret { i64, i64 } %7
}

; core::intrinsics::copy_nonoverlapping
; Function Attrs: inlinehint uwtable
define void @_ZN4core10intrinsics19copy_nonoverlapping17h6fe3ca3ba4278673E(i64* %src, i64* %dst, i64 %count) unnamed_addr #0 {
start:
  %0 = mul i64 8, %count
  %1 = bitcast i64* %dst to i8*
  %2 = bitcast i64* %src to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %1, i8* align 8 %2, i64 %0, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::intrinsics::copy_nonoverlapping
; Function Attrs: inlinehint uwtable
define void @_ZN4core10intrinsics19copy_nonoverlapping17he1b9af375418f5b7E(i8* %src, i8* %dst, i64 %count) unnamed_addr #0 {
start:
  %0 = mul i64 1, %count
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %dst, i8* align 1 %src, i64 %0, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::cmp::impls::<impl core::cmp::PartialOrd for usize>::lt
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN4core3cmp5impls57_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$usize$GT$2lt17hd1f31b451aec6da1E"(i64* noalias readonly align 8 dereferenceable(8) %self, i64* noalias readonly align 8 dereferenceable(8) %other) unnamed_addr #0 {
start:
  %0 = load i64, i64* %self, align 8
  %1 = load i64, i64* %other, align 8
  %2 = icmp ult i64 %0, %1
  ret i1 %2
}

; core::mem::swap
; Function Attrs: inlinehint uwtable
define void @_ZN4core3mem4swap17h8ad51cb2c514cb30E(i64* align 8 dereferenceable(8) %x, i64* align 8 dereferenceable(8) %y) unnamed_addr #0 {
start:
; call core::ptr::swap_nonoverlapping_one
  call void @_ZN4core3ptr23swap_nonoverlapping_one17h12c8b2d352680437E(i64* %x, i64* %y)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::mem::size_of
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3mem7size_of17h62169596611dd2e0E() unnamed_addr #0 {
start:
  %tmp_ret = alloca i64, align 8
  store i64 8, i64* %tmp_ret, align 8
  %0 = load i64, i64* %tmp_ret, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; core::mem::size_of
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3mem7size_of17h68d991970122feb2E() unnamed_addr #0 {
start:
  %tmp_ret = alloca i64, align 8
  store i64 32, i64* %tmp_ret, align 8
  %0 = load i64, i64* %tmp_ret, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; core::mem::size_of
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3mem7size_of17hd7e5cfcb791a5d1fE() unnamed_addr #0 {
start:
  %tmp_ret = alloca i64, align 8
  store i64 8, i64* %tmp_ret, align 8
  %0 = load i64, i64* %tmp_ret, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; core::mem::align_of
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3mem8align_of17h7f293d7f1cd07ba4E() unnamed_addr #0 {
start:
  %tmp_ret = alloca i64, align 8
  store i64 8, i64* %tmp_ret, align 8
  %0 = load i64, i64* %tmp_ret, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; core::num::<impl usize>::checked_add
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_add17h9d0539ad7ca701ccE"(i64 %self, i64 %rhs) unnamed_addr #0 {
start:
  %_0 = alloca { i64, i64 }, align 8
; call core::num::<impl usize>::overflowing_add
  %0 = call { i64, i8 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$15overflowing_add17h154f19bc24a86072E"(i64 %self, i64 %rhs)
  %1 = extractvalue { i64, i8 } %0, 0
  %2 = extractvalue { i64, i8 } %0, 1
  %3 = trunc i8 %2 to i1
  br label %bb1

bb1:                                              ; preds = %start
  br i1 %3, label %bb3, label %bb2

bb2:                                              ; preds = %bb1
  %4 = bitcast { i64, i64 }* %_0 to %"core::option::Option<usize>::Some"*
  %5 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %4, i32 0, i32 1
  store i64 %1, i64* %5, align 8
  %6 = bitcast { i64, i64 }* %_0 to i64*
  store i64 1, i64* %6, align 8
  br label %bb4

bb3:                                              ; preds = %bb1
  %7 = bitcast { i64, i64 }* %_0 to i64*
  store i64 0, i64* %7, align 8
  br label %bb4

bb4:                                              ; preds = %bb2, %bb3
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_0, i32 0, i32 0
  %9 = load i64, i64* %8, align 8, !range !0
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_0, i32 0, i32 1
  %11 = load i64, i64* %10, align 8
  %12 = insertvalue { i64, i64 } undef, i64 %9, 0
  %13 = insertvalue { i64, i64 } %12, i64 %11, 1
  ret { i64, i64 } %13
}

; core::num::<impl usize>::checked_mul
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17h806479b1ea86b354E"(i64 %self, i64 %rhs) unnamed_addr #0 {
start:
  %_0 = alloca { i64, i64 }, align 8
; call core::num::<impl usize>::overflowing_mul
  %0 = call { i64, i8 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$15overflowing_mul17hc6ec270855067cfeE"(i64 %self, i64 %rhs)
  %1 = extractvalue { i64, i8 } %0, 0
  %2 = extractvalue { i64, i8 } %0, 1
  %3 = trunc i8 %2 to i1
  br label %bb1

bb1:                                              ; preds = %start
  br i1 %3, label %bb3, label %bb2

bb2:                                              ; preds = %bb1
  %4 = bitcast { i64, i64 }* %_0 to %"core::option::Option<usize>::Some"*
  %5 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %4, i32 0, i32 1
  store i64 %1, i64* %5, align 8
  %6 = bitcast { i64, i64 }* %_0 to i64*
  store i64 1, i64* %6, align 8
  br label %bb4

bb3:                                              ; preds = %bb1
  %7 = bitcast { i64, i64 }* %_0 to i64*
  store i64 0, i64* %7, align 8
  br label %bb4

bb4:                                              ; preds = %bb2, %bb3
  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_0, i32 0, i32 0
  %9 = load i64, i64* %8, align 8, !range !0
  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_0, i32 0, i32 1
  %11 = load i64, i64* %10, align 8
  %12 = insertvalue { i64, i64 } undef, i64 %9, 0
  %13 = insertvalue { i64, i64 } %12, i64 %11, 1
  ret { i64, i64 } %13
}

; core::num::<impl usize>::saturating_mul
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN4core3num23_$LT$impl$u20$usize$GT$14saturating_mul17hb7b5c8bd037e7bc6E"(i64 %self, i64 %rhs) unnamed_addr #0 {
start:
; call core::num::<impl usize>::checked_mul
  %0 = call { i64, i64 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$11checked_mul17h806479b1ea86b354E"(i64 %self, i64 %rhs)
  %1 = extractvalue { i64, i64 } %0, 0
  %2 = extractvalue { i64, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call core::num::<impl usize>::max_value
  %3 = call i64 @"_ZN4core3num23_$LT$impl$u20$usize$GT$9max_value17h9054e781c42925f6E"()
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::option::Option<T>::unwrap_or
  %4 = call i64 @"_ZN4core6option15Option$LT$T$GT$9unwrap_or17h716b96369b915d77E"(i64 %1, i64 %2, i64 %3)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i64 %4
}

; core::num::<impl usize>::overflowing_add
; Function Attrs: inlinehint uwtable
define internal { i64, i8 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$15overflowing_add17h154f19bc24a86072E"(i64 %self, i64 %rhs) unnamed_addr #0 {
start:
  %tmp_ret = alloca { i64, i8 }, align 8
  %_0 = alloca { i64, i8 }, align 8
  %0 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %self, i64 %rhs)
  %1 = extractvalue { i64, i1 } %0, 0
  %2 = extractvalue { i64, i1 } %0, 1
  %3 = zext i1 %2 to i8
  %4 = bitcast { i64, i8 }* %tmp_ret to i64*
  store i64 %1, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %tmp_ret, i32 0, i32 1
  store i8 %3, i8* %5, align 8
  %6 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %tmp_ret, i32 0, i32 0
  %7 = load i64, i64* %6, align 8
  %8 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %tmp_ret, i32 0, i32 1
  %9 = load i8, i8* %8, align 8, !range !1
  %10 = trunc i8 %9 to i1
  br label %bb1

bb1:                                              ; preds = %start
  %11 = bitcast { i64, i8 }* %_0 to i64*
  store i64 %7, i64* %11, align 8
  %12 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %_0, i32 0, i32 1
  %13 = zext i1 %10 to i8
  store i8 %13, i8* %12, align 8
  %14 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %_0, i32 0, i32 0
  %15 = load i64, i64* %14, align 8
  %16 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %_0, i32 0, i32 1
  %17 = load i8, i8* %16, align 8, !range !1
  %18 = trunc i8 %17 to i1
  %19 = zext i1 %18 to i8
  %20 = insertvalue { i64, i8 } undef, i64 %15, 0
  %21 = insertvalue { i64, i8 } %20, i8 %19, 1
  ret { i64, i8 } %21
}

; core::num::<impl usize>::overflowing_mul
; Function Attrs: inlinehint uwtable
define internal { i64, i8 } @"_ZN4core3num23_$LT$impl$u20$usize$GT$15overflowing_mul17hc6ec270855067cfeE"(i64 %self, i64 %rhs) unnamed_addr #0 {
start:
  %tmp_ret = alloca { i64, i8 }, align 8
  %_0 = alloca { i64, i8 }, align 8
  %0 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %self, i64 %rhs)
  %1 = extractvalue { i64, i1 } %0, 0
  %2 = extractvalue { i64, i1 } %0, 1
  %3 = zext i1 %2 to i8
  %4 = bitcast { i64, i8 }* %tmp_ret to i64*
  store i64 %1, i64* %4, align 8
  %5 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %tmp_ret, i32 0, i32 1
  store i8 %3, i8* %5, align 8
  %6 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %tmp_ret, i32 0, i32 0
  %7 = load i64, i64* %6, align 8
  %8 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %tmp_ret, i32 0, i32 1
  %9 = load i8, i8* %8, align 8, !range !1
  %10 = trunc i8 %9 to i1
  br label %bb1

bb1:                                              ; preds = %start
  %11 = bitcast { i64, i8 }* %_0 to i64*
  store i64 %7, i64* %11, align 8
  %12 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %_0, i32 0, i32 1
  %13 = zext i1 %10 to i8
  store i8 %13, i8* %12, align 8
  %14 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %_0, i32 0, i32 0
  %15 = load i64, i64* %14, align 8
  %16 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %_0, i32 0, i32 1
  %17 = load i8, i8* %16, align 8, !range !1
  %18 = trunc i8 %17 to i1
  %19 = zext i1 %18 to i8
  %20 = insertvalue { i64, i8 } undef, i64 %15, 0
  %21 = insertvalue { i64, i8 } %20, i8 %19, 1
  ret { i64, i8 } %21
}

; core::num::<impl usize>::max_value
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN4core3num23_$LT$impl$u20$usize$GT$9max_value17h9054e781c42925f6E"() unnamed_addr #0 {
start:
  ret i64 -1
}

; core::ptr::swap_nonoverlapping
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr19swap_nonoverlapping17h8775c1adf9b15317E(i64* %x, i64* %y, i64 %count) unnamed_addr #0 {
start:
  %0 = bitcast i64* %x to i8*
  %1 = bitcast i64* %y to i8*
; call core::mem::size_of
  %2 = call i64 @_ZN4core3mem7size_of17h62169596611dd2e0E()
  br label %bb1

bb1:                                              ; preds = %start
  %3 = mul i64 %2, %count
; call core::ptr::swap_nonoverlapping_bytes
  call void @_ZN4core3ptr25swap_nonoverlapping_bytes17h0231428c44134a09E(i8* %0, i8* %1, i64 %3)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; core::ptr::slice_from_raw_parts
; Function Attrs: inlinehint uwtable
define { [0 x i64]*, i64 } @_ZN4core3ptr20slice_from_raw_parts17hceeedbdd3a01887eE(i64* %data, i64 %len) unnamed_addr #0 {
start:
  %_4 = alloca { i64*, i64 }, align 8
  %_3 = alloca %"core::ptr::Repr<isize>", align 8
  %0 = bitcast { i64*, i64 }* %_4 to i64**
  store i64* %data, i64** %0, align 8
  %1 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 1
  store i64 %len, i64* %1, align 8
  %2 = bitcast %"core::ptr::Repr<isize>"* %_3 to { i64*, i64 }*
  %3 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 0
  %4 = load i64*, i64** %3, align 8
  %5 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %2, i32 0, i32 0
  store i64* %4, i64** %7, align 8
  %8 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %2, i32 0, i32 1
  store i64 %6, i64* %8, align 8
  %9 = bitcast %"core::ptr::Repr<isize>"* %_3 to { [0 x i64]*, i64 }*
  %10 = getelementptr inbounds { [0 x i64]*, i64 }, { [0 x i64]*, i64 }* %9, i32 0, i32 0
  %11 = load [0 x i64]*, [0 x i64]** %10, align 8
  %12 = getelementptr inbounds { [0 x i64]*, i64 }, { [0 x i64]*, i64 }* %9, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = insertvalue { [0 x i64]*, i64 } undef, [0 x i64]* %11, 0
  %15 = insertvalue { [0 x i64]*, i64 } %14, i64 %13, 1
  ret { [0 x i64]*, i64 } %15
}

; core::ptr::swap_nonoverlapping_one
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr23swap_nonoverlapping_one17h12c8b2d352680437E(i64* %x, i64* %y) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
  %_18 = alloca i8, align 1
  store i8 0, i8* %_18, align 1
; call core::mem::size_of
  %0 = call i64 @_ZN4core3mem7size_of17h62169596611dd2e0E()
  br label %bb2

bb1:                                              ; preds = %bb10, %bb11
  %1 = bitcast { i8*, i32 }* %personalityslot to i8**
  %2 = load i8*, i8** %1, align 8
  %3 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  %4 = load i32, i32* %3, align 8
  %5 = insertvalue { i8*, i32 } undef, i8* %2, 0
  %6 = insertvalue { i8*, i32 } %5, i32 %4, 1
  resume { i8*, i32 } %6

bb2:                                              ; preds = %start
  %7 = icmp ult i64 %0, 32
  br i1 %7, label %bb4, label %bb3

bb3:                                              ; preds = %bb2
; call core::ptr::swap_nonoverlapping
  call void @_ZN4core3ptr19swap_nonoverlapping17h8775c1adf9b15317E(i64* %x, i64* %y, i64 1)
  br label %bb8

bb4:                                              ; preds = %bb2
  store i8 1, i8* %_18, align 1
; call core::ptr::read
  %8 = call i64 @_ZN4core3ptr4read17h6c5ac2ebfe0041f6E(i64* %x)
  br label %bb5

bb5:                                              ; preds = %bb4
; invoke core::intrinsics::copy_nonoverlapping
  invoke void @_ZN4core10intrinsics19copy_nonoverlapping17h6fe3ca3ba4278673E(i64* %y, i64* %x, i64 1)
          to label %bb6 unwind label %cleanup

bb6:                                              ; preds = %bb5
  store i8 0, i8* %_18, align 1
; invoke core::ptr::write
  invoke void @_ZN4core3ptr5write17h4aea043b48e6b795E(i64* %y, i64 %8)
          to label %bb7 unwind label %cleanup

bb7:                                              ; preds = %bb6
  store i8 0, i8* %_18, align 1
  br label %bb9

bb8:                                              ; preds = %bb3
  br label %bb9

bb9:                                              ; preds = %bb8, %bb7
  ret void

bb10:                                             ; preds = %bb11
  store i8 0, i8* %_18, align 1
  br label %bb1

bb11:                                             ; preds = %cleanup
  %9 = load i8, i8* %_18, align 1, !range !1
  %10 = trunc i8 %9 to i1
  br i1 %10, label %bb10, label %bb1

cleanup:                                          ; preds = %bb6, %bb5
  %11 = landingpad { i8*, i32 }
          cleanup
  %12 = extractvalue { i8*, i32 } %11, 0
  %13 = extractvalue { i8*, i32 } %11, 1
  %14 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 0
  store i8* %12, i8** %14, align 8
  %15 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  store i32 %13, i32* %15, align 8
  br label %bb11
}

; core::ptr::slice_from_raw_parts_mut
; Function Attrs: inlinehint uwtable
define { [0 x i64]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17h94ce74124d6a8a85E(i64* %data, i64 %len) unnamed_addr #0 {
start:
  %_4 = alloca { i64*, i64 }, align 8
  %_3 = alloca %"core::ptr::Repr<isize>", align 8
  %0 = bitcast { i64*, i64 }* %_4 to i64**
  store i64* %data, i64** %0, align 8
  %1 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 1
  store i64 %len, i64* %1, align 8
  %2 = bitcast %"core::ptr::Repr<isize>"* %_3 to { i64*, i64 }*
  %3 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 0
  %4 = load i64*, i64** %3, align 8
  %5 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %_4, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %2, i32 0, i32 0
  store i64* %4, i64** %7, align 8
  %8 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %2, i32 0, i32 1
  store i64 %6, i64* %8, align 8
  %9 = bitcast %"core::ptr::Repr<isize>"* %_3 to { [0 x i64]*, i64 }*
  %10 = getelementptr inbounds { [0 x i64]*, i64 }, { [0 x i64]*, i64 }* %9, i32 0, i32 0
  %11 = load [0 x i64]*, [0 x i64]** %10, align 8
  %12 = getelementptr inbounds { [0 x i64]*, i64 }, { [0 x i64]*, i64 }* %9, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = insertvalue { [0 x i64]*, i64 } undef, [0 x i64]* %11, 0
  %15 = insertvalue { [0 x i64]*, i64 } %14, i64 %13, 1
  ret { [0 x i64]*, i64 } %15
}

; core::ptr::swap_nonoverlapping_bytes
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ptr25swap_nonoverlapping_bytes17h0231428c44134a09E(i8* %x, i8* %y, i64 %len) unnamed_addr #0 {
start:
  %self.i.i2 = alloca %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*, align 8
  %self.i3 = alloca %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*, align 8
  %self.i.i = alloca <4 x i64>*, align 8
  %self.i = alloca <4 x i64>*, align 8
  %t1 = alloca %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>", align 8
  %t = alloca <4 x i64>, align 32
  %i = alloca i64, align 8
; call core::mem::size_of
  %0 = call i64 @_ZN4core3mem7size_of17h68d991970122feb2E()
  br label %bb1

bb1:                                              ; preds = %start
  store i64 0, i64* %i, align 8
  br label %bb2

bb2:                                              ; preds = %bb11, %bb1
  %1 = load i64, i64* %i, align 8
  %2 = add i64 %1, %0
  %3 = icmp ule i64 %2, %len
  br i1 %3, label %bb3, label %bb4

bb3:                                              ; preds = %bb2
  %4 = bitcast <4 x i64>* %t to {}*
  br label %bb5

bb4:                                              ; preds = %bb2
  %5 = load i64, i64* %i, align 8
  %6 = icmp ult i64 %5, %len
  br i1 %6, label %bb12, label %bb20

bb5:                                              ; preds = %bb3
  store <4 x i64>* %t, <4 x i64>** %self.i, align 8
  %7 = load <4 x i64>*, <4 x i64>** %self.i, align 8, !nonnull !2
  store <4 x i64>* %7, <4 x i64>** %self.i.i, align 8
  %8 = load <4 x i64>*, <4 x i64>** %self.i.i, align 8, !nonnull !2
  br label %bb6

bb6:                                              ; preds = %bb5
  %9 = bitcast <4 x i64>* %8 to i8*
  %10 = load i64, i64* %i, align 8
; call core::ptr::<impl *mut T>::add
  %11 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h0420c0e37e430bfcE"(i8* %x, i64 %10)
  br label %bb7

bb7:                                              ; preds = %bb6
  %12 = load i64, i64* %i, align 8
; call core::ptr::<impl *mut T>::add
  %13 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h0420c0e37e430bfcE"(i8* %y, i64 %12)
  br label %bb8

bb8:                                              ; preds = %bb7
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17he1b9af375418f5b7E(i8* %11, i8* %9, i64 %0)
  br label %bb9

bb9:                                              ; preds = %bb8
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17he1b9af375418f5b7E(i8* %13, i8* %11, i64 %0)
  br label %bb10

bb10:                                             ; preds = %bb9
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17he1b9af375418f5b7E(i8* %9, i8* %13, i64 %0)
  br label %bb11

bb11:                                             ; preds = %bb10
  %14 = load i64, i64* %i, align 8
  %15 = add i64 %14, %0
  store i64 %15, i64* %i, align 8
  br label %bb2

bb12:                                             ; preds = %bb4
  %16 = bitcast %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %t1 to {}*
  br label %bb13

bb13:                                             ; preds = %bb12
  %17 = load i64, i64* %i, align 8
  %18 = sub i64 %len, %17
  store %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %t1, %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"** %self.i3, align 8
  %19 = load %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*, %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"** %self.i3, align 8, !nonnull !2
  %20 = bitcast %"core::mem::maybe_uninit::MaybeUninit<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %19 to %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*
  store %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %20, %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"** %self.i.i2, align 8
  %21 = load %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"*, %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"** %self.i.i2, align 8, !nonnull !2
  %22 = bitcast %"core::mem::manually_drop::ManuallyDrop<core::ptr::swap_nonoverlapping_bytes::UnalignedBlock>"* %21 to %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock"*
  br label %bb14

bb14:                                             ; preds = %bb13
  %23 = bitcast %"core::ptr::swap_nonoverlapping_bytes::UnalignedBlock"* %22 to i8*
  %24 = load i64, i64* %i, align 8
; call core::ptr::<impl *mut T>::add
  %25 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h0420c0e37e430bfcE"(i8* %x, i64 %24)
  br label %bb15

bb15:                                             ; preds = %bb14
  %26 = load i64, i64* %i, align 8
; call core::ptr::<impl *mut T>::add
  %27 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h0420c0e37e430bfcE"(i8* %y, i64 %26)
  br label %bb16

bb16:                                             ; preds = %bb15
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17he1b9af375418f5b7E(i8* %25, i8* %23, i64 %18)
  br label %bb17

bb17:                                             ; preds = %bb16
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17he1b9af375418f5b7E(i8* %27, i8* %25, i64 %18)
  br label %bb18

bb18:                                             ; preds = %bb17
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17he1b9af375418f5b7E(i8* %23, i8* %27, i64 %18)
  br label %bb19

bb19:                                             ; preds = %bb18
  br label %bb20

bb20:                                             ; preds = %bb19, %bb4
  ret void
}

; core::ptr::<impl *mut T>::add
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$3add17h0420c0e37e430bfcE"(i8* %self, i64 %count) unnamed_addr #0 {
start:
; call core::ptr::<impl *mut T>::offset
  %0 = call i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h59fdf5f61cd3626bE"(i8* %self, i64 %count)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %0
}

; core::ptr::<impl *mut T>::offset
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h59fdf5f61cd3626bE"(i8* %self, i64 %count) unnamed_addr #0 {
start:
  %tmp_ret = alloca i8*, align 8
  %0 = getelementptr inbounds i8, i8* %self, i64 %count
  store i8* %0, i8** %tmp_ret, align 8
  %1 = load i8*, i8** %tmp_ret, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %1
}

; core::ptr::<impl *mut T>::is_null
; Function Attrs: inlinehint uwtable
define zeroext i1 @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17ha979f8c7c468a2b4E"(i64* %self) unnamed_addr #0 {
start:
  %0 = bitcast i64* %self to i8*
; call core::ptr::null_mut
  %1 = call i8* @_ZN4core3ptr8null_mut17had996423b354d73cE()
  br label %bb1

bb1:                                              ; preds = %start
  %2 = icmp eq i8* %0, %1
  ret i1 %2
}

; core::ptr::<impl *const T>::wrapping_add
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$12wrapping_add17h796bc2d3568076dbE"(i8* %self, i64 %count) unnamed_addr #0 {
start:
; call core::ptr::<impl *const T>::wrapping_offset
  %0 = call i8* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$15wrapping_offset17h45b3ac03c2a8679dE"(i8* %self, i64 %count)
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %0
}

; core::ptr::<impl *const T>::wrapping_offset
; Function Attrs: inlinehint uwtable
define i8* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$15wrapping_offset17h45b3ac03c2a8679dE"(i8* %self, i64 %count) unnamed_addr #0 {
start:
  %tmp_ret = alloca i8*, align 8
  %0 = getelementptr i8, i8* %self, i64 %count
  store i8* %0, i8** %tmp_ret, align 8
  %1 = load i8*, i8** %tmp_ret, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i8* %1
}

; core::ptr::<impl *const T>::add
; Function Attrs: inlinehint uwtable
define i64* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$3add17hca3a566798bffda1E"(i64* %self, i64 %count) unnamed_addr #0 {
start:
; call core::ptr::<impl *const T>::offset
  %0 = call i64* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17h66d880d9e1cefb86E"(i64* %self, i64 %count)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64* %0
}

; core::ptr::<impl *const T>::offset
; Function Attrs: inlinehint uwtable
define i64* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17h66d880d9e1cefb86E"(i64* %self, i64 %count) unnamed_addr #0 {
start:
  %tmp_ret = alloca i64*, align 8
  %0 = getelementptr inbounds i64, i64* %self, i64 %count
  store i64* %0, i64** %tmp_ret, align 8
  %1 = load i64*, i64** %tmp_ret, align 8
  br label %bb1

bb1:                                              ; preds = %start
  ret i64* %1
}

; core::ptr::<impl *const T>::is_null
; Function Attrs: inlinehint uwtable
define zeroext i1 @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17h2cb5e5d9be44649cE"(i64* %self) unnamed_addr #0 {
start:
  %0 = bitcast i64* %self to i8*
; call core::ptr::null
  %1 = call i8* @_ZN4core3ptr4null17h6f7f1f48fb5b772cE()
  br label %bb1

bb1:                                              ; preds = %start
  %2 = icmp eq i8* %0, %1
  ret i1 %2
}

; core::ptr::null
; Function Attrs: inlinehint uwtable
define i8* @_ZN4core3ptr4null17h6f7f1f48fb5b772cE() unnamed_addr #0 {
start:
  ret i8* null
}

; core::ptr::read
; Function Attrs: inlinehint uwtable
define i64 @_ZN4core3ptr4read17h6c5ac2ebfe0041f6E(i64* %src) unnamed_addr #0 {
start:
  %self.i.i = alloca i64*, align 8
  %self.i = alloca i64*, align 8
  %_0.i = alloca i64, align 8
  %tmp = alloca i64, align 8
  %0 = bitcast i64* %_0.i to {}*
  %1 = load i64, i64* %_0.i, align 8
  store i64 %1, i64* %tmp, align 8
  br label %bb1

bb1:                                              ; preds = %start
  store i64* %tmp, i64** %self.i, align 8
  %2 = load i64*, i64** %self.i, align 8, !nonnull !2
  store i64* %2, i64** %self.i.i, align 8
  %3 = load i64*, i64** %self.i.i, align 8, !nonnull !2
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::intrinsics::copy_nonoverlapping
  call void @_ZN4core10intrinsics19copy_nonoverlapping17h6fe3ca3ba4278673E(i64* %src, i64* %3, i64 1)
  br label %bb3

bb3:                                              ; preds = %bb2
  %4 = load i64, i64* %tmp, align 8
  br label %bb4

bb4:                                              ; preds = %bb3
  ret i64 %4
}

; core::ptr::write
; Function Attrs: inlinehint uwtable
define void @_ZN4core3ptr5write17h4aea043b48e6b795E(i64* %dst, i64 %src) unnamed_addr #0 {
start:
  store i64 %src, i64* %dst, align 8
  ret void
}

; core::ptr::unique::Unique<T>::as_ptr
; Function Attrs: inlinehint uwtable
define i64* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17haf927ead46b1fb4dE"(i64* nonnull %self) unnamed_addr #0 {
start:
  ret i64* %self
}

; core::ptr::null_mut
; Function Attrs: inlinehint uwtable
define i8* @_ZN4core3ptr8null_mut17had996423b354d73cE() unnamed_addr #0 {
start:
  ret i8* null
}

; core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next
; Function Attrs: inlinehint uwtable
define { i64, i64 } @"_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h28f5ccce6734b660E"({ i64, i64 }* align 8 dereferenceable(16)) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
  %_15 = alloca i8, align 1
  %n = alloca i64, align 8
  %_5 = alloca { i64, i64 }, align 8
  %_0 = alloca { i64, i64 }, align 8
  %self = alloca { i64, i64 }*, align 8
  store { i64, i64 }* %0, { i64, i64 }** %self, align 8
  store i8 0, i8* %_15, align 1
  %1 = load { i64, i64 }*, { i64, i64 }** %self, align 8, !nonnull !2
  %2 = bitcast { i64, i64 }* %1 to i64*
  %3 = load { i64, i64 }*, { i64, i64 }** %self, align 8, !nonnull !2
  %4 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %3, i32 0, i32 1
; call core::cmp::impls::<impl core::cmp::PartialOrd for usize>::lt
  %5 = call zeroext i1 @"_ZN4core3cmp5impls57_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$usize$GT$2lt17hd1f31b451aec6da1E"(i64* noalias readonly align 8 dereferenceable(8) %2, i64* noalias readonly align 8 dereferenceable(8) %4)
  br label %bb2

bb1:                                              ; preds = %bb12, %bb9
  %6 = bitcast { i8*, i32 }* %personalityslot to i8**
  %7 = load i8*, i8** %6, align 8
  %8 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  %9 = load i32, i32* %8, align 8
  %10 = insertvalue { i8*, i32 } undef, i8* %7, 0
  %11 = insertvalue { i8*, i32 } %10, i32 %9, 1
  resume { i8*, i32 } %11

bb2:                                              ; preds = %start
  br i1 %5, label %bb4, label %bb3

bb3:                                              ; preds = %bb2
  %12 = bitcast { i64, i64 }* %_0 to i64*
  store i64 0, i64* %12, align 8
  br label %bb11

bb4:                                              ; preds = %bb2
  %13 = load { i64, i64 }*, { i64, i64 }** %self, align 8, !nonnull !2
  %14 = bitcast { i64, i64 }* %13 to i64*
  store i8 1, i8* %_15, align 1
; call <usize as core::iter::range::Step>::add_usize
  %15 = call { i64, i64 } @"_ZN49_$LT$usize$u20$as$u20$core..iter..range..Step$GT$9add_usize17h9709d2abb99d1f02E"(i64* noalias readonly align 8 dereferenceable(8) %14, i64 1)
  store { i64, i64 } %15, { i64, i64 }* %_5, align 8
  br label %bb5

bb5:                                              ; preds = %bb4
  %16 = bitcast { i64, i64 }* %_5 to i64*
  %17 = load i64, i64* %16, align 8, !range !0
  %18 = icmp eq i64 %17, 1
  br i1 %18, label %bb7, label %bb6

bb6:                                              ; preds = %bb5
  %19 = bitcast { i64, i64 }* %_0 to i64*
  store i64 0, i64* %19, align 8
  br label %bb17

bb7:                                              ; preds = %bb5
  store i8 0, i8* %_15, align 1
  %20 = bitcast { i64, i64 }* %_5 to %"core::option::Option<usize>::Some"*
  %21 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %20, i32 0, i32 1
  %22 = load i64, i64* %21, align 8
  store i64 %22, i64* %n, align 8
  %23 = load { i64, i64 }*, { i64, i64 }** %self, align 8, !nonnull !2
  %24 = bitcast { i64, i64 }* %23 to i64*
; invoke core::mem::swap
  invoke void @_ZN4core3mem4swap17h8ad51cb2c514cb30E(i64* align 8 dereferenceable(8) %n, i64* align 8 dereferenceable(8) %24)
          to label %bb8 unwind label %cleanup

bb8:                                              ; preds = %bb7
  %25 = load i64, i64* %n, align 8
  %26 = bitcast { i64, i64 }* %_0 to %"core::option::Option<usize>::Some"*
  %27 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %26, i32 0, i32 1
  store i64 %25, i64* %27, align 8
  %28 = bitcast { i64, i64 }* %_0 to i64*
  store i64 1, i64* %28, align 8
  br label %bb17

bb9:                                              ; preds = %bb10
  %29 = bitcast { i64, i64 }* %_5 to i64*
  %30 = load i64, i64* %29, align 8, !range !0
  %31 = icmp eq i64 %30, 1
  br i1 %31, label %bb1, label %bb12

bb10:                                             ; preds = %cleanup
  br label %bb9

bb11:                                             ; preds = %bb3, %bb13
  %32 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_0, i32 0, i32 0
  %33 = load i64, i64* %32, align 8, !range !0
  %34 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_0, i32 0, i32 1
  %35 = load i64, i64* %34, align 8
  %36 = insertvalue { i64, i64 } undef, i64 %33, 0
  %37 = insertvalue { i64, i64 } %36, i64 %35, 1
  ret { i64, i64 } %37

bb12:                                             ; preds = %bb9
  br label %bb1

bb13:                                             ; preds = %bb15, %bb14, %bb16
  store i8 0, i8* %_15, align 1
  br label %bb11

bb14:                                             ; preds = %bb17
  %38 = load i8, i8* %_15, align 1, !range !1
  %39 = trunc i8 %38 to i1
  br i1 %39, label %bb15, label %bb13

bb15:                                             ; preds = %bb14
  store i8 0, i8* %_15, align 1
  br label %bb13

bb16:                                             ; preds = %bb17
  br label %bb13

bb17:                                             ; preds = %bb8, %bb6
  %40 = bitcast { i64, i64 }* %_5 to i64*
  %41 = load i64, i64* %40, align 8, !range !0
  %42 = icmp eq i64 %41, 1
  br i1 %42, label %bb14, label %bb16

cleanup:                                          ; preds = %bb7
  %43 = landingpad { i8*, i32 }
          cleanup
  %44 = extractvalue { i8*, i32 } %43, 0
  %45 = extractvalue { i8*, i32 } %43, 1
  %46 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 0
  store i8* %44, i8** %46, align 8
  %47 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  store i32 %45, i32* %47, align 8
  br label %bb10
}

; core::slice::from_raw_parts
; Function Attrs: inlinehint uwtable
define { [0 x i64]*, i64 } @_ZN4core5slice14from_raw_parts17h35540e84de7ba669E(i64* %data, i64 %len) unnamed_addr #0 {
start:
  br i1 false, label %bb1, label %bb6

bb1:                                              ; preds = %start
  %0 = ptrtoint i64* %data to i64
; call core::mem::align_of
  %1 = call i64 @_ZN4core3mem8align_of17h7f293d7f1cd07ba4E()
  br label %bb2

bb2:                                              ; preds = %bb1
  %2 = icmp eq i64 %1, 0
  %3 = call i1 @llvm.expect.i1(i1 %2, i1 false)
  br i1 %3, label %panic, label %bb3

bb3:                                              ; preds = %bb2
  %4 = urem i64 %0, %1
  %5 = icmp eq i64 %4, 0
  %6 = xor i1 %5, true
  br i1 %6, label %bb5, label %bb4

bb4:                                              ; preds = %bb3
  br label %bb6

bb5:                                              ; preds = %bb3
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h89241d71a860ed98E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @2 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

bb6:                                              ; preds = %bb4, %start
  br i1 false, label %bb7, label %bb12

bb7:                                              ; preds = %bb6
; call core::mem::size_of
  %7 = call i64 @_ZN4core3mem7size_of17hd7e5cfcb791a5d1fE()
  br label %bb8

bb8:                                              ; preds = %bb7
; call core::num::<impl usize>::saturating_mul
  %8 = call i64 @"_ZN4core3num23_$LT$impl$u20$usize$GT$14saturating_mul17hb7b5c8bd037e7bc6E"(i64 %7, i64 %len)
  br label %bb9

bb9:                                              ; preds = %bb8
  %9 = icmp ule i64 %8, 9223372036854775807
  %10 = xor i1 %9, true
  br i1 %10, label %bb11, label %bb10

bb10:                                             ; preds = %bb9
  br label %bb12

bb11:                                             ; preds = %bb9
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h89241d71a860ed98E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @4 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

bb12:                                             ; preds = %bb10, %bb6
; call core::ptr::slice_from_raw_parts
  %11 = call { [0 x i64]*, i64 } @_ZN4core3ptr20slice_from_raw_parts17hceeedbdd3a01887eE(i64* %data, i64 %len)
  %12 = extractvalue { [0 x i64]*, i64 } %11, 0
  %13 = extractvalue { [0 x i64]*, i64 } %11, 1
  br label %bb13

bb13:                                             ; preds = %bb12
  %14 = insertvalue { [0 x i64]*, i64 } undef, [0 x i64]* %12, 0
  %15 = insertvalue { [0 x i64]*, i64 } %14, i64 %13, 1
  ret { [0 x i64]*, i64 } %15

panic:                                            ; preds = %bb2
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h89241d71a860ed98E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast ({ { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 }* @panic_loc.8 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; core::slice::from_raw_parts_mut
; Function Attrs: inlinehint uwtable
define { [0 x i64]*, i64 } @_ZN4core5slice18from_raw_parts_mut17hedc6f5a5b623fa49E(i64* %data, i64 %len) unnamed_addr #0 {
start:
  br i1 false, label %bb1, label %bb6

bb1:                                              ; preds = %start
  %0 = ptrtoint i64* %data to i64
; call core::mem::align_of
  %1 = call i64 @_ZN4core3mem8align_of17h7f293d7f1cd07ba4E()
  br label %bb2

bb2:                                              ; preds = %bb1
  %2 = icmp eq i64 %1, 0
  %3 = call i1 @llvm.expect.i1(i1 %2, i1 false)
  br i1 %3, label %panic, label %bb3

bb3:                                              ; preds = %bb2
  %4 = urem i64 %0, %1
  %5 = icmp eq i64 %4, 0
  %6 = xor i1 %5, true
  br i1 %6, label %bb5, label %bb4

bb4:                                              ; preds = %bb3
  br label %bb6

bb5:                                              ; preds = %bb3
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h89241d71a860ed98E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @5 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

bb6:                                              ; preds = %bb4, %start
  br i1 false, label %bb7, label %bb12

bb7:                                              ; preds = %bb6
; call core::mem::size_of
  %7 = call i64 @_ZN4core3mem7size_of17hd7e5cfcb791a5d1fE()
  br label %bb8

bb8:                                              ; preds = %bb7
; call core::num::<impl usize>::saturating_mul
  %8 = call i64 @"_ZN4core3num23_$LT$impl$u20$usize$GT$14saturating_mul17hb7b5c8bd037e7bc6E"(i64 %7, i64 %len)
  br label %bb9

bb9:                                              ; preds = %bb8
  %9 = icmp ule i64 %8, 9223372036854775807
  %10 = xor i1 %9, true
  br i1 %10, label %bb11, label %bb10

bb10:                                             ; preds = %bb9
  br label %bb12

bb11:                                             ; preds = %bb9
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h89241d71a860ed98E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @6 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

bb12:                                             ; preds = %bb10, %bb6
; call core::ptr::slice_from_raw_parts_mut
  %11 = call { [0 x i64]*, i64 } @_ZN4core3ptr24slice_from_raw_parts_mut17h94ce74124d6a8a85E(i64* %data, i64 %len)
  %12 = extractvalue { [0 x i64]*, i64 } %11, 0
  %13 = extractvalue { [0 x i64]*, i64 } %11, 1
  br label %bb13

bb13:                                             ; preds = %bb12
  %14 = insertvalue { [0 x i64]*, i64 } undef, [0 x i64]* %12, 0
  %15 = insertvalue { [0 x i64]*, i64 } %14, i64 %13, 1
  ret { [0 x i64]*, i64 } %15

panic:                                            ; preds = %bb2
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h89241d71a860ed98E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast ({ { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 }* @panic_loc.9 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; core::slice::<impl [T]>::len
; Function Attrs: inlinehint uwtable
define i64 @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17hd2800026b116b3aeE"([0 x i64]* noalias nonnull readonly align 8 %self.0, i64 %self.1) unnamed_addr #0 {
start:
  %_2 = alloca %"core::ptr::Repr<isize>", align 8
  %0 = bitcast %"core::ptr::Repr<isize>"* %_2 to { [0 x i64]*, i64 }*
  %1 = getelementptr inbounds { [0 x i64]*, i64 }, { [0 x i64]*, i64 }* %0, i32 0, i32 0
  store [0 x i64]* %self.0, [0 x i64]** %1, align 8
  %2 = getelementptr inbounds { [0 x i64]*, i64 }, { [0 x i64]*, i64 }* %0, i32 0, i32 1
  store i64 %self.1, i64* %2, align 8
  %3 = bitcast %"core::ptr::Repr<isize>"* %_2 to { i64*, i64 }*
  %4 = getelementptr inbounds { i64*, i64 }, { i64*, i64 }* %3, i32 0, i32 1
  %5 = load i64, i64* %4, align 8
  ret i64 %5
}

; core::slice::<impl [T]>::iter
; Function Attrs: inlinehint uwtable
define { i64*, i64* } @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$4iter17h0f94ceb2d99e4dddE"([0 x i64]* noalias nonnull readonly align 8 %self.0, i64 %self.1) unnamed_addr #0 {
start:
  %_21 = alloca %"core::marker::PhantomData<&isize>", align 1
  %end = alloca i64*, align 8
  %_0 = alloca { i64*, i64* }, align 8
; call core::slice::<impl [T]>::as_ptr
  %0 = call i64* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6as_ptr17h889bd891d50bb3bfE"([0 x i64]* noalias nonnull readonly align 8 %self.0, i64 %self.1)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::<impl *const T>::is_null
  %1 = call zeroext i1 @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17h2cb5e5d9be44649cE"(i64* %0)
  br label %bb2

bb2:                                              ; preds = %bb1
  %2 = xor i1 %1, true
  call void @llvm.assume(i1 %2)
  br label %bb3

bb3:                                              ; preds = %bb2
; call core::mem::size_of
  %3 = call i64 @_ZN4core3mem7size_of17hd7e5cfcb791a5d1fE()
  br label %bb4

bb4:                                              ; preds = %bb3
  %4 = icmp eq i64 %3, 0
  br i1 %4, label %bb6, label %bb5

bb5:                                              ; preds = %bb4
; call core::slice::<impl [T]>::len
  %5 = call i64 @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17hd2800026b116b3aeE"([0 x i64]* noalias nonnull readonly align 8 %self.0, i64 %self.1)
  br label %bb9

bb6:                                              ; preds = %bb4
  %6 = bitcast i64* %0 to i8*
; call core::slice::<impl [T]>::len
  %7 = call i64 @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$3len17hd2800026b116b3aeE"([0 x i64]* noalias nonnull readonly align 8 %self.0, i64 %self.1)
  br label %bb7

bb7:                                              ; preds = %bb6
; call core::ptr::<impl *const T>::wrapping_add
  %8 = call i8* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$12wrapping_add17h796bc2d3568076dbE"(i8* %6, i64 %7)
  br label %bb8

bb8:                                              ; preds = %bb7
  %9 = bitcast i8* %8 to i64*
  store i64* %9, i64** %end, align 8
  br label %bb11

bb9:                                              ; preds = %bb5
; call core::ptr::<impl *const T>::add
  %10 = call i64* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$3add17hca3a566798bffda1E"(i64* %0, i64 %5)
  store i64* %10, i64** %end, align 8
  br label %bb10

bb10:                                             ; preds = %bb9
  br label %bb11

bb11:                                             ; preds = %bb10, %bb8
  %11 = load i64*, i64** %end, align 8
  %12 = bitcast { i64*, i64* }* %_0 to i64**
  store i64* %0, i64** %12, align 8
  %13 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_0, i32 0, i32 1
  store i64* %11, i64** %13, align 8
  %14 = bitcast { i64*, i64* }* %_0 to %"core::marker::PhantomData<&isize>"*
  %15 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_0, i32 0, i32 0
  %16 = load i64*, i64** %15, align 8
  %17 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %_0, i32 0, i32 1
  %18 = load i64*, i64** %17, align 8
  %19 = insertvalue { i64*, i64* } undef, i64* %16, 0
  %20 = insertvalue { i64*, i64* } %19, i64* %18, 1
  ret { i64*, i64* } %20
}

; core::slice::<impl [T]>::as_ptr
; Function Attrs: inlinehint uwtable
define i64* @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6as_ptr17h889bd891d50bb3bfE"([0 x i64]* noalias nonnull readonly align 8 %self.0, i64 %self.1) unnamed_addr #0 {
start:
  %0 = bitcast [0 x i64]* %self.0 to i64*
  ret i64* %0
}

; core::slice::<impl core::ops::index::IndexMut<I> for [T]>::index_mut
; Function Attrs: inlinehint uwtable
define align 8 dereferenceable(8) i64* @"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17hae2665d649d0961bE"([0 x i64]* nonnull align 8 %self.0, i64 %self.1, i64 %index) unnamed_addr #0 {
start:
; call <usize as core::slice::SliceIndex<[T]>>::index_mut
  %0 = call align 8 dereferenceable(8) i64* @"_ZN68_$LT$usize$u20$as$u20$core..slice..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$9index_mut17hd852e2f14eb0bd1dE"(i64 %index, [0 x i64]* nonnull align 8 %self.0, i64 %self.1)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64* %0
}

; core::option::Option<T>::unwrap_or
; Function Attrs: inlinehint uwtable
define i64 @"_ZN4core6option15Option$LT$T$GT$9unwrap_or17h716b96369b915d77E"(i64, i64, i64 %def) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
  %_6 = alloca i8, align 1
  %_5 = alloca i8, align 1
  %_0 = alloca i64, align 8
  %self = alloca { i64, i64 }, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 0
  store i64 %0, i64* %2, align 8
  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i32 0, i32 1
  store i64 %1, i64* %3, align 8
  store i8 0, i8* %_5, align 1
  store i8 0, i8* %_6, align 1
  store i8 1, i8* %_5, align 1
  store i8 1, i8* %_6, align 1
  %4 = bitcast { i64, i64 }* %self to i64*
  %5 = load i64, i64* %4, align 8, !range !0
  switch i64 %5, label %bb3 [
    i64 0, label %bb2
    i64 1, label %bb4
  ]

bb1:                                              ; preds = %bb8, %bb7, %bb9
  %6 = bitcast { i8*, i32 }* %personalityslot to i8**
  %7 = load i8*, i8** %6, align 8
  %8 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  %9 = load i32, i32* %8, align 8
  %10 = insertvalue { i8*, i32 } undef, i8* %7, 0
  %11 = insertvalue { i8*, i32 } %10, i32 %9, 1
  resume { i8*, i32 } %11

bb2:                                              ; preds = %start
  store i8 0, i8* %_6, align 1
  store i64 %def, i64* %_0, align 8
  br label %bb11

bb3:                                              ; preds = %start
  unreachable

bb4:                                              ; preds = %start
  store i8 0, i8* %_5, align 1
  %12 = bitcast { i64, i64 }* %self to %"core::option::Option<usize>::Some"*
  %13 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %12, i32 0, i32 1
  %14 = load i64, i64* %13, align 8
  store i64 %14, i64* %_0, align 8
  br label %bb11

bb5:                                              ; No predecessors!
  %15 = bitcast { i64, i64 }* %self to i64*
  %16 = load i64, i64* %15, align 8, !range !0
  %17 = icmp eq i64 %16, 1
  br i1 %17, label %bb7, label %bb9

bb6:                                              ; preds = %bb10, %bb11
  %18 = bitcast { i64, i64 }* %self to i64*
  %19 = load i64, i64* %18, align 8, !range !0
  %20 = icmp eq i64 %19, 1
  br i1 %20, label %bb13, label %bb15

bb7:                                              ; preds = %bb5
  %21 = load i8, i8* %_5, align 1, !range !1
  %22 = trunc i8 %21 to i1
  br i1 %22, label %bb8, label %bb1

bb8:                                              ; preds = %bb7
  store i8 0, i8* %_5, align 1
  br label %bb1

bb9:                                              ; preds = %bb5
  br label %bb1

bb10:                                             ; preds = %bb11
  store i8 0, i8* %_6, align 1
  br label %bb6

bb11:                                             ; preds = %bb2, %bb4
  %23 = load i8, i8* %_6, align 1, !range !1
  %24 = trunc i8 %23 to i1
  br i1 %24, label %bb10, label %bb6

bb12:                                             ; preds = %bb14, %bb13, %bb15
  %25 = load i64, i64* %_0, align 8
  ret i64 %25

bb13:                                             ; preds = %bb6
  %26 = load i8, i8* %_5, align 1, !range !1
  %27 = trunc i8 %26 to i1
  br i1 %27, label %bb14, label %bb12

bb14:                                             ; preds = %bb13
  store i8 0, i8* %_5, align 1
  br label %bb12

bb15:                                             ; preds = %bb6
  br label %bb12
}

; <T as core::convert::From<T>>::from
; Function Attrs: uwtable
define i64 @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hf819fa74a5f891eeE"(i64 %t) unnamed_addr #1 {
start:
  ret i64 %t
}

; <T as core::convert::Into<U>>::into
; Function Attrs: uwtable
define i64 @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hab5a7b5d22211336E"(i64 %self) unnamed_addr #1 {
start:
; call <T as core::convert::From<T>>::from
  %0 = call i64 @"_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17hf819fa74a5f891eeE"(i64 %self)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %0
}

; <T as core::convert::TryFrom<U>>::try_from
; Function Attrs: uwtable
define i64 @"_ZN53_$LT$T$u20$as$u20$core..convert..TryFrom$LT$U$GT$$GT$8try_from17h8206762129109ef0E"(i64 %value) unnamed_addr #1 {
start:
  %_0 = alloca i64, align 8
; call <T as core::convert::Into<U>>::into
  %0 = call i64 @"_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17hab5a7b5d22211336E"(i64 %value)
  br label %bb1

bb1:                                              ; preds = %start
  store i64 %0, i64* %_0, align 8
  %1 = load i64, i64* %_0, align 8
  ret i64 %1
}

; alloc::vec::Vec<T>::as_mut_ptr
; Function Attrs: inlinehint uwtable
define i64* @"_ZN5alloc3vec12Vec$LT$T$GT$10as_mut_ptr17h7db90383f1af03efE"(%"alloc::vec::Vec<isize>"* align 8 dereferenceable(24)) unnamed_addr #0 {
start:
  %self = alloca %"alloc::vec::Vec<isize>"*, align 8
  store %"alloc::vec::Vec<isize>"* %0, %"alloc::vec::Vec<isize>"** %self, align 8
  %1 = load %"alloc::vec::Vec<isize>"*, %"alloc::vec::Vec<isize>"** %self, align 8, !nonnull !2
  %2 = bitcast %"alloc::vec::Vec<isize>"* %1 to { i64*, i64 }*
; call alloc::raw_vec::RawVec<T,A>::ptr
  %3 = call i64* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17h14cfb4671224e24dE"({ i64*, i64 }* noalias readonly align 8 dereferenceable(16) %2)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::<impl *mut T>::is_null
  %4 = call zeroext i1 @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17ha979f8c7c468a2b4E"(i64* %3)
  br label %bb2

bb2:                                              ; preds = %bb1
  %5 = xor i1 %4, true
  call void @llvm.assume(i1 %5)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i64* %3
}

; alloc::vec::Vec<T>::as_ptr
; Function Attrs: inlinehint uwtable
define i64* @"_ZN5alloc3vec12Vec$LT$T$GT$6as_ptr17h6f8c42ce0d643832E"(%"alloc::vec::Vec<isize>"* noalias readonly align 8 dereferenceable(24)) unnamed_addr #0 {
start:
  %self = alloca %"alloc::vec::Vec<isize>"*, align 8
  store %"alloc::vec::Vec<isize>"* %0, %"alloc::vec::Vec<isize>"** %self, align 8
  %1 = load %"alloc::vec::Vec<isize>"*, %"alloc::vec::Vec<isize>"** %self, align 8, !nonnull !2
  %2 = bitcast %"alloc::vec::Vec<isize>"* %1 to { i64*, i64 }*
; call alloc::raw_vec::RawVec<T,A>::ptr
  %3 = call i64* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17h14cfb4671224e24dE"({ i64*, i64 }* noalias readonly align 8 dereferenceable(16) %2)
  br label %bb1

bb1:                                              ; preds = %start
; call core::ptr::<impl *mut T>::is_null
  %4 = call zeroext i1 @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17ha979f8c7c468a2b4E"(i64* %3)
  br label %bb2

bb2:                                              ; preds = %bb1
  %5 = xor i1 %4, true
  call void @llvm.assume(i1 %5)
  br label %bb3

bb3:                                              ; preds = %bb2
  ret i64* %3
}

; alloc::raw_vec::RawVec<T,A>::ptr
; Function Attrs: uwtable
define i64* @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17h14cfb4671224e24dE"({ i64*, i64 }* noalias readonly align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %0 = bitcast { i64*, i64 }* %self to i64**
  %1 = load i64*, i64** %0, align 8, !nonnull !2
; call core::ptr::unique::Unique<T>::as_ptr
  %2 = call i64* @"_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17haf927ead46b1fb4dE"(i64* nonnull %1)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64* %2
}

; <I as core::iter::traits::collect::IntoIterator>::into_iter
; Function Attrs: uwtable
define { i64*, i64* } @"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h063b82e0a27c293dE"(i64* %self.0, i64* %self.1) unnamed_addr #1 {
start:
  %0 = insertvalue { i64*, i64* } undef, i64* %self.0, 0
  %1 = insertvalue { i64*, i64* } %0, i64* %self.1, 1
  ret { i64*, i64* } %1
}

; <I as core::iter::traits::collect::IntoIterator>::into_iter
; Function Attrs: uwtable
define { i64, i64 } @"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hf5b8ef5eff0c64c1E"(i64 %self.0, i64 %self.1) unnamed_addr #1 {
start:
  %0 = insertvalue { i64, i64 } undef, i64 %self.0, 0
  %1 = insertvalue { i64, i64 } %0, i64 %self.1, 1
  ret { i64, i64 } %1
}

; <&isize as core::ops::arith::Add<isize>>::add
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN64_$LT$$RF$isize$u20$as$u20$core..ops..arith..Add$LT$isize$GT$$GT$3add17haae222d643ffa60fE"(i64* noalias readonly align 8 dereferenceable(8) %self, i64 %other) unnamed_addr #0 {
start:
  %0 = load i64, i64* %self, align 8
; call <isize as core::ops::arith::Add>::add
  %1 = call i64 @"_ZN47_$LT$isize$u20$as$u20$core..ops..arith..Add$GT$3add17h5b676053fe71193bE"(i64 %0, i64 %other)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; <&isize as core::ops::arith::Rem<isize>>::rem
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN64_$LT$$RF$isize$u20$as$u20$core..ops..arith..Rem$LT$isize$GT$$GT$3rem17h56b516efd6f0069bE"(i64* noalias readonly align 8 dereferenceable(8) %self, i64 %other) unnamed_addr #0 {
start:
  %0 = load i64, i64* %self, align 8
; call <isize as core::ops::arith::Rem>::rem
  %1 = call i64 @"_ZN47_$LT$isize$u20$as$u20$core..ops..arith..Rem$GT$3rem17hcd48825ab2f2796fE"(i64 %0, i64 %other)
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %1
}

; <alloc::vec::Vec<T> as core::ops::deref::Deref>::deref
; Function Attrs: uwtable
define { [0 x i64]*, i64 } @"_ZN68_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h378128d7d9378466E"(%"alloc::vec::Vec<isize>"* noalias readonly align 8 dereferenceable(24)) unnamed_addr #1 {
start:
  %self = alloca %"alloc::vec::Vec<isize>"*, align 8
  store %"alloc::vec::Vec<isize>"* %0, %"alloc::vec::Vec<isize>"** %self, align 8
  %1 = load %"alloc::vec::Vec<isize>"*, %"alloc::vec::Vec<isize>"** %self, align 8, !nonnull !2
; call alloc::vec::Vec<T>::as_ptr
  %2 = call i64* @"_ZN5alloc3vec12Vec$LT$T$GT$6as_ptr17h6f8c42ce0d643832E"(%"alloc::vec::Vec<isize>"* noalias readonly align 8 dereferenceable(24) %1)
  br label %bb1

bb1:                                              ; preds = %start
  %3 = load %"alloc::vec::Vec<isize>"*, %"alloc::vec::Vec<isize>"** %self, align 8, !nonnull !2
  %4 = getelementptr inbounds %"alloc::vec::Vec<isize>", %"alloc::vec::Vec<isize>"* %3, i32 0, i32 3
  %5 = load i64, i64* %4, align 8
; call core::slice::from_raw_parts
  %6 = call { [0 x i64]*, i64 } @_ZN4core5slice14from_raw_parts17h35540e84de7ba669E(i64* %2, i64 %5)
  %7 = extractvalue { [0 x i64]*, i64 } %6, 0
  %8 = extractvalue { [0 x i64]*, i64 } %6, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %9 = insertvalue { [0 x i64]*, i64 } undef, [0 x i64]* %7, 0
  %10 = insertvalue { [0 x i64]*, i64 } %9, i64 %8, 1
  ret { [0 x i64]*, i64 } %10
}

; <usize as core::slice::SliceIndex<[T]>>::index_mut
; Function Attrs: inlinehint uwtable
define align 8 dereferenceable(8) i64* @"_ZN68_$LT$usize$u20$as$u20$core..slice..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$9index_mut17hd852e2f14eb0bd1dE"(i64 %self, [0 x i64]* nonnull align 8, i64) unnamed_addr #0 {
start:
  %slice = alloca { [0 x i64]*, i64 }, align 8
  %2 = getelementptr inbounds { [0 x i64]*, i64 }, { [0 x i64]*, i64 }* %slice, i32 0, i32 0
  store [0 x i64]* %0, [0 x i64]** %2, align 8
  %3 = getelementptr inbounds { [0 x i64]*, i64 }, { [0 x i64]*, i64 }* %slice, i32 0, i32 1
  store i64 %1, i64* %3, align 8
  %4 = getelementptr inbounds { [0 x i64]*, i64 }, { [0 x i64]*, i64 }* %slice, i32 0, i32 0
  %5 = load [0 x i64]*, [0 x i64]** %4, align 8, !nonnull !2
  %6 = getelementptr inbounds { [0 x i64]*, i64 }, { [0 x i64]*, i64 }* %slice, i32 0, i32 1
  %7 = load i64, i64* %6, align 8
  %8 = icmp ult i64 %self, %7
  %9 = call i1 @llvm.expect.i1(i1 %8, i1 true)
  br i1 %9, label %bb1, label %panic

bb1:                                              ; preds = %start
  %10 = getelementptr inbounds { [0 x i64]*, i64 }, { [0 x i64]*, i64 }* %slice, i32 0, i32 0
  %11 = load [0 x i64]*, [0 x i64]** %10, align 8, !nonnull !2
  %12 = getelementptr inbounds { [0 x i64]*, i64 }, { [0 x i64]*, i64 }* %slice, i32 0, i32 1
  %13 = load i64, i64* %12, align 8
  %14 = getelementptr inbounds [0 x i64], [0 x i64]* %11, i64 0, i64 %self
  ret i64* %14

panic:                                            ; preds = %start
; call core::panicking::panic_bounds_check
  call void @_ZN4core9panicking18panic_bounds_check17h0b7abca9936e6c0aE({ [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(24) bitcast ({ { [0 x i8]*, i64 }, i32, i32 }* @panic_bounds_check_loc.a to { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*), i64 %self, i64 %7)
  unreachable
}

; <alloc::vec::Vec<T> as core::ops::deref::DerefMut>::deref_mut
; Function Attrs: uwtable
define { [0 x i64]*, i64 } @"_ZN71_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17h599a33f1d75b3f9cE"(%"alloc::vec::Vec<isize>"* align 8 dereferenceable(24)) unnamed_addr #1 {
start:
  %self = alloca %"alloc::vec::Vec<isize>"*, align 8
  store %"alloc::vec::Vec<isize>"* %0, %"alloc::vec::Vec<isize>"** %self, align 8
  %1 = load %"alloc::vec::Vec<isize>"*, %"alloc::vec::Vec<isize>"** %self, align 8, !nonnull !2
; call alloc::vec::Vec<T>::as_mut_ptr
  %2 = call i64* @"_ZN5alloc3vec12Vec$LT$T$GT$10as_mut_ptr17h7db90383f1af03efE"(%"alloc::vec::Vec<isize>"* align 8 dereferenceable(24) %1)
  br label %bb1

bb1:                                              ; preds = %start
  %3 = load %"alloc::vec::Vec<isize>"*, %"alloc::vec::Vec<isize>"** %self, align 8, !nonnull !2
  %4 = getelementptr inbounds %"alloc::vec::Vec<isize>", %"alloc::vec::Vec<isize>"* %3, i32 0, i32 3
  %5 = load i64, i64* %4, align 8
; call core::slice::from_raw_parts_mut
  %6 = call { [0 x i64]*, i64 } @_ZN4core5slice18from_raw_parts_mut17hedc6f5a5b623fa49E(i64* %2, i64 %5)
  %7 = extractvalue { [0 x i64]*, i64 } %6, 0
  %8 = extractvalue { [0 x i64]*, i64 } %6, 1
  br label %bb2

bb2:                                              ; preds = %bb1
  %9 = insertvalue { [0 x i64]*, i64 } undef, [0 x i64]* %7, 0
  %10 = insertvalue { [0 x i64]*, i64 } %9, i64 %8, 1
  ret { [0 x i64]*, i64 } %10
}

; <alloc::vec::Vec<T> as core::ops::index::IndexMut<I>>::index_mut
; Function Attrs: inlinehint uwtable
define align 8 dereferenceable(8) i64* @"_ZN80_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..index..IndexMut$LT$I$GT$$GT$9index_mut17h034d16c396055127E"(%"alloc::vec::Vec<isize>"* align 8 dereferenceable(24) %self, i64 %index) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
  %_10 = alloca i8, align 1
  store i8 0, i8* %_10, align 1
  store i8 1, i8* %_10, align 1
; invoke <alloc::vec::Vec<T> as core::ops::deref::DerefMut>::deref_mut
  %0 = invoke { [0 x i64]*, i64 } @"_ZN71_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..deref..DerefMut$GT$9deref_mut17h599a33f1d75b3f9cE"(%"alloc::vec::Vec<isize>"* align 8 dereferenceable(24) %self)
          to label %bb2 unwind label %cleanup

bb1:                                              ; preds = %bb4, %bb5
  %1 = bitcast { i8*, i32 }* %personalityslot to i8**
  %2 = load i8*, i8** %1, align 8
  %3 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  %4 = load i32, i32* %3, align 8
  %5 = insertvalue { i8*, i32 } undef, i8* %2, 0
  %6 = insertvalue { i8*, i32 } %5, i32 %4, 1
  resume { i8*, i32 } %6

bb2:                                              ; preds = %start
  %7 = extractvalue { [0 x i64]*, i64 } %0, 0
  %8 = extractvalue { [0 x i64]*, i64 } %0, 1
  store i8 0, i8* %_10, align 1
; invoke core::slice::<impl core::ops::index::IndexMut<I> for [T]>::index_mut
  %9 = invoke align 8 dereferenceable(8) i64* @"_ZN4core5slice77_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$9index_mut17hae2665d649d0961bE"([0 x i64]* nonnull align 8 %7, i64 %8, i64 %index)
          to label %bb3 unwind label %cleanup

bb3:                                              ; preds = %bb2
  ret i64* %9

bb4:                                              ; preds = %bb5
  store i8 0, i8* %_10, align 1
  br label %bb1

bb5:                                              ; preds = %cleanup
  %10 = load i8, i8* %_10, align 1, !range !1
  %11 = trunc i8 %10 to i1
  br i1 %11, label %bb4, label %bb1

cleanup:                                          ; preds = %bb2, %start
  %12 = landingpad { i8*, i32 }
          cleanup
  %13 = extractvalue { i8*, i32 } %12, 0
  %14 = extractvalue { i8*, i32 } %12, 1
  %15 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 0
  store i8* %13, i8** %15, align 8
  %16 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  store i32 %14, i32* %16, align 8
  br label %bb5
}

; <core::slice::Iter<T> as core::iter::traits::iterator::Iterator>::next
; Function Attrs: inlinehint uwtable
define align 8 dereferenceable_or_null(8) i64* @"_ZN85_$LT$core..slice..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha91bd2d222d31b6cE"({ i64*, i64* }* align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %_0.i = alloca i64*, align 8
  %self.i = alloca { i64*, i64* }*, align 8
  %_0 = alloca i64*, align 8
  %0 = bitcast { i64*, i64* }* %self to i64**
  %1 = load i64*, i64** %0, align 8
; call core::ptr::<impl *const T>::is_null
  %2 = call zeroext i1 @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17h2cb5e5d9be44649cE"(i64* %1)
  br label %bb1

bb1:                                              ; preds = %start
  %3 = xor i1 %2, true
  call void @llvm.assume(i1 %3)
  br label %bb2

bb2:                                              ; preds = %bb1
; call core::mem::size_of
  %4 = call i64 @_ZN4core3mem7size_of17hd7e5cfcb791a5d1fE()
  br label %bb3

bb3:                                              ; preds = %bb2
  %5 = icmp ne i64 %4, 0
  br i1 %5, label %bb4, label %bb7

bb4:                                              ; preds = %bb3
  %6 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %self, i32 0, i32 1
  %7 = load i64*, i64** %6, align 8
; call core::ptr::<impl *const T>::is_null
  %8 = call zeroext i1 @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17h2cb5e5d9be44649cE"(i64* %7)
  br label %bb5

bb5:                                              ; preds = %bb4
  %9 = xor i1 %8, true
  call void @llvm.assume(i1 %9)
  br label %bb6

bb6:                                              ; preds = %bb5
  br label %bb7

bb7:                                              ; preds = %bb6, %bb3
  %10 = bitcast { i64*, i64* }* %self to i64**
  %11 = load i64*, i64** %10, align 8
  %12 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %self, i32 0, i32 1
  %13 = load i64*, i64** %12, align 8
  %14 = icmp eq i64* %11, %13
  br i1 %14, label %bb9, label %bb8

bb8:                                              ; preds = %bb7
  store { i64*, i64* }* %self, { i64*, i64* }** %self.i, align 8
; call core::mem::size_of
  %15 = call i64 @_ZN4core3mem7size_of17hd7e5cfcb791a5d1fE()
  %16 = icmp eq i64 %15, 0
  br i1 %16, label %bb3.i, label %bb2.i

bb2.i:                                            ; preds = %bb8
  %17 = load { i64*, i64* }*, { i64*, i64* }** %self.i, align 8, !nonnull !2
  %18 = bitcast { i64*, i64* }* %17 to i64**
  %19 = load i64*, i64** %18, align 8
  %20 = load { i64*, i64* }*, { i64*, i64* }** %self.i, align 8, !nonnull !2
  %21 = bitcast { i64*, i64* }* %20 to i64**
  %22 = load i64*, i64** %21, align 8
; call core::ptr::<impl *const T>::offset
  %23 = call i64* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17h66d880d9e1cefb86E"(i64* %22, i64 1)
  %24 = load { i64*, i64* }*, { i64*, i64* }** %self.i, align 8, !nonnull !2
  %25 = bitcast { i64*, i64* }* %24 to i64**
  store i64* %23, i64** %25, align 8
  store i64* %19, i64** %_0.i, align 8
  br label %"_ZN4core5slice13Iter$LT$T$GT$14post_inc_start17h8479de9ec9ee8a6eE.exit"

bb3.i:                                            ; preds = %bb8
  %26 = load { i64*, i64* }*, { i64*, i64* }** %self.i, align 8, !nonnull !2
  %27 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %26, i32 0, i32 1
  %28 = load i64*, i64** %27, align 8
  %29 = bitcast i64* %28 to i8*
; call core::ptr::<impl *const T>::wrapping_offset
  %30 = call i8* @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$15wrapping_offset17h45b3ac03c2a8679dE"(i8* %29, i64 -1)
  %31 = load { i64*, i64* }*, { i64*, i64* }** %self.i, align 8, !nonnull !2
  %32 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %31, i32 0, i32 1
  %33 = bitcast i8* %30 to i64*
  store i64* %33, i64** %32, align 8
  %34 = load { i64*, i64* }*, { i64*, i64* }** %self.i, align 8, !nonnull !2
  %35 = bitcast { i64*, i64* }* %34 to i64**
  %36 = load i64*, i64** %35, align 8
  store i64* %36, i64** %_0.i, align 8
  br label %"_ZN4core5slice13Iter$LT$T$GT$14post_inc_start17h8479de9ec9ee8a6eE.exit"

"_ZN4core5slice13Iter$LT$T$GT$14post_inc_start17h8479de9ec9ee8a6eE.exit": ; preds = %bb3.i, %bb2.i
  %37 = load i64*, i64** %_0.i, align 8
  br label %bb10

bb9:                                              ; preds = %bb7
  %38 = bitcast i64** %_0 to {}**
  store {}* null, {}** %38, align 8
  br label %bb11

bb10:                                             ; preds = %"_ZN4core5slice13Iter$LT$T$GT$14post_inc_start17h8479de9ec9ee8a6eE.exit"
  store i64* %37, i64** %_0, align 8
  br label %bb11

bb11:                                             ; preds = %bb10, %bb9
  %39 = load i64*, i64** %_0, align 8
  ret i64* %39
}

; rust::rust_loop
; Function Attrs: uwtable
define i64 @_ZN4rust9rust_loop17h3ed0672b8cf44eb1E(i64 %a, i64 %b, %"alloc::vec::Vec<isize>"* align 8 dereferenceable(24) %v) unnamed_addr #1 {
start:
  %_32 = alloca { i64, i64 }, align 8
  %iter1 = alloca { i64, i64 }, align 8
  %_29 = alloca { i64, i64 }, align 8
  %_19 = alloca i64, align 8
  %_12 = alloca i64*, align 8
  %iter = alloca { i64*, i64* }, align 8
  %sum = alloca i64, align 8
  store i64 0, i64* %sum, align 8
; call <alloc::vec::Vec<T> as core::ops::deref::Deref>::deref
  %0 = call { [0 x i64]*, i64 } @"_ZN68_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h378128d7d9378466E"(%"alloc::vec::Vec<isize>"* noalias readonly align 8 dereferenceable(24) %v)
  %1 = extractvalue { [0 x i64]*, i64 } %0, 0
  %2 = extractvalue { [0 x i64]*, i64 } %0, 1
  br label %bb1

bb1:                                              ; preds = %start
; call core::slice::<impl [T]>::iter
  %3 = call { i64*, i64* } @"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$4iter17h0f94ceb2d99e4dddE"([0 x i64]* noalias nonnull readonly align 8 %1, i64 %2)
  %4 = extractvalue { i64*, i64* } %3, 0
  %5 = extractvalue { i64*, i64* } %3, 1
  br label %bb2

bb2:                                              ; preds = %bb1
; call <I as core::iter::traits::collect::IntoIterator>::into_iter
  %6 = call { i64*, i64* } @"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h063b82e0a27c293dE"(i64* %4, i64* %5)
  %7 = extractvalue { i64*, i64* } %6, 0
  %8 = extractvalue { i64*, i64* } %6, 1
  br label %bb3

bb3:                                              ; preds = %bb2
  %9 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %iter, i32 0, i32 0
  store i64* %7, i64** %9, align 8
  %10 = getelementptr inbounds { i64*, i64* }, { i64*, i64* }* %iter, i32 0, i32 1
  store i64* %8, i64** %10, align 8
  br label %bb4

bb4:                                              ; preds = %bb15, %bb3
; call <core::slice::Iter<T> as core::iter::traits::iterator::Iterator>::next
  %11 = call align 8 dereferenceable_or_null(8) i64* @"_ZN85_$LT$core..slice..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17ha91bd2d222d31b6cE"({ i64*, i64* }* align 8 dereferenceable(16) %iter)
  store i64* %11, i64** %_12, align 8
  br label %bb5

bb5:                                              ; preds = %bb4
  %12 = bitcast i64** %_12 to {}**
  %13 = load {}*, {}** %12, align 8
  %14 = icmp eq {}* %13, null
  %15 = select i1 %14, i64 0, i64 1
  switch i64 %15, label %bb7 [
    i64 0, label %bb6
    i64 1, label %bb8
  ]

bb6:                                              ; preds = %bb5
  %16 = bitcast { i64, i64 }* %_29 to i64*
  store i64 0, i64* %16, align 8
  %17 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_29, i32 0, i32 1
  store i64 5, i64* %17, align 8
  %18 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_29, i32 0, i32 0
  %19 = load i64, i64* %18, align 8
  %20 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_29, i32 0, i32 1
  %21 = load i64, i64* %20, align 8
; call <I as core::iter::traits::collect::IntoIterator>::into_iter
  %22 = call { i64, i64 } @"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hf5b8ef5eff0c64c1E"(i64 %19, i64 %21)
  %23 = extractvalue { i64, i64 } %22, 0
  %24 = extractvalue { i64, i64 } %22, 1
  br label %bb16

bb7:                                              ; preds = %bb5
  unreachable

bb8:                                              ; preds = %bb5
  %25 = load i64*, i64** %_12, align 8, !nonnull !2
; call <&isize as core::ops::arith::Rem<isize>>::rem
  %26 = call i64 @"_ZN64_$LT$$RF$isize$u20$as$u20$core..ops..arith..Rem$LT$isize$GT$$GT$3rem17h56b516efd6f0069bE"(i64* noalias readonly align 8 dereferenceable(8) %25, i64 3)
  br label %bb9

bb9:                                              ; preds = %bb8
  %27 = icmp eq i64 %26, 1
  br i1 %27, label %bb11, label %bb10

bb10:                                             ; preds = %bb9
; call <&isize as core::ops::arith::Add<isize>>::add
  %28 = call i64 @"_ZN64_$LT$$RF$isize$u20$as$u20$core..ops..arith..Add$LT$isize$GT$$GT$3add17haae222d643ffa60fE"(i64* noalias readonly align 8 dereferenceable(8) %25, i64 %b)
  store i64 %28, i64* %_19, align 8
  br label %bb13

bb11:                                             ; preds = %bb9
; call <&isize as core::ops::arith::Add<isize>>::add
  %29 = call i64 @"_ZN64_$LT$$RF$isize$u20$as$u20$core..ops..arith..Add$LT$isize$GT$$GT$3add17haae222d643ffa60fE"(i64* noalias readonly align 8 dereferenceable(8) %25, i64 %a)
  store i64 %29, i64* %_19, align 8
  br label %bb12

bb12:                                             ; preds = %bb11
  br label %bb14

bb13:                                             ; preds = %bb10
  br label %bb14

bb14:                                             ; preds = %bb13, %bb12
  %30 = load i64, i64* %sum, align 8
  %31 = load i64, i64* %_19, align 8
  %32 = call { i64, i1 } @llvm.sadd.with.overflow.i64(i64 %30, i64 %31)
  %33 = extractvalue { i64, i1 } %32, 0
  %34 = extractvalue { i64, i1 } %32, 1
  %35 = call i1 @llvm.expect.i1(i1 %34, i1 false)
  br i1 %35, label %panic, label %bb15

bb15:                                             ; preds = %bb14
  store i64 %33, i64* %sum, align 8
  br label %bb4

bb16:                                             ; preds = %bb6
  %36 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %iter1, i32 0, i32 0
  store i64 %23, i64* %36, align 8
  %37 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %iter1, i32 0, i32 1
  store i64 %24, i64* %37, align 8
  br label %bb17

bb17:                                             ; preds = %bb23, %bb16
; call core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next
  %38 = call { i64, i64 } @"_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h28f5ccce6734b660E"({ i64, i64 }* align 8 dereferenceable(16) %iter1)
  store { i64, i64 } %38, { i64, i64 }* %_32, align 8
  br label %bb18

bb18:                                             ; preds = %bb17
  %39 = bitcast { i64, i64 }* %_32 to i64*
  %40 = load i64, i64* %39, align 8, !range !0
  switch i64 %40, label %bb20 [
    i64 0, label %bb19
    i64 1, label %bb21
  ]

bb19:                                             ; preds = %bb18
  %41 = load i64, i64* %sum, align 8
  ret i64 %41

bb20:                                             ; preds = %bb18
  unreachable

bb21:                                             ; preds = %bb18
  %42 = bitcast { i64, i64 }* %_32 to %"core::option::Option<usize>::Some"*
  %43 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %42, i32 0, i32 1
  %44 = load i64, i64* %43, align 8
  %45 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %44, i64 2)
  %46 = extractvalue { i64, i1 } %45, 0
  %47 = extractvalue { i64, i1 } %45, 1
  %48 = call i1 @llvm.expect.i1(i1 %47, i1 false)
  br i1 %48, label %panic2, label %bb22

bb22:                                             ; preds = %bb21
; call <alloc::vec::Vec<T> as core::ops::index::IndexMut<I>>::index_mut
  %49 = call align 8 dereferenceable(8) i64* @"_ZN80_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..index..IndexMut$LT$I$GT$$GT$9index_mut17h034d16c396055127E"(%"alloc::vec::Vec<isize>"* align 8 dereferenceable(24) %v, i64 %44)
  br label %bb23

bb23:                                             ; preds = %bb22
  store i64 %46, i64* %49, align 8
  br label %bb17

panic:                                            ; preds = %bb14
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h89241d71a860ed98E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast ({ { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 }* @panic_loc.c to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

panic2:                                           ; preds = %bb21
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h89241d71a860ed98E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40) bitcast ({ { [0 x i8]*, i64 }, { [0 x i8]*, i64 }, i32, i32 }* @panic_loc.d to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; Function Attrs: nounwind readnone speculatable
declare { i64, i1 } @llvm.sadd.with.overflow.i64(i64, i64) #2

; Function Attrs: nounwind readnone
declare i1 @llvm.expect.i1(i1, i1) #3

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h89241d71a860ed98E({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(40)) unnamed_addr #4

; Function Attrs: argmemonly nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i1) #5

; Function Attrs: nounwind readnone speculatable
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #2

; Function Attrs: nounwind readnone speculatable
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #2

; Function Attrs: nounwind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #6

; Function Attrs: nounwind
declare void @llvm.assume(i1) #7

; core::panicking::panic_bounds_check
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking18panic_bounds_check17h0b7abca9936e6c0aE({ [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly align 8 dereferenceable(24), i64, i64) unnamed_addr #4

attributes #0 = { inlinehint uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #1 = { uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #2 = { nounwind readnone speculatable }
attributes #3 = { nounwind readnone }
attributes #4 = { cold noinline noreturn uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #5 = { argmemonly nounwind }
attributes #6 = { nounwind uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="core2" }
attributes #7 = { nounwind }

!0 = !{i64 0, i64 2}
!1 = !{i8 0, i8 2}
!2 = !{}
