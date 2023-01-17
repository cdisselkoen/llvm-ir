; ModuleID = 'throw.cpp'
source_filename = "throw.cpp"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"class.std::logic_error" = type { %"class.std::exception", %"struct.std::__cow_string" }
%"class.std::exception" = type { i32 (...)** }
%"struct.std::__cow_string" = type { %union.anon }
%union.anon = type { i8* }

@_ZTISt11logic_error = external constant i8*
@.str = private unnamed_addr constant [19 x i8] c"hello, exceptions!\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%p\0A\00", align 1

; Function Attrs: mustprogress norecurse sspstrong uwtable
define i32 @main() local_unnamed_addr #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) {
  %1 = tail call i8* @__cxa_allocate_exception(i64 16) #5
  %2 = bitcast i8* %1 to %"class.std::logic_error"*
  invoke void @_ZNSt11logic_errorC1EPKc(%"class.std::logic_error"* nonnull align 8 dereferenceable(16) %2, i8* getelementptr inbounds ([19 x i8], [19 x i8]* @.str, i64 0, i64 0))
          to label %3 unwind label %4

3:                                                ; preds = %0
  invoke void @__cxa_throw(i8* %1, i8* bitcast (i8** @_ZTISt11logic_error to i8*), i8* bitcast (void (%"class.std::logic_error"*)* @_ZNSt11logic_errorD1Ev to i8*)) #6
          to label %18 unwind label %6

4:                                                ; preds = %0
  %5 = landingpad { i8*, i32 }
          cleanup
          catch i8* bitcast (i8** @_ZTISt11logic_error to i8*)
  tail call void @__cxa_free_exception(i8* %1) #5
  br label %8

6:                                                ; preds = %3
  %7 = landingpad { i8*, i32 }
          catch i8* bitcast (i8** @_ZTISt11logic_error to i8*)
  br label %8

8:                                                ; preds = %6, %4
  %9 = phi { i8*, i32 } [ %7, %6 ], [ %5, %4 ]
  %10 = extractvalue { i8*, i32 } %9, 1
  %11 = tail call i32 @llvm.eh.typeid.for(i8* bitcast (i8** @_ZTISt11logic_error to i8*)) #5
  %12 = icmp eq i32 %10, %11
  br i1 %12, label %13, label %17

13:                                               ; preds = %8
  %14 = extractvalue { i8*, i32 } %9, 0
  %15 = tail call i8* @__cxa_begin_catch(i8* %14) #5
  %16 = tail call i32 (i8*, ...) @printf(i8* nonnull dereferenceable(1) getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i64 0, i64 0), i8* %15)
  tail call void @__cxa_end_catch()
  ret i32 0

17:                                               ; preds = %8
  resume { i8*, i32 } %9

18:                                               ; preds = %3
  unreachable
}

declare i8* @__cxa_allocate_exception(i64) local_unnamed_addr

declare void @_ZNSt11logic_errorC1EPKc(%"class.std::logic_error"* nonnull align 8 dereferenceable(16), i8*) unnamed_addr #1

declare i32 @__gxx_personality_v0(...)

declare void @__cxa_free_exception(i8*) local_unnamed_addr

; Function Attrs: nounwind
declare void @_ZNSt11logic_errorD1Ev(%"class.std::logic_error"* nonnull align 8 dereferenceable(16)) unnamed_addr #2

declare void @__cxa_throw(i8*, i8*, i8*) local_unnamed_addr

; Function Attrs: nofree nosync nounwind readnone
declare i32 @llvm.eh.typeid.for(i8*) #3

declare i8* @__cxa_begin_catch(i8*) local_unnamed_addr

; Function Attrs: nofree nounwind
declare noundef i32 @printf(i8* nocapture noundef readonly, ...) local_unnamed_addr #4

declare void @__cxa_end_catch() local_unnamed_addr

attributes #0 = { mustprogress norecurse sspstrong uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="4" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="4" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { nounwind "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="4" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { nofree nosync nounwind readnone }
attributes #4 = { nofree nounwind "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="4" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { nounwind }
attributes #6 = { noreturn }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 1}
!3 = !{!"clang version 13.0.1"}
