; ModuleID = 'loop.c'
source_filename = "loop.c"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx16.0.0"

; Function Attrs: nofree norecurse nounwind ssp memory(inaccessiblemem: readwrite) uwtable(sync)
define void @loop(i32 noundef %0, i32 noundef %1) local_unnamed_addr #0 {
  %3 = alloca [10 x i32], align 4
  call void @llvm.lifetime.start.p0(i64 40, ptr nonnull %3) #3
  call void @llvm.memset.p0.i64(ptr nonnull align 4 %3, i8 0, i64 40, i1 true)
  %4 = add i32 %1, -1
  %5 = icmp ult i32 %4, 10
  br i1 %5, label %6, label %19

6:                                                ; preds = %2
  %7 = add nsw i32 %0, 3
  %8 = zext nneg i32 %1 to i64
  store volatile i32 %7, ptr %3, align 4, !tbaa !6
  %9 = icmp eq i32 %1, 1
  br i1 %9, label %19, label %10

10:                                               ; preds = %6, %10
  %11 = phi i64 [ %17, %10 ], [ 1, %6 ]
  %12 = getelementptr inbounds nuw [10 x i32], ptr %3, i64 0, i64 %11
  store volatile i32 %7, ptr %12, align 4, !tbaa !6
  %13 = add nsw i64 %11, -1
  %14 = getelementptr inbounds nuw [10 x i32], ptr %3, i64 0, i64 %13
  %15 = load volatile i32, ptr %14, align 4, !tbaa !6
  %16 = add nsw i32 %15, %1
  store volatile i32 %16, ptr %14, align 4, !tbaa !6
  %17 = add nuw nsw i64 %11, 1
  %18 = icmp eq i64 %17, %8
  br i1 %18, label %19, label %10, !llvm.loop !10

19:                                               ; preds = %10, %6, %2
  call void @llvm.lifetime.end.p0(i64 40, ptr nonnull %3) #3
  ret void
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr nocapture writeonly, i8, i64, i1 immarg) #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #1

attributes #0 = { nofree norecurse nounwind ssp memory(inaccessiblemem: readwrite) uwtable(sync) "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+altnzcv,+ccdp,+ccidx,+ccpp,+complxnum,+crc,+dit,+dotprod,+flagm,+fp-armv8,+fp16fml,+fptoint,+fullfp16,+jsconv,+lse,+neon,+pauth,+perfmon,+predres,+ras,+rcpc,+rdm,+sb,+sha2,+sha3,+specrestrict,+ssbs,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8a,+zcm,+zcz" }
attributes #1 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #2 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #3 = { nounwind }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 2, !"SDK Version", [2 x i32] [i32 26, i32 2]}
!1 = !{i32 1, !"wchar_size", i32 4}
!2 = !{i32 8, !"PIC Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 1}
!4 = !{i32 7, !"frame-pointer", i32 1}
!5 = !{!"Homebrew clang version 20.1.8"}
!6 = !{!7, !7, i64 0}
!7 = !{!"int", !8, i64 0}
!8 = !{!"omnipotent char", !9, i64 0}
!9 = !{!"Simple C/C++ TBAA"}
!10 = distinct !{!10, !11, !12}
!11 = !{!"llvm.loop.mustprogress"}
!12 = !{!"llvm.loop.peeled.count", i32 1}
