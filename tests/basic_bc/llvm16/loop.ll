; ModuleID = 'loop.c'
source_filename = "loop.c"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx13.0.0"

; Function Attrs: nofree nounwind ssp memory(inaccessiblemem: readwrite) uwtable(sync)
define void @loop(i32 noundef %0, i32 noundef %1) local_unnamed_addr #0 {
  %3 = alloca [10 x i32], align 4
  call void @llvm.lifetime.start.p0(i64 40, ptr nonnull %3) #3
  call void @llvm.memset.p0.i64(ptr nonnull align 4 %3, i8 0, i64 40, i1 true)
  %4 = add i32 %1, -1
  %5 = icmp ult i32 %4, 10
  br i1 %5, label %6, label %20

6:                                                ; preds = %2
  %7 = add nsw i32 %0, 3
  %8 = zext i32 %1 to i64
  store volatile i32 %7, ptr %3, align 4, !tbaa !5
  %9 = icmp ult i32 %1, 2
  br i1 %9, label %20, label %10

10:                                               ; preds = %6, %10
  %11 = phi i64 [ %18, %10 ], [ 1, %6 ]
  %12 = getelementptr inbounds [10 x i32], ptr %3, i64 0, i64 %11
  store volatile i32 %7, ptr %12, align 4, !tbaa !5
  %13 = add nuw i64 %11, 4294967295
  %14 = and i64 %13, 4294967295
  %15 = getelementptr inbounds [10 x i32], ptr %3, i64 0, i64 %14
  %16 = load volatile i32, ptr %15, align 4, !tbaa !5
  %17 = add nsw i32 %16, %1
  store volatile i32 %17, ptr %15, align 4, !tbaa !5
  %18 = add nuw nsw i64 %11, 1
  %19 = icmp eq i64 %18, %8
  br i1 %19, label %20, label %10, !llvm.loop !9

20:                                               ; preds = %10, %6, %2
  call void @llvm.lifetime.end.p0(i64 40, ptr nonnull %3) #3
  ret void
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr nocapture writeonly, i8, i64, i1 immarg) #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #1

attributes #0 = { nofree nounwind ssp memory(inaccessiblemem: readwrite) uwtable(sync) "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+crc,+crypto,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+lse,+neon,+ras,+rcpc,+rdm,+sha2,+sha3,+sm4,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #1 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #2 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #3 = { nounwind }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 1}
!3 = !{i32 7, !"frame-pointer", i32 1}
!4 = !{!"Homebrew clang version 16.0.6"}
!5 = !{!6, !6, i64 0}
!6 = !{!"int", !7, i64 0}
!7 = !{!"omnipotent char", !8, i64 0}
!8 = !{!"Simple C/C++ TBAA"}
!9 = distinct !{!9, !10, !11}
!10 = !{!"llvm.loop.mustprogress"}
!11 = !{!"llvm.loop.peeled.count", i32 1}
