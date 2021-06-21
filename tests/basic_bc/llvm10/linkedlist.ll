; ModuleID = 'linkedlist.c'
source_filename = "linkedlist.c"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.16.0"

%struct.SimpleLinkedList = type { i32, %struct.SimpleLinkedList* }
%struct.NodeA = type { i32, %struct.NodeB* }
%struct.NodeB = type { i32, %struct.NodeA* }
%struct.SomeOpaqueStruct = type opaque

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @simple_linked_list(i32 %0) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.SimpleLinkedList, align 8
  %4 = alloca %struct.SimpleLinkedList, align 8
  %5 = alloca %struct.SimpleLinkedList, align 8
  %6 = alloca %struct.SimpleLinkedList, align 8
  %7 = alloca %struct.SimpleLinkedList, align 8
  store i32 %0, i32* %2, align 4
  %8 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %3, i32 0, i32 0
  %9 = load i32, i32* %2, align 4
  store i32 %9, i32* %8, align 8
  %10 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %3, i32 0, i32 1
  store %struct.SimpleLinkedList* null, %struct.SimpleLinkedList** %10, align 8
  %11 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %3, i32 0, i32 0
  %12 = load i32, i32* %11, align 8
  %13 = add nsw i32 %12, 2
  store i32 %13, i32* %11, align 8
  %14 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %4, i32 0, i32 0
  %15 = load i32, i32* %2, align 4
  %16 = sub nsw i32 %15, 3
  store i32 %16, i32* %14, align 8
  %17 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %4, i32 0, i32 1
  store %struct.SimpleLinkedList* null, %struct.SimpleLinkedList** %17, align 8
  %18 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %5, i32 0, i32 0
  %19 = load i32, i32* %2, align 4
  %20 = mul nsw i32 %19, 5
  store i32 %20, i32* %18, align 8
  %21 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %5, i32 0, i32 1
  store %struct.SimpleLinkedList* null, %struct.SimpleLinkedList** %21, align 8
  %22 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %6, i32 0, i32 0
  %23 = load i32, i32* %2, align 4
  %24 = sdiv i32 %23, 2
  store i32 %24, i32* %22, align 8
  %25 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %6, i32 0, i32 1
  store %struct.SimpleLinkedList* null, %struct.SimpleLinkedList** %25, align 8
  %26 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %7, i32 0, i32 0
  %27 = load i32, i32* %2, align 4
  %28 = sdiv i32 %27, 100
  store i32 %28, i32* %26, align 8
  %29 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %7, i32 0, i32 1
  store %struct.SimpleLinkedList* null, %struct.SimpleLinkedList** %29, align 8
  %30 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %3, i32 0, i32 1
  store %struct.SimpleLinkedList* %4, %struct.SimpleLinkedList** %30, align 8
  %31 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %4, i32 0, i32 1
  store %struct.SimpleLinkedList* %5, %struct.SimpleLinkedList** %31, align 8
  %32 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %5, i32 0, i32 1
  store %struct.SimpleLinkedList* %6, %struct.SimpleLinkedList** %32, align 8
  %33 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %6, i32 0, i32 1
  store %struct.SimpleLinkedList* %7, %struct.SimpleLinkedList** %33, align 8
  %34 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %7, i32 0, i32 1
  store %struct.SimpleLinkedList* %3, %struct.SimpleLinkedList** %34, align 8
  %35 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %3, i32 0, i32 1
  %36 = load %struct.SimpleLinkedList*, %struct.SimpleLinkedList** %35, align 8
  %37 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %36, i32 0, i32 1
  %38 = load %struct.SimpleLinkedList*, %struct.SimpleLinkedList** %37, align 8
  %39 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %38, i32 0, i32 1
  %40 = load %struct.SimpleLinkedList*, %struct.SimpleLinkedList** %39, align 8
  %41 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %40, i32 0, i32 1
  %42 = load %struct.SimpleLinkedList*, %struct.SimpleLinkedList** %41, align 8
  %43 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %42, i32 0, i32 1
  %44 = load %struct.SimpleLinkedList*, %struct.SimpleLinkedList** %43, align 8
  %45 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %44, i32 0, i32 1
  %46 = load %struct.SimpleLinkedList*, %struct.SimpleLinkedList** %45, align 8
  %47 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %46, i32 0, i32 1
  %48 = load %struct.SimpleLinkedList*, %struct.SimpleLinkedList** %47, align 8
  %49 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %48, i32 0, i32 1
  %50 = load %struct.SimpleLinkedList*, %struct.SimpleLinkedList** %49, align 8
  %51 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %50, i32 0, i32 1
  %52 = load %struct.SimpleLinkedList*, %struct.SimpleLinkedList** %51, align 8
  %53 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %52, i32 0, i32 1
  %54 = load %struct.SimpleLinkedList*, %struct.SimpleLinkedList** %53, align 8
  %55 = getelementptr inbounds %struct.SimpleLinkedList, %struct.SimpleLinkedList* %54, i32 0, i32 0
  %56 = load i32, i32* %55, align 8
  ret i32 %56
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @indirectly_recursive_type(i32 %0) #0 {
  %2 = alloca i32, align 4
  %3 = alloca %struct.NodeA, align 8
  %4 = alloca %struct.NodeB, align 8
  %5 = alloca %struct.NodeA, align 8
  store i32 %0, i32* %2, align 4
  %6 = getelementptr inbounds %struct.NodeA, %struct.NodeA* %3, i32 0, i32 0
  %7 = load i32, i32* %2, align 4
  store i32 %7, i32* %6, align 8
  %8 = getelementptr inbounds %struct.NodeA, %struct.NodeA* %3, i32 0, i32 1
  store %struct.NodeB* null, %struct.NodeB** %8, align 8
  %9 = getelementptr inbounds %struct.NodeB, %struct.NodeB* %4, i32 0, i32 0
  %10 = load i32, i32* %2, align 4
  %11 = add nsw i32 %10, 3
  store i32 %11, i32* %9, align 8
  %12 = getelementptr inbounds %struct.NodeB, %struct.NodeB* %4, i32 0, i32 1
  store %struct.NodeA* null, %struct.NodeA** %12, align 8
  %13 = getelementptr inbounds %struct.NodeA, %struct.NodeA* %5, i32 0, i32 0
  %14 = load i32, i32* %2, align 4
  %15 = sdiv i32 %14, 4
  store i32 %15, i32* %13, align 8
  %16 = getelementptr inbounds %struct.NodeA, %struct.NodeA* %5, i32 0, i32 1
  store %struct.NodeB* null, %struct.NodeB** %16, align 8
  %17 = getelementptr inbounds %struct.NodeA, %struct.NodeA* %3, i32 0, i32 1
  store %struct.NodeB* %4, %struct.NodeB** %17, align 8
  %18 = getelementptr inbounds %struct.NodeB, %struct.NodeB* %4, i32 0, i32 1
  store %struct.NodeA* %5, %struct.NodeA** %18, align 8
  %19 = getelementptr inbounds %struct.NodeA, %struct.NodeA* %5, i32 0, i32 1
  store %struct.NodeB* %4, %struct.NodeB** %19, align 8
  %20 = getelementptr inbounds %struct.NodeA, %struct.NodeA* %3, i32 0, i32 1
  %21 = load %struct.NodeB*, %struct.NodeB** %20, align 8
  %22 = getelementptr inbounds %struct.NodeB, %struct.NodeB* %21, i32 0, i32 1
  %23 = load %struct.NodeA*, %struct.NodeA** %22, align 8
  %24 = getelementptr inbounds %struct.NodeA, %struct.NodeA* %23, i32 0, i32 1
  %25 = load %struct.NodeB*, %struct.NodeB** %24, align 8
  %26 = getelementptr inbounds %struct.NodeB, %struct.NodeB* %25, i32 0, i32 1
  %27 = load %struct.NodeA*, %struct.NodeA** %26, align 8
  %28 = getelementptr inbounds %struct.NodeA, %struct.NodeA* %27, i32 0, i32 1
  %29 = load %struct.NodeB*, %struct.NodeB** %28, align 8
  %30 = getelementptr inbounds %struct.NodeB, %struct.NodeB* %29, i32 0, i32 1
  %31 = load %struct.NodeA*, %struct.NodeA** %30, align 8
  %32 = getelementptr inbounds %struct.NodeA, %struct.NodeA* %31, i32 0, i32 1
  %33 = load %struct.NodeB*, %struct.NodeB** %32, align 8
  %34 = getelementptr inbounds %struct.NodeB, %struct.NodeB* %33, i32 0, i32 0
  %35 = load i32, i32* %34, align 8
  ret i32 %35
}

; Function Attrs: noinline nounwind optnone ssp uwtable
define i32 @takes_opaque_struct(%struct.SomeOpaqueStruct* %0) #0 {
  %2 = alloca %struct.SomeOpaqueStruct*, align 8
  store %struct.SomeOpaqueStruct* %0, %struct.SomeOpaqueStruct** %2, align 8
  %3 = load %struct.SomeOpaqueStruct*, %struct.SomeOpaqueStruct** %2, align 8
  %4 = icmp ne %struct.SomeOpaqueStruct* %3, null
  %5 = zext i1 %4 to i32
  ret i32 %5
}

attributes #0 = { noinline nounwind optnone ssp uwtable "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "frame-pointer"="all" "less-precise-fpmad"="false" "min-legal-vector-width"="0" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="penryn" "target-features"="+cx16,+cx8,+fxsr,+mmx,+sahf,+sse,+sse2,+sse3,+sse4.1,+ssse3,+x87" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 2, !"SDK Version", [2 x i32] [i32 11, i32 3]}
!1 = !{i32 1, !"wchar_size", i32 4}
!2 = !{i32 7, !"PIC Level", i32 2}
!3 = !{!"clang version 10.0.0 "}
