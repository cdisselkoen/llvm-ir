; ModuleID = 'throw.cpp'
source_filename = "throw.cpp"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx13.0.0"

@_ZTISt11logic_error = external constant ptr
@.str = private unnamed_addr constant [19 x i8] c"hello, exceptions!\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%p\0A\00", align 1

; Function Attrs: mustprogress norecurse ssp uwtable(sync)
define noundef i32 @main() local_unnamed_addr #0 personality ptr @__gxx_personality_v0 {
  %1 = tail call ptr @__cxa_allocate_exception(i64 16) #5
  %2 = invoke noundef ptr @_ZNSt11logic_errorC1EPKc(ptr noundef nonnull align 8 dereferenceable(16) %1, ptr noundef nonnull @.str)
          to label %3 unwind label %4

3:                                                ; preds = %0
  invoke void @__cxa_throw(ptr nonnull %1, ptr nonnull @_ZTISt11logic_error, ptr nonnull @_ZNSt11logic_errorD1Ev) #6
          to label %18 unwind label %6

4:                                                ; preds = %0
  %5 = landingpad { ptr, i32 }
          cleanup
          catch ptr @_ZTISt11logic_error
  tail call void @__cxa_free_exception(ptr %1) #5
  br label %8

6:                                                ; preds = %3
  %7 = landingpad { ptr, i32 }
          catch ptr @_ZTISt11logic_error
  br label %8

8:                                                ; preds = %6, %4
  %9 = phi { ptr, i32 } [ %7, %6 ], [ %5, %4 ]
  %10 = extractvalue { ptr, i32 } %9, 1
  %11 = tail call i32 @llvm.eh.typeid.for(ptr nonnull @_ZTISt11logic_error) #5
  %12 = icmp eq i32 %10, %11
  br i1 %12, label %13, label %17

13:                                               ; preds = %8
  %14 = extractvalue { ptr, i32 } %9, 0
  %15 = tail call ptr @__cxa_begin_catch(ptr %14) #5
  %16 = tail call i32 (ptr, ...) @printf(ptr noundef nonnull dereferenceable(1) @.str.1, ptr noundef %15)
  tail call void @__cxa_end_catch()
  ret i32 0

17:                                               ; preds = %8
  resume { ptr, i32 } %9

18:                                               ; preds = %3
  unreachable
}

declare ptr @__cxa_allocate_exception(i64) local_unnamed_addr

declare noundef ptr @_ZNSt11logic_errorC1EPKc(ptr noundef nonnull returned align 8 dereferenceable(16), ptr noundef) unnamed_addr #1

declare i32 @__gxx_personality_v0(...)

declare void @__cxa_free_exception(ptr) local_unnamed_addr

; Function Attrs: nounwind
declare noundef ptr @_ZNSt11logic_errorD1Ev(ptr noundef nonnull returned align 8 dereferenceable(16)) unnamed_addr #2

declare void @__cxa_throw(ptr, ptr, ptr) local_unnamed_addr

; Function Attrs: nofree nosync nounwind memory(none)
declare i32 @llvm.eh.typeid.for(ptr) #3

declare ptr @__cxa_begin_catch(ptr) local_unnamed_addr

; Function Attrs: nofree nounwind
declare noundef i32 @printf(ptr nocapture noundef readonly, ...) local_unnamed_addr #4

declare void @__cxa_end_catch() local_unnamed_addr

attributes #0 = { mustprogress norecurse ssp uwtable(sync) "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+crc,+crypto,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+lse,+neon,+ras,+rcpc,+rdm,+sha2,+sha3,+sm4,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #1 = { "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+crc,+crypto,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+lse,+neon,+ras,+rcpc,+rdm,+sha2,+sha3,+sm4,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #2 = { nounwind "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+crc,+crypto,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+lse,+neon,+ras,+rcpc,+rdm,+sha2,+sha3,+sm4,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #3 = { nofree nosync nounwind memory(none) }
attributes #4 = { nofree nounwind "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+crc,+crypto,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+lse,+neon,+ras,+rcpc,+rdm,+sha2,+sha3,+sm4,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #5 = { nounwind }
attributes #6 = { noreturn }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 1}
!3 = !{i32 7, !"frame-pointer", i32 1}
!4 = !{!"Homebrew clang version 16.0.6"}
