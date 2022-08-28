; ModuleID = 'issue_4.c'
source_filename = "issue_4.c"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx12.0.0"

%struct.output = type { double, double, double }

; Function Attrs: mustprogress nofree norecurse nosync nounwind ssp uwtable willreturn writeonly
define void @run(%struct.output* noalias nocapture writeonly sret(%struct.output) align 8 %0, float noundef %1) local_unnamed_addr #0 {
  %3 = fpext float %1 to double
  %4 = getelementptr inbounds %struct.output, %struct.output* %0, i64 0, i32 0
  store double %3, double* %4, align 8, !tbaa !5
  ret void
}

attributes #0 = { mustprogress nofree norecurse nosync nounwind ssp uwtable willreturn writeonly "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "tune-cpu"="generic" }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 1}
!3 = !{i32 7, !"frame-pointer", i32 2}
!4 = !{!"Homebrew clang version 14.0.6"}
!5 = !{!6, !7, i64 0}
!6 = !{!"output", !7, i64 0, !7, i64 8, !7, i64 16}
!7 = !{!"double", !8, i64 0}
!8 = !{!"omnipotent char", !9, i64 0}
!9 = !{!"Simple C/C++ TBAA"}
