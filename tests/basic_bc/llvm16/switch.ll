; ModuleID = 'switch.c'
source_filename = "switch.c"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx13.0.0"

@str = private unnamed_addr constant [16 x i8] c"reached default\00", align 1

; Function Attrs: nofree nounwind ssp uwtable(sync)
define i32 @has_a_switch(i32 noundef %0) local_unnamed_addr #0 {
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
  %11 = tail call i32 @puts(ptr nonnull dereferenceable(1) @str)
  br label %12

12:                                               ; preds = %1, %10, %9, %8, %7, %6, %5, %4, %3, %2
  %13 = phi i32 [ -1, %10 ], [ -3, %9 ], [ 0, %8 ], [ 77, %7 ], [ -33, %6 ], [ 1, %5 ], [ -5, %4 ], [ -7, %3 ], [ 5, %2 ], [ 3, %1 ]
  %14 = add nsw i32 %13, %0
  ret i32 %14
}

; Function Attrs: nofree nounwind
declare noundef i32 @puts(ptr nocapture noundef readonly) local_unnamed_addr #1

attributes #0 = { nofree nounwind ssp uwtable(sync) "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+crc,+crypto,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+lse,+neon,+ras,+rcpc,+rdm,+sha2,+sha3,+sm4,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #1 = { nofree nounwind }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 1}
!3 = !{i32 7, !"frame-pointer", i32 1}
!4 = !{!"Homebrew clang version 16.0.6"}
