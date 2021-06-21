; ModuleID = 'switch.c'
source_filename = "switch.c"
target datalayout = "e-m:o-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.16.0"

@str = private unnamed_addr constant [16 x i8] c"reached default\00", align 1

; Function Attrs: nofree nounwind ssp uwtable
define i32 @has_a_switch(i32) local_unnamed_addr #0 {
  switch i32 %0, label %10 [
    i32 0, label %12
    i32 1, label %2
    i32 13, label %3
    i32 26, label %4
    i32 33, label %5
    i32 142, label %6
    i32 1678, label %7
    i32 88, label %8
    i32 101, label %9
  ]

2:                                                ; preds = %1
  br label %12

3:                                                ; preds = %1
  br label %12

4:                                                ; preds = %1
  br label %12

5:                                                ; preds = %1
  br label %12

6:                                                ; preds = %1
  br label %12

7:                                                ; preds = %1
  br label %12

8:                                                ; preds = %1
  br label %12

9:                                                ; preds = %1
  br label %12

10:                                               ; preds = %1
  %11 = tail call i32 @puts(i8* getelementptr inbounds ([16 x i8], [16 x i8]* @str, i64 0, i64 0))
  br label %12

12:                                               ; preds = %1, %10, %9, %8, %7, %6, %5, %4, %3, %2
  %13 = phi i32 [ -1, %10 ], [ -3, %9 ], [ 0, %8 ], [ 77, %7 ], [ -33, %6 ], [ 1, %5 ], [ -5, %4 ], [ -7, %3 ], [ 5, %2 ], [ 3, %1 ]
  %14 = add nsw i32 %13, %0
  ret i32 %14
}

; Function Attrs: nofree nounwind
declare i32 @puts(i8* nocapture readonly) local_unnamed_addr #1

attributes #0 = { nofree nounwind ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }
attributes #1 = { nofree nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{!"clang version 9.0.1 "}
