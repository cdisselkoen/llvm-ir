; ModuleID = 'throw.cpp'
source_filename = "throw.cpp"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@_ZTISt11logic_error = external constant ptr
@.str = private unnamed_addr constant [19 x i8] c"hello, exceptions!\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%p\0A\00", align 1

; Function Attrs: mustprogress norecurse uwtable
define dso_local noundef i32 @main() local_unnamed_addr #0 personality ptr @__gxx_personality_v0 {
  %1 = tail call ptr @__cxa_allocate_exception(i64 16) #5
  invoke void @_ZNSt11logic_errorC1EPKc(ptr noundef nonnull align 8 dereferenceable(16) %1, ptr noundef nonnull @.str)
          to label %2 unwind label %3

2:                                                ; preds = %0
  invoke void @__cxa_throw(ptr nonnull %1, ptr nonnull @_ZTISt11logic_error, ptr nonnull @_ZNSt11logic_errorD1Ev) #6
          to label %17 unwind label %5

3:                                                ; preds = %0
  %4 = landingpad { ptr, i32 }
          cleanup
          catch ptr @_ZTISt11logic_error
  tail call void @__cxa_free_exception(ptr %1) #5
  br label %7

5:                                                ; preds = %2
  %6 = landingpad { ptr, i32 }
          catch ptr @_ZTISt11logic_error
  br label %7

7:                                                ; preds = %5, %3
  %8 = phi { ptr, i32 } [ %6, %5 ], [ %4, %3 ]
  %9 = extractvalue { ptr, i32 } %8, 1
  %10 = tail call i32 @llvm.eh.typeid.for(ptr nonnull @_ZTISt11logic_error) #5
  %11 = icmp eq i32 %9, %10
  br i1 %11, label %12, label %16

12:                                               ; preds = %7
  %13 = extractvalue { ptr, i32 } %8, 0
  %14 = tail call ptr @__cxa_begin_catch(ptr %13) #5
  %15 = tail call i32 (ptr, ...) @printf(ptr noundef nonnull dereferenceable(1) @.str.1, ptr noundef %14)
  tail call void @__cxa_end_catch()
  ret i32 0

16:                                               ; preds = %7
  resume { ptr, i32 } %8

17:                                               ; preds = %2
  unreachable
}

declare ptr @__cxa_allocate_exception(i64) local_unnamed_addr

declare void @_ZNSt11logic_errorC1EPKc(ptr noundef nonnull align 8 dereferenceable(16), ptr noundef) unnamed_addr #1

declare i32 @__gxx_personality_v0(...)

declare void @__cxa_free_exception(ptr) local_unnamed_addr

; Function Attrs: nounwind
declare void @_ZNSt11logic_errorD1Ev(ptr noundef nonnull align 8 dereferenceable(16)) unnamed_addr #2

declare void @__cxa_throw(ptr, ptr, ptr) local_unnamed_addr

; Function Attrs: nofree nosync nounwind memory(none)
declare i32 @llvm.eh.typeid.for(ptr) #3

declare ptr @__cxa_begin_catch(ptr) local_unnamed_addr

; Function Attrs: nofree nounwind
declare noundef i32 @printf(ptr nocapture noundef readonly, ...) local_unnamed_addr #4

declare void @__cxa_end_catch() local_unnamed_addr

attributes #0 = { mustprogress norecurse uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nounwind "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { nofree nosync nounwind memory(none) }
attributes #4 = { nofree nounwind "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { nounwind }
attributes #6 = { noreturn }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{!"clang version 17.0.6"}
