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

; Function Attrs: norecurse sspstrong uwtable
define i32 @main() local_unnamed_addr #0 personality i8* bitcast (i32 (...)* @__gxx_personality_v0 to i8*) {
  %1 = tail call i8* @__cxa_allocate_exception(i64 16) #5
  %2 = bitcast i8* %1 to %"class.std::logic_error"*
  invoke void @_ZNSt11logic_errorC1EPKc(%"class.std::logic_error"* %2, i8* getelementptr inbounds ([19 x i8], [19 x i8]* @.str, i64 0, i64 0))
          to label %3 unwind label %4

3:                                                ; preds = %0
  invoke void @__cxa_throw(i8* %1, i8* bitcast (i8** @_ZTISt11logic_error to i8*), i8* bitcast (void (%"class.std::logic_error"*)* @_ZNSt11logic_errorD1Ev to i8*)) #6
          to label %23 unwind label %8

4:                                                ; preds = %0
  %5 = landingpad { i8*, i32 }
          cleanup
          catch i8* bitcast (i8** @_ZTISt11logic_error to i8*)
  %6 = extractvalue { i8*, i32 } %5, 0
  %7 = extractvalue { i8*, i32 } %5, 1
  tail call void @__cxa_free_exception(i8* %1) #5
  br label %12

8:                                                ; preds = %3
  %9 = landingpad { i8*, i32 }
          catch i8* bitcast (i8** @_ZTISt11logic_error to i8*)
  %10 = extractvalue { i8*, i32 } %9, 0
  %11 = extractvalue { i8*, i32 } %9, 1
  br label %12

12:                                               ; preds = %8, %4
  %13 = phi i32 [ %11, %8 ], [ %7, %4 ]
  %14 = phi i8* [ %10, %8 ], [ %6, %4 ]
  %15 = tail call i32 @llvm.eh.typeid.for(i8* bitcast (i8** @_ZTISt11logic_error to i8*)) #5
  %16 = icmp eq i32 %13, %15
  br i1 %16, label %17, label %20

17:                                               ; preds = %12
  %18 = tail call i8* @__cxa_begin_catch(i8* %14) #5
  %19 = tail call i32 (i8*, ...) @printf(i8* nonnull dereferenceable(1) getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i64 0, i64 0), i8* %18)
  tail call void @__cxa_end_catch()
  ret i32 0

20:                                               ; preds = %12
  %21 = insertvalue { i8*, i32 } undef, i8* %14, 0
  %22 = insertvalue { i8*, i32 } %21, i32 %13, 1
  resume { i8*, i32 } %22

23:                                               ; preds = %3
  unreachable
}

declare i8* @__cxa_allocate_exception(i64) local_unnamed_addr

declare void @_ZNSt11logic_errorC1EPKc(%"class.std::logic_error"*, i8*) unnamed_addr #1

declare i32 @__gxx_personality_v0(...)

declare void @__cxa_free_exception(i8*) local_unnamed_addr

; Function Attrs: nounwind
declare void @_ZNSt11logic_errorD1Ev(%"class.std::logic_error"*) unnamed_addr #2

declare void @__cxa_throw(i8*, i8*, i8*) local_unnamed_addr

; Function Attrs: nounwind readnone
declare i32 @llvm.eh.typeid.for(i8*) #3

declare i8* @__cxa_begin_catch(i8*) local_unnamed_addr

; Function Attrs: nofree nounwind
declare i32 @printf(i8* nocapture readonly, ...) local_unnamed_addr #4

declare void @__cxa_end_catch() local_unnamed_addr

attributes #0 = { norecurse sspstrong uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="4" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="4" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #2 = { nounwind "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="4" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { nounwind readnone }
attributes #4 = { nofree nounwind "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="none" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="4" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #5 = { nounwind }
attributes #6 = { noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 10.0.1 "}
