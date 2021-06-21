; ModuleID = 'loop.c'
source_filename = "loop.c"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.16.0"

; Function Attrs: nounwind ssp uwtable
define void @loop(i32 %0, i32 %1) local_unnamed_addr #0 {
  %3 = alloca [10 x i32], align 16
  %4 = bitcast [10 x i32]* %3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 40, i8* nonnull %4) #2
  call void @llvm.memset.p0i8.i64(i8* nonnull align 16 %4, i8 0, i64 40, i1 true)
  %5 = add i32 %1, -1
  %6 = icmp ult i32 %5, 10
  br i1 %6, label %7, label %21

7:                                                ; preds = %2
  %8 = add nsw i32 %0, 3
  %9 = zext i32 %1 to i64
  %10 = getelementptr inbounds [10 x i32], [10 x i32]* %3, i64 0, i64 0
  store volatile i32 %8, i32* %10, align 16, !tbaa !4
  %11 = icmp ugt i32 %1, 1
  br i1 %11, label %12, label %21

12:                                               ; preds = %7, %12
  %13 = phi i64 [ %19, %12 ], [ 1, %7 ]
  %14 = getelementptr inbounds [10 x i32], [10 x i32]* %3, i64 0, i64 %13
  store volatile i32 %8, i32* %14, align 4, !tbaa !4
  %15 = add nsw i64 %13, -1
  %16 = getelementptr inbounds [10 x i32], [10 x i32]* %3, i64 0, i64 %15
  %17 = load volatile i32, i32* %16, align 4, !tbaa !4
  %18 = add nsw i32 %17, %1
  store volatile i32 %18, i32* %16, align 4, !tbaa !4
  %19 = add nuw nsw i64 %13, 1
  %20 = icmp ult i64 %19, %9
  br i1 %20, label %12, label %21, !llvm.loop !8

21:                                               ; preds = %7, %12, %2
  call void @llvm.lifetime.end.p0i8(i64 40, i8* nonnull %4) #2
  ret void
}

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #1

attributes #0 = { nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { argmemonly nounwind willreturn }
attributes #2 = { nounwind }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 2, !"SDK Version", [2 x i32] [i32 11, i32 3]}
!1 = !{i32 1, !"wchar_size", i32 4}
!2 = !{i32 7, !"PIC Level", i32 2}
!3 = !{!"clang version 10.0.0 "}
!4 = !{!5, !5, i64 0}
!5 = !{!"int", !6, i64 0}
!6 = !{!"omnipotent char", !7, i64 0}
!7 = !{!"Simple C/C++ TBAA"}
!8 = distinct !{!8, !9}
!9 = !{!"llvm.loop.peeled.count", i32 1}
