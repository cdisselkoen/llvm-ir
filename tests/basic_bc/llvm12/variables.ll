; ModuleID = 'variables.c'
source_filename = "variables.c"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx11.0.0"

@global = dso_local global i32 5, align 4

; Function Attrs: nofree nounwind ssp uwtable willreturn
define dso_local void @variables(i32 %0, i32* %1) local_unnamed_addr #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store volatile i32 %0, i32* %3, align 4, !tbaa !3
  %5 = bitcast i32* %4 to i8*
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %5)
  store volatile i32 72, i32* %4, align 4, !tbaa !3
  %6 = tail call dereferenceable_or_null(4) i8* @malloc(i64 4) #3
  %7 = bitcast i8* %6 to i32*
  %8 = load volatile i32, i32* %4, align 4, !tbaa !3
  %9 = add nsw i32 %8, 5
  store volatile i32 %9, i32* %4, align 4, !tbaa !3
  %10 = load volatile i32, i32* %7, align 4, !tbaa !3
  %11 = add nsw i32 %10, 5
  store volatile i32 %11, i32* %7, align 4, !tbaa !3
  %12 = load volatile i32, i32* @global, align 4, !tbaa !3
  %13 = add nsw i32 %12, 5
  store volatile i32 %13, i32* @global, align 4, !tbaa !3
  %14 = load volatile i32, i32* %3, align 4, !tbaa !3
  %15 = add nsw i32 %14, 5
  store volatile i32 %15, i32* %3, align 4, !tbaa !3
  %16 = load volatile i32, i32* %1, align 4, !tbaa !3
  %17 = add nsw i32 %16, 5
  store volatile i32 %17, i32* %1, align 4, !tbaa !3
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %5)
  ret void
}

; Function Attrs: argmemonly nofree nosync nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: inaccessiblememonly nofree nounwind willreturn allocsize(0)
declare noalias noundef i8* @malloc(i64) local_unnamed_addr #2

; Function Attrs: argmemonly nofree nosync nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #1

attributes #0 = { nofree nounwind ssp uwtable willreturn "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "tune-cpu"="generic" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { argmemonly nofree nosync nounwind willreturn }
attributes #2 = { inaccessiblememonly nofree nounwind willreturn allocsize(0) "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "no-infs-fp-math"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "tune-cpu"="generic" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #3 = { allocsize(0) }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"Homebrew clang version 12.0.1"}
!3 = !{!4, !4, i64 0}
!4 = !{!"int", !5, i64 0}
!5 = !{!"omnipotent char", !6, i64 0}
!6 = !{!"Simple C/C++ TBAA"}
