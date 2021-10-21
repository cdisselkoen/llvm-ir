; ModuleID = 'issue_4.c'
source_filename = "issue_4.c"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx11.0.0"

%struct.output = type { double, double, double }

; Function Attrs: nofree norecurse nounwind ssp uwtable willreturn writeonly
define dso_local void @run(%struct.output* noalias nocapture sret(%struct.output) align 8 %0, float %1) local_unnamed_addr #0 {
  %3 = fpext float %1 to double
  %4 = getelementptr inbounds %struct.output, %struct.output* %0, i64 0, i32 0
  store double %3, double* %4, align 8, !tbaa !3
  ret void
}

attributes #0 = { nofree norecurse nounwind ssp uwtable willreturn writeonly "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "tune-cpu"="generic" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"Homebrew clang version 12.0.1"}
!3 = !{!4, !5, i64 0}
!4 = !{!"output", !5, i64 0, !5, i64 8, !5, i64 16}
!5 = !{!"double", !6, i64 0}
!6 = !{!"omnipotent char", !7, i64 0}
!7 = !{!"Simple C/C++ TBAA"}
