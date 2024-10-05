; ModuleID = 'loop.c'
source_filename = "loop.c"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx12.0.0"

; Function Attrs: nofree norecurse nounwind ssp memory(inaccessiblemem: readwrite) uwtable
define void @loop(i32 noundef %0, i32 noundef %1) local_unnamed_addr #0 {
  %3 = alloca [10 x i32], align 16
  call void @llvm.lifetime.start.p0(i64 40, ptr nonnull %3) #3
  call void @llvm.memset.p0.i64(ptr nonnull align 16 %3, i8 0, i64 40, i1 true)
  %4 = add i32 %1, -1
  %5 = icmp ult i32 %4, 10
  br i1 %5, label %6, label %41

6:                                                ; preds = %2
  %7 = add nsw i32 %0, 3
  store volatile i32 %7, ptr %3, align 16, !tbaa !5
  %8 = icmp eq i32 %1, 1
  br i1 %8, label %41, label %9

9:                                                ; preds = %6
  %10 = zext nneg i32 %1 to i64
  %11 = add nsw i64 %10, -1
  %12 = and i64 %11, 1
  %13 = icmp eq i32 %1, 2
  br i1 %13, label %32, label %14

14:                                               ; preds = %9
  %15 = and i64 %11, -2
  br label %16

16:                                               ; preds = %16, %14
  %17 = phi i64 [ 1, %14 ], [ %29, %16 ]
  %18 = phi i64 [ 0, %14 ], [ %30, %16 ]
  %19 = getelementptr inbounds [10 x i32], ptr %3, i64 0, i64 %17
  store volatile i32 %7, ptr %19, align 4, !tbaa !5
  %20 = add nsw i64 %17, -1
  %21 = getelementptr inbounds [10 x i32], ptr %3, i64 0, i64 %20
  %22 = load volatile i32, ptr %21, align 4, !tbaa !5
  %23 = add nsw i32 %22, %1
  store volatile i32 %23, ptr %21, align 4, !tbaa !5
  %24 = add nuw nsw i64 %17, 1
  %25 = getelementptr inbounds [10 x i32], ptr %3, i64 0, i64 %24
  store volatile i32 %7, ptr %25, align 4, !tbaa !5
  %26 = getelementptr inbounds [10 x i32], ptr %3, i64 0, i64 %17
  %27 = load volatile i32, ptr %26, align 4, !tbaa !5
  %28 = add nsw i32 %27, %1
  store volatile i32 %28, ptr %26, align 4, !tbaa !5
  %29 = add nuw nsw i64 %17, 2
  %30 = add i64 %18, 2
  %31 = icmp eq i64 %30, %15
  br i1 %31, label %32, label %16, !llvm.loop !9

32:                                               ; preds = %16, %9
  %33 = phi i64 [ 1, %9 ], [ %29, %16 ]
  %34 = icmp eq i64 %12, 0
  br i1 %34, label %41, label %35

35:                                               ; preds = %32
  %36 = getelementptr inbounds [10 x i32], ptr %3, i64 0, i64 %33
  store volatile i32 %7, ptr %36, align 4, !tbaa !5
  %37 = add nsw i64 %33, -1
  %38 = getelementptr inbounds [10 x i32], ptr %3, i64 0, i64 %37
  %39 = load volatile i32, ptr %38, align 4, !tbaa !5
  %40 = add nsw i32 %39, %1
  store volatile i32 %40, ptr %38, align 4, !tbaa !5
  br label %41

41:                                               ; preds = %35, %32, %6, %2
  call void @llvm.lifetime.end.p0(i64 40, ptr nonnull %3) #3
  ret void
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr nocapture writeonly, i8, i64, i1 immarg) #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #1

attributes #0 = { nofree norecurse nounwind ssp memory(inaccessiblemem: readwrite) uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cmov,+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "tune-cpu"="generic" }
attributes #1 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #2 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #3 = { nounwind }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 2}
!3 = !{i32 7, !"frame-pointer", i32 2}
!4 = !{!"clang version 19.1.0"}
!5 = !{!6, !6, i64 0}
!6 = !{!"int", !7, i64 0}
!7 = !{!"omnipotent char", !8, i64 0}
!8 = !{!"Simple C/C++ TBAA"}
!9 = distinct !{!9, !10, !11}
!10 = !{!"llvm.loop.mustprogress"}
!11 = !{!"llvm.loop.peeled.count", i32 1}
