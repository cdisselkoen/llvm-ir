; ModuleID = 'loop.c'
source_filename = "loop.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.16.0"

; Function Attrs: nounwind ssp uwtable
define void @loop(i32, i32) local_unnamed_addr #0 {
  %3 = alloca [10 x i32], align 16
  %4 = bitcast [10 x i32]* %3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 40, i8* nonnull %4) #2
  call void @llvm.memset.p0i8.i64(i8* nonnull align 16 %4, i8 0, i64 40, i1 true)
  %5 = add i32 %1, -1
  %6 = icmp ult i32 %5, 10
  br i1 %6, label %7, label %22

; <label>:7:                                      ; preds = %2
  %8 = add nsw i32 %0, 3
  %9 = sext i32 %1 to i64
  br label %10

; <label>:10:                                     ; preds = %7, %19
  %11 = phi i64 [ 0, %7 ], [ %20, %19 ]
  %12 = getelementptr inbounds [10 x i32], [10 x i32]* %3, i64 0, i64 %11
  store volatile i32 %8, i32* %12, align 4, !tbaa !3
  %13 = icmp eq i64 %11, 0
  br i1 %13, label %19, label %14

; <label>:14:                                     ; preds = %10
  %15 = add nsw i64 %11, -1
  %16 = getelementptr inbounds [10 x i32], [10 x i32]* %3, i64 0, i64 %15
  %17 = load volatile i32, i32* %16, align 4, !tbaa !3
  %18 = add nsw i32 %17, %1
  store volatile i32 %18, i32* %16, align 4, !tbaa !3
  br label %19

; <label>:19:                                     ; preds = %10, %14
  %20 = add nuw nsw i64 %11, 1
  %21 = icmp slt i64 %20, %9
  br i1 %21, label %10, label %22

; <label>:22:                                     ; preds = %19, %2
  call void @llvm.lifetime.end.p0i8(i64 40, i8* nonnull %4) #2
  ret void
}

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.start.p0i8(i64, i8* nocapture) #1

; Function Attrs: argmemonly nounwind
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1) #1

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.end.p0i8(i64, i8* nocapture) #1

attributes #0 = { nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { argmemonly nounwind }
attributes #2 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 8.0.1 (tags/RELEASE_801/final)"}
!3 = !{!4, !4, i64 0}
!4 = !{!"int", !5, i64 0}
!5 = !{!"omnipotent char", !6, i64 0}
!6 = !{!"Simple C/C++ TBAA"}
