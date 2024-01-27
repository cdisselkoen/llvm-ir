; ModuleID = 'linkedlist.c'
source_filename = "linkedlist.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%struct.SimpleLinkedList = type { i32, ptr }
%struct.NodeA = type { i32, ptr }
%struct.NodeB = type { i32, ptr }

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @simple_linked_list(i32 noundef %0) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.SimpleLinkedList, align 8
  %4 = alloca %struct.SimpleLinkedList, align 8
  %5 = alloca %struct.SimpleLinkedList, align 8
  %6 = alloca %struct.SimpleLinkedList, align 8
  %7 = alloca %struct.SimpleLinkedList, align 8
  store i32 %0, ptr %2, align 4
  %8 = getelementptr inbounds %struct.SimpleLinkedList, ptr %3, i32 0, i32 0
  %9 = load i32, ptr %2, align 4
  store i32 %9, ptr %8, align 8
  %10 = getelementptr inbounds %struct.SimpleLinkedList, ptr %3, i32 0, i32 1
  store ptr null, ptr %10, align 8
  %11 = getelementptr inbounds %struct.SimpleLinkedList, ptr %3, i32 0, i32 0
  %12 = load i32, ptr %11, align 8
  %13 = add nsw i32 %12, 2
  store i32 %13, ptr %11, align 8
  %14 = getelementptr inbounds %struct.SimpleLinkedList, ptr %4, i32 0, i32 0
  %15 = load i32, ptr %2, align 4
  %16 = sub nsw i32 %15, 3
  store i32 %16, ptr %14, align 8
  %17 = getelementptr inbounds %struct.SimpleLinkedList, ptr %4, i32 0, i32 1
  store ptr null, ptr %17, align 8
  %18 = getelementptr inbounds %struct.SimpleLinkedList, ptr %5, i32 0, i32 0
  %19 = load i32, ptr %2, align 4
  %20 = mul nsw i32 %19, 5
  store i32 %20, ptr %18, align 8
  %21 = getelementptr inbounds %struct.SimpleLinkedList, ptr %5, i32 0, i32 1
  store ptr null, ptr %21, align 8
  %22 = getelementptr inbounds %struct.SimpleLinkedList, ptr %6, i32 0, i32 0
  %23 = load i32, ptr %2, align 4
  %24 = sdiv i32 %23, 2
  store i32 %24, ptr %22, align 8
  %25 = getelementptr inbounds %struct.SimpleLinkedList, ptr %6, i32 0, i32 1
  store ptr null, ptr %25, align 8
  %26 = getelementptr inbounds %struct.SimpleLinkedList, ptr %7, i32 0, i32 0
  %27 = load i32, ptr %2, align 4
  %28 = sdiv i32 %27, 100
  store i32 %28, ptr %26, align 8
  %29 = getelementptr inbounds %struct.SimpleLinkedList, ptr %7, i32 0, i32 1
  store ptr null, ptr %29, align 8
  %30 = getelementptr inbounds %struct.SimpleLinkedList, ptr %3, i32 0, i32 1
  store ptr %4, ptr %30, align 8
  %31 = getelementptr inbounds %struct.SimpleLinkedList, ptr %4, i32 0, i32 1
  store ptr %5, ptr %31, align 8
  %32 = getelementptr inbounds %struct.SimpleLinkedList, ptr %5, i32 0, i32 1
  store ptr %6, ptr %32, align 8
  %33 = getelementptr inbounds %struct.SimpleLinkedList, ptr %6, i32 0, i32 1
  store ptr %7, ptr %33, align 8
  %34 = getelementptr inbounds %struct.SimpleLinkedList, ptr %7, i32 0, i32 1
  store ptr %3, ptr %34, align 8
  %35 = getelementptr inbounds %struct.SimpleLinkedList, ptr %3, i32 0, i32 1
  %36 = load ptr, ptr %35, align 8
  %37 = getelementptr inbounds %struct.SimpleLinkedList, ptr %36, i32 0, i32 1
  %38 = load ptr, ptr %37, align 8
  %39 = getelementptr inbounds %struct.SimpleLinkedList, ptr %38, i32 0, i32 1
  %40 = load ptr, ptr %39, align 8
  %41 = getelementptr inbounds %struct.SimpleLinkedList, ptr %40, i32 0, i32 1
  %42 = load ptr, ptr %41, align 8
  %43 = getelementptr inbounds %struct.SimpleLinkedList, ptr %42, i32 0, i32 1
  %44 = load ptr, ptr %43, align 8
  %45 = getelementptr inbounds %struct.SimpleLinkedList, ptr %44, i32 0, i32 1
  %46 = load ptr, ptr %45, align 8
  %47 = getelementptr inbounds %struct.SimpleLinkedList, ptr %46, i32 0, i32 1
  %48 = load ptr, ptr %47, align 8
  %49 = getelementptr inbounds %struct.SimpleLinkedList, ptr %48, i32 0, i32 1
  %50 = load ptr, ptr %49, align 8
  %51 = getelementptr inbounds %struct.SimpleLinkedList, ptr %50, i32 0, i32 1
  %52 = load ptr, ptr %51, align 8
  %53 = getelementptr inbounds %struct.SimpleLinkedList, ptr %52, i32 0, i32 1
  %54 = load ptr, ptr %53, align 8
  %55 = getelementptr inbounds %struct.SimpleLinkedList, ptr %54, i32 0, i32 0
  %56 = load i32, ptr %55, align 8
  ret i32 %56
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @indirectly_recursive_type(i32 noundef %0) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.NodeA, align 8
  %4 = alloca %struct.NodeB, align 8
  %5 = alloca %struct.NodeA, align 8
  store i32 %0, ptr %2, align 4
  %6 = getelementptr inbounds %struct.NodeA, ptr %3, i32 0, i32 0
  %7 = load i32, ptr %2, align 4
  store i32 %7, ptr %6, align 8
  %8 = getelementptr inbounds %struct.NodeA, ptr %3, i32 0, i32 1
  store ptr null, ptr %8, align 8
  %9 = getelementptr inbounds %struct.NodeB, ptr %4, i32 0, i32 0
  %10 = load i32, ptr %2, align 4
  %11 = add nsw i32 %10, 3
  store i32 %11, ptr %9, align 8
  %12 = getelementptr inbounds %struct.NodeB, ptr %4, i32 0, i32 1
  store ptr null, ptr %12, align 8
  %13 = getelementptr inbounds %struct.NodeA, ptr %5, i32 0, i32 0
  %14 = load i32, ptr %2, align 4
  %15 = sdiv i32 %14, 4
  store i32 %15, ptr %13, align 8
  %16 = getelementptr inbounds %struct.NodeA, ptr %5, i32 0, i32 1
  store ptr null, ptr %16, align 8
  %17 = getelementptr inbounds %struct.NodeA, ptr %3, i32 0, i32 1
  store ptr %4, ptr %17, align 8
  %18 = getelementptr inbounds %struct.NodeB, ptr %4, i32 0, i32 1
  store ptr %5, ptr %18, align 8
  %19 = getelementptr inbounds %struct.NodeA, ptr %5, i32 0, i32 1
  store ptr %4, ptr %19, align 8
  %20 = getelementptr inbounds %struct.NodeA, ptr %3, i32 0, i32 1
  %21 = load ptr, ptr %20, align 8
  %22 = getelementptr inbounds %struct.NodeB, ptr %21, i32 0, i32 1
  %23 = load ptr, ptr %22, align 8
  %24 = getelementptr inbounds %struct.NodeA, ptr %23, i32 0, i32 1
  %25 = load ptr, ptr %24, align 8
  %26 = getelementptr inbounds %struct.NodeB, ptr %25, i32 0, i32 1
  %27 = load ptr, ptr %26, align 8
  %28 = getelementptr inbounds %struct.NodeA, ptr %27, i32 0, i32 1
  %29 = load ptr, ptr %28, align 8
  %30 = getelementptr inbounds %struct.NodeB, ptr %29, i32 0, i32 1
  %31 = load ptr, ptr %30, align 8
  %32 = getelementptr inbounds %struct.NodeA, ptr %31, i32 0, i32 1
  %33 = load ptr, ptr %32, align 8
  %34 = getelementptr inbounds %struct.NodeB, ptr %33, i32 0, i32 0
  %35 = load i32, ptr %34, align 8
  ret i32 %35
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @takes_opaque_struct(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  store ptr %0, ptr %2, align 8
  %3 = load ptr, ptr %2, align 8
  %4 = icmp ne ptr %3, null
  %5 = zext i1 %4 to i32
  ret i32 %5
}

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{i32 7, !"frame-pointer", i32 2}
!5 = !{!"clang version 17.0.6"}
