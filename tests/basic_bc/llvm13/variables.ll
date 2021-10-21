; ModuleID = 'variables.c'
source_filename = "variables.c"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx11.0.0"

@global = global i32 5, align 4

; Function Attrs: nofree nounwind ssp uwtable
define void @variables(i32 %0, i32* %1) local_unnamed_addr #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store volatile i32 %0, i32* %3, align 4, !tbaa !5
  %5 = bitcast i32* %4 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %5)
  store volatile i32 72, i32* %4, align 4, !tbaa !5
  %6 = tail call align 16 dereferenceable_or_null(4) i8* @malloc(i64 4) #3
  %7 = bitcast i8* %6 to i32*
  %8 = load volatile i32, i32* %4, align 4, !tbaa !5
  %9 = add nsw i32 %8, 5
  store volatile i32 %9, i32* %4, align 4, !tbaa !5
  %10 = load volatile i32, i32* %7, align 16, !tbaa !5
  %11 = add nsw i32 %10, 5
  store volatile i32 %11, i32* %7, align 16, !tbaa !5
  %12 = load volatile i32, i32* @global, align 4, !tbaa !5
  %13 = add nsw i32 %12, 5
  store volatile i32 %13, i32* @global, align 4, !tbaa !5
  %14 = load volatile i32, i32* %3, align 4, !tbaa !5
  %15 = add nsw i32 %14, 5
  store volatile i32 %15, i32* %3, align 4, !tbaa !5
  %16 = load volatile i32, i32* %1, align 4, !tbaa !5
  %17 = add nsw i32 %16, 5
  store volatile i32 %17, i32* %1, align 4, !tbaa !5
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %5)
  ret void
}

; Function Attrs: argmemonly mustprogress nofree nosync nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: inaccessiblememonly mustprogress nofree nounwind willreturn allocsize(0)
declare noalias noundef align 16 i8* @malloc(i64 noundef) local_unnamed_addr #2

; Function Attrs: argmemonly mustprogress nofree nosync nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #1

attributes #0 = { nofree nounwind ssp uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "tune-cpu"="generic" }
attributes #1 = { argmemonly mustprogress nofree nosync nounwind willreturn }
attributes #2 = { inaccessiblememonly mustprogress nofree nounwind willreturn allocsize(0) "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "tune-cpu"="generic" }
attributes #3 = { allocsize(0) }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 1}
!3 = !{i32 7, !"frame-pointer", i32 2}
!4 = !{!"Homebrew clang version 13.0.0"}
!5 = !{!6, !6, i64 0}
!6 = !{!"int", !7, i64 0}
!7 = !{!"omnipotent char", !8, i64 0}
!8 = !{!"Simple C/C++ TBAA"}
