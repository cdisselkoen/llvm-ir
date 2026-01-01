; ModuleID = 'linkedlist.c'
source_filename = "linkedlist.c"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx16.0.0"

%struct.SimpleLinkedList = type { i32, ptr }
%struct.NodeA = type { i32, ptr }
%struct.NodeB = type { i32, ptr }

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i32 @simple_linked_list(i32 noundef %0) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.SimpleLinkedList, align 8
  %4 = alloca %struct.SimpleLinkedList, align 8
  %5 = alloca %struct.SimpleLinkedList, align 8
  %6 = alloca %struct.SimpleLinkedList, align 8
  %7 = alloca %struct.SimpleLinkedList, align 8
  store i32 %0, ptr %2, align 4
  %8 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %3, i32 0, i32 0
  %9 = load i32, ptr %2, align 4
  store i32 %9, ptr %8, align 8
  %10 = getelementptr i8, ptr %3, i64 4
  call void @llvm.memset.p0.i64(ptr align 4 %10, i8 0, i64 4, i1 false)
  %11 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %3, i32 0, i32 1
  store ptr null, ptr %11, align 8
  %12 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %3, i32 0, i32 0
  %13 = load i32, ptr %12, align 8
  %14 = add nsw i32 %13, 2
  store i32 %14, ptr %12, align 8
  %15 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %4, i32 0, i32 0
  %16 = load i32, ptr %2, align 4
  %17 = sub nsw i32 %16, 3
  store i32 %17, ptr %15, align 8
  %18 = getelementptr i8, ptr %4, i64 4
  call void @llvm.memset.p0.i64(ptr align 4 %18, i8 0, i64 4, i1 false)
  %19 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %4, i32 0, i32 1
  store ptr null, ptr %19, align 8
  %20 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %5, i32 0, i32 0
  %21 = load i32, ptr %2, align 4
  %22 = mul nsw i32 %21, 5
  store i32 %22, ptr %20, align 8
  %23 = getelementptr i8, ptr %5, i64 4
  call void @llvm.memset.p0.i64(ptr align 4 %23, i8 0, i64 4, i1 false)
  %24 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %5, i32 0, i32 1
  store ptr null, ptr %24, align 8
  %25 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %6, i32 0, i32 0
  %26 = load i32, ptr %2, align 4
  %27 = sdiv i32 %26, 2
  store i32 %27, ptr %25, align 8
  %28 = getelementptr i8, ptr %6, i64 4
  call void @llvm.memset.p0.i64(ptr align 4 %28, i8 0, i64 4, i1 false)
  %29 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %6, i32 0, i32 1
  store ptr null, ptr %29, align 8
  %30 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %7, i32 0, i32 0
  %31 = load i32, ptr %2, align 4
  %32 = sdiv i32 %31, 100
  store i32 %32, ptr %30, align 8
  %33 = getelementptr i8, ptr %7, i64 4
  call void @llvm.memset.p0.i64(ptr align 4 %33, i8 0, i64 4, i1 false)
  %34 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %7, i32 0, i32 1
  store ptr null, ptr %34, align 8
  %35 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %3, i32 0, i32 1
  store ptr %4, ptr %35, align 8
  %36 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %4, i32 0, i32 1
  store ptr %5, ptr %36, align 8
  %37 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %5, i32 0, i32 1
  store ptr %6, ptr %37, align 8
  %38 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %6, i32 0, i32 1
  store ptr %7, ptr %38, align 8
  %39 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %7, i32 0, i32 1
  store ptr %3, ptr %39, align 8
  %40 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %3, i32 0, i32 1
  %41 = load ptr, ptr %40, align 8
  %42 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %41, i32 0, i32 1
  %43 = load ptr, ptr %42, align 8
  %44 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %43, i32 0, i32 1
  %45 = load ptr, ptr %44, align 8
  %46 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %45, i32 0, i32 1
  %47 = load ptr, ptr %46, align 8
  %48 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %47, i32 0, i32 1
  %49 = load ptr, ptr %48, align 8
  %50 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %49, i32 0, i32 1
  %51 = load ptr, ptr %50, align 8
  %52 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %51, i32 0, i32 1
  %53 = load ptr, ptr %52, align 8
  %54 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %53, i32 0, i32 1
  %55 = load ptr, ptr %54, align 8
  %56 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %55, i32 0, i32 1
  %57 = load ptr, ptr %56, align 8
  %58 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %57, i32 0, i32 1
  %59 = load ptr, ptr %58, align 8
  %60 = getelementptr inbounds nuw %struct.SimpleLinkedList, ptr %59, i32 0, i32 0
  %61 = load i32, ptr %60, align 8
  ret i32 %61
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr nocapture writeonly, i8, i64, i1 immarg) #1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i32 @indirectly_recursive_type(i32 noundef %0) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.NodeA, align 8
  %4 = alloca %struct.NodeB, align 8
  %5 = alloca %struct.NodeA, align 8
  store i32 %0, ptr %2, align 4
  %6 = getelementptr inbounds nuw %struct.NodeA, ptr %3, i32 0, i32 0
  %7 = load i32, ptr %2, align 4
  store i32 %7, ptr %6, align 8
  %8 = getelementptr i8, ptr %3, i64 4
  call void @llvm.memset.p0.i64(ptr align 4 %8, i8 0, i64 4, i1 false)
  %9 = getelementptr inbounds nuw %struct.NodeA, ptr %3, i32 0, i32 1
  store ptr null, ptr %9, align 8
  %10 = getelementptr inbounds nuw %struct.NodeB, ptr %4, i32 0, i32 0
  %11 = load i32, ptr %2, align 4
  %12 = add nsw i32 %11, 3
  store i32 %12, ptr %10, align 8
  %13 = getelementptr i8, ptr %4, i64 4
  call void @llvm.memset.p0.i64(ptr align 4 %13, i8 0, i64 4, i1 false)
  %14 = getelementptr inbounds nuw %struct.NodeB, ptr %4, i32 0, i32 1
  store ptr null, ptr %14, align 8
  %15 = getelementptr inbounds nuw %struct.NodeA, ptr %5, i32 0, i32 0
  %16 = load i32, ptr %2, align 4
  %17 = sdiv i32 %16, 4
  store i32 %17, ptr %15, align 8
  %18 = getelementptr i8, ptr %5, i64 4
  call void @llvm.memset.p0.i64(ptr align 4 %18, i8 0, i64 4, i1 false)
  %19 = getelementptr inbounds nuw %struct.NodeA, ptr %5, i32 0, i32 1
  store ptr null, ptr %19, align 8
  %20 = getelementptr inbounds nuw %struct.NodeA, ptr %3, i32 0, i32 1
  store ptr %4, ptr %20, align 8
  %21 = getelementptr inbounds nuw %struct.NodeB, ptr %4, i32 0, i32 1
  store ptr %5, ptr %21, align 8
  %22 = getelementptr inbounds nuw %struct.NodeA, ptr %5, i32 0, i32 1
  store ptr %4, ptr %22, align 8
  %23 = getelementptr inbounds nuw %struct.NodeA, ptr %3, i32 0, i32 1
  %24 = load ptr, ptr %23, align 8
  %25 = getelementptr inbounds nuw %struct.NodeB, ptr %24, i32 0, i32 1
  %26 = load ptr, ptr %25, align 8
  %27 = getelementptr inbounds nuw %struct.NodeA, ptr %26, i32 0, i32 1
  %28 = load ptr, ptr %27, align 8
  %29 = getelementptr inbounds nuw %struct.NodeB, ptr %28, i32 0, i32 1
  %30 = load ptr, ptr %29, align 8
  %31 = getelementptr inbounds nuw %struct.NodeA, ptr %30, i32 0, i32 1
  %32 = load ptr, ptr %31, align 8
  %33 = getelementptr inbounds nuw %struct.NodeB, ptr %32, i32 0, i32 1
  %34 = load ptr, ptr %33, align 8
  %35 = getelementptr inbounds nuw %struct.NodeA, ptr %34, i32 0, i32 1
  %36 = load ptr, ptr %35, align 8
  %37 = getelementptr inbounds nuw %struct.NodeB, ptr %36, i32 0, i32 0
  %38 = load i32, ptr %37, align 8
  ret i32 %38
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i32 @takes_opaque_struct(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %3 = load ptr, ptr %2, align 8
  %4 = icmp ne ptr %3, null
  %5 = zext i1 %4 to i32
  ret i32 %5
}

attributes #0 = { noinline nounwind optnone ssp uwtable(sync) "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+altnzcv,+ccdp,+ccidx,+ccpp,+complxnum,+crc,+dit,+dotprod,+flagm,+fp-armv8,+fp16fml,+fptoint,+fullfp16,+jsconv,+lse,+neon,+pauth,+perfmon,+predres,+ras,+rcpc,+rdm,+sb,+sha2,+sha3,+specrestrict,+ssbs,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8a,+zcm,+zcz" }
attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: write) }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 2, !"SDK Version", [2 x i32] [i32 26, i32 2]}
!1 = !{i32 1, !"wchar_size", i32 4}
!2 = !{i32 8, !"PIC Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 1}
!4 = !{i32 7, !"frame-pointer", i32 1}
!5 = !{!"Homebrew clang version 20.1.8"}
