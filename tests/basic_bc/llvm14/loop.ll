; ModuleID = 'loop.c'
source_filename = "loop.c"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx12.0.0"

; Function Attrs: nofree nounwind ssp uwtable
define void @loop(i32 noundef %0, i32 noundef %1) local_unnamed_addr #0 {
  %3 = alloca [10 x i32], align 16
  %4 = bitcast [10 x i32]* %3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 40, i8* nonnull %4) #3
  call void @llvm.memset.p0i8.i64(i8* nonnull align 16 %4, i8 0, i64 40, i1 true)
  %5 = add i32 %1, -1
  %6 = icmp ult i32 %5, 10
  br i1 %6, label %7, label %46

7:                                                ; preds = %2
  %8 = add nsw i32 %0, 3
  %9 = getelementptr inbounds [10 x i32], [10 x i32]* %3, i64 0, i64 0
  store volatile i32 %8, i32* %9, align 16, !tbaa !5
  %10 = icmp ult i32 %1, 2
  br i1 %10, label %46, label %11

11:                                               ; preds = %7
  %12 = zext i32 %1 to i64
  %13 = add nsw i64 %12, -1
  %14 = and i64 %13, 1
  %15 = icmp eq i32 %1, 2
  br i1 %15, label %36, label %16

16:                                               ; preds = %11
  %17 = and i64 %13, -2
  br label %18

18:                                               ; preds = %18, %16
  %19 = phi i64 [ 1, %16 ], [ %33, %18 ]
  %20 = phi i64 [ 0, %16 ], [ %34, %18 ]
  %21 = getelementptr inbounds [10 x i32], [10 x i32]* %3, i64 0, i64 %19
  store volatile i32 %8, i32* %21, align 4, !tbaa !5
  %22 = add nuw i64 %19, 4294967295
  %23 = and i64 %22, 4294967295
  %24 = getelementptr inbounds [10 x i32], [10 x i32]* %3, i64 0, i64 %23
  %25 = load volatile i32, i32* %24, align 4, !tbaa !5
  %26 = add nsw i32 %25, %1
  store volatile i32 %26, i32* %24, align 4, !tbaa !5
  %27 = add nuw nsw i64 %19, 1
  %28 = getelementptr inbounds [10 x i32], [10 x i32]* %3, i64 0, i64 %27
  store volatile i32 %8, i32* %28, align 4, !tbaa !5
  %29 = and i64 %19, 4294967295
  %30 = getelementptr inbounds [10 x i32], [10 x i32]* %3, i64 0, i64 %29
  %31 = load volatile i32, i32* %30, align 4, !tbaa !5
  %32 = add nsw i32 %31, %1
  store volatile i32 %32, i32* %30, align 4, !tbaa !5
  %33 = add nuw nsw i64 %19, 2
  %34 = add i64 %20, 2
  %35 = icmp eq i64 %34, %17
  br i1 %35, label %36, label %18, !llvm.loop !9

36:                                               ; preds = %18, %11
  %37 = phi i64 [ 1, %11 ], [ %33, %18 ]
  %38 = icmp eq i64 %14, 0
  br i1 %38, label %46, label %39

39:                                               ; preds = %36
  %40 = getelementptr inbounds [10 x i32], [10 x i32]* %3, i64 0, i64 %37
  store volatile i32 %8, i32* %40, align 4, !tbaa !5
  %41 = add nuw i64 %37, 4294967295
  %42 = and i64 %41, 4294967295
  %43 = getelementptr inbounds [10 x i32], [10 x i32]* %3, i64 0, i64 %42
  %44 = load volatile i32, i32* %43, align 4, !tbaa !5
  %45 = add nsw i32 %44, %1
  store volatile i32 %45, i32* %43, align 4, !tbaa !5
  br label %46

46:                                               ; preds = %39, %36, %7, %2
  call void @llvm.lifetime.end.p0i8(i64 40, i8* nonnull %4) #3
  ret void
}

; Function Attrs: argmemonly mustprogress nofree nosync nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: argmemonly mustprogress nofree nounwind willreturn writeonly
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #2

; Function Attrs: argmemonly mustprogress nofree nosync nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #1

attributes #0 = { nofree nounwind ssp uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "tune-cpu"="generic" }
attributes #1 = { argmemonly mustprogress nofree nosync nounwind willreturn }
attributes #2 = { argmemonly mustprogress nofree nounwind willreturn writeonly }
attributes #3 = { nounwind }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 1}
!3 = !{i32 7, !"frame-pointer", i32 2}
!4 = !{!"Homebrew clang version 14.0.6"}
!5 = !{!6, !6, i64 0}
!6 = !{!"int", !7, i64 0}
!7 = !{!"omnipotent char", !8, i64 0}
!8 = !{!"Simple C/C++ TBAA"}
!9 = distinct !{!9, !10, !11}
!10 = !{!"llvm.loop.mustprogress"}
!11 = !{!"llvm.loop.peeled.count", i32 1}
