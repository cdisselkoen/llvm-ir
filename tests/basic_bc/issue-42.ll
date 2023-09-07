; ModuleID = 'stack-copies'
source_filename = "stack-copies"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"[]u8" = type { i8*, i64 }
%std.builtin.StackTrace = type { i64, %"[]usize" }
%"[]usize" = type { i64*, i64 }
%std.target.LinuxVersionRange = type { %std.builtin.Range, %std.builtin.Version }
%std.builtin.Range = type { %std.builtin.Version, %std.builtin.Version }
%std.builtin.Version = type { i32, i32, i32 }
%std.target.Set = type { [5 x i64] }
%"[]std.target.x86.Feature" = type { i8*, i64 }
%std.target.Model = type { %"[]u8", %"?[:0]const u8", %std.target.Set }
%"?[:0]const u8" = type { %"[]u8", i1 }
%std.Thread.Mutex = type { %std.Thread.Mutex.AtomicMutex }
%std.Thread.Mutex.AtomicMutex = type { i32 }
%std.os.linux.siginfo_t = type { i32, i32, i32, %std.os.linux.siginfo_fields_union }
%std.os.linux.siginfo_fields_union = type { %"std.os.linux.struct:3360:13", [80 x i8] }
%"std.os.linux.struct:3360:13" = type { %"std.os.linux.union:3361:16", %"std.os.linux.union:3371:17" }
%"std.os.linux.union:3361:16" = type { %"std.os.linux.struct:3362:21" }
%"std.os.linux.struct:3362:21" = type { i32, i32 }
%"std.os.linux.union:3371:17" = type { %std.os.linux.sigval, [16 x i8] }
%std.os.linux.sigval = type { i8* }
%"?u64" = type { i64, i1 }
%std.Thread.StaticResetEvent = type { %std.Thread.StaticResetEvent.AtomicEvent }
%std.Thread.StaticResetEvent.AtomicEvent = type { i32 }
%std.fmt.Specifier = type { { i64, [8 x i8] }, i2 }
%"?usize" = type { i64, i1 }
%"?u8" = type { i8, i1 }
%std.fs.file.File = type { i32 }
%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)" = type { %std.fs.file.File }
%"std.debug.struct:296:56" = type { i64 }
%"std.debug.struct:298:40" = type { %"[]u8" }
%Big = type { [4096 x i8], %Small }
%Small = type { [128 x i8] }
%"[]Big" = type { %Big*, i64 }
%std.os.linux.Sigaction = type { %"std.os.linux.union:3060:14", [32 x i32], i32, void ()* }
%"std.os.linux.union:3060:14" = type { void (i32)* }
%std.fmt.FormatOptions = type { %"?usize", %"?usize", i2, i8 }
%std.os.linux.k_sigaction = type { void (i32)*, i64, void ()*, [2 x i32] }
%"?std.Thread.Mutex.State" = type { i32, i1 }
%std.os.linux.timespec = type { i64, i64 }
%"std.os.struct:4985:53" = type { i16 }
%std.fmt.Parser = type { %"[]u8", i64 }
%std.fmt.ArgState = type { i64, i32, i64 }
%"?u32" = type { i32, i1 }
%std.target.Target = type { %std.target.Cpu, %std.target.Os, i5 }
%std.target.Cpu = type { i6, %std.target.Model*, %std.target.Set }
%std.target.Os = type { i6, %std.target.VersionRange }
%std.target.VersionRange = type { { %std.builtin.Range, [12 x i8] }, i2 }

@panic = internal unnamed_addr constant void (%"[]u8"*, %std.builtin.StackTrace*)* @std.builtin.default_panic, align 8
@zig_is_stage2 = internal unnamed_addr constant i1 false, align 1
@output_mode = internal unnamed_addr constant i2 1, align 1
@link_mode = internal unnamed_addr constant i1 false, align 1
@os = internal unnamed_addr constant { <{ i6, [3 x i8] }>, { %std.target.LinuxVersionRange, i2, [3 x i8] } } { <{ i6, [3 x i8] }> <{ i6 8, [3 x i8] undef }>, { %std.target.LinuxVersionRange, i2, [3 x i8] } { %std.target.LinuxVersionRange { %std.builtin.Range { %std.builtin.Version { i32 6, i32 1, i32 38 }, %std.builtin.Version { i32 6, i32 1, i32 38 } }, %std.builtin.Version { i32 2, i32 19, i32 0 } }, i2 -2, [3 x i8] undef } }, align 4
@mode = internal unnamed_addr constant i2 0, align 1
@runtime_safety = internal unnamed_addr constant i1 true, align 1
@native_os = internal unnamed_addr constant i6 8, align 1
@have_segfault_handling_support = internal unnamed_addr constant i1 true, align 1
@enable_segfault_handler = internal unnamed_addr constant i1 true, align 1
@panic_stage = internal thread_local unnamed_addr global i64 0, align 8
@panicking = internal unnamed_addr global i8 0, align 1
@single_threaded = internal unnamed_addr constant i1 false, align 1
@featureSet = internal unnamed_addr constant void (%std.target.Set*, %"[]std.target.x86.Feature"*)* @"std.target.Feature.feature_set_fns(std.target.x86.Feature).featureSet", align 8
@0 = internal unnamed_addr constant [7 x i8] c"x86_64\00", align 1
@1 = internal unnamed_addr constant [7 x i8] c"x86-64\00", align 1
@x86_64 = internal unnamed_addr constant { %"[]u8", { %"[]u8", i1, [7 x i8] }, %std.target.Set } { %"[]u8" { i8* getelementptr inbounds ([7 x i8], [7 x i8]* @0, i64 0, i64 0), i64 6 }, { %"[]u8", i1, [7 x i8] } { %"[]u8" { i8* getelementptr inbounds ([7 x i8], [7 x i8]* @1, i64 0, i64 0), i64 6 }, i1 true, [7 x i8] undef }, %std.target.Set { [5 x i64] [i64 4611686190226079760, i64 18040786788685828, i64 272, i64 0, i64 0] } }, align 8
@cpu = internal unnamed_addr constant { <{ i6, [7 x i8] }>, %std.target.Model*, %std.target.Set } { <{ i6, [7 x i8] }> <{ i6 -30, [7 x i8] undef }>, %std.target.Model* bitcast ({ %"[]u8", { %"[]u8", i1, [7 x i8] }, %std.target.Set }* @x86_64 to %std.target.Model*), %std.target.Set { [5 x i64] [i64 -3170532783948821392, i64 1432177808459365388, i64 15676, i64 0, i64 0] } }, align 8
@abi = internal unnamed_addr constant i5 1, align 1
@target = internal unnamed_addr constant { { <{ i6, [7 x i8] }>, %std.target.Model*, %std.target.Set }, { <{ i6, [3 x i8] }>, { %std.target.LinuxVersionRange, i2, [3 x i8] } }, <{ i5, [3 x i8] }> } { { <{ i6, [7 x i8] }>, %std.target.Model*, %std.target.Set } { <{ i6, [7 x i8] }> <{ i6 -30, [7 x i8] undef }>, %std.target.Model* bitcast ({ %"[]u8", { %"[]u8", i1, [7 x i8] }, %std.target.Set }* @x86_64 to %std.target.Model*), %std.target.Set { [5 x i64] [i64 -3170532783948821392, i64 1432177808459365388, i64 15676, i64 0, i64 0] } }, { <{ i6, [3 x i8] }>, { %std.target.LinuxVersionRange, i2, [3 x i8] } } { <{ i6, [3 x i8] }> <{ i6 8, [3 x i8] undef }>, { %std.target.LinuxVersionRange, i2, [3 x i8] } { %std.target.LinuxVersionRange { %std.builtin.Range { %std.builtin.Version { i32 6, i32 1, i32 38 }, %std.builtin.Version { i32 6, i32 1, i32 38 } }, %std.builtin.Version { i32 2, i32 19, i32 0 } }, i2 -2, [3 x i8] undef } }, <{ i5, [3 x i8] }> <{ i5 1, [3 x i8] undef }> }, align 8
@target.1 = internal unnamed_addr constant { { <{ i6, [7 x i8] }>, %std.target.Model*, %std.target.Set }, { <{ i6, [3 x i8] }>, { %std.target.LinuxVersionRange, i2, [3 x i8] } }, <{ i5, [3 x i8] }> } { { <{ i6, [7 x i8] }>, %std.target.Model*, %std.target.Set } { <{ i6, [7 x i8] }> <{ i6 -30, [7 x i8] undef }>, %std.target.Model* bitcast ({ %"[]u8", { %"[]u8", i1, [7 x i8] }, %std.target.Set }* @x86_64 to %std.target.Model*), %std.target.Set { [5 x i64] [i64 -3170532783948821392, i64 1432177808459365388, i64 15676, i64 0, i64 0] } }, { <{ i6, [3 x i8] }>, { %std.target.LinuxVersionRange, i2, [3 x i8] } } { <{ i6, [3 x i8] }> <{ i6 8, [3 x i8] undef }>, { %std.target.LinuxVersionRange, i2, [3 x i8] } { %std.target.LinuxVersionRange { %std.builtin.Range { %std.builtin.Version { i32 6, i32 1, i32 38 }, %std.builtin.Version { i32 6, i32 1, i32 38 } }, %std.builtin.Version { i32 2, i32 19, i32 0 } }, i2 -2, [3 x i8] undef } }, <{ i5, [3 x i8] }> <{ i5 1, [3 x i8] undef }> }, align 8
@link_libc = internal unnamed_addr constant i1 false, align 1
@use_pthreads = internal unnamed_addr constant i1 false, align 1
@panic_mutex = internal unnamed_addr global %std.Thread.Mutex zeroinitializer, align 4
@is_windows = internal unnamed_addr constant i1 false, align 1
@mode.2 = internal unnamed_addr constant i1 false, align 1
@is_async = internal unnamed_addr constant i1 false, align 1
@native_arch = internal unnamed_addr constant i6 -30, align 1
@is_mips = internal unnamed_addr constant i1 false, align 1
@is_sparc = internal unnamed_addr constant i1 false, align 1
@DFL = internal unnamed_addr constant void (i32, %std.os.linux.siginfo_t*, i8*)* null, align 8
@empty_sigset = internal unnamed_addr constant [32 x i32] zeroinitializer, align 4
@empty_sigset.3 = internal unnamed_addr constant [32 x i32] zeroinitializer, align 4
@is_windows.4 = internal unnamed_addr constant i1 false, align 1
@assert = internal unnamed_addr constant void (i1)* @std.debug.assert, align 8
@strip_debug_info = internal unnamed_addr constant i1 true, align 1
@errno = internal unnamed_addr constant i16 (i64)* @std.os.linux.getErrno, align 8
@tls_thread_id = internal thread_local unnamed_addr global %"?u64" { i64 undef, i1 false }, align 8
@2 = internal unnamed_addr constant [4 x i8] c"any\00", align 1
@ANY = internal unnamed_addr constant [4 x i8]* @2, align 8
@app_mask = internal unnamed_addr constant [32 x i32] [i32 -4, i32 2147483647, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1, i32 -1], align 4
@assert.5 = internal unnamed_addr constant void (i1)* @std.debug.assert, align 8
@restore_rt = internal unnamed_addr constant void ()* @std.os.linux.x86_64.restore_rt, align 8
@restore = internal unnamed_addr constant void ()* @std.os.linux.x86_64.restore_rt, align 8
@restore.6 = internal unnamed_addr constant void ()* @std.os.linux.x86_64.restore_rt, align 8
@syscall4 = internal unnamed_addr constant i64 (i64, i64, i64, i64, i64)* @std.os.linux.x86_64.syscall4, align 8
@syscall3 = internal unnamed_addr constant i64 (i64, i64, i64, i64)* @std.os.linux.x86_64.syscall3, align 8
@unexpected_error_tracing = internal unnamed_addr constant i1 true, align 1
@syscall0 = internal unnamed_addr constant i64 (i64)* @std.os.linux.x86_64.syscall0, align 8
@syscall2 = internal unnamed_addr constant i64 (i64, i64, i64)* @std.os.linux.x86_64.syscall2, align 8
@syscall1 = internal unnamed_addr constant i64 (i64, i64)* @std.os.linux.x86_64.syscall1, align 8
@target.7 = internal unnamed_addr constant { { <{ i6, [7 x i8] }>, %std.target.Model*, %std.target.Set }, { <{ i6, [3 x i8] }>, { %std.target.LinuxVersionRange, i2, [3 x i8] } }, <{ i5, [3 x i8] }> } { { <{ i6, [7 x i8] }>, %std.target.Model*, %std.target.Set } { <{ i6, [7 x i8] }> <{ i6 -30, [7 x i8] undef }>, %std.target.Model* bitcast ({ %"[]u8", { %"[]u8", i1, [7 x i8] }, %std.target.Set }* @x86_64 to %std.target.Model*), %std.target.Set { [5 x i64] [i64 -3170532783948821392, i64 1432177808459365388, i64 15676, i64 0, i64 0] } }, { <{ i6, [3 x i8] }>, { %std.target.LinuxVersionRange, i2, [3 x i8] } } { <{ i6, [3 x i8] }> <{ i6 8, [3 x i8] undef }>, { %std.target.LinuxVersionRange, i2, [3 x i8] } { %std.target.LinuxVersionRange { %std.builtin.Range { %std.builtin.Version { i32 6, i32 1, i32 38 }, %std.builtin.Version { i32 6, i32 1, i32 38 } }, %std.builtin.Version { i32 2, i32 19, i32 0 } }, i2 -2, [3 x i8] undef } }, <{ i5, [3 x i8] }> <{ i5 1, [3 x i8] undef }> }, align 8
@stderr_mutex = internal unnamed_addr global %std.Thread.Mutex zeroinitializer, align 4
@assert.8 = internal unnamed_addr constant void (i1)* @std.debug.assert, align 8
@assert.9 = internal unnamed_addr constant void (i1)* @std.debug.assert, align 8
@3 = internal unnamed_addr constant [20 x i8] c"index out of bounds\00", align 1
@4 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([20 x i8], [20 x i8]* @3, i64 0, i64 0), i64 19 }, align 8
@5 = internal unnamed_addr constant %std.Thread.StaticResetEvent zeroinitializer, align 4
@6 = internal unnamed_addr constant [25 x i8] c"reached unreachable code\00", align 1
@7 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([25 x i8], [25 x i8]* @6, i64 0, i64 0), i64 24 }, align 8
@8 = internal unnamed_addr constant { void (i32, %std.os.linux.siginfo_t*, i8*)*, [32 x i32], <{ i32, [4 x i8] }>, void ()* } { void (i32, %std.os.linux.siginfo_t*, i8*)* null, [32 x i32] zeroinitializer, <{ i32, [4 x i8] }> <{ i32 0, [4 x i8] undef }>, void ()* null }, align 8
@9 = internal unnamed_addr constant [5 x i64] zeroinitializer, align 8
@10 = internal unnamed_addr constant [17 x i8] c"division by zero\00", align 1
@11 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([17 x i8], [17 x i8]* @10, i64 0, i64 0), i64 16 }, align 8
@12 = internal unnamed_addr constant [45 x i8] c"remainder division by zero or negative value\00", align 1
@13 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([45 x i8], [45 x i8]* @12, i64 0, i64 0), i64 44 }, align 8
@14 = internal unnamed_addr constant [28 x i8] c"integer cast truncated bits\00", align 1
@15 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([28 x i8], [28 x i8]* @14, i64 0, i64 0), i64 27 }, align 8
@16 = internal unnamed_addr constant [18 x i8] c"thread {} panic: \00", align 1
@17 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([18 x i8], [18 x i8]* @16, i64 0, i64 0), i64 7 }, align 8
@18 = internal unnamed_addr constant { %"[]u8", i8, <{ i2, [6 x i8] }>, %std.fmt.Specifier, %std.fmt.Specifier, %std.fmt.Specifier } { %"[]u8" zeroinitializer, i8 32, <{ i2, [6 x i8] }> <{ i2 -2, [6 x i8] undef }>, %std.fmt.Specifier { { i64, [8 x i8] } undef, i2 0 }, %std.fmt.Specifier { { i64, [8 x i8] } undef, i2 0 }, %std.fmt.Specifier { { i64, [8 x i8] } undef, i2 0 } }, align 8
@19 = internal unnamed_addr constant i64 0, align 8
@20 = internal unnamed_addr constant { %"?usize", %"?usize", i2, <{ i8, [6 x i8] }> } { %"?usize" { i64 undef, i1 false }, %"?usize" { i64 undef, i1 false }, i2 -2, <{ i8, [6 x i8] }> <{ i8 32, [6 x i8] undef }> }, align 8
@21 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([18 x i8], [18 x i8]* @16, i64 0, i64 9), i64 8 }, align 8
@22 = internal unnamed_addr constant [17 x i8] c"integer overflow\00", align 1
@23 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([17 x i8], [17 x i8]* @22, i64 0, i64 0), i64 16 }, align 8
@24 = internal unnamed_addr constant %"?usize" { i64 undef, i1 false }, align 8
@25 = internal unnamed_addr constant [26 x i8] c"attempt to use null value\00", align 1
@26 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([26 x i8], [26 x i8]* @25, i64 0, i64 0), i64 25 }, align 8
@27 = internal unnamed_addr constant %"[]u8" zeroinitializer, align 8
@28 = internal unnamed_addr constant %"?u8" { i8 undef, i1 false }, align 1
@29 = internal unnamed_addr constant %"?u8" { i8 undef, i1 false }, align 1
@30 = internal unnamed_addr constant %"?usize" { i64 undef, i1 false }, align 8
@31 = internal unnamed_addr constant %"[]u8" zeroinitializer, align 8
@32 = internal unnamed_addr constant [1 x i8] c"s", align 1
@33 = internal unnamed_addr constant { %"[]u8", i8, <{ i2, [6 x i8] }>, %std.fmt.Specifier, %std.fmt.Specifier, %std.fmt.Specifier } { %"[]u8" { i8* getelementptr inbounds ([1 x i8], [1 x i8]* @32, i64 0, i64 0), i64 1 }, i8 32, <{ i2, [6 x i8] }> <{ i2 -2, [6 x i8] undef }>, %std.fmt.Specifier { { i64, [8 x i8] } undef, i2 0 }, %std.fmt.Specifier { { i64, [8 x i8] } undef, i2 0 }, %std.fmt.Specifier { { i64, [8 x i8] } undef, i2 0 } }, align 8
@34 = internal unnamed_addr constant i64 0, align 8
@35 = internal unnamed_addr constant { %"?usize", %"?usize", i2, <{ i8, [6 x i8] }> } { %"?usize" { i64 undef, i1 false }, %"?usize" { i64 undef, i1 false }, i2 -2, <{ i8, [6 x i8] }> <{ i8 32, [6 x i8] undef }> }, align 8
@36 = internal unnamed_addr constant [5 x i8] c"{s}\0A\00", align 1
@37 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([5 x i8], [5 x i8]* @36, i64 0, i64 3), i64 1 }, align 8
@38 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([1 x i8], [1 x i8]* @32, i64 0, i64 0), i64 1 }, align 8
@39 = internal unnamed_addr constant [8 x i8] c"{ ... }\00", align 1
@40 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([8 x i8], [8 x i8]* @39, i64 0, i64 0), i64 7 }, align 8
@41 = internal unnamed_addr constant %"?u64" { i64 undef, i1 false }, align 8
@42 = internal unnamed_addr constant [36 x i8] c"Panicked during a panic. Aborting.\0A\00", align 1
@43 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([36 x i8], [36 x i8]* @42, i64 0, i64 0), i64 35 }, align 8
@44 = internal unnamed_addr constant [51 x i8] c"attempt to cast negative value to unsigned integer\00", align 1
@45 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([51 x i8], [51 x i8]* @44, i64 0, i64 0), i64 50 }, align 8
@46 = internal unnamed_addr constant [31 x i8] c"cast causes pointer to be null\00", align 1
@47 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([31 x i8], [31 x i8]* @46, i64 0, i64 0), i64 30 }, align 8
@48 = internal unnamed_addr constant %"?usize" { i64 undef, i1 false }, align 8
@49 = internal unnamed_addr constant [49 x i8] c"Unable to dump stack trace: debug info stripped\0A\00", align 1
@50 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([49 x i8], [49 x i8]* @49, i64 0, i64 0), i64 48 }, align 8
@51 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([49 x i8], [49 x i8]* @49, i64 0, i64 0), i64 48 }, align 8
@52 = internal unnamed_addr constant [43 x i8] c"shift amount is greater than the type size\00", align 1
@53 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([43 x i8], [43 x i8]* @52, i64 0, i64 0), i64 42 }, align 8
@54 = internal unnamed_addr constant [23 x i8] c"unexpected errno: {d}\0A\00", align 1
@55 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([23 x i8], [23 x i8]* @54, i64 0, i64 0), i64 18 }, align 8
@56 = internal unnamed_addr constant [1 x i8] c"d", align 1
@57 = internal unnamed_addr constant { %"[]u8", i8, <{ i2, [6 x i8] }>, %std.fmt.Specifier, %std.fmt.Specifier, %std.fmt.Specifier } { %"[]u8" { i8* getelementptr inbounds ([1 x i8], [1 x i8]* @56, i64 0, i64 0), i64 1 }, i8 32, <{ i2, [6 x i8] }> <{ i2 -2, [6 x i8] undef }>, %std.fmt.Specifier { { i64, [8 x i8] } undef, i2 0 }, %std.fmt.Specifier { { i64, [8 x i8] } undef, i2 0 }, %std.fmt.Specifier { { i64, [8 x i8] } undef, i2 0 } }, align 8
@58 = internal unnamed_addr constant i64 0, align 8
@59 = internal unnamed_addr constant { %"?usize", %"?usize", i2, <{ i8, [6 x i8] }> } { %"?usize" { i64 undef, i1 false }, %"?usize" { i64 undef, i1 false }, i2 -2, <{ i8, [6 x i8] }> <{ i8 32, [6 x i8] undef }> }, align 8
@60 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([23 x i8], [23 x i8]* @54, i64 0, i64 21), i64 1 }, align 8
@61 = internal unnamed_addr constant %"[]u8" { i8* getelementptr inbounds ([1 x i8], [1 x i8]* @56, i64 0, i64 0), i64 1 }, align 8

; Function Attrs: cold nobuiltin noreturn nounwind
define internal fastcc void @std.builtin.default_panic(%"[]u8"* nonnull readonly align 8 %0, %std.builtin.StackTrace* align 8 %1) unnamed_addr #0 {
Entry:
  %first_trace_addr = alloca i64, align 8
  %2 = alloca %"?usize", align 8
  %3 = alloca %"?usize", align 8
  %error_return_trace = alloca %std.builtin.StackTrace*, align 8
  store %std.builtin.StackTrace* %1, %std.builtin.StackTrace** %error_return_trace, align 8
  %4 = call i8* @llvm.returnaddress(i32 0)
  %5 = ptrtoint i8* %4 to i64
  store i64 %5, i64* %first_trace_addr, align 8
  %6 = load %std.builtin.StackTrace*, %std.builtin.StackTrace** %error_return_trace, align 8
  %7 = load i64, i64* %first_trace_addr, align 8
  %8 = getelementptr inbounds %"?usize", %"?usize"* %2, i32 0, i32 0
  store i64 %7, i64* %8, align 8
  %9 = getelementptr inbounds %"?usize", %"?usize"* %2, i32 0, i32 1
  store i1 true, i1* %9, align 1
  %10 = getelementptr inbounds %"?usize", %"?usize"* %3, i32 0, i32 0
  store i64 %7, i64* %10, align 8
  %11 = getelementptr inbounds %"?usize", %"?usize"* %3, i32 0, i32 1
  store i1 true, i1* %11, align 1
  call fastcc void @std.debug.panicImpl(%std.builtin.StackTrace* %6, %"?usize"* %3, %"[]u8"* %0)
  unreachable
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @"std.target.Feature.feature_set_fns(std.target.x86.Feature).featureSet"(%std.target.Set* nonnull sret(%std.target.Set) %0, %"[]std.target.x86.Feature"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %x = alloca %std.target.Set, align 8
  %i = alloca i64, align 8
  %feature = alloca i8, align 1
  call fastcc void @std.target.Set.empty_workaround(%std.target.Set* sret(%std.target.Set) %x)
  store i64 0, i64* %i, align 8
  %2 = getelementptr inbounds %"[]std.target.x86.Feature", %"[]std.target.x86.Feature"* %1, i32 0, i32 1
  %3 = load i64, i64* %2, align 8
  br label %ForCond

ForCond:                                          ; preds = %ForBody, %Entry
  %4 = load i64, i64* %i, align 8
  %5 = icmp ult i64 %4, %3
  br i1 %5, label %ForBody, label %ForEnd

ForBody:                                          ; preds = %ForCond
  %6 = getelementptr inbounds %"[]std.target.x86.Feature", %"[]std.target.x86.Feature"* %1, i32 0, i32 0
  %7 = load i8*, i8** %6, align 8
  %8 = getelementptr inbounds i8, i8* %7, i64 %4
  %9 = load i8, i8* %8, align 1
  store i8 %9, i8* %feature, align 1
  %10 = load i8, i8* %feature, align 1
  %11 = zext i8 %10 to i9
  call fastcc void @std.target.Set.addFeature(%std.target.Set* %x, i9 %11)
  %12 = add nuw i64 %4, 1
  store i64 %12, i64* %i, align 8
  br label %ForCond

ForEnd:                                           ; preds = %ForCond
  %13 = bitcast %std.target.Set* %x to i8*
  %14 = bitcast %std.target.Set* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %14, i8* align 8 %13, i64 40, i1 false)
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.debug.assert(i1 %0) unnamed_addr #1 {
Entry:
  %ok = alloca i1, align 1
  store i1 %0, i1* %ok, align 1
  %1 = load i1, i1* %ok, align 1
  %2 = icmp eq i1 %1, false
  br i1 %2, label %Then, label %Else

Then:                                             ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

Else:                                             ; preds = %Entry
  br label %EndIf

EndIf:                                            ; preds = %Else
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.os.linux.getErrno(i64 %0) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %signed_r = alloca i64, align 8
  %int = alloca i64, align 8
  %r = alloca i64, align 8
  store i64 %0, i64* %r, align 8
  %1 = load i64, i64* %r, align 8
  store i64 %1, i64* %signed_r, align 8
  %2 = load i64, i64* %signed_r, align 8
  %3 = sext i64 %2 to i65
  %4 = icmp sgt i65 %3, -4096
  br i1 %4, label %BoolAndTrue, label %BoolAndFalse

BoolAndTrue:                                      ; preds = %Entry
  %5 = load i64, i64* %signed_r, align 8
  %6 = icmp slt i64 %5, 0
  br label %BoolAndFalse

BoolAndFalse:                                     ; preds = %BoolAndTrue, %Entry
  %7 = phi i1 [ %4, %Entry ], [ %6, %BoolAndTrue ]
  br i1 %7, label %Then, label %Else

Then:                                             ; preds = %BoolAndFalse
  %8 = load i64, i64* %signed_r, align 8
  %9 = call { i64, i1 } @llvm.ssub.with.overflow.i64(i64 0, i64 %8)
  %10 = extractvalue { i64, i1 } %9, 0
  %11 = extractvalue { i64, i1 } %9, 1
  br i1 %11, label %OverflowFail, label %OverflowOk

Else:                                             ; preds = %BoolAndFalse
  store i64 0, i64* %int, align 8
  br label %EndIf

EndIf:                                            ; preds = %Else, %OverflowOk
  %12 = load i64, i64* %int, align 8
  %13 = icmp sge i64 %12, 0
  br i1 %13, label %SignCastOk, label %SignCastFail

OverflowFail:                                     ; preds = %Then
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %Then
  store i64 %10, i64* %int, align 8
  br label %EndIf

SignCastOk:                                       ; preds = %EndIf
  %14 = trunc i64 %12 to i16
  %15 = zext i16 %14 to i64
  %16 = icmp eq i64 %12, %15
  br i1 %16, label %CastShortenOk, label %CastShortenFail

SignCastFail:                                     ; preds = %EndIf
  call fastcc void @std.builtin.default_panic(%"[]u8"* @45, %std.builtin.StackTrace* null)
  unreachable

CastShortenOk:                                    ; preds = %SignCastOk
  store i16 %14, i16* %result, align 2
  %17 = load i16, i16* %result, align 2
  ret i16 %17

CastShortenFail:                                  ; preds = %SignCastOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @15, %std.builtin.StackTrace* null)
  unreachable
}

; Function Attrs: naked nobuiltin nounwind
define internal void @std.os.linux.x86_64.restore_rt() unnamed_addr #2 {
Entry:
  call void asm sideeffect "syscall", "{rax},~{rcx},~{r11},~{memory},~{dirflag},~{fpsr},~{flags}"(i64 15)
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.os.linux.x86_64.syscall4(i64 %0, i64 %1, i64 %2, i64 %3, i64 %4) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %number = alloca i64, align 8
  %arg1 = alloca i64, align 8
  %arg2 = alloca i64, align 8
  %arg3 = alloca i64, align 8
  %arg4 = alloca i64, align 8
  store i64 %0, i64* %number, align 8
  store i64 %1, i64* %arg1, align 8
  store i64 %2, i64* %arg2, align 8
  store i64 %3, i64* %arg3, align 8
  store i64 %4, i64* %arg4, align 8
  %5 = load i64, i64* %number, align 8
  %6 = load i64, i64* %arg1, align 8
  %7 = load i64, i64* %arg2, align 8
  %8 = load i64, i64* %arg3, align 8
  %9 = load i64, i64* %arg4, align 8
  %10 = call i64 asm sideeffect "syscall", "={rax},{rax},{rdi},{rsi},{rdx},{r10},~{rcx},~{r11},~{memory},~{dirflag},~{fpsr},~{flags}"(i64 %5, i64 %6, i64 %7, i64 %8, i64 %9)
  store i64 %10, i64* %result, align 8
  %11 = load i64, i64* %result, align 8
  ret i64 %11
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.os.linux.x86_64.syscall3(i64 %0, i64 %1, i64 %2, i64 %3) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %number = alloca i64, align 8
  %arg1 = alloca i64, align 8
  %arg2 = alloca i64, align 8
  %arg3 = alloca i64, align 8
  store i64 %0, i64* %number, align 8
  store i64 %1, i64* %arg1, align 8
  store i64 %2, i64* %arg2, align 8
  store i64 %3, i64* %arg3, align 8
  %4 = load i64, i64* %number, align 8
  %5 = load i64, i64* %arg1, align 8
  %6 = load i64, i64* %arg2, align 8
  %7 = load i64, i64* %arg3, align 8
  %8 = call i64 asm sideeffect "syscall", "={rax},{rax},{rdi},{rsi},{rdx},~{rcx},~{r11},~{memory},~{dirflag},~{fpsr},~{flags}"(i64 %4, i64 %5, i64 %6, i64 %7)
  store i64 %8, i64* %result, align 8
  %9 = load i64, i64* %result, align 8
  ret i64 %9
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.os.linux.x86_64.syscall0(i64 %0) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %number = alloca i64, align 8
  store i64 %0, i64* %number, align 8
  %1 = load i64, i64* %number, align 8
  %2 = call i64 asm sideeffect "syscall", "={rax},{rax},~{rcx},~{r11},~{memory},~{dirflag},~{fpsr},~{flags}"(i64 %1)
  store i64 %2, i64* %result, align 8
  %3 = load i64, i64* %result, align 8
  ret i64 %3
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.os.linux.x86_64.syscall2(i64 %0, i64 %1, i64 %2) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %number = alloca i64, align 8
  %arg1 = alloca i64, align 8
  %arg2 = alloca i64, align 8
  store i64 %0, i64* %number, align 8
  store i64 %1, i64* %arg1, align 8
  store i64 %2, i64* %arg2, align 8
  %3 = load i64, i64* %number, align 8
  %4 = load i64, i64* %arg1, align 8
  %5 = load i64, i64* %arg2, align 8
  %6 = call i64 asm sideeffect "syscall", "={rax},{rax},{rdi},{rsi},~{rcx},~{r11},~{memory},~{dirflag},~{fpsr},~{flags}"(i64 %3, i64 %4, i64 %5)
  store i64 %6, i64* %result, align 8
  %7 = load i64, i64* %result, align 8
  ret i64 %7
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.os.linux.x86_64.syscall1(i64 %0, i64 %1) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %number = alloca i64, align 8
  %arg1 = alloca i64, align 8
  store i64 %0, i64* %number, align 8
  store i64 %1, i64* %arg1, align 8
  %2 = load i64, i64* %number, align 8
  %3 = load i64, i64* %arg1, align 8
  %4 = call i64 asm sideeffect "syscall", "={rax},{rax},{rdi},~{rcx},~{r11},~{memory},~{dirflag},~{fpsr},~{flags}"(i64 %2, i64 %3)
  store i64 %4, i64* %result, align 8
  %5 = load i64, i64* %result, align 8
  ret i64 %5
}

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i8* @llvm.returnaddress(i32 immarg) #3

; Function Attrs: cold nobuiltin noreturn nounwind
define internal fastcc void @std.debug.panicImpl(%std.builtin.StackTrace* align 8 %0, %"?usize"* nonnull readonly align 8 %1, %"[]u8"* nonnull readonly align 8 %2) unnamed_addr #0 {
Entry:
  %3 = alloca %std.fs.file.File, align 4
  %stderr = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %current_thread_id = alloca i64, align 8
  %4 = alloca %"std.debug.struct:296:56", align 8
  %5 = alloca i16, align 2
  %6 = alloca %"std.debug.struct:298:40", align 8
  %7 = alloca i16, align 2
  %t = alloca %std.builtin.StackTrace*, align 8
  %event = alloca %std.Thread.StaticResetEvent, align 4
  %8 = alloca %std.fs.file.File, align 4
  %stderr8 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %9 = alloca i16, align 2
  %trace = alloca %std.builtin.StackTrace*, align 8
  store %std.builtin.StackTrace* %0, %std.builtin.StackTrace** %trace, align 8
  call fastcc void @std.debug.resetSegfaultHandler()
  %10 = load i64, i64* @panic_stage, align 8
  switch i64 %10, label %SwitchElse [
    i64 0, label %SwitchProng
    i64 1, label %SwitchProng4
  ]

SwitchProng:                                      ; preds = %Entry
  store i64 1, i64* @panic_stage, align 8
  %11 = atomicrmw add i8* @panicking, i8 1 seq_cst, align 1
  call fastcc void @std.Thread.Mutex.lock(%std.Thread.Mutex* @panic_mutex)
  call fastcc void @std.io.getStdErr(%std.fs.file.File* sret(%std.fs.file.File) %3)
  call fastcc void @std.fs.file.File.writer(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* sret(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)") %stderr, %std.fs.file.File* %3)
  %12 = call fastcc i64 @std.Thread.getCurrentId()
  store i64 %12, i64* %current_thread_id, align 8
  %13 = load i64, i64* %current_thread_id, align 8
  %14 = getelementptr inbounds %"std.debug.struct:296:56", %"std.debug.struct:296:56"* %4, i32 0, i32 0
  store i64 %13, i64* %14, align 8
  %15 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).print"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %stderr, %"std.debug.struct:296:56"* %4)
  store i16 %15, i16* %5, align 2
  %16 = icmp ne i16 %15, 0
  br i1 %16, label %UnwrapErrError, label %UnwrapErrOk

UnwrapErrError:                                   ; preds = %SwitchProng
  call fastcc void @std.os.abort()
  unreachable

UnwrapErrOk:                                      ; preds = %SwitchProng
  br label %UnwrapErrEnd

UnwrapErrEnd:                                     ; preds = %UnwrapErrOk
  %17 = getelementptr inbounds %"std.debug.struct:298:40", %"std.debug.struct:298:40"* %6, i32 0, i32 0
  %18 = bitcast %"[]u8"* %2 to i8*
  %19 = bitcast %"[]u8"* %17 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %19, i8* align 8 %18, i64 16, i1 false)
  %20 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).print.10"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %stderr, %"std.debug.struct:298:40"* %6)
  store i16 %20, i16* %7, align 2
  %21 = icmp ne i16 %20, 0
  br i1 %21, label %UnwrapErrError1, label %UnwrapErrOk2

UnwrapErrError1:                                  ; preds = %UnwrapErrEnd
  call fastcc void @std.os.abort()
  unreachable

UnwrapErrOk2:                                     ; preds = %UnwrapErrEnd
  br label %UnwrapErrEnd3

UnwrapErrEnd3:                                    ; preds = %UnwrapErrOk2
  %22 = load %std.builtin.StackTrace*, %std.builtin.StackTrace** %trace, align 8
  %23 = icmp ne %std.builtin.StackTrace* %22, null
  br i1 %23, label %OptionalThen, label %OptionalElse

OptionalThen:                                     ; preds = %UnwrapErrEnd3
  %24 = load %std.builtin.StackTrace*, %std.builtin.StackTrace** %trace, align 8
  store %std.builtin.StackTrace* %24, %std.builtin.StackTrace** %t, align 8
  %25 = load %std.builtin.StackTrace*, %std.builtin.StackTrace** %t, align 8
  call fastcc void @std.debug.dumpStackTrace(%std.builtin.StackTrace* %25)
  br label %OptionalEndIf

OptionalElse:                                     ; preds = %UnwrapErrEnd3
  br label %OptionalEndIf

OptionalEndIf:                                    ; preds = %OptionalElse, %OptionalThen
  call fastcc void @std.debug.dumpCurrentStackTrace(%"?usize"* %1)
  call fastcc void @std.Thread.Mutex.unlock(%std.Thread.Mutex* @panic_mutex)
  %26 = atomicrmw sub i8* @panicking, i8 1 seq_cst, align 1
  %27 = icmp ne i8 %26, 1
  br i1 %27, label %Then, label %Else

Then:                                             ; preds = %OptionalEndIf
  %28 = bitcast %std.Thread.StaticResetEvent* %event to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %28, i8* align 4 bitcast (%std.Thread.StaticResetEvent* @5 to i8*), i64 4, i1 false)
  call fastcc void @std.Thread.StaticResetEvent.wait(%std.Thread.StaticResetEvent* %event)
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

Else:                                             ; preds = %OptionalEndIf
  br label %EndIf

SwitchProng4:                                     ; preds = %Entry
  store i64 2, i64* @panic_stage, align 8
  call fastcc void @std.io.getStdErr(%std.fs.file.File* sret(%std.fs.file.File) %8)
  call fastcc void @std.fs.file.File.writer(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* sret(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)") %stderr8, %std.fs.file.File* %8)
  %29 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).print.11"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %stderr8)
  store i16 %29, i16* %9, align 2
  %30 = icmp ne i16 %29, 0
  br i1 %30, label %UnwrapErrError5, label %UnwrapErrOk6

UnwrapErrError5:                                  ; preds = %SwitchProng4
  call fastcc void @std.os.abort()
  unreachable

UnwrapErrOk6:                                     ; preds = %SwitchProng4
  br label %UnwrapErrEnd7

SwitchElse:                                       ; preds = %Entry
  br label %SwitchEnd

EndIf:                                            ; preds = %Else
  br label %SwitchEnd

UnwrapErrEnd7:                                    ; preds = %UnwrapErrOk6
  br label %SwitchEnd

SwitchEnd:                                        ; preds = %UnwrapErrEnd7, %EndIf, %SwitchElse
  call fastcc void @std.os.abort()
  unreachable
}

; Function Attrs: nobuiltin nounwind
define void @foo(%Big* nonnull readonly %0, i64 %1) #1 {
Entry:
  %xs = alloca %"[]Big", align 8
  %i = alloca i64, align 8
  %x = alloca %Big, align 1
  %xs_ptr = alloca %Big*, align 8
  %xs_len = alloca i64, align 8
  store %Big* %0, %Big** %xs_ptr, align 8
  store i64 %1, i64* %xs_len, align 8
  %2 = load i64, i64* %xs_len, align 8
  %3 = load %Big*, %Big** %xs_ptr, align 8
  %4 = icmp ule i64 0, %2
  br i1 %4, label %BoundsCheckOk, label %BoundsCheckFail

ForCond:                                          ; preds = %ForBody, %BoundsCheckOk
  %5 = load i64, i64* %i, align 8
  %6 = icmp ult i64 %5, %19
  br i1 %6, label %ForBody, label %ForEnd

ForBody:                                          ; preds = %ForCond
  %7 = getelementptr inbounds %"[]Big", %"[]Big"* %xs, i32 0, i32 0
  %8 = load %Big*, %Big** %7, align 8
  %9 = getelementptr inbounds %Big, %Big* %8, i64 %5
  %10 = bitcast %Big* %9 to i8*
  %11 = bitcast %Big* %x to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %11, i8* align 1 %10, i64 4224, i1 false)
  %12 = getelementptr inbounds %Big, %Big* %x, i32 0, i32 1
  call fastcc void @bar(%Small* %12)
  %13 = add nuw i64 %5, 1
  store i64 %13, i64* %i, align 8
  br label %ForCond

ForEnd:                                           ; preds = %ForCond
  ret void

BoundsCheckFail:                                  ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %Entry
  %14 = getelementptr inbounds %Big, %Big* %3, i64 0
  %15 = sub nuw i64 %2, 0
  %16 = getelementptr inbounds %"[]Big", %"[]Big"* %xs, i32 0, i32 0
  store %Big* %14, %Big** %16, align 8
  %17 = getelementptr inbounds %"[]Big", %"[]Big"* %xs, i32 0, i32 1
  store i64 %15, i64* %17, align 8
  store i64 0, i64* %i, align 8
  %18 = getelementptr inbounds %"[]Big", %"[]Big"* %xs, i32 0, i32 1
  %19 = load i64, i64* %18, align 8
  br label %ForCond
}

; Function Attrs: argmemonly nofree nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #4

; Function Attrs: nobuiltin noinline nounwind
define internal fastcc void @bar(%Small* nonnull readonly align 1 %0) unnamed_addr #5 {
Entry:
  %x = alloca %Small*, align 8
  store %Small* %0, %Small** %x, align 8
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.debug.resetSegfaultHandler() unnamed_addr #1 {
Entry:
  %act = alloca %std.os.linux.Sigaction, align 8
  %0 = bitcast %std.os.linux.Sigaction* %act to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %0, i8* align 8 bitcast ({ void (i32, %std.os.linux.siginfo_t*, i8*)*, [32 x i32], <{ i32, [4 x i8] }>, void ()* }* @8 to i8*), i64 152, i1 false)
  call fastcc void @std.os.sigaction(i6 11, %std.os.linux.Sigaction* %act, %std.os.linux.Sigaction* null)
  call fastcc void @std.os.sigaction(i6 4, %std.os.linux.Sigaction* %act, %std.os.linux.Sigaction* null)
  call fastcc void @std.os.sigaction(i6 7, %std.os.linux.Sigaction* %act, %std.os.linux.Sigaction* null)
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.Thread.Mutex.lock(%std.Thread.Mutex* nonnull align 4 %0) unnamed_addr #1 {
Entry:
  %m = alloca %std.Thread.Mutex*, align 8
  store %std.Thread.Mutex* %0, %std.Thread.Mutex** %m, align 8
  %1 = load %std.Thread.Mutex*, %std.Thread.Mutex** %m, align 8
  %2 = getelementptr inbounds %std.Thread.Mutex, %std.Thread.Mutex* %1, i32 0, i32 0
  call fastcc void @std.Thread.Mutex.AtomicMutex.lock(%std.Thread.Mutex.AtomicMutex* %2)
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.io.getStdErr(%std.fs.file.File* nonnull sret(%std.fs.file.File) %0) unnamed_addr #1 {
Entry:
  %1 = getelementptr inbounds %std.fs.file.File, %std.fs.file.File* %0, i32 0, i32 0
  %2 = call fastcc i32 @std.io.getStdErrHandle()
  store i32 %2, i32* %1, align 4
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.fs.file.File.writer(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull sret(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)") %0, %std.fs.file.File* nonnull readonly align 4 %1) unnamed_addr #1 {
Entry:
  %2 = getelementptr inbounds %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, i32 0, i32 0
  %3 = bitcast %std.fs.file.File* %1 to i8*
  %4 = bitcast %std.fs.file.File* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %4, i8* align 4 %3, i64 4, i1 false)
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.Thread.getCurrentId() unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %0 = call fastcc i64 @std.Thread.LinuxThreadImpl.getCurrentId()
  store i64 %0, i64* %result, align 8
  %1 = load i64, i64* %result, align 8
  ret i64 %1
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).print"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0, %"std.debug.struct:296:56"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %2 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %3 = alloca %"std.debug.struct:296:56", align 8
  %4 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0 to i8*
  %5 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %5, i8* align 4 %4, i64 4, i1 false)
  %6 = bitcast %"std.debug.struct:296:56"* %1 to i8*
  %7 = bitcast %"std.debug.struct:296:56"* %3 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %7, i8* align 8 %6, i64 8, i1 false)
  %8 = call fastcc i16 @std.fmt.format(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"std.debug.struct:296:56"* %1)
  store i16 %8, i16* %result, align 2
  %9 = load i16, i16* %result, align 2
  ret i16 %9
}

; Function Attrs: cold nobuiltin noreturn nounwind
define internal fastcc void @std.os.abort() unnamed_addr #0 {
Entry:
  %0 = alloca i16, align 2
  %1 = alloca i16, align 2
  %2 = call fastcc i16 @std.os.raise(i8 6)
  store i16 %2, i16* %0, align 2
  %3 = icmp ne i16 %2, 0
  br i1 %3, label %UnwrapErrError, label %UnwrapErrOk

UnwrapErrError:                                   ; preds = %Entry
  br label %UnwrapErrEnd

UnwrapErrOk:                                      ; preds = %Entry
  br label %UnwrapErrEnd

UnwrapErrEnd:                                     ; preds = %UnwrapErrOk, %UnwrapErrError
  %4 = call fastcc i16 @std.os.raise(i8 9)
  store i16 %4, i16* %1, align 2
  %5 = icmp ne i16 %4, 0
  br i1 %5, label %UnwrapErrError1, label %UnwrapErrOk2

UnwrapErrError1:                                  ; preds = %UnwrapErrEnd
  br label %UnwrapErrEnd3

UnwrapErrOk2:                                     ; preds = %UnwrapErrEnd
  br label %UnwrapErrEnd3

UnwrapErrEnd3:                                    ; preds = %UnwrapErrOk2, %UnwrapErrError1
  call fastcc void @std.os.exit(i8 127)
  unreachable
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).print.10"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0, %"std.debug.struct:298:40"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %2 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %3 = alloca %"std.debug.struct:298:40", align 8
  %4 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0 to i8*
  %5 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %5, i8* align 4 %4, i64 4, i1 false)
  %6 = bitcast %"std.debug.struct:298:40"* %1 to i8*
  %7 = bitcast %"std.debug.struct:298:40"* %3 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %7, i8* align 8 %6, i64 16, i1 false)
  %8 = call fastcc i16 @std.fmt.format.12(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"std.debug.struct:298:40"* %1)
  store i16 %8, i16* %result, align 2
  %9 = load i16, i16* %result, align 2
  ret i16 %9
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.debug.dumpStackTrace(%std.builtin.StackTrace* nonnull readonly align 8 %0) unnamed_addr #1 {
Entry:
  %1 = alloca %std.fs.file.File, align 4
  %stderr = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %2 = alloca i16, align 2
  call fastcc void @std.io.getStdErr(%std.fs.file.File* sret(%std.fs.file.File) %1)
  call fastcc void @std.fs.file.File.writer(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* sret(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)") %stderr, %std.fs.file.File* %1)
  %3 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).print.13"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %stderr)
  store i16 %3, i16* %2, align 2
  %4 = icmp ne i16 %3, 0
  br i1 %4, label %UnwrapErrError, label %UnwrapErrOk

UnwrapErrError:                                   ; preds = %Entry
  ret void

UnwrapErrOk:                                      ; preds = %Entry
  br label %UnwrapErrEnd

UnwrapErrEnd:                                     ; preds = %UnwrapErrOk
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.debug.dumpCurrentStackTrace(%"?usize"* nonnull readonly align 8 %0) unnamed_addr #1 {
Entry:
  %1 = alloca %std.fs.file.File, align 4
  %stderr = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %2 = alloca i16, align 2
  call fastcc void @std.io.getStdErr(%std.fs.file.File* sret(%std.fs.file.File) %1)
  call fastcc void @std.fs.file.File.writer(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* sret(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)") %stderr, %std.fs.file.File* %1)
  %3 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).print.14"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %stderr)
  store i16 %3, i16* %2, align 2
  %4 = icmp ne i16 %3, 0
  br i1 %4, label %UnwrapErrError, label %UnwrapErrOk

UnwrapErrError:                                   ; preds = %Entry
  ret void

UnwrapErrOk:                                      ; preds = %Entry
  br label %UnwrapErrEnd

UnwrapErrEnd:                                     ; preds = %UnwrapErrOk
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.Thread.Mutex.unlock(%std.Thread.Mutex* nonnull align 4 %0) unnamed_addr #1 {
Entry:
  %m = alloca %std.Thread.Mutex*, align 8
  store %std.Thread.Mutex* %0, %std.Thread.Mutex** %m, align 8
  %1 = load %std.Thread.Mutex*, %std.Thread.Mutex** %m, align 8
  %2 = getelementptr inbounds %std.Thread.Mutex, %std.Thread.Mutex* %1, i32 0, i32 0
  call fastcc void @std.Thread.Mutex.AtomicMutex.unlock(%std.Thread.Mutex.AtomicMutex* %2)
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.Thread.StaticResetEvent.wait(%std.Thread.StaticResetEvent* nonnull align 4 %0) unnamed_addr #1 {
Entry:
  %ev = alloca %std.Thread.StaticResetEvent*, align 8
  store %std.Thread.StaticResetEvent* %0, %std.Thread.StaticResetEvent** %ev, align 8
  %1 = load %std.Thread.StaticResetEvent*, %std.Thread.StaticResetEvent** %ev, align 8
  %2 = getelementptr inbounds %std.Thread.StaticResetEvent, %std.Thread.StaticResetEvent* %1, i32 0, i32 0
  call fastcc void @std.Thread.StaticResetEvent.AtomicEvent.wait(%std.Thread.StaticResetEvent.AtomicEvent* %2)
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).print.11"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %1 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %2 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0 to i8*
  %3 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %1 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %3, i8* align 4 %2, i64 4, i1 false)
  %4 = call fastcc i16 @std.fmt.format.15(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0)
  store i16 %4, i16* %result, align 2
  %5 = load i16, i16* %result, align 2
  ret i16 %5
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.os.sigaction(i6 %0, %std.os.linux.Sigaction* align 8 %1, %std.os.linux.Sigaction* align 8 %2) unnamed_addr #1 {
Entry:
  %3 = alloca i16, align 2
  %sig = alloca i6, align 1
  %act = alloca %std.os.linux.Sigaction*, align 8
  %oact = alloca %std.os.linux.Sigaction*, align 8
  store i6 %0, i6* %sig, align 1
  store %std.os.linux.Sigaction* %1, %std.os.linux.Sigaction** %act, align 8
  store %std.os.linux.Sigaction* %2, %std.os.linux.Sigaction** %oact, align 8
  %4 = load i6, i6* %sig, align 1
  %5 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %act, align 8
  %6 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %oact, align 8
  %7 = call fastcc i64 @std.os.linux.sigaction(i6 %4, %std.os.linux.Sigaction* %5, %std.os.linux.Sigaction* %6)
  %8 = call fastcc i16 @std.os.linux.getErrno(i64 %7)
  store i16 %8, i16* %3, align 2
  switch i16 %8, label %SwitchElse [
    i16 0, label %SwitchProng
    i16 14, label %SwitchProng1
    i16 22, label %SwitchProng2
  ]

SwitchElse:                                       ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng:                                      ; preds = %Entry
  ret void

SwitchProng1:                                     ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng2:                                     ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.target.Set.empty_workaround(%std.target.Set* nonnull sret(%std.target.Set) %0) unnamed_addr #1 {
Entry:
  %1 = getelementptr inbounds %std.target.Set, %std.target.Set* %0, i32 0, i32 0
  %2 = bitcast [5 x i64]* %1 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %2, i8* align 8 bitcast ([5 x i64]* @9 to i8*), i64 40, i1 false)
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.target.Set.addFeature(%std.target.Set* nonnull align 8 %0, i9 %1) unnamed_addr #1 {
Entry:
  %usize_index = alloca i9, align 2
  %bit_index = alloca i6, align 1
  %set = alloca %std.target.Set*, align 8
  %arch_feature_index = alloca i9, align 2
  store %std.target.Set* %0, %std.target.Set** %set, align 8
  store i9 %1, i9* %arch_feature_index, align 2
  %2 = load i9, i9* %arch_feature_index, align 2
  br i1 false, label %DivZeroFail, label %DivZeroOk

DivZeroFail:                                      ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @11, %std.builtin.StackTrace* null)
  unreachable

DivZeroOk:                                        ; preds = %Entry
  %3 = udiv i9 %2, 64
  store i9 %3, i9* %usize_index, align 2
  %4 = load i9, i9* %arch_feature_index, align 2
  br i1 false, label %RemZeroFail, label %RemZeroOk

RemZeroOk:                                        ; preds = %DivZeroOk
  %5 = urem i9 %4, 64
  %6 = trunc i9 %5 to i6
  %7 = zext i6 %6 to i9
  %8 = icmp eq i9 %5, %7
  br i1 %8, label %CastShortenOk, label %CastShortenFail

RemZeroFail:                                      ; preds = %DivZeroOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @13, %std.builtin.StackTrace* null)
  unreachable

CastShortenOk:                                    ; preds = %RemZeroOk
  store i6 %6, i6* %bit_index, align 1
  %9 = load %std.target.Set*, %std.target.Set** %set, align 8
  %10 = getelementptr inbounds %std.target.Set, %std.target.Set* %9, i32 0, i32 0
  %11 = load i9, i9* %usize_index, align 2
  %12 = zext i9 %11 to i64
  %13 = icmp ult i64 %12, 5
  br i1 %13, label %BoundsCheckOk, label %BoundsCheckFail

CastShortenFail:                                  ; preds = %RemZeroOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @15, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckFail:                                  ; preds = %CastShortenOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %CastShortenOk
  %14 = getelementptr inbounds [5 x i64], [5 x i64]* %10, i64 0, i64 %12
  %15 = load i64, i64* %14, align 8
  %16 = load i6, i6* %bit_index, align 1
  %17 = zext i6 %16 to i64
  %18 = shl i64 1, %17
  %19 = or i64 %15, %18
  store i64 %19, i64* %14, align 8
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.Thread.Mutex.AtomicMutex.lock(%std.Thread.Mutex.AtomicMutex* nonnull align 4 %0) unnamed_addr #1 {
Entry:
  %1 = alloca i32, align 4
  %s = alloca i32, align 4
  %m = alloca %std.Thread.Mutex.AtomicMutex*, align 8
  store %std.Thread.Mutex.AtomicMutex* %0, %std.Thread.Mutex.AtomicMutex** %m, align 8
  %2 = load %std.Thread.Mutex.AtomicMutex*, %std.Thread.Mutex.AtomicMutex** %m, align 8
  %3 = getelementptr inbounds %std.Thread.Mutex.AtomicMutex, %std.Thread.Mutex.AtomicMutex* %2, i32 0, i32 0
  %4 = atomicrmw xchg i32* %3, i32 1 acquire, align 4
  store i32 %4, i32* %1, align 4
  switch i32 %4, label %SwitchElse [
    i32 0, label %SwitchProng
  ]

SwitchElse:                                       ; preds = %Entry
  store i32 %4, i32* %s, align 4
  %5 = load %std.Thread.Mutex.AtomicMutex*, %std.Thread.Mutex.AtomicMutex** %m, align 8
  %6 = load i32, i32* %s, align 4
  call fastcc void @std.Thread.Mutex.AtomicMutex.lockSlow(%std.Thread.Mutex.AtomicMutex* %5, i32 %6)
  br label %SwitchEnd

SwitchProng:                                      ; preds = %Entry
  br label %SwitchEnd

SwitchEnd:                                        ; preds = %SwitchProng, %SwitchElse
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i32 @std.io.getStdErrHandle() unnamed_addr #1 {
Entry:
  %result = alloca i32, align 4
  store i32 2, i32* %result, align 4
  %0 = load i32, i32* %result, align 4
  ret i32 %0
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.fs.file.File.write({ i64, i16 }* nonnull sret({ i64, i16 }) %0, %std.fs.file.File* nonnull readonly align 4 %1, %"[]u8"* nonnull readonly align 8 %2) unnamed_addr #1 {
Entry:
  %3 = getelementptr inbounds %std.fs.file.File, %std.fs.file.File* %1, i32 0, i32 0
  %4 = load i32, i32* %3, align 4
  call fastcc void @std.os.write({ i64, i16 }* sret({ i64, i16 }) %0, i32 %4, %"[]u8"* %2)
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.os.write({ i64, i16 }* nonnull sret({ i64, i16 }) %0, i32 %1, %"[]u8"* nonnull readonly align 8 %2) unnamed_addr #1 {
Entry:
  %adjusted_len = alloca i64, align 8
  %rc = alloca i64, align 8
  %3 = alloca i16, align 2
  %err = alloca i16, align 2
  %fd = alloca i32, align 4
  store i32 %1, i32* %fd, align 4
  %4 = getelementptr inbounds %"[]u8", %"[]u8"* %2, i32 0, i32 1
  %5 = load i64, i64* %4, align 8
  %6 = call i64 @llvm.umin.i64(i64 2147479552, i64 %5)
  store i64 %6, i64* %adjusted_len, align 8
  br label %WhileCond

WhileCond:                                        ; preds = %SwitchProng1, %Entry
  br label %WhileBody

WhileBody:                                        ; preds = %WhileCond
  %7 = load i32, i32* %fd, align 4
  %8 = getelementptr inbounds %"[]u8", %"[]u8"* %2, i32 0, i32 0
  %9 = load i8*, i8** %8, align 8
  %10 = load i64, i64* %adjusted_len, align 8
  %11 = call fastcc i64 @std.os.linux.write(i32 %7, i8* %9, i64 %10)
  store i64 %11, i64* %rc, align 8
  %12 = load i64, i64* %rc, align 8
  %13 = call fastcc i16 @std.os.linux.getErrno(i64 %12)
  store i16 %13, i16* %3, align 2
  switch i16 %13, label %SwitchElse [
    i16 0, label %SwitchProng
    i16 4, label %SwitchProng1
    i16 22, label %SwitchProng2
    i16 14, label %SwitchProng3
    i16 11, label %SwitchProng4
    i16 9, label %SwitchProng5
    i16 89, label %SwitchProng6
    i16 122, label %SwitchProng7
    i16 27, label %SwitchProng8
    i16 5, label %SwitchProng9
    i16 28, label %SwitchProng10
    i16 1, label %SwitchProng11
    i16 32, label %SwitchProng12
    i16 104, label %SwitchProng13
  ]

SwitchElse:                                       ; preds = %WhileBody
  store i16 %13, i16* %err, align 2
  %14 = load i16, i16* %err, align 2
  %15 = call fastcc i16 @std.os.unexpectedErrno(i16 %14)
  %16 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 %15, i16* %16, align 2
  ret void

SwitchProng:                                      ; preds = %WhileBody
  %17 = load i64, i64* %rc, align 8
  %18 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %18, align 2
  %19 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 0
  store i64 %17, i64* %19, align 8
  ret void

SwitchProng1:                                     ; preds = %WhileBody
  br label %WhileCond

SwitchProng2:                                     ; preds = %WhileBody
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng3:                                     ; preds = %WhileBody
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng4:                                     ; preds = %WhileBody
  %20 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 10, i16* %20, align 2
  ret void

SwitchProng5:                                     ; preds = %WhileBody
  %21 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 9, i16* %21, align 2
  ret void

SwitchProng6:                                     ; preds = %WhileBody
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng7:                                     ; preds = %WhileBody
  %22 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 1, i16* %22, align 2
  ret void

SwitchProng8:                                     ; preds = %WhileBody
  %23 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 2, i16* %23, align 2
  ret void

SwitchProng9:                                     ; preds = %WhileBody
  %24 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 3, i16* %24, align 2
  ret void

SwitchProng10:                                    ; preds = %WhileBody
  %25 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 4, i16* %25, align 2
  ret void

SwitchProng11:                                    ; preds = %WhileBody
  %26 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 5, i16* %26, align 2
  ret void

SwitchProng12:                                    ; preds = %WhileBody
  %27 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 6, i16* %27, align 2
  ret void

SwitchProng13:                                    ; preds = %WhileBody
  %28 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 11, i16* %28, align 2
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.Thread.LinuxThreadImpl.getCurrentId() unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %tid = alloca i32, align 4
  %0 = load i1, i1* getelementptr inbounds (%"?u64", %"?u64"* @tls_thread_id, i32 0, i32 1), align 1
  br i1 %0, label %OptionalNonNull, label %OptionalNull

OptionalNull:                                     ; preds = %Entry
  %1 = call fastcc i32 @std.os.linux.gettid()
  store i32 %1, i32* %tid, align 4
  %2 = load i32, i32* %tid, align 4
  store i1 true, i1* getelementptr inbounds (%"?u64", %"?u64"* @tls_thread_id, i32 0, i32 1), align 1
  %3 = zext i32 %2 to i64
  store i64 %3, i64* getelementptr inbounds (%"?u64", %"?u64"* @tls_thread_id, i32 0, i32 0), align 8
  %4 = load i32, i32* %tid, align 4
  %5 = zext i32 %4 to i64
  store i64 %5, i64* %result, align 8
  %6 = load i64, i64* %result, align 8
  ret i64 %6

OptionalNonNull:                                  ; preds = %Entry
  %7 = load i64, i64* getelementptr inbounds (%"?u64", %"?u64"* @tls_thread_id, i32 0, i32 0), align 8
  store i64 %7, i64* %result, align 8
  %8 = load i64, i64* %result, align 8
  ret i64 %8
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.format(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0, %"std.debug.struct:296:56"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %2 = alloca i16, align 2
  %3 = alloca i64, align 8
  %4 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %5 = alloca i16, align 2
  %6 = alloca i16, align 2
  %7 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"[]u8"* @17)
  store i16 %7, i16* %2, align 2
  %8 = icmp ne i16 %7, 0
  br i1 %8, label %ErrRetReturn, label %ErrRetContinue

ErrRetReturn:                                     ; preds = %Entry
  %9 = load i16, i16* %2, align 2
  store i16 %9, i16* %result, align 2
  ret i16 %9

ErrRetContinue:                                   ; preds = %Entry
  %10 = getelementptr inbounds %"std.debug.struct:296:56", %"std.debug.struct:296:56"* %1, i32 0, i32 0
  %11 = load i64, i64* %10, align 8
  store i64 %11, i64* %3, align 8
  %12 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0 to i8*
  %13 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %4 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %13, i8* align 4 %12, i64 4, i1 false)
  %14 = call fastcc i16 @std.fmt.formatType(i64 %11, %std.fmt.FormatOptions* bitcast ({ %"?usize", %"?usize", i2, <{ i8, [6 x i8] }> }* @20 to %std.fmt.FormatOptions*), %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, i64 3)
  store i16 %14, i16* %5, align 2
  %15 = icmp ne i16 %14, 0
  br i1 %15, label %ErrRetReturn1, label %ErrRetContinue2

ErrRetReturn1:                                    ; preds = %ErrRetContinue
  %16 = load i16, i16* %5, align 2
  store i16 %16, i16* %result, align 2
  ret i16 %16

ErrRetContinue2:                                  ; preds = %ErrRetContinue
  %17 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"[]u8"* @21)
  store i16 %17, i16* %6, align 2
  %18 = icmp ne i16 %17, 0
  br i1 %18, label %ErrRetReturn3, label %ErrRetContinue4

ErrRetReturn3:                                    ; preds = %ErrRetContinue2
  %19 = load i16, i16* %6, align 2
  store i16 %19, i16* %result, align 2
  ret i16 %19

ErrRetContinue4:                                  ; preds = %ErrRetContinue2
  store i16 0, i16* %result, align 2
  ret i16 0
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.os.raise(i8 %0) unnamed_addr #1 {
Entry:
  %1 = alloca [6 x i64], align 8
  %result = alloca i16, align 2
  %set = alloca [32 x i32], align 4
  %tid = alloca i32, align 4
  %rc = alloca i64, align 8
  %2 = alloca i16, align 2
  %err = alloca i16, align 2
  %sig = alloca i8, align 1
  store i8 %0, i8* %sig, align 1
  %3 = bitcast [32 x i32]* %set to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %3, i8 -86, i64 128, i1 false)
  %4 = ptrtoint i8* %3 to i64
  %5 = getelementptr inbounds [6 x i64], [6 x i64]* %1, i64 0, i64 0
  store i64 1296236545, i64* %5, align 8
  %6 = getelementptr inbounds [6 x i64], [6 x i64]* %1, i64 0, i64 1
  store i64 %4, i64* %6, align 8
  %7 = getelementptr inbounds [6 x i64], [6 x i64]* %1, i64 0, i64 2
  store i64 128, i64* %7, align 8
  %8 = getelementptr inbounds [6 x i64], [6 x i64]* %1, i64 0, i64 3
  store i64 0, i64* %8, align 8
  %9 = getelementptr inbounds [6 x i64], [6 x i64]* %1, i64 0, i64 4
  store i64 0, i64* %9, align 8
  %10 = getelementptr inbounds [6 x i64], [6 x i64]* %1, i64 0, i64 5
  store i64 0, i64* %10, align 8
  %11 = ptrtoint [6 x i64]* %1 to i64
  %12 = call i64 asm sideeffect "rolq $$3,  %rdi ; rolq $$13, %rdi\0Arolq $$61, %rdi ; rolq $$51, %rdi\0Axchgq %rbx,%rbx\0A", "={rdx},{rax},0,~{cc},~{memory}"(i64 %11, i64 0)
  %13 = call fastcc i64 @std.os.linux.sigprocmask(i32 0, [32 x i32]* @app_mask, [32 x i32]* %set)
  %14 = call fastcc i32 @std.os.linux.gettid()
  store i32 %14, i32* %tid, align 4
  %15 = load i32, i32* %tid, align 4
  %16 = load i8, i8* %sig, align 1
  %17 = zext i8 %16 to i32
  %18 = call fastcc i64 @std.os.linux.tkill(i32 %15, i32 %17)
  store i64 %18, i64* %rc, align 8
  %19 = call fastcc i64 @std.os.linux.sigprocmask(i32 2, [32 x i32]* %set, [32 x i32]* null)
  %20 = load i64, i64* %rc, align 8
  %21 = call fastcc i16 @std.os.linux.getErrno(i64 %20)
  store i16 %21, i16* %2, align 2
  switch i16 %21, label %SwitchElse [
    i16 0, label %SwitchProng
  ]

SwitchElse:                                       ; preds = %Entry
  store i16 %21, i16* %err, align 2
  %22 = load i16, i16* %err, align 2
  %23 = call fastcc i16 @std.os.unexpectedErrno(i16 %22)
  store i16 %23, i16* %result, align 2
  %24 = load i16, i16* %result, align 2
  ret i16 %24

SwitchProng:                                      ; preds = %Entry
  store i16 0, i16* %result, align 2
  %25 = load i16, i16* %result, align 2
  ret i16 %25
}

; Function Attrs: nobuiltin noreturn nounwind
define internal fastcc void @std.os.exit(i8 %0) unnamed_addr #6 {
Entry:
  %status = alloca i8, align 1
  store i8 %0, i8* %status, align 1
  %1 = load i8, i8* %status, align 1
  %2 = zext i8 %1 to i32
  call fastcc void @std.os.linux.exit_group(i32 %2)
  unreachable
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.format.12(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0, %"std.debug.struct:298:40"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %2 = alloca %"[]u8", align 8
  %3 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %4 = alloca i16, align 2
  %5 = alloca i16, align 2
  %6 = getelementptr inbounds %"std.debug.struct:298:40", %"std.debug.struct:298:40"* %1, i32 0, i32 0
  %7 = bitcast %"[]u8"* %6 to i8*
  %8 = bitcast %"[]u8"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %8, i8* align 8 %7, i64 16, i1 false)
  %9 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0 to i8*
  %10 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %3 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %10, i8* align 4 %9, i64 4, i1 false)
  %11 = call fastcc i16 @std.fmt.formatType.16(%"[]u8"* %6, %std.fmt.FormatOptions* bitcast ({ %"?usize", %"?usize", i2, <{ i8, [6 x i8] }> }* @35 to %std.fmt.FormatOptions*), %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, i64 3)
  store i16 %11, i16* %4, align 2
  %12 = icmp ne i16 %11, 0
  br i1 %12, label %ErrRetReturn, label %ErrRetContinue

ErrRetReturn:                                     ; preds = %Entry
  %13 = load i16, i16* %4, align 2
  store i16 %13, i16* %result, align 2
  ret i16 %13

ErrRetContinue:                                   ; preds = %Entry
  %14 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"[]u8"* @37)
  store i16 %14, i16* %5, align 2
  %15 = icmp ne i16 %14, 0
  br i1 %15, label %ErrRetReturn1, label %ErrRetContinue2

ErrRetReturn1:                                    ; preds = %ErrRetContinue
  %16 = load i16, i16* %5, align 2
  store i16 %16, i16* %result, align 2
  ret i16 %16

ErrRetContinue2:                                  ; preds = %ErrRetContinue
  store i16 0, i16* %result, align 2
  ret i16 0
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).print.13"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %1 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %2 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0 to i8*
  %3 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %1 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %3, i8* align 4 %2, i64 4, i1 false)
  %4 = call fastcc i16 @std.fmt.format.17(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0)
  store i16 %4, i16* %result, align 2
  %5 = load i16, i16* %result, align 2
  ret i16 %5
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).print.14"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %1 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %2 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0 to i8*
  %3 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %1 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %3, i8* align 4 %2, i64 4, i1 false)
  %4 = call fastcc i16 @std.fmt.format.18(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0)
  store i16 %4, i16* %result, align 2
  %5 = load i16, i16* %result, align 2
  ret i16 %5
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.Thread.Mutex.AtomicMutex.unlock(%std.Thread.Mutex.AtomicMutex* nonnull align 4 %0) unnamed_addr #1 {
Entry:
  %1 = alloca i32, align 4
  %m = alloca %std.Thread.Mutex.AtomicMutex*, align 8
  store %std.Thread.Mutex.AtomicMutex* %0, %std.Thread.Mutex.AtomicMutex** %m, align 8
  %2 = load %std.Thread.Mutex.AtomicMutex*, %std.Thread.Mutex.AtomicMutex** %m, align 8
  %3 = getelementptr inbounds %std.Thread.Mutex.AtomicMutex, %std.Thread.Mutex.AtomicMutex* %2, i32 0, i32 0
  %4 = atomicrmw xchg i32* %3, i32 0 release, align 4
  store i32 %4, i32* %1, align 4
  switch i32 %4, label %SwitchElse [
    i32 0, label %SwitchProng
    i32 1, label %SwitchProng1
    i32 2, label %SwitchProng2
  ]

SwitchProng:                                      ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchElse:                                       ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng1:                                     ; preds = %Entry
  br label %SwitchEnd

SwitchProng2:                                     ; preds = %Entry
  %5 = load %std.Thread.Mutex.AtomicMutex*, %std.Thread.Mutex.AtomicMutex** %m, align 8
  call fastcc void @std.Thread.Mutex.AtomicMutex.unlockSlow(%std.Thread.Mutex.AtomicMutex* %5)
  br label %SwitchEnd

SwitchEnd:                                        ; preds = %SwitchProng2, %SwitchProng1
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.Thread.StaticResetEvent.AtomicEvent.wait(%std.Thread.StaticResetEvent.AtomicEvent* nonnull align 4 %0) unnamed_addr #1 {
Entry:
  %1 = alloca i1, align 1
  %ev = alloca %std.Thread.StaticResetEvent.AtomicEvent*, align 8
  store %std.Thread.StaticResetEvent.AtomicEvent* %0, %std.Thread.StaticResetEvent.AtomicEvent** %ev, align 8
  %2 = load %std.Thread.StaticResetEvent.AtomicEvent*, %std.Thread.StaticResetEvent.AtomicEvent** %ev, align 8
  %3 = call fastcc i1 @std.Thread.StaticResetEvent.AtomicEvent.timedWait(%std.Thread.StaticResetEvent.AtomicEvent* %2, %"?u64"* @41)
  store i1 %3, i1* %1, align 1
  switch i1 %3, label %SwitchElse [
    i1 true, label %SwitchProng
    i1 false, label %SwitchProng1
  ]

SwitchProng:                                      ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng1:                                     ; preds = %Entry
  ret void

SwitchElse:                                       ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.format.15(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %1 = alloca i16, align 2
  %2 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"[]u8"* @43)
  store i16 %2, i16* %1, align 2
  %3 = icmp ne i16 %2, 0
  br i1 %3, label %ErrRetReturn, label %ErrRetContinue

ErrRetReturn:                                     ; preds = %Entry
  %4 = load i16, i16* %1, align 2
  store i16 %4, i16* %result, align 2
  ret i16 %4

ErrRetContinue:                                   ; preds = %Entry
  store i16 0, i16* %result, align 2
  ret i16 0
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i1 @std.target.Arch.isMIPS(i6 %0) unnamed_addr #1 {
Entry:
  %result = alloca i1, align 1
  %arch = alloca i6, align 1
  store i6 %0, i6* %arch, align 1
  %1 = load i6, i6* %arch, align 1
  switch i6 %1, label %SwitchElse [
    i6 12, label %SwitchProng
    i6 13, label %SwitchProng
    i6 14, label %SwitchProng
    i6 15, label %SwitchProng
  ]

SwitchElse:                                       ; preds = %Entry
  store i1 false, i1* %result, align 1
  br label %SwitchEnd

SwitchProng:                                      ; preds = %Entry, %Entry, %Entry, %Entry
  store i1 true, i1* %result, align 1
  br label %SwitchEnd

SwitchEnd:                                        ; preds = %SwitchProng, %SwitchElse
  %2 = load i1, i1* %result, align 1
  ret i1 %2
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i1 @std.target.Arch.isSPARC(i6 %0) unnamed_addr #1 {
Entry:
  %result = alloca i1, align 1
  %arch = alloca i6, align 1
  store i6 %0, i6* %arch, align 1
  %1 = load i6, i6* %arch, align 1
  switch i6 %1, label %SwitchElse [
    i6 25, label %SwitchProng
    i6 27, label %SwitchProng
    i6 26, label %SwitchProng
  ]

SwitchElse:                                       ; preds = %Entry
  store i1 false, i1* %result, align 1
  br label %SwitchEnd

SwitchProng:                                      ; preds = %Entry, %Entry, %Entry
  store i1 true, i1* %result, align 1
  br label %SwitchEnd

SwitchEnd:                                        ; preds = %SwitchProng, %SwitchElse
  %2 = load i1, i1* %result, align 1
  ret i1 %2
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.os.linux.sigaction(i6 %0, %std.os.linux.Sigaction* noalias align 8 %1, %std.os.linux.Sigaction* noalias align 8 %2) unnamed_addr #1 {
Entry:
  %3 = alloca [6 x i64], align 8
  %result = alloca i64, align 8
  %ksa = alloca %std.os.linux.k_sigaction, align 8
  %oldksa = alloca %std.os.linux.k_sigaction, align 8
  %new = alloca %std.os.linux.Sigaction*, align 8
  %restorer_fn = alloca void ()*, align 8
  %ksa_arg = alloca i64, align 8
  %oldksa_arg = alloca i64, align 8
  %result13 = alloca i64, align 8
  %old = alloca %std.os.linux.Sigaction*, align 8
  %sig = alloca i6, align 1
  %act = alloca %std.os.linux.Sigaction*, align 8
  %oact = alloca %std.os.linux.Sigaction*, align 8
  store i6 %0, i6* %sig, align 1
  store %std.os.linux.Sigaction* %1, %std.os.linux.Sigaction** %act, align 8
  store %std.os.linux.Sigaction* %2, %std.os.linux.Sigaction** %oact, align 8
  %4 = load i6, i6* %sig, align 1
  %5 = icmp uge i6 %4, 1
  call fastcc void @std.debug.assert(i1 %5)
  %6 = load i6, i6* %sig, align 1
  %7 = icmp ne i6 %6, 9
  call fastcc void @std.debug.assert(i1 %7)
  %8 = load i6, i6* %sig, align 1
  %9 = icmp ne i6 %8, 19
  call fastcc void @std.debug.assert(i1 %9)
  %10 = bitcast %std.os.linux.k_sigaction* %ksa to i8*
  call void @llvm.memset.p0i8.i64(i8* align 8 %10, i8 -86, i64 32, i1 false)
  %11 = ptrtoint i8* %10 to i64
  %12 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 0
  store i64 1296236545, i64* %12, align 8
  %13 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 1
  store i64 %11, i64* %13, align 8
  %14 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 2
  store i64 32, i64* %14, align 8
  %15 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 3
  store i64 0, i64* %15, align 8
  %16 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 4
  store i64 0, i64* %16, align 8
  %17 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 5
  store i64 0, i64* %17, align 8
  %18 = ptrtoint [6 x i64]* %3 to i64
  %19 = call i64 asm sideeffect "rolq $$3,  %rdi ; rolq $$13, %rdi\0Arolq $$61, %rdi ; rolq $$51, %rdi\0Axchgq %rbx,%rbx\0A", "={rdx},{rax},0,~{cc},~{memory}"(i64 %18, i64 0)
  %20 = bitcast %std.os.linux.k_sigaction* %oldksa to i8*
  call void @llvm.memset.p0i8.i64(i8* align 8 %20, i8 -86, i64 32, i1 false)
  %21 = ptrtoint i8* %20 to i64
  %22 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 0
  store i64 1296236545, i64* %22, align 8
  %23 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 1
  store i64 %21, i64* %23, align 8
  %24 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 2
  store i64 32, i64* %24, align 8
  %25 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 3
  store i64 0, i64* %25, align 8
  %26 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 4
  store i64 0, i64* %26, align 8
  %27 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 5
  store i64 0, i64* %27, align 8
  %28 = ptrtoint [6 x i64]* %3 to i64
  %29 = call i64 asm sideeffect "rolq $$3,  %rdi ; rolq $$13, %rdi\0Arolq $$61, %rdi ; rolq $$51, %rdi\0Axchgq %rbx,%rbx\0A", "={rdx},{rax},0,~{cc},~{memory}"(i64 %28, i64 0)
  %30 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %act, align 8
  %31 = icmp ne %std.os.linux.Sigaction* %30, null
  br i1 %31, label %OptionalThen, label %OptionalElse

OptionalThen:                                     ; preds = %Entry
  %32 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %act, align 8
  store %std.os.linux.Sigaction* %32, %std.os.linux.Sigaction** %new, align 8
  %33 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %new, align 8
  %34 = getelementptr inbounds %std.os.linux.Sigaction, %std.os.linux.Sigaction* %33, i32 0, i32 2
  %35 = load i32, i32* %34, align 4
  %36 = and i32 %35, 4
  %37 = icmp ne i32 %36, 0
  br i1 %37, label %Then, label %Else

Then:                                             ; preds = %OptionalThen
  store void ()* @std.os.linux.x86_64.restore_rt, void ()** %restorer_fn, align 8
  br label %EndIf

Else:                                             ; preds = %OptionalThen
  store void ()* @std.os.linux.x86_64.restore_rt, void ()** %restorer_fn, align 8
  br label %EndIf

EndIf:                                            ; preds = %Else, %Then
  %38 = getelementptr inbounds %std.os.linux.k_sigaction, %std.os.linux.k_sigaction* %ksa, i32 0, i32 0
  %39 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %new, align 8
  %40 = getelementptr inbounds %std.os.linux.Sigaction, %std.os.linux.Sigaction* %39, i32 0, i32 0
  %41 = getelementptr inbounds %"std.os.linux.union:3060:14", %"std.os.linux.union:3060:14"* %40, i32 0, i32 0
  %42 = load void (i32)*, void (i32)** %41, align 8
  store void (i32)* %42, void (i32)** %38, align 8
  %43 = getelementptr inbounds %std.os.linux.k_sigaction, %std.os.linux.k_sigaction* %ksa, i32 0, i32 1
  %44 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %new, align 8
  %45 = getelementptr inbounds %std.os.linux.Sigaction, %std.os.linux.Sigaction* %44, i32 0, i32 2
  %46 = load i32, i32* %45, align 4
  %47 = or i32 %46, 67108864
  %48 = zext i32 %47 to i64
  store i64 %48, i64* %43, align 8
  %49 = getelementptr inbounds %std.os.linux.k_sigaction, %std.os.linux.k_sigaction* %ksa, i32 0, i32 3
  %50 = bitcast [2 x i32]* %49 to i8*
  call void @llvm.memset.p0i8.i64(i8* align 4 %50, i8 -86, i64 8, i1 false)
  %51 = ptrtoint i8* %50 to i64
  %52 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 0
  store i64 1296236545, i64* %52, align 8
  %53 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 1
  store i64 %51, i64* %53, align 8
  %54 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 2
  store i64 8, i64* %54, align 8
  %55 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 3
  store i64 0, i64* %55, align 8
  %56 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 4
  store i64 0, i64* %56, align 8
  %57 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 5
  store i64 0, i64* %57, align 8
  %58 = ptrtoint [6 x i64]* %3 to i64
  %59 = call i64 asm sideeffect "rolq $$3,  %rdi ; rolq $$13, %rdi\0Arolq $$61, %rdi ; rolq $$51, %rdi\0Axchgq %rbx,%rbx\0A", "={rdx},{rax},0,~{cc},~{memory}"(i64 %58, i64 0)
  %60 = getelementptr inbounds %std.os.linux.k_sigaction, %std.os.linux.k_sigaction* %ksa, i32 0, i32 2
  %61 = load void ()*, void ()** %restorer_fn, align 8
  %62 = icmp ne void ()* %61, null
  br i1 %62, label %PtrCastOk, label %PtrCastFail

OptionalElse:                                     ; preds = %Entry
  br label %OptionalEndIf

OptionalEndIf:                                    ; preds = %OptionalElse, %PtrCastOk17
  %63 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %act, align 8
  %64 = icmp ne %std.os.linux.Sigaction* %63, null
  br i1 %64, label %Then1, label %Else2

Then1:                                            ; preds = %OptionalEndIf
  %65 = ptrtoint %std.os.linux.k_sigaction* %ksa to i64
  store i64 %65, i64* %ksa_arg, align 8
  br label %EndIf3

Else2:                                            ; preds = %OptionalEndIf
  store i64 0, i64* %ksa_arg, align 8
  br label %EndIf3

EndIf3:                                           ; preds = %Else2, %Then1
  %66 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %oact, align 8
  %67 = icmp ne %std.os.linux.Sigaction* %66, null
  br i1 %67, label %Then4, label %Else5

Then4:                                            ; preds = %EndIf3
  %68 = ptrtoint %std.os.linux.k_sigaction* %oldksa to i64
  store i64 %68, i64* %oldksa_arg, align 8
  br label %EndIf6

Else5:                                            ; preds = %EndIf3
  store i64 0, i64* %oldksa_arg, align 8
  br label %EndIf6

EndIf6:                                           ; preds = %Else5, %Then4
  %69 = load i6, i6* %sig, align 1
  %70 = zext i6 %69 to i64
  %71 = load i64, i64* %ksa_arg, align 8
  %72 = load i64, i64* %oldksa_arg, align 8
  %73 = call fastcc i64 @std.os.linux.x86_64.syscall4(i64 13, i64 %70, i64 %71, i64 %72, i64 8)
  store i64 %73, i64* %result13, align 8
  %74 = load i64, i64* %result13, align 8
  %75 = call fastcc i16 @std.os.linux.getErrno(i64 %74)
  %76 = icmp ne i16 %75, 0
  br i1 %76, label %Then7, label %Else8

Then7:                                            ; preds = %EndIf6
  %77 = load i64, i64* %result13, align 8
  store i64 %77, i64* %result, align 8
  %78 = load i64, i64* %result, align 8
  ret i64 %78

Else8:                                            ; preds = %EndIf6
  br label %EndIf9

EndIf9:                                           ; preds = %Else8
  %79 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %oact, align 8
  %80 = icmp ne %std.os.linux.Sigaction* %79, null
  br i1 %80, label %OptionalThen10, label %OptionalElse11

OptionalThen10:                                   ; preds = %EndIf9
  %81 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %oact, align 8
  store %std.os.linux.Sigaction* %81, %std.os.linux.Sigaction** %old, align 8
  %82 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %old, align 8
  %83 = getelementptr inbounds %std.os.linux.Sigaction, %std.os.linux.Sigaction* %82, i32 0, i32 0
  %84 = getelementptr inbounds %"std.os.linux.union:3060:14", %"std.os.linux.union:3060:14"* %83, i32 0, i32 0
  %85 = getelementptr inbounds %std.os.linux.k_sigaction, %std.os.linux.k_sigaction* %oldksa, i32 0, i32 0
  %86 = load void (i32)*, void (i32)** %85, align 8
  store void (i32)* %86, void (i32)** %84, align 8
  %87 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %old, align 8
  %88 = getelementptr inbounds %std.os.linux.Sigaction, %std.os.linux.Sigaction* %87, i32 0, i32 2
  %89 = getelementptr inbounds %std.os.linux.k_sigaction, %std.os.linux.k_sigaction* %oldksa, i32 0, i32 1
  %90 = load i64, i64* %89, align 8
  %91 = trunc i64 %90 to i32
  store i32 %91, i32* %88, align 4
  %92 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %old, align 8
  %93 = getelementptr inbounds %std.os.linux.Sigaction, %std.os.linux.Sigaction* %92, i32 0, i32 1
  %94 = bitcast [32 x i32]* %93 to i8*
  %95 = icmp ne i8* %94, null
  br i1 %95, label %PtrCastOk19, label %PtrCastFail18

OptionalElse11:                                   ; preds = %EndIf9
  br label %OptionalEndIf12

OptionalEndIf12:                                  ; preds = %OptionalElse11, %PtrCastOk21
  store i64 0, i64* %result, align 8
  %96 = load i64, i64* %result, align 8
  ret i64 %96

PtrCastFail:                                      ; preds = %EndIf
  call fastcc void @std.builtin.default_panic(%"[]u8"* @47, %std.builtin.StackTrace* null)
  unreachable

PtrCastOk:                                        ; preds = %EndIf
  store void ()* %61, void ()** %60, align 8
  %97 = getelementptr inbounds %std.os.linux.k_sigaction, %std.os.linux.k_sigaction* %ksa, i32 0, i32 3
  %98 = bitcast [2 x i32]* %97 to i8*
  %99 = icmp ne i8* %98, null
  br i1 %99, label %PtrCastOk15, label %PtrCastFail14

PtrCastFail14:                                    ; preds = %PtrCastOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @47, %std.builtin.StackTrace* null)
  unreachable

PtrCastOk15:                                      ; preds = %PtrCastOk
  %100 = load %std.os.linux.Sigaction*, %std.os.linux.Sigaction** %new, align 8
  %101 = getelementptr inbounds %std.os.linux.Sigaction, %std.os.linux.Sigaction* %100, i32 0, i32 1
  %102 = bitcast [32 x i32]* %101 to i8*
  %103 = icmp ne i8* %102, null
  br i1 %103, label %PtrCastOk17, label %PtrCastFail16

PtrCastFail16:                                    ; preds = %PtrCastOk15
  call fastcc void @std.builtin.default_panic(%"[]u8"* @47, %std.builtin.StackTrace* null)
  unreachable

PtrCastOk17:                                      ; preds = %PtrCastOk15
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %98, i8* align 4 %102, i64 8, i1 false)
  br label %OptionalEndIf

PtrCastFail18:                                    ; preds = %OptionalThen10
  call fastcc void @std.builtin.default_panic(%"[]u8"* @47, %std.builtin.StackTrace* null)
  unreachable

PtrCastOk19:                                      ; preds = %OptionalThen10
  %104 = getelementptr inbounds %std.os.linux.k_sigaction, %std.os.linux.k_sigaction* %oldksa, i32 0, i32 3
  %105 = bitcast [2 x i32]* %104 to i8*
  %106 = icmp ne i8* %105, null
  br i1 %106, label %PtrCastOk21, label %PtrCastFail20

PtrCastFail20:                                    ; preds = %PtrCastOk19
  call fastcc void @std.builtin.default_panic(%"[]u8"* @47, %std.builtin.StackTrace* null)
  unreachable

PtrCastOk21:                                      ; preds = %PtrCastOk19
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %94, i8* align 4 %105, i64 8, i1 false)
  br label %OptionalEndIf12
}

; Function Attrs: cold nobuiltin nounwind
define internal fastcc void @std.Thread.Mutex.AtomicMutex.lockSlow(%std.Thread.Mutex.AtomicMutex* nonnull align 4 %0, i32 %1) unnamed_addr #7 {
Entry:
  %new_state = alloca i32, align 4
  %spin = alloca i8, align 1
  %2 = alloca %"?std.Thread.Mutex.State", align 4
  %state = alloca i32, align 4
  %3 = alloca i8, align 1
  %iter = alloca i8, align 1
  %4 = alloca i32, align 4
  %5 = alloca i16, align 2
  %m = alloca %std.Thread.Mutex.AtomicMutex*, align 8
  %current_state = alloca i32, align 4
  store %std.Thread.Mutex.AtomicMutex* %0, %std.Thread.Mutex.AtomicMutex** %m, align 8
  store i32 %1, i32* %current_state, align 4
  %6 = load i32, i32* %current_state, align 4
  store i32 %6, i32* %new_state, align 4
  store i8 0, i8* %spin, align 1
  br label %WhileCond

WhileCond:                                        ; preds = %OverflowOk19, %Entry
  %7 = load i8, i8* %spin, align 1
  %8 = icmp ult i8 %7, 100
  br i1 %8, label %WhileBody, label %WhileEnd5

WhileBody:                                        ; preds = %WhileCond
  %9 = load %std.Thread.Mutex.AtomicMutex*, %std.Thread.Mutex.AtomicMutex** %m, align 8
  %10 = getelementptr inbounds %std.Thread.Mutex.AtomicMutex, %std.Thread.Mutex.AtomicMutex* %9, i32 0, i32 0
  %11 = load i32, i32* %new_state, align 4
  %12 = cmpxchg weak i32* %10, i32 0, i32 %11 acquire monotonic, align 4
  %13 = extractvalue { i32, i1 } %12, 0
  %14 = getelementptr inbounds %"?std.Thread.Mutex.State", %"?std.Thread.Mutex.State"* %2, i32 0, i32 0
  store i32 %13, i32* %14, align 4
  %15 = extractvalue { i32, i1 } %12, 1
  %16 = xor i1 %15, true
  %17 = getelementptr inbounds %"?std.Thread.Mutex.State", %"?std.Thread.Mutex.State"* %2, i32 0, i32 1
  store i1 %16, i1* %17, align 1
  %18 = getelementptr inbounds %"?std.Thread.Mutex.State", %"?std.Thread.Mutex.State"* %2, i32 0, i32 1
  %19 = load i1, i1* %18, align 1
  br i1 %19, label %OptionalNonNull, label %OptionalNull

OptionalNull:                                     ; preds = %WhileBody
  ret void

OptionalNonNull:                                  ; preds = %WhileBody
  %20 = getelementptr inbounds %"?std.Thread.Mutex.State", %"?std.Thread.Mutex.State"* %2, i32 0, i32 0
  %21 = load i32, i32* %20, align 4
  store i32 %21, i32* %state, align 4
  br label %OptionalEnd

OptionalEnd:                                      ; preds = %OptionalNonNull
  %22 = load i32, i32* %state, align 4
  switch i32 %22, label %SwitchElse [
    i32 0, label %SwitchProng1
    i32 1, label %SwitchProng2
    i32 2, label %SwitchProng
  ]

SwitchProng:                                      ; preds = %OptionalEnd
  br label %WhileEnd5

SwitchElse:                                       ; preds = %OptionalEnd
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng1:                                     ; preds = %OptionalEnd
  br label %SwitchEnd

SwitchProng2:                                     ; preds = %OptionalEnd
  br label %SwitchEnd

SwitchEnd:                                        ; preds = %SwitchProng2, %SwitchProng1
  %23 = load i8, i8* %spin, align 1
  %24 = call { i8, i1 } @llvm.uadd.with.overflow.i8(i8 %23, i8 1)
  %25 = extractvalue { i8, i1 } %24, 0
  %26 = extractvalue { i8, i1 } %24, 1
  br i1 %26, label %OverflowFail, label %OverflowOk

WhileCond3:                                       ; preds = %OverflowOk17, %OverflowOk
  %27 = load i8, i8* %iter, align 1
  %28 = icmp ugt i8 %27, 0
  br i1 %28, label %WhileBody4, label %WhileEnd

WhileBody4:                                       ; preds = %WhileCond3
  call void asm sideeffect "pause", "~{memory},~{dirflag},~{fpsr},~{flags}"() #10
  %29 = load i8, i8* %iter, align 1
  %30 = call { i8, i1 } @llvm.usub.with.overflow.i8(i8 %29, i8 1)
  %31 = extractvalue { i8, i1 } %30, 0
  %32 = extractvalue { i8, i1 } %30, 1
  br i1 %32, label %OverflowFail16, label %OverflowOk17

WhileEnd:                                         ; preds = %WhileCond3
  %33 = load i8, i8* %spin, align 1
  %34 = call { i8, i1 } @llvm.uadd.with.overflow.i8(i8 %33, i8 1)
  %35 = extractvalue { i8, i1 } %34, 0
  %36 = extractvalue { i8, i1 } %34, 1
  br i1 %36, label %OverflowFail18, label %OverflowOk19

WhileEnd5:                                        ; preds = %SwitchProng, %WhileCond
  store i32 2, i32* %new_state, align 4
  br label %WhileCond6

WhileCond6:                                       ; preds = %SwitchEnd15, %WhileEnd5
  br label %WhileBody7

WhileBody7:                                       ; preds = %WhileCond6
  %37 = load %std.Thread.Mutex.AtomicMutex*, %std.Thread.Mutex.AtomicMutex** %m, align 8
  %38 = getelementptr inbounds %std.Thread.Mutex.AtomicMutex, %std.Thread.Mutex.AtomicMutex* %37, i32 0, i32 0
  %39 = load i32, i32* %new_state, align 4
  %40 = atomicrmw xchg i32* %38, i32 %39 acquire, align 4
  store i32 %40, i32* %4, align 4
  switch i32 %40, label %SwitchElse9 [
    i32 0, label %SwitchProng8
  ]

SwitchProng8:                                     ; preds = %WhileBody7
  ret void

SwitchElse9:                                      ; preds = %WhileBody7
  br label %SwitchEnd10

SwitchEnd10:                                      ; preds = %SwitchElse9
  %41 = load %std.Thread.Mutex.AtomicMutex*, %std.Thread.Mutex.AtomicMutex** %m, align 8
  %42 = getelementptr inbounds %std.Thread.Mutex.AtomicMutex, %std.Thread.Mutex.AtomicMutex* %41, i32 0, i32 0
  %43 = icmp ne i32* %42, null
  br i1 %43, label %PtrCastOk, label %PtrCastFail

SwitchElse11:                                     ; preds = %PtrCastOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng12:                                    ; preds = %PtrCastOk
  br label %SwitchEnd15

SwitchProng13:                                    ; preds = %PtrCastOk
  br label %SwitchEnd15

SwitchProng14:                                    ; preds = %PtrCastOk
  br label %SwitchEnd15

SwitchEnd15:                                      ; preds = %SwitchProng14, %SwitchProng13, %SwitchProng12
  br label %WhileCond6

OverflowFail:                                     ; preds = %SwitchEnd
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %SwitchEnd
  store i8 %25, i8* %3, align 1
  %44 = call fastcc i8 @std.math.min(i8 %25)
  store i8 %44, i8* %iter, align 1
  br label %WhileCond3

OverflowFail16:                                   ; preds = %WhileBody4
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk17:                                     ; preds = %WhileBody4
  store i8 %31, i8* %iter, align 1
  br label %WhileCond3

OverflowFail18:                                   ; preds = %WhileEnd
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk19:                                     ; preds = %WhileEnd
  store i8 %35, i8* %spin, align 1
  br label %WhileCond

PtrCastFail:                                      ; preds = %SwitchEnd10
  call fastcc void @std.builtin.default_panic(%"[]u8"* @47, %std.builtin.StackTrace* null)
  unreachable

PtrCastOk:                                        ; preds = %SwitchEnd10
  %45 = load i32, i32* %new_state, align 4
  %46 = call fastcc i64 @std.os.linux.futex_wait(i32* %42, i32 128, i32 %45, %std.os.linux.timespec* null)
  %47 = call fastcc i16 @std.os.linux.getErrno(i64 %46)
  store i16 %47, i16* %5, align 2
  switch i16 %47, label %SwitchElse11 [
    i16 0, label %SwitchProng12
    i16 4, label %SwitchProng13
    i16 11, label %SwitchProng14
  ]
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare i64 @llvm.umin.i64(i64, i64) #8

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.os.linux.write(i32 %0, i8* nonnull readonly align 1 %1, i64 %2) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %fd = alloca i32, align 4
  %buf = alloca i8*, align 8
  %count = alloca i64, align 8
  store i32 %0, i32* %fd, align 4
  store i8* %1, i8** %buf, align 8
  store i64 %2, i64* %count, align 8
  %3 = load i32, i32* %fd, align 4
  %4 = sext i32 %3 to i64
  %5 = sext i32 %3 to i64
  %6 = load i8*, i8** %buf, align 8
  %7 = ptrtoint i8* %6 to i64
  %8 = load i64, i64* %count, align 8
  %9 = call fastcc i64 @std.os.linux.x86_64.syscall3(i64 1, i64 %5, i64 %7, i64 %8)
  store i64 %9, i64* %result, align 8
  %10 = load i64, i64* %result, align 8
  ret i64 %10
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.os.unexpectedErrno(i16 %0) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %1 = alloca %"std.os.struct:4985:53", align 2
  %err = alloca i16, align 2
  store i16 %0, i16* %err, align 2
  %2 = load i16, i16* %err, align 2
  %3 = getelementptr inbounds %"std.os.struct:4985:53", %"std.os.struct:4985:53"* %1, i32 0, i32 0
  store i16 %2, i16* %3, align 2
  call fastcc void @std.debug.print(%"std.os.struct:4985:53"* %1)
  call fastcc void @std.debug.dumpCurrentStackTrace(%"?usize"* @48)
  store i16 12, i16* %result, align 2
  %4 = load i16, i16* %result, align 2
  ret i16 %4
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i32 @std.os.linux.gettid() unnamed_addr #1 {
Entry:
  %result = alloca i32, align 4
  %0 = call fastcc i64 @std.os.linux.x86_64.syscall0(i64 186)
  %1 = trunc i64 %0 to i32
  store i32 %1, i32* %result, align 4
  %2 = load i32, i32* %result, align 4
  ret i32 %2
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0, %"[]u8"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %index = alloca i64, align 8
  %2 = alloca %"[]u8", align 8
  %3 = alloca { i64, i16 }, align 8
  store i64 0, i64* %index, align 8
  br label %WhileCond

WhileCond:                                        ; preds = %OverflowOk, %Entry
  %4 = load i64, i64* %index, align 8
  %5 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = icmp ne i64 %4, %6
  br i1 %7, label %WhileBody, label %WhileEnd

WhileBody:                                        ; preds = %WhileCond
  %8 = load i64, i64* %index, align 8
  %9 = load i64, i64* %index, align 8
  %10 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %11 = load i64, i64* %10, align 8
  %12 = icmp ule i64 %9, %11
  br i1 %12, label %BoundsCheckOk, label %BoundsCheckFail

ErrRetReturn:                                     ; preds = %BoundsCheckOk2
  %13 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %3, i32 0, i32 1
  %14 = load i16, i16* %13, align 2
  store i16 %14, i16* %result, align 2
  ret i16 %14

ErrRetContinue:                                   ; preds = %BoundsCheckOk2
  %15 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %3, i32 0, i32 0
  %16 = load i64, i64* %15, align 8
  %17 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %8, i64 %16)
  %18 = extractvalue { i64, i1 } %17, 0
  %19 = extractvalue { i64, i1 } %17, 1
  br i1 %19, label %OverflowFail, label %OverflowOk

WhileEnd:                                         ; preds = %WhileCond
  store i16 0, i16* %result, align 2
  ret i16 0

BoundsCheckFail:                                  ; preds = %WhileBody
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %WhileBody
  %20 = icmp ule i64 %11, %11
  br i1 %20, label %BoundsCheckOk2, label %BoundsCheckFail1

BoundsCheckFail1:                                 ; preds = %BoundsCheckOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk2:                                   ; preds = %BoundsCheckOk
  %21 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %22 = load i8*, i8** %21, align 8
  %23 = getelementptr inbounds i8, i8* %22, i64 %9
  %24 = sub nuw i64 %11, %9
  %25 = getelementptr inbounds %"[]u8", %"[]u8"* %2, i32 0, i32 0
  store i8* %23, i8** %25, align 8
  %26 = getelementptr inbounds %"[]u8", %"[]u8"* %2, i32 0, i32 1
  store i64 %24, i64* %26, align 8
  call fastcc void @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).write"({ i64, i16 }* sret({ i64, i16 }) %3, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"[]u8"* %2)
  %27 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %3, i32 0, i32 1
  %28 = load i16, i16* %27, align 2
  %29 = icmp ne i16 %28, 0
  br i1 %29, label %ErrRetReturn, label %ErrRetContinue

OverflowFail:                                     ; preds = %ErrRetContinue
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %ErrRetContinue
  store i64 %18, i64* %index, align 8
  br label %WhileCond
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.formatType(i64 %0, %std.fmt.FormatOptions* nonnull readonly align 8 %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %2, i64 %3) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %4 = alloca i64, align 8
  %5 = alloca %std.fmt.FormatOptions, align 8
  %6 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %value = alloca i64, align 8
  %max_depth = alloca i64, align 8
  store i64 %0, i64* %value, align 8
  store i64 %3, i64* %max_depth, align 8
  %7 = load i64, i64* %value, align 8
  store i64 %7, i64* %4, align 8
  %8 = bitcast %std.fmt.FormatOptions* %1 to i8*
  %9 = bitcast %std.fmt.FormatOptions* %5 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %9, i8* align 8 %8, i64 40, i1 false)
  %10 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2 to i8*
  %11 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %6 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %11, i8* align 4 %10, i64 4, i1 false)
  %12 = call fastcc i16 @std.fmt.formatValue(i64 %7, %std.fmt.FormatOptions* %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2)
  store i16 %12, i16* %result, align 2
  %13 = load i16, i16* %result, align 2
  ret i16 %13
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).write"({ i64, i16 }* nonnull sret({ i64, i16 }) %0, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %1, %"[]u8"* nonnull readonly align 8 %2) unnamed_addr #1 {
Entry:
  %3 = getelementptr inbounds %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %1, i32 0, i32 0
  call fastcc void @std.fs.file.File.write({ i64, i16 }* sret({ i64, i16 }) %0, %std.fs.file.File* %3, %"[]u8"* %2)
  ret void
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #8

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.fmt.Parser.specifier({ %std.fmt.Specifier, i16 }* nonnull sret({ %std.fmt.Specifier, i16 }) %0, %std.fmt.Parser* nonnull align 8 %1) unnamed_addr #1 {
Entry:
  %arg_name = alloca %"[]u8", align 8
  %2 = alloca %"?usize", align 8
  %i = alloca i64, align 8
  %self = alloca %std.fmt.Parser*, align 8
  store %std.fmt.Parser* %1, %std.fmt.Parser** %self, align 8
  %3 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %4 = call fastcc i1 @std.fmt.Parser.maybe(%std.fmt.Parser* %3, i8 91)
  br i1 %4, label %Then, label %Else2

Then:                                             ; preds = %Entry
  %5 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  call fastcc void @std.fmt.Parser.until(%"[]u8"* sret(%"[]u8") %arg_name, %std.fmt.Parser* %5, i8 93)
  %6 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %7 = call fastcc i1 @std.fmt.Parser.maybe(%std.fmt.Parser* %6, i8 93)
  %8 = icmp eq i1 %7, false
  br i1 %8, label %Then1, label %Else

Then1:                                            ; preds = %Then
  %9 = getelementptr inbounds { %std.fmt.Specifier, i16 }, { %std.fmt.Specifier, i16 }* %0, i32 0, i32 1
  store i16 13, i16* %9, align 2
  ret void

Else:                                             ; preds = %Then
  br label %EndIf

EndIf:                                            ; preds = %Else
  %10 = getelementptr inbounds { %std.fmt.Specifier, i16 }, { %std.fmt.Specifier, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %10, align 2
  %11 = getelementptr inbounds { %std.fmt.Specifier, i16 }, { %std.fmt.Specifier, i16 }* %0, i32 0, i32 0
  %12 = getelementptr inbounds %std.fmt.Specifier, %std.fmt.Specifier* %11, i32 0, i32 1
  store i2 -2, i2* %12, align 1
  %13 = getelementptr inbounds %std.fmt.Specifier, %std.fmt.Specifier* %11, i32 0, i32 0
  %14 = bitcast { i64, [8 x i8] }* %13 to %"[]u8"*
  %15 = bitcast %"[]u8"* %arg_name to i8*
  %16 = bitcast %"[]u8"* %14 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %16, i8* align 8 %15, i64 16, i1 false)
  ret void

Else2:                                            ; preds = %Entry
  br label %EndIf3

EndIf3:                                           ; preds = %Else2
  %17 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  call fastcc void @std.fmt.Parser.number(%"?usize"* sret(%"?usize") %2, %std.fmt.Parser* %17)
  %18 = getelementptr inbounds %"?usize", %"?usize"* %2, i32 0, i32 1
  %19 = load i1, i1* %18, align 1
  br i1 %19, label %OptionalThen, label %OptionalElse

OptionalThen:                                     ; preds = %EndIf3
  %20 = getelementptr inbounds %"?usize", %"?usize"* %2, i32 0, i32 0
  %21 = load i64, i64* %20, align 8
  store i64 %21, i64* %i, align 8
  %22 = getelementptr inbounds { %std.fmt.Specifier, i16 }, { %std.fmt.Specifier, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %22, align 2
  %23 = getelementptr inbounds { %std.fmt.Specifier, i16 }, { %std.fmt.Specifier, i16 }* %0, i32 0, i32 0
  %24 = getelementptr inbounds %std.fmt.Specifier, %std.fmt.Specifier* %23, i32 0, i32 1
  store i2 1, i2* %24, align 1
  %25 = getelementptr inbounds %std.fmt.Specifier, %std.fmt.Specifier* %23, i32 0, i32 0
  %26 = bitcast { i64, [8 x i8] }* %25 to i64*
  %27 = load i64, i64* %i, align 8
  store i64 %27, i64* %26, align 8
  ret void

OptionalElse:                                     ; preds = %EndIf3
  br label %OptionalEndIf

OptionalEndIf:                                    ; preds = %OptionalElse
  %28 = getelementptr inbounds { %std.fmt.Specifier, i16 }, { %std.fmt.Specifier, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %28, align 2
  %29 = getelementptr inbounds { %std.fmt.Specifier, i16 }, { %std.fmt.Specifier, i16 }* %0, i32 0, i32 0
  %30 = getelementptr inbounds %std.fmt.Specifier, %std.fmt.Specifier* %29, i32 0, i32 1
  store i2 0, i2* %30, align 1
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i1 @std.fmt.Parser.maybe(%std.fmt.Parser* nonnull align 8 %0, i8 %1) unnamed_addr #1 {
Entry:
  %result = alloca i1, align 1
  %self = alloca %std.fmt.Parser*, align 8
  %val = alloca i8, align 1
  store %std.fmt.Parser* %0, %std.fmt.Parser** %self, align 8
  store i8 %1, i8* %val, align 1
  %2 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %3 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %2, i32 0, i32 1
  %4 = load i64, i64* %3, align 8
  %5 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %6 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %5, i32 0, i32 0
  %7 = getelementptr inbounds %"[]u8", %"[]u8"* %6, i32 0, i32 1
  %8 = load i64, i64* %7, align 8
  %9 = icmp ult i64 %4, %8
  br i1 %9, label %BoolAndTrue, label %BoolAndFalse

BoolAndTrue:                                      ; preds = %Entry
  %10 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %11 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %10, i32 0, i32 0
  %12 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %13 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %12, i32 0, i32 1
  %14 = load i64, i64* %13, align 8
  %15 = getelementptr inbounds %"[]u8", %"[]u8"* %11, i32 0, i32 1
  %16 = load i64, i64* %15, align 8
  %17 = icmp ult i64 %14, %16
  br i1 %17, label %BoundsCheckOk, label %BoundsCheckFail

BoolAndFalse:                                     ; preds = %BoundsCheckOk, %Entry
  %18 = phi i1 [ %9, %Entry ], [ %31, %BoundsCheckOk ]
  br i1 %18, label %Then, label %Else

Then:                                             ; preds = %BoolAndFalse
  %19 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %20 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %19, i32 0, i32 1
  %21 = load i64, i64* %20, align 8
  %22 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %21, i64 1)
  %23 = extractvalue { i64, i1 } %22, 0
  %24 = extractvalue { i64, i1 } %22, 1
  br i1 %24, label %OverflowFail, label %OverflowOk

Else:                                             ; preds = %BoolAndFalse
  br label %EndIf

EndIf:                                            ; preds = %Else
  store i1 false, i1* %result, align 1
  %25 = load i1, i1* %result, align 1
  ret i1 %25

BoundsCheckFail:                                  ; preds = %BoolAndTrue
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %BoolAndTrue
  %26 = getelementptr inbounds %"[]u8", %"[]u8"* %11, i32 0, i32 0
  %27 = load i8*, i8** %26, align 8
  %28 = getelementptr inbounds i8, i8* %27, i64 %14
  %29 = load i8, i8* %28, align 1
  %30 = load i8, i8* %val, align 1
  %31 = icmp eq i8 %29, %30
  br label %BoolAndFalse

OverflowFail:                                     ; preds = %Then
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %Then
  store i64 %23, i64* %20, align 8
  store i1 true, i1* %result, align 1
  %32 = load i1, i1* %result, align 1
  ret i1 %32
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.fmt.Parser.until(%"[]u8"* nonnull sret(%"[]u8") %0, %std.fmt.Parser* nonnull align 8 %1, i8 %2) unnamed_addr #1 {
Entry:
  %start = alloca i64, align 8
  %self = alloca %std.fmt.Parser*, align 8
  %ch = alloca i8, align 1
  store %std.fmt.Parser* %1, %std.fmt.Parser** %self, align 8
  store i8 %2, i8* %ch, align 1
  %3 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %4 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %3, i32 0, i32 1
  %5 = load i64, i64* %4, align 8
  store i64 %5, i64* %start, align 8
  %6 = load i64, i64* %start, align 8
  %7 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %8 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %7, i32 0, i32 0
  %9 = getelementptr inbounds %"[]u8", %"[]u8"* %8, i32 0, i32 1
  %10 = load i64, i64* %9, align 8
  %11 = icmp uge i64 %6, %10
  br i1 %11, label %Then, label %Else

Then:                                             ; preds = %Entry
  %12 = bitcast %"[]u8"* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %12, i8* align 8 bitcast (%"[]u8"* @27 to i8*), i64 16, i1 false)
  ret void

Else:                                             ; preds = %Entry
  br label %EndIf

EndIf:                                            ; preds = %Else
  br label %WhileCond

WhileCond:                                        ; preds = %OverflowOk, %EndIf
  %13 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %14 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %13, i32 0, i32 1
  %15 = load i64, i64* %14, align 8
  %16 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %17 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %16, i32 0, i32 0
  %18 = getelementptr inbounds %"[]u8", %"[]u8"* %17, i32 0, i32 1
  %19 = load i64, i64* %18, align 8
  %20 = icmp ult i64 %15, %19
  br i1 %20, label %WhileBody, label %WhileEnd

WhileBody:                                        ; preds = %WhileCond
  %21 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %22 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %21, i32 0, i32 0
  %23 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %24 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %23, i32 0, i32 1
  %25 = load i64, i64* %24, align 8
  %26 = getelementptr inbounds %"[]u8", %"[]u8"* %22, i32 0, i32 1
  %27 = load i64, i64* %26, align 8
  %28 = icmp ult i64 %25, %27
  br i1 %28, label %BoundsCheckOk, label %BoundsCheckFail

Then1:                                            ; preds = %BoundsCheckOk
  br label %WhileEnd

Else2:                                            ; preds = %BoundsCheckOk
  br label %EndIf3

EndIf3:                                           ; preds = %Else2
  %29 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %30 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %29, i32 0, i32 1
  %31 = load i64, i64* %30, align 8
  %32 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %31, i64 1)
  %33 = extractvalue { i64, i1 } %32, 0
  %34 = extractvalue { i64, i1 } %32, 1
  br i1 %34, label %OverflowFail, label %OverflowOk

WhileEnd:                                         ; preds = %Then1, %WhileCond
  %35 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %36 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %35, i32 0, i32 0
  %37 = load i64, i64* %start, align 8
  %38 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %39 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %38, i32 0, i32 1
  %40 = load i64, i64* %39, align 8
  %41 = getelementptr inbounds %"[]u8", %"[]u8"* %36, i32 0, i32 1
  %42 = load i64, i64* %41, align 8
  %43 = icmp ule i64 %37, %40
  br i1 %43, label %BoundsCheckOk5, label %BoundsCheckFail4

BoundsCheckFail:                                  ; preds = %WhileBody
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %WhileBody
  %44 = getelementptr inbounds %"[]u8", %"[]u8"* %22, i32 0, i32 0
  %45 = load i8*, i8** %44, align 8
  %46 = getelementptr inbounds i8, i8* %45, i64 %25
  %47 = load i8, i8* %46, align 1
  %48 = load i8, i8* %ch, align 1
  %49 = icmp eq i8 %47, %48
  br i1 %49, label %Then1, label %Else2

OverflowFail:                                     ; preds = %EndIf3
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %EndIf3
  store i64 %33, i64* %30, align 8
  br label %WhileCond

BoundsCheckFail4:                                 ; preds = %WhileEnd
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk5:                                   ; preds = %WhileEnd
  %50 = icmp ule i64 %40, %42
  br i1 %50, label %BoundsCheckOk7, label %BoundsCheckFail6

BoundsCheckFail6:                                 ; preds = %BoundsCheckOk5
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk7:                                   ; preds = %BoundsCheckOk5
  %51 = getelementptr inbounds %"[]u8", %"[]u8"* %36, i32 0, i32 0
  %52 = load i8*, i8** %51, align 8
  %53 = getelementptr inbounds i8, i8* %52, i64 %37
  %54 = sub nuw i64 %40, %37
  %55 = getelementptr inbounds %"[]u8", %"[]u8"* %0, i32 0, i32 0
  store i8* %53, i8** %55, align 8
  %56 = getelementptr inbounds %"[]u8", %"[]u8"* %0, i32 0, i32 1
  store i64 %54, i64* %56, align 8
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.fmt.Parser.number(%"?usize"* nonnull sret(%"?usize") %0, %std.fmt.Parser* nonnull align 8 %1) unnamed_addr #1 {
Entry:
  %r = alloca %"?usize", align 8
  %self = alloca %std.fmt.Parser*, align 8
  store %std.fmt.Parser* %1, %std.fmt.Parser** %self, align 8
  %2 = bitcast %"?usize"* %r to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %2, i8* align 8 bitcast (%"?usize"* @24 to i8*), i64 16, i1 false)
  br label %WhileCond

WhileCond:                                        ; preds = %OverflowOk10, %Entry
  %3 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %4 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %3, i32 0, i32 1
  %5 = load i64, i64* %4, align 8
  %6 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %7 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %6, i32 0, i32 0
  %8 = getelementptr inbounds %"[]u8", %"[]u8"* %7, i32 0, i32 1
  %9 = load i64, i64* %8, align 8
  %10 = icmp ult i64 %5, %9
  br i1 %10, label %WhileBody, label %WhileEnd

WhileBody:                                        ; preds = %WhileCond
  %11 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %12 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %11, i32 0, i32 0
  %13 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %14 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %13, i32 0, i32 1
  %15 = load i64, i64* %14, align 8
  %16 = getelementptr inbounds %"[]u8", %"[]u8"* %12, i32 0, i32 1
  %17 = load i64, i64* %16, align 8
  %18 = icmp ult i64 %15, %17
  br i1 %18, label %BoundsCheckOk, label %BoundsCheckFail

SwitchRangeYes:                                   ; preds = %BoundsCheckOk
  %19 = getelementptr inbounds %"?usize", %"?usize"* %r, i32 0, i32 1
  %20 = load i1, i1* %19, align 1
  %21 = icmp eq i1 %20, false
  br i1 %21, label %Then, label %Else

Then:                                             ; preds = %SwitchRangeYes
  %22 = getelementptr inbounds %"?usize", %"?usize"* %r, i32 0, i32 1
  store i1 true, i1* %22, align 1
  %23 = getelementptr inbounds %"?usize", %"?usize"* %r, i32 0, i32 0
  store i64 0, i64* %23, align 8
  br label %EndIf

Else:                                             ; preds = %SwitchRangeYes
  br label %EndIf

SwitchElse:                                       ; preds = %SwitchRangeNo
  br label %WhileEnd

EndIf:                                            ; preds = %Else, %Then
  %24 = getelementptr inbounds %"?usize", %"?usize"* %r, i32 0, i32 1
  %25 = load i1, i1* %24, align 1
  br i1 %25, label %UnwrapOptionalOk, label %UnwrapOptionalFail

SwitchEnd:                                        ; preds = %OverflowOk8
  %26 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %27 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %26, i32 0, i32 1
  %28 = load i64, i64* %27, align 8
  %29 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %28, i64 1)
  %30 = extractvalue { i64, i1 } %29, 0
  %31 = extractvalue { i64, i1 } %29, 1
  br i1 %31, label %OverflowFail9, label %OverflowOk10

WhileEnd:                                         ; preds = %SwitchElse, %WhileCond
  %32 = bitcast %"?usize"* %r to i8*
  %33 = bitcast %"?usize"* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %33, i8* align 8 %32, i64 16, i1 false)
  ret void

SwitchRangeNo:                                    ; preds = %BoundsCheckOk
  br label %SwitchElse

BoundsCheckFail:                                  ; preds = %WhileBody
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %WhileBody
  %34 = getelementptr inbounds %"[]u8", %"[]u8"* %12, i32 0, i32 0
  %35 = load i8*, i8** %34, align 8
  %36 = getelementptr inbounds i8, i8* %35, i64 %15
  %37 = load i8, i8* %36, align 1
  %38 = icmp uge i8 %37, 48
  %39 = icmp ule i8 %37, 57
  %40 = and i1 %38, %39
  br i1 %40, label %SwitchRangeYes, label %SwitchRangeNo

UnwrapOptionalFail:                               ; preds = %EndIf
  call fastcc void @std.builtin.default_panic(%"[]u8"* @26, %std.builtin.StackTrace* null)
  unreachable

UnwrapOptionalOk:                                 ; preds = %EndIf
  %41 = getelementptr inbounds %"?usize", %"?usize"* %r, i32 0, i32 0
  %42 = load i64, i64* %41, align 8
  %43 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %42, i64 10)
  %44 = extractvalue { i64, i1 } %43, 0
  %45 = extractvalue { i64, i1 } %43, 1
  br i1 %45, label %OverflowFail, label %OverflowOk

OverflowFail:                                     ; preds = %UnwrapOptionalOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %UnwrapOptionalOk
  store i64 %44, i64* %41, align 8
  %46 = getelementptr inbounds %"?usize", %"?usize"* %r, i32 0, i32 1
  %47 = load i1, i1* %46, align 1
  br i1 %47, label %UnwrapOptionalOk2, label %UnwrapOptionalFail1

UnwrapOptionalFail1:                              ; preds = %OverflowOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @26, %std.builtin.StackTrace* null)
  unreachable

UnwrapOptionalOk2:                                ; preds = %OverflowOk
  %48 = getelementptr inbounds %"?usize", %"?usize"* %r, i32 0, i32 0
  %49 = load i64, i64* %48, align 8
  %50 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %51 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %50, i32 0, i32 0
  %52 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %53 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %52, i32 0, i32 1
  %54 = load i64, i64* %53, align 8
  %55 = getelementptr inbounds %"[]u8", %"[]u8"* %51, i32 0, i32 1
  %56 = load i64, i64* %55, align 8
  %57 = icmp ult i64 %54, %56
  br i1 %57, label %BoundsCheckOk4, label %BoundsCheckFail3

BoundsCheckFail3:                                 ; preds = %UnwrapOptionalOk2
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk4:                                   ; preds = %UnwrapOptionalOk2
  %58 = getelementptr inbounds %"[]u8", %"[]u8"* %51, i32 0, i32 0
  %59 = load i8*, i8** %58, align 8
  %60 = getelementptr inbounds i8, i8* %59, i64 %54
  %61 = load i8, i8* %60, align 1
  %62 = call { i8, i1 } @llvm.usub.with.overflow.i8(i8 %61, i8 48)
  %63 = extractvalue { i8, i1 } %62, 0
  %64 = extractvalue { i8, i1 } %62, 1
  br i1 %64, label %OverflowFail5, label %OverflowOk6

OverflowFail5:                                    ; preds = %BoundsCheckOk4
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk6:                                      ; preds = %BoundsCheckOk4
  %65 = zext i8 %63 to i64
  %66 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %49, i64 %65)
  %67 = extractvalue { i64, i1 } %66, 0
  %68 = extractvalue { i64, i1 } %66, 1
  br i1 %68, label %OverflowFail7, label %OverflowOk8

OverflowFail7:                                    ; preds = %OverflowOk6
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk8:                                      ; preds = %OverflowOk6
  store i64 %67, i64* %48, align 8
  br label %SwitchEnd

OverflowFail9:                                    ; preds = %SwitchEnd
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk10:                                     ; preds = %SwitchEnd
  store i64 %30, i64* %27, align 8
  br label %WhileCond
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #8

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i8, i1 } @llvm.usub.with.overflow.i8(i8, i8) #8

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.fmt.Parser.char(%"?u8"* nonnull sret(%"?u8") %0, %std.fmt.Parser* nonnull align 8 %1) unnamed_addr #1 {
Entry:
  %ch = alloca i8, align 1
  %self = alloca %std.fmt.Parser*, align 8
  store %std.fmt.Parser* %1, %std.fmt.Parser** %self, align 8
  %2 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %3 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %2, i32 0, i32 1
  %4 = load i64, i64* %3, align 8
  %5 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %6 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %5, i32 0, i32 0
  %7 = getelementptr inbounds %"[]u8", %"[]u8"* %6, i32 0, i32 1
  %8 = load i64, i64* %7, align 8
  %9 = icmp ult i64 %4, %8
  br i1 %9, label %Then, label %Else

Then:                                             ; preds = %Entry
  %10 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %11 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %10, i32 0, i32 0
  %12 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %13 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %12, i32 0, i32 1
  %14 = load i64, i64* %13, align 8
  %15 = getelementptr inbounds %"[]u8", %"[]u8"* %11, i32 0, i32 1
  %16 = load i64, i64* %15, align 8
  %17 = icmp ult i64 %14, %16
  br i1 %17, label %BoundsCheckOk, label %BoundsCheckFail

Else:                                             ; preds = %Entry
  br label %EndIf

EndIf:                                            ; preds = %Else
  %18 = bitcast %"?u8"* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %18, i8* align 1 getelementptr inbounds (%"?u8", %"?u8"* @28, i32 0, i32 0), i64 2, i1 false)
  ret void

BoundsCheckFail:                                  ; preds = %Then
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %Then
  %19 = getelementptr inbounds %"[]u8", %"[]u8"* %11, i32 0, i32 0
  %20 = load i8*, i8** %19, align 8
  %21 = getelementptr inbounds i8, i8* %20, i64 %14
  %22 = load i8, i8* %21, align 1
  store i8 %22, i8* %ch, align 1
  %23 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %24 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %23, i32 0, i32 1
  %25 = load i64, i64* %24, align 8
  %26 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %25, i64 1)
  %27 = extractvalue { i64, i1 } %26, 0
  %28 = extractvalue { i64, i1 } %26, 1
  br i1 %28, label %OverflowFail, label %OverflowOk

OverflowFail:                                     ; preds = %BoundsCheckOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %BoundsCheckOk
  store i64 %27, i64* %24, align 8
  %29 = load i8, i8* %ch, align 1
  %30 = getelementptr inbounds %"?u8", %"?u8"* %0, i32 0, i32 1
  store i1 true, i1* %30, align 1
  %31 = getelementptr inbounds %"?u8", %"?u8"* %0, i32 0, i32 0
  store i8 %29, i8* %31, align 1
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.fmt.Parser.peek(%"?u8"* nonnull sret(%"?u8") %0, %std.fmt.Parser* nonnull align 8 %1, i64 %2) unnamed_addr #1 {
Entry:
  %3 = alloca %"?u8", align 1
  %self = alloca %std.fmt.Parser*, align 8
  %n = alloca i64, align 8
  store %std.fmt.Parser* %1, %std.fmt.Parser** %self, align 8
  store i64 %2, i64* %n, align 8
  %4 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %5 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %4, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = load i64, i64* %n, align 8
  %8 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %6, i64 %7)
  %9 = extractvalue { i64, i1 } %8, 0
  %10 = extractvalue { i64, i1 } %8, 1
  br i1 %10, label %OverflowFail, label %OverflowOk

Then:                                             ; preds = %OverflowOk
  %11 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %12 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %11, i32 0, i32 0
  %13 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %14 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %13, i32 0, i32 1
  %15 = load i64, i64* %14, align 8
  %16 = load i64, i64* %n, align 8
  %17 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %15, i64 %16)
  %18 = extractvalue { i64, i1 } %17, 0
  %19 = extractvalue { i64, i1 } %17, 1
  br i1 %19, label %OverflowFail1, label %OverflowOk2

Else:                                             ; preds = %OverflowOk
  %20 = bitcast %"?u8"* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %20, i8* align 1 getelementptr inbounds (%"?u8", %"?u8"* @29, i32 0, i32 0), i64 2, i1 false)
  br label %EndIf

EndIf:                                            ; preds = %Else, %BoundsCheckOk
  ret void

OverflowFail:                                     ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %Entry
  %21 = load %std.fmt.Parser*, %std.fmt.Parser** %self, align 8
  %22 = getelementptr inbounds %std.fmt.Parser, %std.fmt.Parser* %21, i32 0, i32 0
  %23 = getelementptr inbounds %"[]u8", %"[]u8"* %22, i32 0, i32 1
  %24 = load i64, i64* %23, align 8
  %25 = icmp ult i64 %9, %24
  br i1 %25, label %Then, label %Else

OverflowFail1:                                    ; preds = %Then
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk2:                                      ; preds = %Then
  %26 = getelementptr inbounds %"[]u8", %"[]u8"* %12, i32 0, i32 1
  %27 = load i64, i64* %26, align 8
  %28 = icmp ult i64 %18, %27
  br i1 %28, label %BoundsCheckOk, label %BoundsCheckFail

BoundsCheckFail:                                  ; preds = %OverflowOk2
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %OverflowOk2
  %29 = getelementptr inbounds %"[]u8", %"[]u8"* %12, i32 0, i32 0
  %30 = load i8*, i8** %29, align 8
  %31 = getelementptr inbounds i8, i8* %30, i64 %18
  %32 = load i8, i8* %31, align 1
  %33 = getelementptr inbounds %"?u8", %"?u8"* %0, i32 0, i32 1
  store i1 true, i1* %33, align 1
  %34 = getelementptr inbounds %"?u8", %"?u8"* %0, i32 0, i32 0
  store i8 %32, i8* %34, align 1
  %35 = getelementptr inbounds %"?u8", %"?u8"* %3, i32 0, i32 0
  store i8 %32, i8* %35, align 1
  %36 = getelementptr inbounds %"?u8", %"?u8"* %3, i32 0, i32 1
  store i1 true, i1* %36, align 1
  br label %EndIf
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.fmt.ArgState.nextArg(%"?usize"* nonnull sret(%"?usize") %0, %std.fmt.ArgState* nonnull align 8 %1, %"?usize"* nonnull readonly align 8 %2) unnamed_addr #1 {
Entry:
  %arg = alloca i64, align 8
  %next_index = alloca i64, align 8
  %self = alloca %std.fmt.ArgState*, align 8
  store %std.fmt.ArgState* %1, %std.fmt.ArgState** %self, align 8
  %3 = getelementptr inbounds %"?usize", %"?usize"* %2, i32 0, i32 1
  %4 = load i1, i1* %3, align 1
  br i1 %4, label %OptionalNonNull, label %OptionalNull

OptionalNull:                                     ; preds = %Entry
  %5 = load %std.fmt.ArgState*, %std.fmt.ArgState** %self, align 8
  %6 = getelementptr inbounds %std.fmt.ArgState, %std.fmt.ArgState* %5, i32 0, i32 0
  %7 = load i64, i64* %6, align 8
  store i64 %7, i64* %arg, align 8
  %8 = load %std.fmt.ArgState*, %std.fmt.ArgState** %self, align 8
  %9 = getelementptr inbounds %std.fmt.ArgState, %std.fmt.ArgState* %8, i32 0, i32 0
  %10 = load i64, i64* %9, align 8
  %11 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %10, i64 1)
  %12 = extractvalue { i64, i1 } %11, 0
  %13 = extractvalue { i64, i1 } %11, 1
  br i1 %13, label %OverflowFail, label %OverflowOk

BlockEnd:                                         ; preds = %OverflowOk
  store i64 %29, i64* %next_index, align 8
  br label %OptionalEnd

OptionalNonNull:                                  ; preds = %Entry
  %14 = getelementptr inbounds %"?usize", %"?usize"* %2, i32 0, i32 0
  %15 = load i64, i64* %14, align 8
  store i64 %15, i64* %next_index, align 8
  br label %OptionalEnd

OptionalEnd:                                      ; preds = %OptionalNonNull, %BlockEnd
  %16 = load i64, i64* %next_index, align 8
  %17 = load %std.fmt.ArgState*, %std.fmt.ArgState** %self, align 8
  %18 = getelementptr inbounds %std.fmt.ArgState, %std.fmt.ArgState* %17, i32 0, i32 2
  %19 = load i64, i64* %18, align 8
  %20 = icmp uge i64 %16, %19
  br i1 %20, label %Then, label %Else

Then:                                             ; preds = %OptionalEnd
  %21 = bitcast %"?usize"* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %21, i8* align 8 bitcast (%"?usize"* @30 to i8*), i64 16, i1 false)
  ret void

Else:                                             ; preds = %OptionalEnd
  br label %EndIf

EndIf:                                            ; preds = %Else
  %22 = load %std.fmt.ArgState*, %std.fmt.ArgState** %self, align 8
  %23 = getelementptr inbounds %std.fmt.ArgState, %std.fmt.ArgState* %22, i32 0, i32 1
  %24 = load i32, i32* %23, align 4
  %25 = load i64, i64* %next_index, align 8
  %26 = trunc i64 %25 to i5
  %27 = zext i5 %26 to i64
  %28 = icmp eq i64 %25, %27
  br i1 %28, label %CastShortenOk, label %CastShortenFail

OverflowFail:                                     ; preds = %OptionalNull
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %OptionalNull
  store i64 %12, i64* %9, align 8
  %29 = load i64, i64* %arg, align 8
  store i64 %29, i64* %next_index, align 8
  br label %BlockEnd

CastShortenOk:                                    ; preds = %EndIf
  %30 = zext i5 %26 to i32
  %31 = shl i32 1, %30
  %32 = or i32 %24, %31
  store i32 %32, i32* %23, align 4
  %33 = load i64, i64* %next_index, align 8
  %34 = getelementptr inbounds %"?usize", %"?usize"* %0, i32 0, i32 1
  store i1 true, i1* %34, align 1
  %35 = getelementptr inbounds %"?usize", %"?usize"* %0, i32 0, i32 0
  store i64 %33, i64* %35, align 8
  ret void

CastShortenFail:                                  ; preds = %EndIf
  call fastcc void @std.builtin.default_panic(%"[]u8"* @15, %std.builtin.StackTrace* null)
  unreachable
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.formatValue(i64 %0, %std.fmt.FormatOptions* nonnull readonly align 8 %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %2) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %3 = alloca i64, align 8
  %4 = alloca %std.fmt.FormatOptions, align 8
  %5 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %value = alloca i64, align 8
  store i64 %0, i64* %value, align 8
  %6 = load i64, i64* %value, align 8
  store i64 %6, i64* %3, align 8
  %7 = bitcast %std.fmt.FormatOptions* %1 to i8*
  %8 = bitcast %std.fmt.FormatOptions* %4 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %8, i8* align 8 %7, i64 40, i1 false)
  %9 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2 to i8*
  %10 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %5 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %10, i8* align 4 %9, i64 4, i1 false)
  %11 = call fastcc i16 @std.fmt.formatIntValue(i64 %6, %std.fmt.FormatOptions* %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2)
  store i16 %11, i16* %result, align 2
  %12 = load i16, i16* %result, align 2
  ret i16 %12
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i1 @std.fmt.ArgState.hasUnusedArgs(%std.fmt.ArgState* nonnull align 8 %0) unnamed_addr #1 {
Entry:
  %result = alloca i1, align 1
  %self = alloca %std.fmt.ArgState*, align 8
  store %std.fmt.ArgState* %0, %std.fmt.ArgState** %self, align 8
  %1 = load %std.fmt.ArgState*, %std.fmt.ArgState** %self, align 8
  %2 = getelementptr inbounds %std.fmt.ArgState, %std.fmt.ArgState* %1, i32 0, i32 1
  %3 = load i32, i32* %2, align 4
  %4 = call i32 @llvm.ctpop.i32(i32 %3)
  %5 = trunc i32 %4 to i6
  %6 = load %std.fmt.ArgState*, %std.fmt.ArgState** %self, align 8
  %7 = getelementptr inbounds %std.fmt.ArgState, %std.fmt.ArgState* %6, i32 0, i32 2
  %8 = load i64, i64* %7, align 8
  %9 = zext i6 %5 to i64
  %10 = icmp ne i64 %9, %8
  store i1 %10, i1* %result, align 1
  %11 = load i1, i1* %result, align 1
  ret i1 %11
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.ctpop.i32(i32) #8

; Function Attrs: argmemonly nofree nounwind willreturn writeonly
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #9

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.os.linux.sigprocmask(i32 %0, [32 x i32]* noalias align 4 %1, [32 x i32]* noalias align 4 %2) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %flags = alloca i32, align 4
  %set = alloca [32 x i32]*, align 8
  %oldset = alloca [32 x i32]*, align 8
  store i32 %0, i32* %flags, align 4
  store [32 x i32]* %1, [32 x i32]** %set, align 8
  store [32 x i32]* %2, [32 x i32]** %oldset, align 8
  %3 = load i32, i32* %flags, align 4
  %4 = zext i32 %3 to i64
  %5 = load [32 x i32]*, [32 x i32]** %set, align 8
  %6 = ptrtoint [32 x i32]* %5 to i64
  %7 = load [32 x i32]*, [32 x i32]** %oldset, align 8
  %8 = ptrtoint [32 x i32]* %7 to i64
  %9 = call fastcc i64 @std.os.linux.x86_64.syscall4(i64 14, i64 %4, i64 %6, i64 %8, i64 8)
  store i64 %9, i64* %result, align 8
  %10 = load i64, i64* %result, align 8
  ret i64 %10
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.os.linux.tkill(i32 %0, i32 %1) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %tid = alloca i32, align 4
  %sig = alloca i32, align 4
  store i32 %0, i32* %tid, align 4
  store i32 %1, i32* %sig, align 4
  %2 = load i32, i32* %tid, align 4
  %3 = sext i32 %2 to i64
  %4 = sext i32 %2 to i64
  %5 = load i32, i32* %sig, align 4
  %6 = sext i32 %5 to i64
  %7 = sext i32 %5 to i64
  %8 = call fastcc i64 @std.os.linux.x86_64.syscall2(i64 200, i64 %4, i64 %7)
  store i64 %8, i64* %result, align 8
  %9 = load i64, i64* %result, align 8
  ret i64 %9
}

; Function Attrs: nobuiltin noreturn nounwind
define internal fastcc void @std.os.linux.exit_group(i32 %0) unnamed_addr #6 {
Entry:
  %status = alloca i32, align 4
  store i32 %0, i32* %status, align 4
  %1 = load i32, i32* %status, align 4
  %2 = sext i32 %1 to i64
  %3 = sext i32 %1 to i64
  %4 = call fastcc i64 @std.os.linux.x86_64.syscall1(i64 231, i64 %3)
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.formatType.16(%"[]u8"* nonnull readonly align 8 %0, %std.fmt.FormatOptions* nonnull readonly align 8 %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %2, i64 %3) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %4 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %max_depth = alloca i64, align 8
  store i64 %3, i64* %max_depth, align 8
  %5 = load i64, i64* %max_depth, align 8
  %6 = icmp eq i64 %5, 0
  br i1 %6, label %Then, label %Else

Then:                                             ; preds = %Entry
  %7 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2, %"[]u8"* @40)
  store i16 %7, i16* %result, align 2
  %8 = load i16, i16* %result, align 2
  ret i16 %8

Else:                                             ; preds = %Entry
  br label %EndIf

EndIf:                                            ; preds = %Else
  %9 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2 to i8*
  %10 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %4 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %10, i8* align 4 %9, i64 4, i1 false)
  %11 = call fastcc i16 @std.fmt.formatBuf(%"[]u8"* %0, %std.fmt.FormatOptions* %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2)
  store i16 %11, i16* %result, align 2
  %12 = load i16, i16* %result, align 2
  ret i16 %12
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.formatBuf(%"[]u8"* nonnull readonly align 8 %0, %std.fmt.FormatOptions* nonnull readonly align 8 %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %2) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %min_width = alloca i64, align 8
  %3 = alloca { i64, i16 }, align 8
  %width = alloca i64, align 8
  %padding = alloca i64, align 8
  %4 = alloca i16, align 2
  %5 = alloca i16, align 2
  %left_padding = alloca i64, align 8
  %right_padding = alloca i64, align 8
  %6 = alloca i16, align 2
  %7 = alloca i16, align 2
  %8 = alloca i16, align 2
  %9 = alloca i16, align 2
  %10 = alloca i16, align 2
  %11 = alloca i16, align 2
  %12 = getelementptr inbounds %std.fmt.FormatOptions, %std.fmt.FormatOptions* %1, i32 0, i32 1
  %13 = getelementptr inbounds %"?usize", %"?usize"* %12, i32 0, i32 1
  %14 = load i1, i1* %13, align 1
  br i1 %14, label %OptionalThen, label %OptionalElse

OptionalThen:                                     ; preds = %Entry
  %15 = getelementptr inbounds %"?usize", %"?usize"* %12, i32 0, i32 0
  %16 = load i64, i64* %15, align 8
  store i64 %16, i64* %min_width, align 8
  call fastcc void @std.unicode.utf8CountCodepoints({ i64, i16 }* sret({ i64, i16 }) %3, %"[]u8"* %0)
  %17 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %3, i32 0, i32 1
  %18 = load i16, i16* %17, align 2
  %19 = icmp ne i16 %18, 0
  br i1 %19, label %UnwrapErrError, label %UnwrapErrOk

UnwrapErrError:                                   ; preds = %OptionalThen
  %20 = getelementptr inbounds %"[]u8", %"[]u8"* %0, i32 0, i32 1
  %21 = load i64, i64* %20, align 8
  store i64 %21, i64* %width, align 8
  br label %UnwrapErrEnd

UnwrapErrOk:                                      ; preds = %OptionalThen
  %22 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %3, i32 0, i32 0
  %23 = load i64, i64* %22, align 8
  store i64 %23, i64* %width, align 8
  br label %UnwrapErrEnd

UnwrapErrEnd:                                     ; preds = %UnwrapErrOk, %UnwrapErrError
  %24 = load i64, i64* %width, align 8
  %25 = load i64, i64* %min_width, align 8
  %26 = icmp ult i64 %24, %25
  br i1 %26, label %Then, label %Else

Then:                                             ; preds = %UnwrapErrEnd
  %27 = load i64, i64* %min_width, align 8
  %28 = load i64, i64* %width, align 8
  %29 = call { i64, i1 } @llvm.usub.with.overflow.i64(i64 %27, i64 %28)
  %30 = extractvalue { i64, i1 } %29, 0
  %31 = extractvalue { i64, i1 } %29, 1
  br i1 %31, label %OverflowFail, label %OverflowOk

Else:                                             ; preds = %UnwrapErrEnd
  store i64 0, i64* %padding, align 8
  br label %EndIf

EndIf:                                            ; preds = %Else, %OverflowOk
  %32 = load i64, i64* %padding, align 8
  %33 = icmp eq i64 %32, 0
  br i1 %33, label %Then1, label %Else2

Then1:                                            ; preds = %EndIf
  %34 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2, %"[]u8"* %0)
  store i16 %34, i16* %result, align 2
  %35 = load i16, i16* %result, align 2
  ret i16 %35

Else2:                                            ; preds = %EndIf
  br label %EndIf3

EndIf3:                                           ; preds = %Else2
  %36 = getelementptr inbounds %std.fmt.FormatOptions, %std.fmt.FormatOptions* %1, i32 0, i32 2
  %37 = load i2, i2* %36, align 1
  switch i2 %37, label %SwitchElse [
    i2 0, label %SwitchProng
    i2 1, label %SwitchProng5
    i2 -2, label %SwitchProng11
  ]

SwitchProng:                                      ; preds = %EndIf3
  %38 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2, %"[]u8"* %0)
  store i16 %38, i16* %4, align 2
  %39 = icmp ne i16 %38, 0
  br i1 %39, label %ErrRetReturn, label %ErrRetContinue

ErrRetReturn:                                     ; preds = %SwitchProng
  %40 = load i16, i16* %4, align 2
  store i16 %40, i16* %result, align 2
  ret i16 %40

ErrRetContinue:                                   ; preds = %SwitchProng
  %41 = getelementptr inbounds %std.fmt.FormatOptions, %std.fmt.FormatOptions* %1, i32 0, i32 3
  %42 = load i8, i8* %41, align 1
  %43 = load i64, i64* %padding, align 8
  %44 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeByteNTimes"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2, i8 %42, i64 %43)
  store i16 %44, i16* %5, align 2
  %45 = icmp ne i16 %44, 0
  br i1 %45, label %ErrRetReturn4, label %ErrRetContinue15

ErrRetReturn4:                                    ; preds = %ErrRetContinue
  %46 = load i16, i16* %5, align 2
  store i16 %46, i16* %result, align 2
  ret i16 %46

SwitchProng5:                                     ; preds = %EndIf3
  %47 = load i64, i64* %padding, align 8
  br i1 false, label %DivZeroFail, label %DivZeroOk

ErrRetReturn6:                                    ; preds = %DivZeroOk23
  %48 = load i16, i16* %6, align 2
  store i16 %48, i16* %result, align 2
  ret i16 %48

ErrRetContinue7:                                  ; preds = %DivZeroOk23
  %49 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2, %"[]u8"* %0)
  store i16 %49, i16* %7, align 2
  %50 = icmp ne i16 %49, 0
  br i1 %50, label %ErrRetReturn8, label %ErrRetContinue9

ErrRetReturn8:                                    ; preds = %ErrRetContinue7
  %51 = load i16, i16* %7, align 2
  store i16 %51, i16* %result, align 2
  ret i16 %51

ErrRetContinue9:                                  ; preds = %ErrRetContinue7
  %52 = getelementptr inbounds %std.fmt.FormatOptions, %std.fmt.FormatOptions* %1, i32 0, i32 3
  %53 = load i8, i8* %52, align 1
  %54 = load i64, i64* %right_padding, align 8
  %55 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeByteNTimes"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2, i8 %53, i64 %54)
  store i16 %55, i16* %8, align 2
  %56 = icmp ne i16 %55, 0
  br i1 %56, label %ErrRetReturn10, label %ErrRetContinue16

ErrRetReturn10:                                   ; preds = %ErrRetContinue9
  %57 = load i16, i16* %8, align 2
  store i16 %57, i16* %result, align 2
  ret i16 %57

SwitchProng11:                                    ; preds = %EndIf3
  %58 = getelementptr inbounds %std.fmt.FormatOptions, %std.fmt.FormatOptions* %1, i32 0, i32 3
  %59 = load i8, i8* %58, align 1
  %60 = load i64, i64* %padding, align 8
  %61 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeByteNTimes"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2, i8 %59, i64 %60)
  store i16 %61, i16* %9, align 2
  %62 = icmp ne i16 %61, 0
  br i1 %62, label %ErrRetReturn12, label %ErrRetContinue13

ErrRetReturn12:                                   ; preds = %SwitchProng11
  %63 = load i16, i16* %9, align 2
  store i16 %63, i16* %result, align 2
  ret i16 %63

ErrRetContinue13:                                 ; preds = %SwitchProng11
  %64 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2, %"[]u8"* %0)
  store i16 %64, i16* %10, align 2
  %65 = icmp ne i16 %64, 0
  br i1 %65, label %ErrRetReturn14, label %ErrRetContinue17

ErrRetReturn14:                                   ; preds = %ErrRetContinue13
  %66 = load i16, i16* %10, align 2
  store i16 %66, i16* %result, align 2
  ret i16 %66

SwitchElse:                                       ; preds = %EndIf3
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

ErrRetContinue15:                                 ; preds = %ErrRetContinue
  br label %SwitchEnd

ErrRetContinue16:                                 ; preds = %ErrRetContinue9
  br label %SwitchEnd

ErrRetContinue17:                                 ; preds = %ErrRetContinue13
  br label %SwitchEnd

OptionalElse:                                     ; preds = %Entry
  %67 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2, %"[]u8"* %0)
  store i16 %67, i16* %11, align 2
  %68 = icmp ne i16 %67, 0
  br i1 %68, label %ErrRetReturn18, label %ErrRetContinue19

ErrRetReturn18:                                   ; preds = %OptionalElse
  %69 = load i16, i16* %11, align 2
  store i16 %69, i16* %result, align 2
  ret i16 %69

SwitchEnd:                                        ; preds = %ErrRetContinue17, %ErrRetContinue16, %ErrRetContinue15
  br label %OptionalEndIf

ErrRetContinue19:                                 ; preds = %OptionalElse
  br label %OptionalEndIf

OptionalEndIf:                                    ; preds = %ErrRetContinue19, %SwitchEnd
  store i16 0, i16* %result, align 2
  ret i16 0

OverflowFail:                                     ; preds = %Then
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %Then
  store i64 %30, i64* %padding, align 8
  br label %EndIf

DivZeroFail:                                      ; preds = %SwitchProng5
  call fastcc void @std.builtin.default_panic(%"[]u8"* @11, %std.builtin.StackTrace* null)
  unreachable

DivZeroOk:                                        ; preds = %SwitchProng5
  %70 = udiv i64 %47, 2
  store i64 %70, i64* %left_padding, align 8
  %71 = load i64, i64* %padding, align 8
  %72 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %71, i64 1)
  %73 = extractvalue { i64, i1 } %72, 0
  %74 = extractvalue { i64, i1 } %72, 1
  br i1 %74, label %OverflowFail20, label %OverflowOk21

OverflowFail20:                                   ; preds = %DivZeroOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk21:                                     ; preds = %DivZeroOk
  br i1 false, label %DivZeroFail22, label %DivZeroOk23

DivZeroFail22:                                    ; preds = %OverflowOk21
  call fastcc void @std.builtin.default_panic(%"[]u8"* @11, %std.builtin.StackTrace* null)
  unreachable

DivZeroOk23:                                      ; preds = %OverflowOk21
  %75 = udiv i64 %73, 2
  store i64 %75, i64* %right_padding, align 8
  %76 = getelementptr inbounds %std.fmt.FormatOptions, %std.fmt.FormatOptions* %1, i32 0, i32 3
  %77 = load i8, i8* %76, align 1
  %78 = load i64, i64* %left_padding, align 8
  %79 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeByteNTimes"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2, i8 %77, i64 %78)
  store i16 %79, i16* %6, align 2
  %80 = icmp ne i16 %79, 0
  br i1 %80, label %ErrRetReturn6, label %ErrRetContinue7
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.format.17(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %1 = alloca i16, align 2
  %2 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"[]u8"* @50)
  store i16 %2, i16* %1, align 2
  %3 = icmp ne i16 %2, 0
  br i1 %3, label %ErrRetReturn, label %ErrRetContinue

ErrRetReturn:                                     ; preds = %Entry
  %4 = load i16, i16* %1, align 2
  store i16 %4, i16* %result, align 2
  ret i16 %4

ErrRetContinue:                                   ; preds = %Entry
  store i16 0, i16* %result, align 2
  ret i16 0
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.format.18(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %1 = alloca i16, align 2
  %2 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"[]u8"* @51)
  store i16 %2, i16* %1, align 2
  %3 = icmp ne i16 %2, 0
  br i1 %3, label %ErrRetReturn, label %ErrRetContinue

ErrRetReturn:                                     ; preds = %Entry
  %4 = load i16, i16* %1, align 2
  store i16 %4, i16* %result, align 2
  ret i16 %4

ErrRetContinue:                                   ; preds = %Entry
  store i16 0, i16* %result, align 2
  ret i16 0
}

; Function Attrs: cold nobuiltin nounwind
define internal fastcc void @std.Thread.Mutex.AtomicMutex.unlockSlow(%std.Thread.Mutex.AtomicMutex* nonnull align 4 %0) unnamed_addr #7 {
Entry:
  %1 = alloca i16, align 2
  %m = alloca %std.Thread.Mutex.AtomicMutex*, align 8
  store %std.Thread.Mutex.AtomicMutex* %0, %std.Thread.Mutex.AtomicMutex** %m, align 8
  %2 = load %std.Thread.Mutex.AtomicMutex*, %std.Thread.Mutex.AtomicMutex** %m, align 8
  %3 = getelementptr inbounds %std.Thread.Mutex.AtomicMutex, %std.Thread.Mutex.AtomicMutex* %2, i32 0, i32 0
  %4 = icmp ne i32* %3, null
  br i1 %4, label %PtrCastOk, label %PtrCastFail

SwitchElse:                                       ; preds = %PtrCastOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng:                                      ; preds = %PtrCastOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng1:                                     ; preds = %PtrCastOk
  br label %SwitchEnd

SwitchEnd:                                        ; preds = %SwitchProng1
  ret void

PtrCastFail:                                      ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @47, %std.builtin.StackTrace* null)
  unreachable

PtrCastOk:                                        ; preds = %Entry
  %5 = call fastcc i64 @std.os.linux.futex_wake(i32* %3, i32 129, i32 1)
  %6 = call fastcc i16 @std.os.linux.getErrno(i64 %5)
  store i16 %6, i16* %1, align 2
  switch i16 %6, label %SwitchElse [
    i16 0, label %SwitchProng1
    i16 14, label %SwitchProng
  ]
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i1 @std.Thread.StaticResetEvent.AtomicEvent.timedWait(%std.Thread.StaticResetEvent.AtomicEvent* nonnull align 4 %0, %"?u64"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %result = alloca i1, align 1
  %waiters = alloca i32, align 4
  %2 = alloca %"?u32", align 4
  %3 = alloca i16, align 2
  %_ = alloca i16, align 2
  %ev = alloca %std.Thread.StaticResetEvent.AtomicEvent*, align 8
  store %std.Thread.StaticResetEvent.AtomicEvent* %0, %std.Thread.StaticResetEvent.AtomicEvent** %ev, align 8
  %4 = load %std.Thread.StaticResetEvent.AtomicEvent*, %std.Thread.StaticResetEvent.AtomicEvent** %ev, align 8
  %5 = getelementptr inbounds %std.Thread.StaticResetEvent.AtomicEvent, %std.Thread.StaticResetEvent.AtomicEvent* %4, i32 0, i32 0
  %6 = load atomic i32, i32* %5 acquire, align 4
  store i32 %6, i32* %waiters, align 4
  br label %WhileCond

WhileCond:                                        ; preds = %OptionalEnd, %Entry
  %7 = load i32, i32* %waiters, align 4
  %8 = icmp ne i32 %7, 1
  br i1 %8, label %WhileBody, label %WhileEnd

WhileBody:                                        ; preds = %WhileCond
  %9 = load %std.Thread.StaticResetEvent.AtomicEvent*, %std.Thread.StaticResetEvent.AtomicEvent** %ev, align 8
  %10 = getelementptr inbounds %std.Thread.StaticResetEvent.AtomicEvent, %std.Thread.StaticResetEvent.AtomicEvent* %9, i32 0, i32 0
  %11 = load i32, i32* %waiters, align 4
  %12 = load i32, i32* %waiters, align 4
  %13 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %12, i32 2)
  %14 = extractvalue { i32, i1 } %13, 0
  %15 = extractvalue { i32, i1 } %13, 1
  br i1 %15, label %OverflowFail, label %OverflowOk

OptionalNull:                                     ; preds = %OverflowOk
  %16 = load %std.Thread.StaticResetEvent.AtomicEvent*, %std.Thread.StaticResetEvent.AtomicEvent** %ev, align 8
  %17 = getelementptr inbounds %std.Thread.StaticResetEvent.AtomicEvent, %std.Thread.StaticResetEvent.AtomicEvent* %16, i32 0, i32 0
  %18 = call fastcc i16 @std.Thread.StaticResetEvent.LinuxFutex.wait(i32* %17, %"?u64"* %1)
  store i16 %18, i16* %3, align 2
  %19 = icmp ne i16 %18, 0
  br i1 %19, label %TryElse, label %TryOk

TryOk:                                            ; preds = %OptionalNull
  store i1 false, i1* %result, align 1
  %20 = load i1, i1* %result, align 1
  ret i1 %20

TryElse:                                          ; preds = %OptionalNull
  %21 = load i16, i16* %3, align 2
  store i16 %21, i16* %_, align 2
  store i1 true, i1* %result, align 1
  %22 = load i1, i1* %result, align 1
  ret i1 %22

OptionalNonNull:                                  ; preds = %OverflowOk
  %23 = getelementptr inbounds %"?u32", %"?u32"* %2, i32 0, i32 0
  %24 = load i32, i32* %23, align 4
  store i32 %24, i32* %waiters, align 4
  br label %OptionalEnd

OptionalEnd:                                      ; preds = %OptionalNonNull
  br label %WhileCond

WhileEnd:                                         ; preds = %WhileCond
  store i1 false, i1* %result, align 1
  %25 = load i1, i1* %result, align 1
  ret i1 %25

OverflowFail:                                     ; preds = %WhileBody
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %WhileBody
  %26 = cmpxchg weak i32* %10, i32 %11, i32 %14 acquire acquire, align 4
  %27 = extractvalue { i32, i1 } %26, 0
  %28 = getelementptr inbounds %"?u32", %"?u32"* %2, i32 0, i32 0
  store i32 %27, i32* %28, align 4
  %29 = extractvalue { i32, i1 } %26, 1
  %30 = xor i1 %29, true
  %31 = getelementptr inbounds %"?u32", %"?u32"* %2, i32 0, i32 1
  store i1 %30, i1* %31, align 1
  %32 = getelementptr inbounds %"?u32", %"?u32"* %2, i32 0, i32 1
  %33 = load i1, i1* %32, align 1
  br i1 %33, label %OptionalNonNull, label %OptionalNull
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i64, i1 } @llvm.ssub.with.overflow.i64(i64, i64) #8

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i8, i1 } @llvm.uadd.with.overflow.i8(i8, i8) #8

; Function Attrs: nobuiltin nounwind
define internal fastcc i8 @std.math.min(i8 %0) unnamed_addr #1 {
Entry:
  %result = alloca i8, align 1
  %y = alloca i8, align 1
  store i8 %0, i8* %y, align 1
  %1 = load i8, i8* %y, align 1
  %2 = icmp ult i8 32, %1
  br i1 %2, label %Then, label %Else

Then:                                             ; preds = %Entry
  store i8 32, i8* %result, align 1
  %3 = load i8, i8* %result, align 1
  ret i8 %3

Else:                                             ; preds = %Entry
  %4 = load i8, i8* %y, align 1
  store i8 %4, i8* %result, align 1
  %5 = load i8, i8* %result, align 1
  ret i8 %5
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.os.linux.futex_wait(i32* nonnull readonly align 4 %0, i32 %1, i32 %2, %std.os.linux.timespec* align 8 %3) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %uaddr = alloca i32*, align 8
  %futex_op = alloca i32, align 4
  %val = alloca i32, align 4
  %timeout = alloca %std.os.linux.timespec*, align 8
  store i32* %0, i32** %uaddr, align 8
  store i32 %1, i32* %futex_op, align 4
  store i32 %2, i32* %val, align 4
  store %std.os.linux.timespec* %3, %std.os.linux.timespec** %timeout, align 8
  %4 = load i32*, i32** %uaddr, align 8
  %5 = ptrtoint i32* %4 to i64
  %6 = load i32, i32* %futex_op, align 4
  %7 = zext i32 %6 to i64
  %8 = load i32, i32* %val, align 4
  %9 = zext i32 %8 to i64
  %10 = load %std.os.linux.timespec*, %std.os.linux.timespec** %timeout, align 8
  %11 = ptrtoint %std.os.linux.timespec* %10 to i64
  %12 = call fastcc i64 @std.os.linux.x86_64.syscall4(i64 202, i64 %5, i64 %7, i64 %9, i64 %11)
  store i64 %12, i64* %result, align 8
  %13 = load i64, i64* %result, align 8
  ret i64 %13
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.debug.print(%"std.os.struct:4985:53"* nonnull readonly align 2 %0) unnamed_addr #1 {
Entry:
  %1 = alloca %std.fs.file.File, align 4
  %stderr = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %2 = alloca %"std.os.struct:4985:53", align 2
  %3 = alloca i16, align 2
  call fastcc void @std.Thread.Mutex.lock(%std.Thread.Mutex* @stderr_mutex)
  call fastcc void @std.io.getStdErr(%std.fs.file.File* sret(%std.fs.file.File) %1)
  call fastcc void @std.fs.file.File.writer(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* sret(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)") %stderr, %std.fs.file.File* %1)
  %4 = bitcast %"std.os.struct:4985:53"* %0 to i8*
  %5 = bitcast %"std.os.struct:4985:53"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 2 %5, i8* align 2 %4, i64 2, i1 false)
  %6 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).print.20"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %stderr, %"std.os.struct:4985:53"* %0)
  store i16 %6, i16* %3, align 2
  %7 = icmp ne i16 %6, 0
  br i1 %7, label %UnwrapErrError, label %UnwrapErrOk

UnwrapErrError:                                   ; preds = %Entry
  call fastcc void @std.Thread.Mutex.unlock(%std.Thread.Mutex* @stderr_mutex)
  ret void

UnwrapErrOk:                                      ; preds = %Entry
  br label %UnwrapErrEnd

UnwrapErrEnd:                                     ; preds = %UnwrapErrOk
  call fastcc void @std.Thread.Mutex.unlock(%std.Thread.Mutex* @stderr_mutex)
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.formatIntValue(i64 %0, %std.fmt.FormatOptions* nonnull readonly align 8 %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %2) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %int_value = alloca i64, align 8
  %3 = alloca i64, align 8
  %4 = alloca %std.fmt.FormatOptions, align 8
  %5 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %value = alloca i64, align 8
  store i64 %0, i64* %value, align 8
  %6 = load i64, i64* %value, align 8
  store i64 %6, i64* %int_value, align 8
  %7 = load i64, i64* %int_value, align 8
  store i64 %7, i64* %3, align 8
  %8 = bitcast %std.fmt.FormatOptions* %1 to i8*
  %9 = bitcast %std.fmt.FormatOptions* %4 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %9, i8* align 8 %8, i64 40, i1 false)
  %10 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2 to i8*
  %11 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %5 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %11, i8* align 4 %10, i64 4, i1 false)
  %12 = call fastcc i16 @std.fmt.formatInt(i64 %7, i8 10, i1 false, %std.fmt.FormatOptions* %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2)
  store i16 %12, i16* %result, align 2
  %13 = load i16, i16* %result, align 2
  ret i16 %13
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.formatInt(i64 %0, i8 %1, i1 %2, %std.fmt.FormatOptions* nonnull readonly align 8 %3, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %4) unnamed_addr #1 {
Entry:
  %5 = alloca [6 x i64], align 8
  %result = alloca i16, align 2
  %int_value = alloca i64, align 8
  %6 = alloca i64, align 8
  %abs_value = alloca i64, align 8
  %buf = alloca [65 x i8], align 1
  %a = alloca i64, align 8
  %index = alloca i64, align 8
  %digit = alloca i64, align 8
  %7 = alloca %"[]u8", align 8
  %8 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %value = alloca i64, align 8
  %base = alloca i8, align 1
  %case = alloca i1, align 1
  store i64 %0, i64* %value, align 8
  store i8 %1, i8* %base, align 1
  store i1 %2, i1* %case, align 1
  %9 = load i8, i8* %base, align 1
  %10 = icmp uge i8 %9, 2
  call fastcc void @std.debug.assert(i1 %10)
  %11 = load i64, i64* %value, align 8
  store i64 %11, i64* %int_value, align 8
  %12 = load i64, i64* %int_value, align 8
  store i64 %12, i64* %6, align 8
  %13 = call fastcc i64 @std.math.absCast(i64 %12)
  store i64 %13, i64* %abs_value, align 8
  %14 = bitcast [65 x i8]* %buf to i8*
  call void @llvm.memset.p0i8.i64(i8* align 1 %14, i8 -86, i64 65, i1 false)
  %15 = ptrtoint i8* %14 to i64
  %16 = getelementptr inbounds [6 x i64], [6 x i64]* %5, i64 0, i64 0
  store i64 1296236545, i64* %16, align 8
  %17 = getelementptr inbounds [6 x i64], [6 x i64]* %5, i64 0, i64 1
  store i64 %15, i64* %17, align 8
  %18 = getelementptr inbounds [6 x i64], [6 x i64]* %5, i64 0, i64 2
  store i64 65, i64* %18, align 8
  %19 = getelementptr inbounds [6 x i64], [6 x i64]* %5, i64 0, i64 3
  store i64 0, i64* %19, align 8
  %20 = getelementptr inbounds [6 x i64], [6 x i64]* %5, i64 0, i64 4
  store i64 0, i64* %20, align 8
  %21 = getelementptr inbounds [6 x i64], [6 x i64]* %5, i64 0, i64 5
  store i64 0, i64* %21, align 8
  %22 = ptrtoint [6 x i64]* %5 to i64
  %23 = call i64 asm sideeffect "rolq $$3,  %rdi ; rolq $$13, %rdi\0Arolq $$61, %rdi ; rolq $$51, %rdi\0Axchgq %rbx,%rbx\0A", "={rdx},{rax},0,~{cc},~{memory}"(i64 %22, i64 0)
  %24 = load i64, i64* %abs_value, align 8
  store i64 %24, i64* %a, align 8
  store i64 65, i64* %index, align 8
  br label %WhileCond

WhileCond:                                        ; preds = %EndIf, %Entry
  br label %WhileBody

WhileBody:                                        ; preds = %WhileCond
  %25 = load i64, i64* %a, align 8
  %26 = load i8, i8* %base, align 1
  %27 = zext i8 %26 to i64
  %28 = icmp eq i64 %27, 0
  br i1 %28, label %RemZeroFail, label %RemZeroOk

Then:                                             ; preds = %DivZeroOk
  br label %WhileEnd

Else:                                             ; preds = %DivZeroOk
  br label %EndIf

EndIf:                                            ; preds = %Else
  br label %WhileCond

WhileEnd:                                         ; preds = %Then
  %29 = load i64, i64* %index, align 8
  %30 = icmp ule i64 %29, 65
  br i1 %30, label %BoundsCheckOk2, label %BoundsCheckFail1

RemZeroOk:                                        ; preds = %WhileBody
  %31 = urem i64 %25, %27
  store i64 %31, i64* %digit, align 8
  %32 = load i64, i64* %index, align 8
  %33 = call { i64, i1 } @llvm.usub.with.overflow.i64(i64 %32, i64 1)
  %34 = extractvalue { i64, i1 } %33, 0
  %35 = extractvalue { i64, i1 } %33, 1
  br i1 %35, label %OverflowFail, label %OverflowOk

RemZeroFail:                                      ; preds = %WhileBody
  call fastcc void @std.builtin.default_panic(%"[]u8"* @13, %std.builtin.StackTrace* null)
  unreachable

OverflowFail:                                     ; preds = %RemZeroOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %RemZeroOk
  store i64 %34, i64* %index, align 8
  %36 = load i64, i64* %index, align 8
  %37 = icmp ult i64 %36, 65
  br i1 %37, label %BoundsCheckOk, label %BoundsCheckFail

BoundsCheckFail:                                  ; preds = %OverflowOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %OverflowOk
  %38 = getelementptr inbounds [65 x i8], [65 x i8]* %buf, i64 0, i64 %36
  %39 = load i64, i64* %digit, align 8
  %40 = trunc i64 %39 to i8
  %41 = zext i8 %40 to i64
  %42 = icmp eq i64 %39, %41
  br i1 %42, label %CastShortenOk, label %CastShortenFail

CastShortenOk:                                    ; preds = %BoundsCheckOk
  %43 = load i1, i1* %case, align 1
  %44 = call fastcc i8 @std.fmt.digitToChar(i8 %40, i1 %43)
  store i8 %44, i8* %38, align 1
  %45 = load i64, i64* %a, align 8
  %46 = load i8, i8* %base, align 1
  %47 = zext i8 %46 to i64
  %48 = icmp eq i64 %47, 0
  br i1 %48, label %DivZeroFail, label %DivZeroOk

CastShortenFail:                                  ; preds = %BoundsCheckOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @15, %std.builtin.StackTrace* null)
  unreachable

DivZeroFail:                                      ; preds = %CastShortenOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @11, %std.builtin.StackTrace* null)
  unreachable

DivZeroOk:                                        ; preds = %CastShortenOk
  %49 = udiv i64 %45, %47
  store i64 %49, i64* %a, align 8
  %50 = load i64, i64* %a, align 8
  %51 = icmp eq i64 %50, 0
  br i1 %51, label %Then, label %Else

BoundsCheckFail1:                                 ; preds = %WhileEnd
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk2:                                   ; preds = %WhileEnd
  br i1 true, label %BoundsCheckOk4, label %BoundsCheckFail3

BoundsCheckFail3:                                 ; preds = %BoundsCheckOk2
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk4:                                   ; preds = %BoundsCheckOk2
  %52 = getelementptr inbounds [65 x i8], [65 x i8]* %buf, i64 0, i64 %29
  %53 = sub nuw i64 65, %29
  %54 = getelementptr inbounds %"[]u8", %"[]u8"* %7, i32 0, i32 0
  store i8* %52, i8** %54, align 8
  %55 = getelementptr inbounds %"[]u8", %"[]u8"* %7, i32 0, i32 1
  store i64 %53, i64* %55, align 8
  %56 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %4 to i8*
  %57 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %8 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %57, i8* align 4 %56, i64 4, i1 false)
  %58 = call fastcc i16 @std.fmt.formatBuf(%"[]u8"* %7, %std.fmt.FormatOptions* %3, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %4)
  store i16 %58, i16* %result, align 2
  %59 = load i16, i16* %result, align 2
  ret i16 %59
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.math.absCast(i64 %0) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %x = alloca i64, align 8
  store i64 %0, i64* %x, align 8
  %1 = load i64, i64* %x, align 8
  store i64 %1, i64* %result, align 8
  %2 = load i64, i64* %result, align 8
  ret i64 %2
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i64, i1 } @llvm.usub.with.overflow.i64(i64, i64) #8

; Function Attrs: nobuiltin nounwind
define internal fastcc i8 @std.fmt.digitToChar(i8 %0, i1 %1) unnamed_addr #1 {
Entry:
  %result = alloca i8, align 1
  %2 = alloca i8, align 1
  %digit = alloca i8, align 1
  %case = alloca i1, align 1
  store i8 %0, i8* %digit, align 1
  store i1 %1, i1* %case, align 1
  %3 = load i8, i8* %digit, align 1
  %4 = icmp ule i8 %3, 9
  %5 = and i1 true, %4
  br i1 %5, label %SwitchRangeYes, label %SwitchRangeNo

SwitchRangeYes:                                   ; preds = %Entry
  %6 = load i8, i8* %digit, align 1
  %7 = call { i8, i1 } @llvm.uadd.with.overflow.i8(i8 %6, i8 48)
  %8 = extractvalue { i8, i1 } %7, 0
  %9 = extractvalue { i8, i1 } %7, 1
  br i1 %9, label %OverflowFail, label %OverflowOk

SwitchRangeNo:                                    ; preds = %Entry
  %10 = icmp uge i8 %3, 10
  %11 = icmp ule i8 %3, 35
  %12 = and i1 %10, %11
  br i1 %12, label %SwitchRangeYes1, label %SwitchRangeNo2

SwitchRangeYes1:                                  ; preds = %SwitchRangeNo
  %13 = load i8, i8* %digit, align 1
  %14 = load i1, i1* %case, align 1
  %15 = icmp eq i1 %14, true
  br i1 %15, label %Then, label %Else

Then:                                             ; preds = %SwitchRangeYes1
  store i8 65, i8* %2, align 1
  store i8 65, i8* %2, align 1
  br label %EndIf

Else:                                             ; preds = %SwitchRangeYes1
  store i8 97, i8* %2, align 1
  store i8 97, i8* %2, align 1
  br label %EndIf

EndIf:                                            ; preds = %Else, %Then
  %16 = phi i8 [ 65, %Then ], [ 97, %Else ]
  %17 = call { i8, i1 } @llvm.usub.with.overflow.i8(i8 %16, i8 10)
  %18 = extractvalue { i8, i1 } %17, 0
  %19 = extractvalue { i8, i1 } %17, 1
  br i1 %19, label %OverflowFail3, label %OverflowOk4

SwitchRangeNo2:                                   ; preds = %SwitchRangeNo
  br label %SwitchElse

SwitchElse:                                       ; preds = %SwitchRangeNo2
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchEnd:                                        ; preds = %OverflowOk6, %OverflowOk
  %20 = load i8, i8* %result, align 1
  ret i8 %20

OverflowFail:                                     ; preds = %SwitchRangeYes
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %SwitchRangeYes
  store i8 %8, i8* %result, align 1
  br label %SwitchEnd

OverflowFail3:                                    ; preds = %EndIf
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk4:                                      ; preds = %EndIf
  %21 = call { i8, i1 } @llvm.uadd.with.overflow.i8(i8 %13, i8 %18)
  %22 = extractvalue { i8, i1 } %21, 0
  %23 = extractvalue { i8, i1 } %21, 1
  br i1 %23, label %OverflowFail5, label %OverflowOk6

OverflowFail5:                                    ; preds = %OverflowOk4
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk6:                                      ; preds = %OverflowOk4
  store i8 %22, i8* %result, align 1
  br label %SwitchEnd
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.unicode.utf8CountCodepoints({ i64, i16 }* nonnull sret({ i64, i16 }) %0, %"[]u8"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %len = alloca i64, align 8
  %i = alloca i64, align 8
  %2 = alloca %"[]u8", align 8
  %3 = alloca [8 x i8]*, align 8
  %v = alloca i64, align 8
  %4 = alloca { i3, i16 }, align 2
  %5 = alloca { i64, i16 }, align 8
  %n = alloca i3, align 1
  %6 = alloca %"[]u8", align 8
  %7 = alloca { i21, i16 }, align 4
  %8 = alloca { i64, i16 }, align 8
  store i64 0, i64* %len, align 8
  store i64 0, i64* %i, align 8
  br label %WhileCond

WhileCond:                                        ; preds = %EndIf10, %Entry
  %9 = load i64, i64* %i, align 8
  %10 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %11 = load i64, i64* %10, align 8
  %12 = icmp ult i64 %9, %11
  br i1 %12, label %WhileBody, label %WhileEnd11

WhileBody:                                        ; preds = %WhileCond
  br label %WhileCond1

WhileCond1:                                       ; preds = %OverflowOk21, %WhileBody
  %13 = load i64, i64* %i, align 8
  %14 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %13, i64 8)
  %15 = extractvalue { i64, i1 } %14, 0
  %16 = extractvalue { i64, i1 } %14, 1
  br i1 %16, label %OverflowFail, label %OverflowOk

WhileBody2:                                       ; preds = %OverflowOk
  %17 = load i64, i64* %i, align 8
  %18 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %19 = load i64, i64* %18, align 8
  %20 = icmp ule i64 %17, %19
  br i1 %20, label %BoundsCheckOk, label %BoundsCheckFail

Then:                                             ; preds = %BoundsCheckOk17
  br label %WhileEnd

Else:                                             ; preds = %BoundsCheckOk17
  br label %EndIf

EndIf:                                            ; preds = %Else
  %21 = load i64, i64* %len, align 8
  %22 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %21, i64 8)
  %23 = extractvalue { i64, i1 } %22, 0
  %24 = extractvalue { i64, i1 } %22, 1
  br i1 %24, label %OverflowFail18, label %OverflowOk19

WhileEnd:                                         ; preds = %Then, %OverflowOk
  %25 = load i64, i64* %i, align 8
  %26 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %27 = load i64, i64* %26, align 8
  %28 = icmp ult i64 %25, %27
  br i1 %28, label %Then3, label %Else9

Then3:                                            ; preds = %WhileEnd
  %29 = load i64, i64* %i, align 8
  %30 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %31 = load i64, i64* %30, align 8
  %32 = icmp ult i64 %29, %31
  br i1 %32, label %BoundsCheckOk23, label %BoundsCheckFail22

ErrRetReturn:                                     ; preds = %BoundsCheckOk23
  %33 = getelementptr inbounds { i3, i16 }, { i3, i16 }* %4, i32 0, i32 1
  %34 = load i16, i16* %33, align 2
  %35 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 %34, i16* %35, align 2
  %36 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %5, i32 0, i32 1
  store i16 %34, i16* %36, align 2
  ret void

ErrRetContinue:                                   ; preds = %BoundsCheckOk23
  %37 = getelementptr inbounds { i3, i16 }, { i3, i16 }* %4, i32 0, i32 0
  %38 = load i3, i3* %37, align 1
  store i3 %38, i3* %n, align 1
  %39 = load i64, i64* %i, align 8
  %40 = load i3, i3* %n, align 1
  %41 = zext i3 %40 to i64
  %42 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %39, i64 %41)
  %43 = extractvalue { i64, i1 } %42, 0
  %44 = extractvalue { i64, i1 } %42, 1
  br i1 %44, label %OverflowFail24, label %OverflowOk25

Then4:                                            ; preds = %OverflowOk25
  %45 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 15, i16* %45, align 2
  ret void

Else5:                                            ; preds = %OverflowOk25
  br label %EndIf6

EndIf6:                                           ; preds = %Else5
  %46 = load i3, i3* %n, align 1
  switch i3 %46, label %SwitchElse [
    i3 1, label %SwitchProng
  ]

SwitchElse:                                       ; preds = %EndIf6
  %47 = load i64, i64* %i, align 8
  %48 = load i64, i64* %i, align 8
  %49 = load i3, i3* %n, align 1
  %50 = zext i3 %49 to i64
  %51 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %48, i64 %50)
  %52 = extractvalue { i64, i1 } %51, 0
  %53 = extractvalue { i64, i1 } %51, 1
  br i1 %53, label %OverflowFail26, label %OverflowOk27

ErrRetReturn7:                                    ; preds = %BoundsCheckOk31
  %54 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %7, i32 0, i32 1
  %55 = load i16, i16* %54, align 2
  %56 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 %55, i16* %56, align 2
  %57 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %8, i32 0, i32 1
  store i16 %55, i16* %57, align 2
  ret void

ErrRetContinue8:                                  ; preds = %BoundsCheckOk31
  %58 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %7, i32 0, i32 0
  br label %SwitchEnd

SwitchProng:                                      ; preds = %EndIf6
  br label %SwitchEnd

SwitchEnd:                                        ; preds = %SwitchProng, %ErrRetContinue8
  %59 = load i64, i64* %i, align 8
  %60 = load i3, i3* %n, align 1
  %61 = zext i3 %60 to i64
  %62 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %59, i64 %61)
  %63 = extractvalue { i64, i1 } %62, 0
  %64 = extractvalue { i64, i1 } %62, 1
  br i1 %64, label %OverflowFail32, label %OverflowOk33

Else9:                                            ; preds = %WhileEnd
  br label %EndIf10

EndIf10:                                          ; preds = %Else9, %OverflowOk35
  br label %WhileCond

WhileEnd11:                                       ; preds = %WhileCond
  %65 = load i64, i64* %len, align 8
  %66 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %66, align 2
  %67 = getelementptr inbounds { i64, i16 }, { i64, i16 }* %0, i32 0, i32 0
  store i64 %65, i64* %67, align 8
  ret void

OverflowFail:                                     ; preds = %WhileCond1
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %WhileCond1
  %68 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %69 = load i64, i64* %68, align 8
  %70 = icmp ule i64 %15, %69
  br i1 %70, label %WhileBody2, label %WhileEnd

BoundsCheckFail:                                  ; preds = %WhileBody2
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %WhileBody2
  %71 = icmp ule i64 %19, %19
  br i1 %71, label %BoundsCheckOk13, label %BoundsCheckFail12

BoundsCheckFail12:                                ; preds = %BoundsCheckOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk13:                                  ; preds = %BoundsCheckOk
  %72 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %73 = load i8*, i8** %72, align 8
  %74 = getelementptr inbounds i8, i8* %73, i64 %17
  %75 = sub nuw i64 %19, %17
  %76 = getelementptr inbounds %"[]u8", %"[]u8"* %2, i32 0, i32 0
  store i8* %74, i8** %76, align 8
  %77 = getelementptr inbounds %"[]u8", %"[]u8"* %2, i32 0, i32 1
  store i64 %75, i64* %77, align 8
  %78 = getelementptr inbounds %"[]u8", %"[]u8"* %2, i32 0, i32 1
  %79 = load i64, i64* %78, align 8
  br i1 true, label %BoundsCheckOk15, label %BoundsCheckFail14

BoundsCheckFail14:                                ; preds = %BoundsCheckOk13
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk15:                                  ; preds = %BoundsCheckOk13
  %80 = icmp ule i64 8, %79
  br i1 %80, label %BoundsCheckOk17, label %BoundsCheckFail16

BoundsCheckFail16:                                ; preds = %BoundsCheckOk15
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk17:                                  ; preds = %BoundsCheckOk15
  %81 = getelementptr inbounds %"[]u8", %"[]u8"* %2, i32 0, i32 0
  %82 = load i8*, i8** %81, align 8
  %83 = getelementptr inbounds i8, i8* %82, i64 0
  %84 = bitcast i8* %83 to [8 x i8]*
  store [8 x i8]* %84, [8 x i8]** %3, align 8
  %85 = call fastcc i64 @std.mem.readIntNative([8 x i8]* %84)
  store i64 %85, i64* %v, align 8
  %86 = load i64, i64* %v, align 8
  %87 = and i64 %86, -9187201950435737472
  %88 = icmp ne i64 %87, 0
  br i1 %88, label %Then, label %Else

OverflowFail18:                                   ; preds = %EndIf
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk19:                                     ; preds = %EndIf
  store i64 %23, i64* %len, align 8
  %89 = load i64, i64* %i, align 8
  %90 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %89, i64 8)
  %91 = extractvalue { i64, i1 } %90, 0
  %92 = extractvalue { i64, i1 } %90, 1
  br i1 %92, label %OverflowFail20, label %OverflowOk21

OverflowFail20:                                   ; preds = %OverflowOk19
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk21:                                     ; preds = %OverflowOk19
  store i64 %91, i64* %i, align 8
  br label %WhileCond1

BoundsCheckFail22:                                ; preds = %Then3
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk23:                                  ; preds = %Then3
  %93 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %94 = load i8*, i8** %93, align 8
  %95 = getelementptr inbounds i8, i8* %94, i64 %29
  %96 = load i8, i8* %95, align 1
  call fastcc void @std.unicode.utf8ByteSequenceLength({ i3, i16 }* sret({ i3, i16 }) %4, i8 %96)
  %97 = getelementptr inbounds { i3, i16 }, { i3, i16 }* %4, i32 0, i32 1
  %98 = load i16, i16* %97, align 2
  %99 = icmp ne i16 %98, 0
  br i1 %99, label %ErrRetReturn, label %ErrRetContinue

OverflowFail24:                                   ; preds = %ErrRetContinue
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk25:                                     ; preds = %ErrRetContinue
  %100 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %101 = load i64, i64* %100, align 8
  %102 = icmp ugt i64 %43, %101
  br i1 %102, label %Then4, label %Else5

OverflowFail26:                                   ; preds = %SwitchElse
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk27:                                     ; preds = %SwitchElse
  %103 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %104 = load i64, i64* %103, align 8
  %105 = icmp ule i64 %47, %52
  br i1 %105, label %BoundsCheckOk29, label %BoundsCheckFail28

BoundsCheckFail28:                                ; preds = %OverflowOk27
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk29:                                  ; preds = %OverflowOk27
  %106 = icmp ule i64 %52, %104
  br i1 %106, label %BoundsCheckOk31, label %BoundsCheckFail30

BoundsCheckFail30:                                ; preds = %BoundsCheckOk29
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk31:                                  ; preds = %BoundsCheckOk29
  %107 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %108 = load i8*, i8** %107, align 8
  %109 = getelementptr inbounds i8, i8* %108, i64 %47
  %110 = sub nuw i64 %52, %47
  %111 = getelementptr inbounds %"[]u8", %"[]u8"* %6, i32 0, i32 0
  store i8* %109, i8** %111, align 8
  %112 = getelementptr inbounds %"[]u8", %"[]u8"* %6, i32 0, i32 1
  store i64 %110, i64* %112, align 8
  call fastcc void @std.unicode.utf8Decode({ i21, i16 }* sret({ i21, i16 }) %7, %"[]u8"* %6)
  %113 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %7, i32 0, i32 1
  %114 = load i16, i16* %113, align 2
  %115 = icmp ne i16 %114, 0
  br i1 %115, label %ErrRetReturn7, label %ErrRetContinue8

OverflowFail32:                                   ; preds = %SwitchEnd
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk33:                                     ; preds = %SwitchEnd
  store i64 %63, i64* %i, align 8
  %116 = load i64, i64* %len, align 8
  %117 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %116, i64 1)
  %118 = extractvalue { i64, i1 } %117, 0
  %119 = extractvalue { i64, i1 } %117, 1
  br i1 %119, label %OverflowFail34, label %OverflowOk35

OverflowFail34:                                   ; preds = %OverflowOk33
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk35:                                     ; preds = %OverflowOk33
  store i64 %118, i64* %len, align 8
  br label %EndIf10
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeByteNTimes"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0, i8 %1, i64 %2) unnamed_addr #1 {
Entry:
  %3 = alloca [6 x i64], align 8
  %result = alloca i16, align 2
  %bytes = alloca [256 x i8], align 1
  %4 = alloca [256 x i8]*, align 8
  %5 = alloca i8, align 1
  %6 = alloca %"[]u8", align 8
  %remaining = alloca i64, align 8
  %7 = alloca i64, align 8
  %to_write = alloca i64, align 8
  %8 = alloca %"[]u8", align 8
  %9 = alloca i16, align 2
  %byte = alloca i8, align 1
  %n = alloca i64, align 8
  store i8 %1, i8* %byte, align 1
  store i64 %2, i64* %n, align 8
  %10 = bitcast [256 x i8]* %bytes to i8*
  call void @llvm.memset.p0i8.i64(i8* align 1 %10, i8 -86, i64 256, i1 false)
  %11 = ptrtoint i8* %10 to i64
  %12 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 0
  store i64 1296236545, i64* %12, align 8
  %13 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 1
  store i64 %11, i64* %13, align 8
  %14 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 2
  store i64 256, i64* %14, align 8
  %15 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 3
  store i64 0, i64* %15, align 8
  %16 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 4
  store i64 0, i64* %16, align 8
  %17 = getelementptr inbounds [6 x i64], [6 x i64]* %3, i64 0, i64 5
  store i64 0, i64* %17, align 8
  %18 = ptrtoint [6 x i64]* %3 to i64
  %19 = call i64 asm sideeffect "rolq $$3,  %rdi ; rolq $$13, %rdi\0Arolq $$61, %rdi ; rolq $$51, %rdi\0Axchgq %rbx,%rbx\0A", "={rdx},{rax},0,~{cc},~{memory}"(i64 %18, i64 0)
  br i1 true, label %BoundsCheckOk, label %BoundsCheckFail

WhileCond:                                        ; preds = %OverflowOk, %BoundsCheckOk
  %20 = load i64, i64* %remaining, align 8
  %21 = icmp ugt i64 %20, 0
  br i1 %21, label %WhileBody, label %WhileEnd

WhileBody:                                        ; preds = %WhileCond
  %22 = load i64, i64* %remaining, align 8
  store i64 %22, i64* %7, align 8
  %23 = call fastcc i64 @std.math.min.19(i64 %22, i64 256)
  store i64 %23, i64* %to_write, align 8
  %24 = load i64, i64* %to_write, align 8
  %25 = icmp ule i64 0, %24
  br i1 %25, label %BoundsCheckOk2, label %BoundsCheckFail1

ErrRetReturn:                                     ; preds = %BoundsCheckOk4
  %26 = load i16, i16* %9, align 2
  store i16 %26, i16* %result, align 2
  ret i16 %26

ErrRetContinue:                                   ; preds = %BoundsCheckOk4
  %27 = load i64, i64* %remaining, align 8
  %28 = load i64, i64* %to_write, align 8
  %29 = call { i64, i1 } @llvm.usub.with.overflow.i64(i64 %27, i64 %28)
  %30 = extractvalue { i64, i1 } %29, 0
  %31 = extractvalue { i64, i1 } %29, 1
  br i1 %31, label %OverflowFail, label %OverflowOk

WhileEnd:                                         ; preds = %WhileCond
  store i16 0, i16* %result, align 2
  ret i16 0

BoundsCheckFail:                                  ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %Entry
  %32 = getelementptr inbounds [256 x i8], [256 x i8]* %bytes, i64 0, i64 0
  %33 = bitcast i8* %32 to [256 x i8]*
  store [256 x i8]* %33, [256 x i8]** %4, align 8
  %34 = load i8, i8* %byte, align 1
  store i8 %34, i8* %5, align 1
  %35 = getelementptr inbounds %"[]u8", %"[]u8"* %6, i32 0, i32 0
  %36 = getelementptr inbounds [256 x i8], [256 x i8]* %33, i64 0, i64 0
  store i8* %36, i8** %35, align 8
  %37 = getelementptr inbounds %"[]u8", %"[]u8"* %6, i32 0, i32 1
  store i64 256, i64* %37, align 8
  call fastcc void @std.mem.set(%"[]u8"* %6, i8 %34)
  %38 = load i64, i64* %n, align 8
  store i64 %38, i64* %remaining, align 8
  br label %WhileCond

BoundsCheckFail1:                                 ; preds = %WhileBody
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk2:                                   ; preds = %WhileBody
  %39 = icmp ule i64 %24, 256
  br i1 %39, label %BoundsCheckOk4, label %BoundsCheckFail3

BoundsCheckFail3:                                 ; preds = %BoundsCheckOk2
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk4:                                   ; preds = %BoundsCheckOk2
  %40 = getelementptr inbounds [256 x i8], [256 x i8]* %bytes, i64 0, i64 0
  %41 = sub nuw i64 %24, 0
  %42 = getelementptr inbounds %"[]u8", %"[]u8"* %8, i32 0, i32 0
  store i8* %40, i8** %42, align 8
  %43 = getelementptr inbounds %"[]u8", %"[]u8"* %8, i32 0, i32 1
  store i64 %41, i64* %43, align 8
  %44 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"[]u8"* %8)
  store i16 %44, i16* %9, align 2
  %45 = icmp ne i16 %44, 0
  br i1 %45, label %ErrRetReturn, label %ErrRetContinue

OverflowFail:                                     ; preds = %ErrRetContinue
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %ErrRetContinue
  store i64 %30, i64* %remaining, align 8
  br label %WhileCond
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.mem.readIntNative([8 x i8]* nonnull readonly align 1 %0) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %bytes = alloca [8 x i8]*, align 8
  store [8 x i8]* %0, [8 x i8]** %bytes, align 8
  %1 = load [8 x i8]*, [8 x i8]** %bytes, align 8
  %2 = bitcast [8 x i8]* %1 to i64*
  %3 = icmp ne i64* %2, null
  br i1 %3, label %PtrCastOk, label %PtrCastFail

PtrCastFail:                                      ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @47, %std.builtin.StackTrace* null)
  unreachable

PtrCastOk:                                        ; preds = %Entry
  %4 = load i64, i64* %2, align 1
  store i64 %4, i64* %result, align 8
  %5 = load i64, i64* %result, align 8
  ret i64 %5
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.unicode.utf8ByteSequenceLength({ i3, i16 }* nonnull sret({ i3, i16 }) %0, i8 %1) unnamed_addr #1 {
Entry:
  %first_byte = alloca i8, align 1
  store i8 %1, i8* %first_byte, align 1
  %2 = load i8, i8* %first_byte, align 1
  %3 = icmp ule i8 %2, 127
  %4 = and i1 true, %3
  br i1 %4, label %SwitchRangeYes, label %SwitchRangeNo

SwitchRangeYes:                                   ; preds = %Entry
  %5 = getelementptr inbounds { i3, i16 }, { i3, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %5, align 2
  %6 = getelementptr inbounds { i3, i16 }, { i3, i16 }* %0, i32 0, i32 0
  store i3 1, i3* %6, align 1
  br label %SwitchEnd

SwitchRangeNo:                                    ; preds = %Entry
  %7 = icmp uge i8 %2, -64
  %8 = icmp ule i8 %2, -33
  %9 = and i1 %7, %8
  br i1 %9, label %SwitchRangeYes1, label %SwitchRangeNo2

SwitchRangeYes1:                                  ; preds = %SwitchRangeNo
  %10 = getelementptr inbounds { i3, i16 }, { i3, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %10, align 2
  %11 = getelementptr inbounds { i3, i16 }, { i3, i16 }* %0, i32 0, i32 0
  store i3 2, i3* %11, align 1
  br label %SwitchEnd

SwitchRangeNo2:                                   ; preds = %SwitchRangeNo
  %12 = icmp uge i8 %2, -32
  %13 = icmp ule i8 %2, -17
  %14 = and i1 %12, %13
  br i1 %14, label %SwitchRangeYes3, label %SwitchRangeNo4

SwitchRangeYes3:                                  ; preds = %SwitchRangeNo2
  %15 = getelementptr inbounds { i3, i16 }, { i3, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %15, align 2
  %16 = getelementptr inbounds { i3, i16 }, { i3, i16 }* %0, i32 0, i32 0
  store i3 3, i3* %16, align 1
  br label %SwitchEnd

SwitchRangeNo4:                                   ; preds = %SwitchRangeNo2
  %17 = icmp uge i8 %2, -16
  %18 = icmp ule i8 %2, -9
  %19 = and i1 %17, %18
  br i1 %19, label %SwitchRangeYes5, label %SwitchRangeNo6

SwitchRangeYes5:                                  ; preds = %SwitchRangeNo4
  %20 = getelementptr inbounds { i3, i16 }, { i3, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %20, align 2
  %21 = getelementptr inbounds { i3, i16 }, { i3, i16 }* %0, i32 0, i32 0
  store i3 -4, i3* %21, align 1
  br label %SwitchEnd

SwitchRangeNo6:                                   ; preds = %SwitchRangeNo4
  br label %SwitchElse

SwitchElse:                                       ; preds = %SwitchRangeNo6
  %22 = getelementptr inbounds { i3, i16 }, { i3, i16 }* %0, i32 0, i32 1
  store i16 14, i16* %22, align 2
  br label %SwitchEnd

SwitchEnd:                                        ; preds = %SwitchElse, %SwitchRangeYes5, %SwitchRangeYes3, %SwitchRangeYes1, %SwitchRangeYes
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.unicode.utf8Decode({ i21, i16 }* nonnull sret({ i21, i16 }) %0, %"[]u8"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %2 = alloca { i21, i16 }, align 4
  %3 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %4 = load i64, i64* %3, align 8
  switch i64 %4, label %SwitchElse [
    i64 1, label %SwitchProng
    i64 2, label %SwitchProng1
    i64 3, label %SwitchProng2
    i64 4, label %SwitchProng3
  ]

SwitchElse:                                       ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng:                                      ; preds = %Entry
  %5 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = icmp ult i64 0, %6
  br i1 %7, label %BoundsCheckOk, label %BoundsCheckFail

SwitchProng1:                                     ; preds = %Entry
  call fastcc void @std.unicode.utf8Decode2({ i21, i16 }* sret({ i21, i16 }) %0, %"[]u8"* %1)
  %8 = bitcast { i21, i16 }* %0 to i8*
  %9 = bitcast { i21, i16 }* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %9, i8* align 4 %8, i64 8, i1 false)
  br label %SwitchEnd

SwitchProng2:                                     ; preds = %Entry
  call fastcc void @std.unicode.utf8Decode3({ i21, i16 }* sret({ i21, i16 }) %0, %"[]u8"* %1)
  %10 = bitcast { i21, i16 }* %0 to i8*
  %11 = bitcast { i21, i16 }* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %11, i8* align 4 %10, i64 8, i1 false)
  br label %SwitchEnd

SwitchProng3:                                     ; preds = %Entry
  call fastcc void @std.unicode.utf8Decode4({ i21, i16 }* sret({ i21, i16 }) %0, %"[]u8"* %1)
  %12 = bitcast { i21, i16 }* %0 to i8*
  %13 = bitcast { i21, i16 }* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %13, i8* align 4 %12, i64 8, i1 false)
  br label %SwitchEnd

SwitchEnd:                                        ; preds = %SwitchProng3, %SwitchProng2, %SwitchProng1, %BoundsCheckOk
  ret void

BoundsCheckFail:                                  ; preds = %SwitchProng
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %SwitchProng
  %14 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %15 = load i8*, i8** %14, align 8
  %16 = getelementptr inbounds i8, i8* %15, i64 0
  %17 = load i8, i8* %16, align 1
  %18 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %18, align 2
  %19 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 0
  %20 = zext i8 %17 to i21
  store i21 %20, i21* %19, align 4
  %21 = zext i8 %17 to i21
  store i21 %21, i21* %19, align 4
  %22 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %2, i32 0, i32 1
  store i16 0, i16* %22, align 2
  %23 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %2, i32 0, i32 0
  store i21 %21, i21* %23, align 4
  br label %SwitchEnd
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.unicode.utf8Decode2({ i21, i16 }* nonnull sret({ i21, i16 }) %0, %"[]u8"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %value = alloca i21, align 4
  %2 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %3 = load i64, i64* %2, align 8
  %4 = icmp eq i64 %3, 2
  call fastcc void @std.debug.assert(i1 %4)
  %5 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = icmp ult i64 0, %6
  br i1 %7, label %BoundsCheckOk, label %BoundsCheckFail

Then:                                             ; preds = %BoundsCheckOk7
  %8 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 16, i16* %8, align 2
  ret void

Else:                                             ; preds = %BoundsCheckOk7
  br label %EndIf

EndIf:                                            ; preds = %Else
  %9 = load i21, i21* %value, align 4
  br i1 true, label %CheckOk, label %CheckFail

Then1:                                            ; preds = %BoundsCheckOk9
  %10 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 17, i16* %10, align 2
  ret void

Else2:                                            ; preds = %BoundsCheckOk9
  br label %EndIf3

EndIf3:                                           ; preds = %Else2
  %11 = load i21, i21* %value, align 4
  %12 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %12, align 2
  %13 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 0
  store i21 %11, i21* %13, align 4
  ret void

BoundsCheckFail:                                  ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %Entry
  %14 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %15 = load i8*, i8** %14, align 8
  %16 = getelementptr inbounds i8, i8* %15, i64 0
  %17 = load i8, i8* %16, align 1
  %18 = and i8 %17, -32
  %19 = icmp eq i8 %18, -64
  call fastcc void @std.debug.assert(i1 %19)
  %20 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %21 = load i64, i64* %20, align 8
  %22 = icmp ult i64 0, %21
  br i1 %22, label %BoundsCheckOk5, label %BoundsCheckFail4

BoundsCheckFail4:                                 ; preds = %BoundsCheckOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk5:                                   ; preds = %BoundsCheckOk
  %23 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %24 = load i8*, i8** %23, align 8
  %25 = getelementptr inbounds i8, i8* %24, i64 0
  %26 = load i8, i8* %25, align 1
  %27 = and i8 %26, 31
  %28 = zext i8 %27 to i21
  store i21 %28, i21* %value, align 4
  %29 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %30 = load i64, i64* %29, align 8
  %31 = icmp ult i64 1, %30
  br i1 %31, label %BoundsCheckOk7, label %BoundsCheckFail6

BoundsCheckFail6:                                 ; preds = %BoundsCheckOk5
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk7:                                   ; preds = %BoundsCheckOk5
  %32 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %33 = load i8*, i8** %32, align 8
  %34 = getelementptr inbounds i8, i8* %33, i64 1
  %35 = load i8, i8* %34, align 1
  %36 = and i8 %35, -64
  %37 = icmp ne i8 %36, -128
  br i1 %37, label %Then, label %Else

CheckFail:                                        ; preds = %EndIf
  call fastcc void @std.builtin.default_panic(%"[]u8"* @53, %std.builtin.StackTrace* null)
  unreachable

CheckOk:                                          ; preds = %EndIf
  %38 = shl i21 %9, 6
  store i21 %38, i21* %value, align 4
  %39 = load i21, i21* %value, align 4
  %40 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %41 = load i64, i64* %40, align 8
  %42 = icmp ult i64 1, %41
  br i1 %42, label %BoundsCheckOk9, label %BoundsCheckFail8

BoundsCheckFail8:                                 ; preds = %CheckOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk9:                                   ; preds = %CheckOk
  %43 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %44 = load i8*, i8** %43, align 8
  %45 = getelementptr inbounds i8, i8* %44, i64 1
  %46 = load i8, i8* %45, align 1
  %47 = and i8 %46, 63
  %48 = zext i8 %47 to i21
  %49 = or i21 %39, %48
  store i21 %49, i21* %value, align 4
  %50 = load i21, i21* %value, align 4
  %51 = icmp ult i21 %50, 128
  br i1 %51, label %Then1, label %Else2
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.unicode.utf8Decode3({ i21, i16 }* nonnull sret({ i21, i16 }) %0, %"[]u8"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %value = alloca i21, align 4
  %2 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %3 = load i64, i64* %2, align 8
  %4 = icmp eq i64 %3, 3
  call fastcc void @std.debug.assert(i1 %4)
  %5 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = icmp ult i64 0, %6
  br i1 %7, label %BoundsCheckOk, label %BoundsCheckFail

Then:                                             ; preds = %BoundsCheckOk13
  %8 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 16, i16* %8, align 2
  ret void

Else:                                             ; preds = %BoundsCheckOk13
  br label %EndIf

EndIf:                                            ; preds = %Else
  %9 = load i21, i21* %value, align 4
  br i1 true, label %CheckOk, label %CheckFail

Then1:                                            ; preds = %BoundsCheckOk17
  %10 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 16, i16* %10, align 2
  ret void

Else2:                                            ; preds = %BoundsCheckOk17
  br label %EndIf3

EndIf3:                                           ; preds = %Else2
  %11 = load i21, i21* %value, align 4
  br i1 true, label %CheckOk19, label %CheckFail18

Then4:                                            ; preds = %BoundsCheckOk21
  %12 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 17, i16* %12, align 2
  ret void

Else5:                                            ; preds = %BoundsCheckOk21
  br label %EndIf6

EndIf6:                                           ; preds = %Else5
  %13 = load i21, i21* %value, align 4
  %14 = icmp ule i21 55296, %13
  br i1 %14, label %BoolAndTrue, label %BoolAndFalse

BoolAndTrue:                                      ; preds = %EndIf6
  %15 = load i21, i21* %value, align 4
  %16 = icmp ule i21 %15, 57343
  br label %BoolAndFalse

BoolAndFalse:                                     ; preds = %BoolAndTrue, %EndIf6
  %17 = phi i1 [ %14, %EndIf6 ], [ %16, %BoolAndTrue ]
  br i1 %17, label %Then7, label %Else8

Then7:                                            ; preds = %BoolAndFalse
  %18 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 18, i16* %18, align 2
  ret void

Else8:                                            ; preds = %BoolAndFalse
  br label %EndIf9

EndIf9:                                           ; preds = %Else8
  %19 = load i21, i21* %value, align 4
  %20 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %20, align 2
  %21 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 0
  store i21 %19, i21* %21, align 4
  ret void

BoundsCheckFail:                                  ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %Entry
  %22 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %23 = load i8*, i8** %22, align 8
  %24 = getelementptr inbounds i8, i8* %23, i64 0
  %25 = load i8, i8* %24, align 1
  %26 = and i8 %25, -16
  %27 = icmp eq i8 %26, -32
  call fastcc void @std.debug.assert(i1 %27)
  %28 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %29 = load i64, i64* %28, align 8
  %30 = icmp ult i64 0, %29
  br i1 %30, label %BoundsCheckOk11, label %BoundsCheckFail10

BoundsCheckFail10:                                ; preds = %BoundsCheckOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk11:                                  ; preds = %BoundsCheckOk
  %31 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %32 = load i8*, i8** %31, align 8
  %33 = getelementptr inbounds i8, i8* %32, i64 0
  %34 = load i8, i8* %33, align 1
  %35 = and i8 %34, 15
  %36 = zext i8 %35 to i21
  store i21 %36, i21* %value, align 4
  %37 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %38 = load i64, i64* %37, align 8
  %39 = icmp ult i64 1, %38
  br i1 %39, label %BoundsCheckOk13, label %BoundsCheckFail12

BoundsCheckFail12:                                ; preds = %BoundsCheckOk11
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk13:                                  ; preds = %BoundsCheckOk11
  %40 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %41 = load i8*, i8** %40, align 8
  %42 = getelementptr inbounds i8, i8* %41, i64 1
  %43 = load i8, i8* %42, align 1
  %44 = and i8 %43, -64
  %45 = icmp ne i8 %44, -128
  br i1 %45, label %Then, label %Else

CheckFail:                                        ; preds = %EndIf
  call fastcc void @std.builtin.default_panic(%"[]u8"* @53, %std.builtin.StackTrace* null)
  unreachable

CheckOk:                                          ; preds = %EndIf
  %46 = shl i21 %9, 6
  store i21 %46, i21* %value, align 4
  %47 = load i21, i21* %value, align 4
  %48 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %49 = load i64, i64* %48, align 8
  %50 = icmp ult i64 1, %49
  br i1 %50, label %BoundsCheckOk15, label %BoundsCheckFail14

BoundsCheckFail14:                                ; preds = %CheckOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk15:                                  ; preds = %CheckOk
  %51 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %52 = load i8*, i8** %51, align 8
  %53 = getelementptr inbounds i8, i8* %52, i64 1
  %54 = load i8, i8* %53, align 1
  %55 = and i8 %54, 63
  %56 = zext i8 %55 to i21
  %57 = or i21 %47, %56
  store i21 %57, i21* %value, align 4
  %58 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %59 = load i64, i64* %58, align 8
  %60 = icmp ult i64 2, %59
  br i1 %60, label %BoundsCheckOk17, label %BoundsCheckFail16

BoundsCheckFail16:                                ; preds = %BoundsCheckOk15
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk17:                                  ; preds = %BoundsCheckOk15
  %61 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %62 = load i8*, i8** %61, align 8
  %63 = getelementptr inbounds i8, i8* %62, i64 2
  %64 = load i8, i8* %63, align 1
  %65 = and i8 %64, -64
  %66 = icmp ne i8 %65, -128
  br i1 %66, label %Then1, label %Else2

CheckFail18:                                      ; preds = %EndIf3
  call fastcc void @std.builtin.default_panic(%"[]u8"* @53, %std.builtin.StackTrace* null)
  unreachable

CheckOk19:                                        ; preds = %EndIf3
  %67 = shl i21 %11, 6
  store i21 %67, i21* %value, align 4
  %68 = load i21, i21* %value, align 4
  %69 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %70 = load i64, i64* %69, align 8
  %71 = icmp ult i64 2, %70
  br i1 %71, label %BoundsCheckOk21, label %BoundsCheckFail20

BoundsCheckFail20:                                ; preds = %CheckOk19
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk21:                                  ; preds = %CheckOk19
  %72 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %73 = load i8*, i8** %72, align 8
  %74 = getelementptr inbounds i8, i8* %73, i64 2
  %75 = load i8, i8* %74, align 1
  %76 = and i8 %75, 63
  %77 = zext i8 %76 to i21
  %78 = or i21 %68, %77
  store i21 %78, i21* %value, align 4
  %79 = load i21, i21* %value, align 4
  %80 = icmp ult i21 %79, 2048
  br i1 %80, label %Then4, label %Else5
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.unicode.utf8Decode4({ i21, i16 }* nonnull sret({ i21, i16 }) %0, %"[]u8"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %value = alloca i21, align 4
  %2 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %3 = load i64, i64* %2, align 8
  %4 = icmp eq i64 %3, 4
  call fastcc void @std.debug.assert(i1 %4)
  %5 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %6 = load i64, i64* %5, align 8
  %7 = icmp ult i64 0, %6
  br i1 %7, label %BoundsCheckOk, label %BoundsCheckFail

Then:                                             ; preds = %BoundsCheckOk16
  %8 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 16, i16* %8, align 2
  ret void

Else:                                             ; preds = %BoundsCheckOk16
  br label %EndIf

EndIf:                                            ; preds = %Else
  %9 = load i21, i21* %value, align 4
  br i1 true, label %CheckOk, label %CheckFail

Then1:                                            ; preds = %BoundsCheckOk20
  %10 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 16, i16* %10, align 2
  ret void

Else2:                                            ; preds = %BoundsCheckOk20
  br label %EndIf3

EndIf3:                                           ; preds = %Else2
  %11 = load i21, i21* %value, align 4
  br i1 true, label %CheckOk22, label %CheckFail21

Then4:                                            ; preds = %BoundsCheckOk26
  %12 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 16, i16* %12, align 2
  ret void

Else5:                                            ; preds = %BoundsCheckOk26
  br label %EndIf6

EndIf6:                                           ; preds = %Else5
  %13 = load i21, i21* %value, align 4
  br i1 true, label %CheckOk28, label %CheckFail27

Then7:                                            ; preds = %BoundsCheckOk30
  %14 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 17, i16* %14, align 2
  ret void

Else8:                                            ; preds = %BoundsCheckOk30
  br label %EndIf9

EndIf9:                                           ; preds = %Else8
  %15 = load i21, i21* %value, align 4
  %16 = icmp ugt i21 %15, -983041
  br i1 %16, label %Then10, label %Else11

Then10:                                           ; preds = %EndIf9
  %17 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 19, i16* %17, align 2
  ret void

Else11:                                           ; preds = %EndIf9
  br label %EndIf12

EndIf12:                                          ; preds = %Else11
  %18 = load i21, i21* %value, align 4
  %19 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 1
  store i16 0, i16* %19, align 2
  %20 = getelementptr inbounds { i21, i16 }, { i21, i16 }* %0, i32 0, i32 0
  store i21 %18, i21* %20, align 4
  ret void

BoundsCheckFail:                                  ; preds = %Entry
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %Entry
  %21 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %22 = load i8*, i8** %21, align 8
  %23 = getelementptr inbounds i8, i8* %22, i64 0
  %24 = load i8, i8* %23, align 1
  %25 = and i8 %24, -8
  %26 = icmp eq i8 %25, -16
  call fastcc void @std.debug.assert(i1 %26)
  %27 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %28 = load i64, i64* %27, align 8
  %29 = icmp ult i64 0, %28
  br i1 %29, label %BoundsCheckOk14, label %BoundsCheckFail13

BoundsCheckFail13:                                ; preds = %BoundsCheckOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk14:                                  ; preds = %BoundsCheckOk
  %30 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %31 = load i8*, i8** %30, align 8
  %32 = getelementptr inbounds i8, i8* %31, i64 0
  %33 = load i8, i8* %32, align 1
  %34 = and i8 %33, 7
  %35 = zext i8 %34 to i21
  store i21 %35, i21* %value, align 4
  %36 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %37 = load i64, i64* %36, align 8
  %38 = icmp ult i64 1, %37
  br i1 %38, label %BoundsCheckOk16, label %BoundsCheckFail15

BoundsCheckFail15:                                ; preds = %BoundsCheckOk14
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk16:                                  ; preds = %BoundsCheckOk14
  %39 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %40 = load i8*, i8** %39, align 8
  %41 = getelementptr inbounds i8, i8* %40, i64 1
  %42 = load i8, i8* %41, align 1
  %43 = and i8 %42, -64
  %44 = icmp ne i8 %43, -128
  br i1 %44, label %Then, label %Else

CheckFail:                                        ; preds = %EndIf
  call fastcc void @std.builtin.default_panic(%"[]u8"* @53, %std.builtin.StackTrace* null)
  unreachable

CheckOk:                                          ; preds = %EndIf
  %45 = shl i21 %9, 6
  store i21 %45, i21* %value, align 4
  %46 = load i21, i21* %value, align 4
  %47 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %48 = load i64, i64* %47, align 8
  %49 = icmp ult i64 1, %48
  br i1 %49, label %BoundsCheckOk18, label %BoundsCheckFail17

BoundsCheckFail17:                                ; preds = %CheckOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk18:                                  ; preds = %CheckOk
  %50 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %51 = load i8*, i8** %50, align 8
  %52 = getelementptr inbounds i8, i8* %51, i64 1
  %53 = load i8, i8* %52, align 1
  %54 = and i8 %53, 63
  %55 = zext i8 %54 to i21
  %56 = or i21 %46, %55
  store i21 %56, i21* %value, align 4
  %57 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %58 = load i64, i64* %57, align 8
  %59 = icmp ult i64 2, %58
  br i1 %59, label %BoundsCheckOk20, label %BoundsCheckFail19

BoundsCheckFail19:                                ; preds = %BoundsCheckOk18
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk20:                                  ; preds = %BoundsCheckOk18
  %60 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %61 = load i8*, i8** %60, align 8
  %62 = getelementptr inbounds i8, i8* %61, i64 2
  %63 = load i8, i8* %62, align 1
  %64 = and i8 %63, -64
  %65 = icmp ne i8 %64, -128
  br i1 %65, label %Then1, label %Else2

CheckFail21:                                      ; preds = %EndIf3
  call fastcc void @std.builtin.default_panic(%"[]u8"* @53, %std.builtin.StackTrace* null)
  unreachable

CheckOk22:                                        ; preds = %EndIf3
  %66 = shl i21 %11, 6
  store i21 %66, i21* %value, align 4
  %67 = load i21, i21* %value, align 4
  %68 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %69 = load i64, i64* %68, align 8
  %70 = icmp ult i64 2, %69
  br i1 %70, label %BoundsCheckOk24, label %BoundsCheckFail23

BoundsCheckFail23:                                ; preds = %CheckOk22
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk24:                                  ; preds = %CheckOk22
  %71 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %72 = load i8*, i8** %71, align 8
  %73 = getelementptr inbounds i8, i8* %72, i64 2
  %74 = load i8, i8* %73, align 1
  %75 = and i8 %74, 63
  %76 = zext i8 %75 to i21
  %77 = or i21 %67, %76
  store i21 %77, i21* %value, align 4
  %78 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %79 = load i64, i64* %78, align 8
  %80 = icmp ult i64 3, %79
  br i1 %80, label %BoundsCheckOk26, label %BoundsCheckFail25

BoundsCheckFail25:                                ; preds = %BoundsCheckOk24
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk26:                                  ; preds = %BoundsCheckOk24
  %81 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %82 = load i8*, i8** %81, align 8
  %83 = getelementptr inbounds i8, i8* %82, i64 3
  %84 = load i8, i8* %83, align 1
  %85 = and i8 %84, -64
  %86 = icmp ne i8 %85, -128
  br i1 %86, label %Then4, label %Else5

CheckFail27:                                      ; preds = %EndIf6
  call fastcc void @std.builtin.default_panic(%"[]u8"* @53, %std.builtin.StackTrace* null)
  unreachable

CheckOk28:                                        ; preds = %EndIf6
  %87 = shl i21 %13, 6
  store i21 %87, i21* %value, align 4
  %88 = load i21, i21* %value, align 4
  %89 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 1
  %90 = load i64, i64* %89, align 8
  %91 = icmp ult i64 3, %90
  br i1 %91, label %BoundsCheckOk30, label %BoundsCheckFail29

BoundsCheckFail29:                                ; preds = %CheckOk28
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk30:                                  ; preds = %CheckOk28
  %92 = getelementptr inbounds %"[]u8", %"[]u8"* %1, i32 0, i32 0
  %93 = load i8*, i8** %92, align 8
  %94 = getelementptr inbounds i8, i8* %93, i64 3
  %95 = load i8, i8* %94, align 1
  %96 = and i8 %95, 63
  %97 = zext i8 %96 to i21
  %98 = or i21 %88, %97
  store i21 %98, i21* %value, align 4
  %99 = load i21, i21* %value, align 4
  %100 = icmp ult i21 %99, 65536
  br i1 %100, label %Then7, label %Else8
}

; Function Attrs: nobuiltin nounwind
define internal fastcc void @std.mem.set(%"[]u8"* nonnull readonly align 8 %0, i8 %1) unnamed_addr #1 {
Entry:
  %i = alloca i64, align 8
  %d = alloca i8*, align 8
  %value = alloca i8, align 1
  store i8 %1, i8* %value, align 1
  store i64 0, i64* %i, align 8
  %2 = getelementptr inbounds %"[]u8", %"[]u8"* %0, i32 0, i32 1
  %3 = load i64, i64* %2, align 8
  br label %ForCond

ForCond:                                          ; preds = %ForBody, %Entry
  %4 = load i64, i64* %i, align 8
  %5 = icmp ult i64 %4, %3
  br i1 %5, label %ForBody, label %ForEnd

ForBody:                                          ; preds = %ForCond
  %6 = getelementptr inbounds %"[]u8", %"[]u8"* %0, i32 0, i32 0
  %7 = load i8*, i8** %6, align 8
  %8 = getelementptr inbounds i8, i8* %7, i64 %4
  store i8* %8, i8** %d, align 8
  %9 = load i8*, i8** %d, align 8
  %10 = load i8, i8* %value, align 1
  store i8 %10, i8* %9, align 1
  %11 = add nuw i64 %4, 1
  store i64 %11, i64* %i, align 8
  br label %ForCond

ForEnd:                                           ; preds = %ForCond
  ret void
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.math.min.19(i64 %0, i64 %1) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %x = alloca i64, align 8
  %y = alloca i64, align 8
  store i64 %0, i64* %x, align 8
  store i64 %1, i64* %y, align 8
  %2 = load i64, i64* %x, align 8
  %3 = load i64, i64* %y, align 8
  %4 = icmp ult i64 %2, %3
  br i1 %4, label %Then, label %Else

Then:                                             ; preds = %Entry
  %5 = load i64, i64* %x, align 8
  store i64 %5, i64* %result, align 8
  %6 = load i64, i64* %result, align 8
  ret i64 %6

Else:                                             ; preds = %Entry
  %7 = load i64, i64* %y, align 8
  store i64 %7, i64* %result, align 8
  %8 = load i64, i64* %result, align 8
  ret i64 %8
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i64 @std.os.linux.futex_wake(i32* nonnull readonly align 4 %0, i32 %1, i32 %2) unnamed_addr #1 {
Entry:
  %result = alloca i64, align 8
  %uaddr = alloca i32*, align 8
  %futex_op = alloca i32, align 4
  %val = alloca i32, align 4
  store i32* %0, i32** %uaddr, align 8
  store i32 %1, i32* %futex_op, align 4
  store i32 %2, i32* %val, align 4
  %3 = load i32*, i32** %uaddr, align 8
  %4 = ptrtoint i32* %3 to i64
  %5 = load i32, i32* %futex_op, align 4
  %6 = zext i32 %5 to i64
  %7 = load i32, i32* %val, align 4
  %8 = zext i32 %7 to i64
  %9 = call fastcc i64 @std.os.linux.x86_64.syscall3(i64 202, i64 %4, i64 %6, i64 %8)
  store i64 %9, i64* %result, align 8
  %10 = load i64, i64* %result, align 8
  ret i64 %10
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #8

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.Thread.StaticResetEvent.LinuxFutex.wait(i32* nonnull align 4 %0, %"?u64"* nonnull readonly align 8 %1) unnamed_addr #1 {
Entry:
  %2 = alloca [6 x i64], align 8
  %result = alloca i16, align 2
  %ts = alloca %std.os.linux.timespec, align 8
  %ts_ptr = alloca %std.os.linux.timespec*, align 8
  %timeout_ns = alloca i64, align 8
  %waiting = alloca i32, align 4
  %expected = alloca i32, align 4
  %ptr = alloca i32*, align 8
  %rc = alloca i64, align 8
  %3 = alloca i16, align 2
  %waiters = alloca i32*, align 8
  store i32* %0, i32** %waiters, align 8
  %4 = bitcast %std.os.linux.timespec* %ts to i8*
  call void @llvm.memset.p0i8.i64(i8* align 8 %4, i8 -86, i64 16, i1 false)
  %5 = ptrtoint i8* %4 to i64
  %6 = getelementptr inbounds [6 x i64], [6 x i64]* %2, i64 0, i64 0
  store i64 1296236545, i64* %6, align 8
  %7 = getelementptr inbounds [6 x i64], [6 x i64]* %2, i64 0, i64 1
  store i64 %5, i64* %7, align 8
  %8 = getelementptr inbounds [6 x i64], [6 x i64]* %2, i64 0, i64 2
  store i64 16, i64* %8, align 8
  %9 = getelementptr inbounds [6 x i64], [6 x i64]* %2, i64 0, i64 3
  store i64 0, i64* %9, align 8
  %10 = getelementptr inbounds [6 x i64], [6 x i64]* %2, i64 0, i64 4
  store i64 0, i64* %10, align 8
  %11 = getelementptr inbounds [6 x i64], [6 x i64]* %2, i64 0, i64 5
  store i64 0, i64* %11, align 8
  %12 = ptrtoint [6 x i64]* %2 to i64
  %13 = call i64 asm sideeffect "rolq $$3,  %rdi ; rolq $$13, %rdi\0Arolq $$61, %rdi ; rolq $$51, %rdi\0Axchgq %rbx,%rbx\0A", "={rdx},{rax},0,~{cc},~{memory}"(i64 %12, i64 0)
  store %std.os.linux.timespec* null, %std.os.linux.timespec** %ts_ptr, align 8
  %14 = getelementptr inbounds %"?u64", %"?u64"* %1, i32 0, i32 1
  %15 = load i1, i1* %14, align 1
  br i1 %15, label %OptionalThen, label %OptionalElse

OptionalThen:                                     ; preds = %Entry
  %16 = getelementptr inbounds %"?u64", %"?u64"* %1, i32 0, i32 0
  %17 = load i64, i64* %16, align 8
  store i64 %17, i64* %timeout_ns, align 8
  store %std.os.linux.timespec* %ts, %std.os.linux.timespec** %ts_ptr, align 8
  %18 = getelementptr inbounds %std.os.linux.timespec, %std.os.linux.timespec* %ts, i32 0, i32 0
  %19 = load i64, i64* %timeout_ns, align 8
  br i1 false, label %DivZeroFail, label %DivZeroOk

OptionalElse:                                     ; preds = %Entry
  br label %OptionalEndIf

OptionalEndIf:                                    ; preds = %OptionalElse, %SignCastOk4
  br label %WhileCond

WhileCond:                                        ; preds = %SwitchProng2, %SwitchProng, %OptionalEndIf
  br label %WhileBody

WhileBody:                                        ; preds = %WhileCond
  %20 = load i32*, i32** %waiters, align 8
  %21 = load atomic i32, i32* %20 acquire, align 4
  store i32 %21, i32* %waiting, align 4
  %22 = load i32, i32* %waiting, align 4
  %23 = icmp eq i32 %22, 1
  br i1 %23, label %Then, label %Else

Then:                                             ; preds = %WhileBody
  store i16 0, i16* %result, align 2
  %24 = load i16, i16* %result, align 2
  ret i16 %24

Else:                                             ; preds = %WhileBody
  br label %EndIf

EndIf:                                            ; preds = %Else
  %25 = load i32, i32* %waiting, align 4
  %26 = icmp sge i32 %25, 0
  br i1 %26, label %SignCastOk6, label %SignCastFail7

SwitchElse:                                       ; preds = %PtrCastOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @7, %std.builtin.StackTrace* null)
  unreachable

SwitchProng:                                      ; preds = %PtrCastOk
  br label %WhileCond

SwitchProng1:                                     ; preds = %PtrCastOk
  store i16 20, i16* %result, align 2
  %27 = load i16, i16* %result, align 2
  ret i16 %27

SwitchProng2:                                     ; preds = %PtrCastOk
  br label %WhileCond

SwitchProng3:                                     ; preds = %PtrCastOk
  store i16 0, i16* %result, align 2
  %28 = load i16, i16* %result, align 2
  ret i16 %28

DivZeroFail:                                      ; preds = %OptionalThen
  call fastcc void @std.builtin.default_panic(%"[]u8"* @11, %std.builtin.StackTrace* null)
  unreachable

DivZeroOk:                                        ; preds = %OptionalThen
  %29 = udiv i64 %19, 1000000000
  %30 = icmp sge i64 %29, 0
  br i1 %30, label %SignCastOk, label %SignCastFail

SignCastOk:                                       ; preds = %DivZeroOk
  store i64 %29, i64* %18, align 8
  %31 = getelementptr inbounds %std.os.linux.timespec, %std.os.linux.timespec* %ts, i32 0, i32 1
  %32 = load i64, i64* %timeout_ns, align 8
  br i1 false, label %RemZeroFail, label %RemZeroOk

SignCastFail:                                     ; preds = %DivZeroOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @15, %std.builtin.StackTrace* null)
  unreachable

RemZeroOk:                                        ; preds = %SignCastOk
  %33 = urem i64 %32, 1000000000
  %34 = icmp sge i64 %33, 0
  br i1 %34, label %SignCastOk4, label %SignCastFail5

RemZeroFail:                                      ; preds = %SignCastOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @13, %std.builtin.StackTrace* null)
  unreachable

SignCastOk4:                                      ; preds = %RemZeroOk
  store i64 %33, i64* %31, align 8
  br label %OptionalEndIf

SignCastFail5:                                    ; preds = %RemZeroOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @15, %std.builtin.StackTrace* null)
  unreachable

SignCastOk6:                                      ; preds = %EndIf
  store i32 %25, i32* %expected, align 4
  %35 = load i32*, i32** %waiters, align 8
  %36 = icmp ne i32* %35, null
  br i1 %36, label %PtrCastOk, label %PtrCastFail

SignCastFail7:                                    ; preds = %EndIf
  call fastcc void @std.builtin.default_panic(%"[]u8"* @15, %std.builtin.StackTrace* null)
  unreachable

PtrCastFail:                                      ; preds = %SignCastOk6
  call fastcc void @std.builtin.default_panic(%"[]u8"* @47, %std.builtin.StackTrace* null)
  unreachable

PtrCastOk:                                        ; preds = %SignCastOk6
  store i32* %35, i32** %ptr, align 8
  %37 = load i32*, i32** %ptr, align 8
  %38 = load i32, i32* %expected, align 4
  %39 = load %std.os.linux.timespec*, %std.os.linux.timespec** %ts_ptr, align 8
  %40 = call fastcc i64 @std.os.linux.futex_wait(i32* %37, i32 128, i32 %38, %std.os.linux.timespec* %39)
  store i64 %40, i64* %rc, align 8
  %41 = load i64, i64* %rc, align 8
  %42 = call fastcc i16 @std.os.linux.getErrno(i64 %41)
  store i16 %42, i16* %3, align 2
  switch i16 %42, label %SwitchElse [
    i16 0, label %SwitchProng
    i16 110, label %SwitchProng1
    i16 4, label %SwitchProng2
    i16 11, label %SwitchProng3
  ]
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).print.20"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0, %"std.os.struct:4985:53"* nonnull readonly align 2 %1) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %2 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %3 = alloca %"std.os.struct:4985:53", align 2
  %4 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0 to i8*
  %5 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %5, i8* align 4 %4, i64 4, i1 false)
  %6 = bitcast %"std.os.struct:4985:53"* %1 to i8*
  %7 = bitcast %"std.os.struct:4985:53"* %3 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 2 %7, i8* align 2 %6, i64 2, i1 false)
  %8 = call fastcc i16 @std.fmt.format.21(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"std.os.struct:4985:53"* %1)
  store i16 %8, i16* %result, align 2
  %9 = load i16, i16* %result, align 2
  ret i16 %9
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.format.21(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %0, %"std.os.struct:4985:53"* nonnull readonly align 2 %1) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %2 = alloca i16, align 2
  %3 = alloca i16, align 2
  %4 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %5 = alloca i16, align 2
  %6 = alloca i16, align 2
  %7 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"[]u8"* @55)
  store i16 %7, i16* %2, align 2
  %8 = icmp ne i16 %7, 0
  br i1 %8, label %ErrRetReturn, label %ErrRetContinue

ErrRetReturn:                                     ; preds = %Entry
  %9 = load i16, i16* %2, align 2
  store i16 %9, i16* %result, align 2
  ret i16 %9

ErrRetContinue:                                   ; preds = %Entry
  %10 = getelementptr inbounds %"std.os.struct:4985:53", %"std.os.struct:4985:53"* %1, i32 0, i32 0
  %11 = load i16, i16* %10, align 2
  store i16 %11, i16* %3, align 2
  %12 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0 to i8*
  %13 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %4 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %13, i8* align 4 %12, i64 4, i1 false)
  %14 = call fastcc i16 @std.fmt.formatType.22(i16 %11, %std.fmt.FormatOptions* bitcast ({ %"?usize", %"?usize", i2, <{ i8, [6 x i8] }> }* @59 to %std.fmt.FormatOptions*), %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, i64 3)
  store i16 %14, i16* %5, align 2
  %15 = icmp ne i16 %14, 0
  br i1 %15, label %ErrRetReturn1, label %ErrRetContinue2

ErrRetReturn1:                                    ; preds = %ErrRetContinue
  %16 = load i16, i16* %5, align 2
  store i16 %16, i16* %result, align 2
  ret i16 %16

ErrRetContinue2:                                  ; preds = %ErrRetContinue
  %17 = call fastcc i16 @"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write).writeAll"(%"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %0, %"[]u8"* @60)
  store i16 %17, i16* %6, align 2
  %18 = icmp ne i16 %17, 0
  br i1 %18, label %ErrRetReturn3, label %ErrRetContinue4

ErrRetReturn3:                                    ; preds = %ErrRetContinue2
  %19 = load i16, i16* %6, align 2
  store i16 %19, i16* %result, align 2
  ret i16 %19

ErrRetContinue4:                                  ; preds = %ErrRetContinue2
  store i16 0, i16* %result, align 2
  ret i16 0
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.formatType.22(i16 %0, %std.fmt.FormatOptions* nonnull readonly align 8 %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %2, i64 %3) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %4 = alloca i16, align 2
  %5 = alloca %std.fmt.FormatOptions, align 8
  %6 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %value = alloca i16, align 2
  %max_depth = alloca i64, align 8
  store i16 %0, i16* %value, align 2
  store i64 %3, i64* %max_depth, align 8
  %7 = load i16, i16* %value, align 2
  store i16 %7, i16* %4, align 2
  %8 = bitcast %std.fmt.FormatOptions* %1 to i8*
  %9 = bitcast %std.fmt.FormatOptions* %5 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %9, i8* align 8 %8, i64 40, i1 false)
  %10 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2 to i8*
  %11 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %6 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %11, i8* align 4 %10, i64 4, i1 false)
  %12 = call fastcc i16 @std.fmt.formatValue.23(i16 %7, %std.fmt.FormatOptions* %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2)
  store i16 %12, i16* %result, align 2
  %13 = load i16, i16* %result, align 2
  ret i16 %13
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.formatValue.23(i16 %0, %std.fmt.FormatOptions* nonnull readonly align 8 %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %2) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %3 = alloca i16, align 2
  %4 = alloca %std.fmt.FormatOptions, align 8
  %5 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %value = alloca i16, align 2
  store i16 %0, i16* %value, align 2
  %6 = load i16, i16* %value, align 2
  store i16 %6, i16* %3, align 2
  %7 = bitcast %std.fmt.FormatOptions* %1 to i8*
  %8 = bitcast %std.fmt.FormatOptions* %4 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %8, i8* align 8 %7, i64 40, i1 false)
  %9 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2 to i8*
  %10 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %5 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %10, i8* align 4 %9, i64 4, i1 false)
  %11 = call fastcc i16 @std.fmt.formatIntValue.24(i16 %6, %std.fmt.FormatOptions* %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2)
  store i16 %11, i16* %result, align 2
  %12 = load i16, i16* %result, align 2
  ret i16 %12
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.formatIntValue.24(i16 %0, %std.fmt.FormatOptions* nonnull readonly align 8 %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %2) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %int_value = alloca i16, align 2
  %3 = alloca i16, align 2
  %4 = alloca %std.fmt.FormatOptions, align 8
  %5 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %value = alloca i16, align 2
  store i16 %0, i16* %value, align 2
  %6 = load i16, i16* %value, align 2
  store i16 %6, i16* %int_value, align 2
  %7 = load i16, i16* %int_value, align 2
  store i16 %7, i16* %3, align 2
  %8 = bitcast %std.fmt.FormatOptions* %1 to i8*
  %9 = bitcast %std.fmt.FormatOptions* %4 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %9, i8* align 8 %8, i64 40, i1 false)
  %10 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2 to i8*
  %11 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %5 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %11, i8* align 4 %10, i64 4, i1 false)
  %12 = call fastcc i16 @std.fmt.formatInt.25(i16 %7, i8 10, i1 false, %std.fmt.FormatOptions* %1, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %2)
  store i16 %12, i16* %result, align 2
  %13 = load i16, i16* %result, align 2
  ret i16 %13
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.fmt.formatInt.25(i16 %0, i8 %1, i1 %2, %std.fmt.FormatOptions* nonnull readonly align 8 %3, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* nonnull readonly align 4 %4) unnamed_addr #1 {
Entry:
  %5 = alloca [6 x i64], align 8
  %result = alloca i16, align 2
  %int_value = alloca i16, align 2
  %6 = alloca i16, align 2
  %abs_value = alloca i16, align 2
  %buf = alloca [17 x i8], align 1
  %a = alloca i16, align 2
  %index = alloca i64, align 8
  %digit = alloca i16, align 2
  %7 = alloca %"[]u8", align 8
  %8 = alloca %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)", align 4
  %value = alloca i16, align 2
  %base = alloca i8, align 1
  %case = alloca i1, align 1
  store i16 %0, i16* %value, align 2
  store i8 %1, i8* %base, align 1
  store i1 %2, i1* %case, align 1
  %9 = load i8, i8* %base, align 1
  %10 = icmp uge i8 %9, 2
  call fastcc void @std.debug.assert(i1 %10)
  %11 = load i16, i16* %value, align 2
  store i16 %11, i16* %int_value, align 2
  %12 = load i16, i16* %int_value, align 2
  store i16 %12, i16* %6, align 2
  %13 = call fastcc i16 @std.math.absCast.26(i16 %12)
  store i16 %13, i16* %abs_value, align 2
  %14 = bitcast [17 x i8]* %buf to i8*
  call void @llvm.memset.p0i8.i64(i8* align 1 %14, i8 -86, i64 17, i1 false)
  %15 = ptrtoint i8* %14 to i64
  %16 = getelementptr inbounds [6 x i64], [6 x i64]* %5, i64 0, i64 0
  store i64 1296236545, i64* %16, align 8
  %17 = getelementptr inbounds [6 x i64], [6 x i64]* %5, i64 0, i64 1
  store i64 %15, i64* %17, align 8
  %18 = getelementptr inbounds [6 x i64], [6 x i64]* %5, i64 0, i64 2
  store i64 17, i64* %18, align 8
  %19 = getelementptr inbounds [6 x i64], [6 x i64]* %5, i64 0, i64 3
  store i64 0, i64* %19, align 8
  %20 = getelementptr inbounds [6 x i64], [6 x i64]* %5, i64 0, i64 4
  store i64 0, i64* %20, align 8
  %21 = getelementptr inbounds [6 x i64], [6 x i64]* %5, i64 0, i64 5
  store i64 0, i64* %21, align 8
  %22 = ptrtoint [6 x i64]* %5 to i64
  %23 = call i64 asm sideeffect "rolq $$3,  %rdi ; rolq $$13, %rdi\0Arolq $$61, %rdi ; rolq $$51, %rdi\0Axchgq %rbx,%rbx\0A", "={rdx},{rax},0,~{cc},~{memory}"(i64 %22, i64 0)
  %24 = load i16, i16* %abs_value, align 2
  store i16 %24, i16* %a, align 2
  store i64 17, i64* %index, align 8
  br label %WhileCond

WhileCond:                                        ; preds = %EndIf, %Entry
  br label %WhileBody

WhileBody:                                        ; preds = %WhileCond
  %25 = load i16, i16* %a, align 2
  %26 = load i8, i8* %base, align 1
  %27 = zext i8 %26 to i16
  %28 = icmp eq i16 %27, 0
  br i1 %28, label %RemZeroFail, label %RemZeroOk

Then:                                             ; preds = %DivZeroOk
  br label %WhileEnd

Else:                                             ; preds = %DivZeroOk
  br label %EndIf

EndIf:                                            ; preds = %Else
  br label %WhileCond

WhileEnd:                                         ; preds = %Then
  %29 = load i64, i64* %index, align 8
  %30 = icmp ule i64 %29, 17
  br i1 %30, label %BoundsCheckOk2, label %BoundsCheckFail1

RemZeroOk:                                        ; preds = %WhileBody
  %31 = urem i16 %25, %27
  store i16 %31, i16* %digit, align 2
  %32 = load i64, i64* %index, align 8
  %33 = call { i64, i1 } @llvm.usub.with.overflow.i64(i64 %32, i64 1)
  %34 = extractvalue { i64, i1 } %33, 0
  %35 = extractvalue { i64, i1 } %33, 1
  br i1 %35, label %OverflowFail, label %OverflowOk

RemZeroFail:                                      ; preds = %WhileBody
  call fastcc void @std.builtin.default_panic(%"[]u8"* @13, %std.builtin.StackTrace* null)
  unreachable

OverflowFail:                                     ; preds = %RemZeroOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @23, %std.builtin.StackTrace* null)
  unreachable

OverflowOk:                                       ; preds = %RemZeroOk
  store i64 %34, i64* %index, align 8
  %36 = load i64, i64* %index, align 8
  %37 = icmp ult i64 %36, 17
  br i1 %37, label %BoundsCheckOk, label %BoundsCheckFail

BoundsCheckFail:                                  ; preds = %OverflowOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk:                                    ; preds = %OverflowOk
  %38 = getelementptr inbounds [17 x i8], [17 x i8]* %buf, i64 0, i64 %36
  %39 = load i16, i16* %digit, align 2
  %40 = trunc i16 %39 to i8
  %41 = zext i8 %40 to i16
  %42 = icmp eq i16 %39, %41
  br i1 %42, label %CastShortenOk, label %CastShortenFail

CastShortenOk:                                    ; preds = %BoundsCheckOk
  %43 = load i1, i1* %case, align 1
  %44 = call fastcc i8 @std.fmt.digitToChar(i8 %40, i1 %43)
  store i8 %44, i8* %38, align 1
  %45 = load i16, i16* %a, align 2
  %46 = load i8, i8* %base, align 1
  %47 = zext i8 %46 to i16
  %48 = icmp eq i16 %47, 0
  br i1 %48, label %DivZeroFail, label %DivZeroOk

CastShortenFail:                                  ; preds = %BoundsCheckOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @15, %std.builtin.StackTrace* null)
  unreachable

DivZeroFail:                                      ; preds = %CastShortenOk
  call fastcc void @std.builtin.default_panic(%"[]u8"* @11, %std.builtin.StackTrace* null)
  unreachable

DivZeroOk:                                        ; preds = %CastShortenOk
  %49 = udiv i16 %45, %47
  store i16 %49, i16* %a, align 2
  %50 = load i16, i16* %a, align 2
  %51 = icmp eq i16 %50, 0
  br i1 %51, label %Then, label %Else

BoundsCheckFail1:                                 ; preds = %WhileEnd
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk2:                                   ; preds = %WhileEnd
  br i1 true, label %BoundsCheckOk4, label %BoundsCheckFail3

BoundsCheckFail3:                                 ; preds = %BoundsCheckOk2
  call fastcc void @std.builtin.default_panic(%"[]u8"* @4, %std.builtin.StackTrace* null)
  unreachable

BoundsCheckOk4:                                   ; preds = %BoundsCheckOk2
  %52 = getelementptr inbounds [17 x i8], [17 x i8]* %buf, i64 0, i64 %29
  %53 = sub nuw i64 17, %29
  %54 = getelementptr inbounds %"[]u8", %"[]u8"* %7, i32 0, i32 0
  store i8* %52, i8** %54, align 8
  %55 = getelementptr inbounds %"[]u8", %"[]u8"* %7, i32 0, i32 1
  store i64 %53, i64* %55, align 8
  %56 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %4 to i8*
  %57 = bitcast %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %8 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %57, i8* align 4 %56, i64 4, i1 false)
  %58 = call fastcc i16 @std.fmt.formatBuf(%"[]u8"* %7, %std.fmt.FormatOptions* %3, %"std.io.writer.Writer(std.fs.file.File,std.os.WriteError,std.fs.file.File.write)"* %4)
  store i16 %58, i16* %result, align 2
  %59 = load i16, i16* %result, align 2
  ret i16 %59
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i16 @std.math.absCast.26(i16 %0) unnamed_addr #1 {
Entry:
  %result = alloca i16, align 2
  %x = alloca i16, align 2
  store i16 %0, i16* %x, align 2
  %1 = load i16, i16* %x, align 2
  store i16 %1, i16* %result, align 2
  %2 = load i16, i16* %result, align 2
  ret i16 %2
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i1 @std.target.Target.isDarwin(%std.target.Target* nonnull readonly align 8 %0) unnamed_addr #1 {
Entry:
  %result = alloca i1, align 1
  %1 = getelementptr inbounds %std.target.Target, %std.target.Target* %0, i32 0, i32 1
  %2 = getelementptr inbounds %std.target.Os, %std.target.Os* %1, i32 0, i32 0
  %3 = load i6, i6* %2, align 1
  %4 = call fastcc i1 @std.target.Tag.isDarwin(i6 %3)
  store i1 %4, i1* %result, align 1
  %5 = load i1, i1* %result, align 1
  ret i1 %5
}

; Function Attrs: nobuiltin nounwind
define internal fastcc i1 @std.target.Tag.isDarwin(i6 %0) unnamed_addr #1 {
Entry:
  %result = alloca i1, align 1
  %tag = alloca i6, align 1
  store i6 %0, i6* %tag, align 1
  %1 = load i6, i6* %tag, align 1
  switch i6 %1, label %SwitchElse [
    i6 6, label %SwitchProng
    i6 10, label %SwitchProng
    i6 27, label %SwitchProng
    i6 26, label %SwitchProng
  ]

SwitchElse:                                       ; preds = %Entry
  store i1 false, i1* %result, align 1
  br label %SwitchEnd

SwitchProng:                                      ; preds = %Entry, %Entry, %Entry, %Entry
  store i1 true, i1* %result, align 1
  br label %SwitchEnd

SwitchEnd:                                        ; preds = %SwitchProng, %SwitchElse
  %2 = load i1, i1* %result, align 1
  ret i1 %2
}

attributes #0 = { cold nobuiltin noreturn nounwind "frame-pointer"="all" "probe-stack"="__zig_probe_stack" "target-cpu"="x86-64" "target-features"="-16bit-mode,-32bit-mode,-3dnow,-3dnowa,+64bit,+adx,+aes,-amx-bf16,-amx-int8,-amx-tile,+avx,+avx2,-avx512bf16,-avx512bitalg,-avx512bw,-avx512cd,-avx512dq,-avx512er,-avx512f,-avx512ifma,-avx512pf,-avx512vbmi,-avx512vbmi2,-avx512vl,-avx512vnni,-avx512vp2intersect,-avx512vpopcntdq,-avxvnni,+bmi,+bmi2,-branchfusion,-cldemote,+clflushopt,+clwb,-clzero,+cmov,+cx16,+cx8,-enqcmd,-ermsb,+f16c,-false-deps-lzcnt-tzcnt,-false-deps-popcnt,-fast-11bytenop,-fast-15bytenop,-fast-7bytenop,-fast-bextr,-fast-gather,-fast-hops,-fast-lzcnt,-fast-movbe,-fast-scalar-fsqrt,-fast-scalar-shift-masks,-fast-shld-rotate,-fast-variable-crosslane-shuffle,-fast-variable-perlane-shuffle,-fast-vector-fsqrt,-fast-vector-shift-masks,+fma,-fma4,+fsgsbase,-fsrm,+fxsr,+gfni,-hreset,-idivl-to-divb,+idivq-to-divl,+invpcid,-kl,-lea-sp,-lea-uses-ag,-lvi-cfi,-lvi-load-hardening,-lwp,+lzcnt,+macrofusion,+mmx,+movbe,+movdir64b,+movdiri,-mwaitx,+nopl,-pad-short-functions,+pclmul,-pconfig,+pku,+popcnt,-prefer-128-bit,-prefer-256-bit,-prefer-mask-registers,-prefetchwt1,+prfchw,+ptwrite,+rdpid,+rdrnd,+rdseed,-retpoline,-retpoline-external-thunk,-retpoline-indirect-branches,-retpoline-indirect-calls,-rtm,+sahf,-serialize,-seses,-sgx,+sha,+shstk,+slow-3ops-lea,+slow-incdec,-slow-lea,-slow-pmaddwd,-slow-pmulld,-slow-shld,-slow-two-mem-ops,-slow-unaligned-mem-16,-slow-unaligned-mem-32,-soft-float,+sse,+sse2,+sse3,+sse4.1,+sse4.2,-sse4a,-sse-unaligned-mem,+ssse3,-tbm,-tsxldtrk,-uintr,-use-aa,-use-glm-div-sqrt-costs,+vaes,+vpclmulqdq,+vzeroupper,+waitpkg,-wbnoinvd,-widekl,+x87,-xop,+xsave,+xsavec,+xsaveopt,+xsaves" }
attributes #1 = { nobuiltin nounwind "frame-pointer"="all" "probe-stack"="__zig_probe_stack" "target-cpu"="x86-64" "target-features"="-16bit-mode,-32bit-mode,-3dnow,-3dnowa,+64bit,+adx,+aes,-amx-bf16,-amx-int8,-amx-tile,+avx,+avx2,-avx512bf16,-avx512bitalg,-avx512bw,-avx512cd,-avx512dq,-avx512er,-avx512f,-avx512ifma,-avx512pf,-avx512vbmi,-avx512vbmi2,-avx512vl,-avx512vnni,-avx512vp2intersect,-avx512vpopcntdq,-avxvnni,+bmi,+bmi2,-branchfusion,-cldemote,+clflushopt,+clwb,-clzero,+cmov,+cx16,+cx8,-enqcmd,-ermsb,+f16c,-false-deps-lzcnt-tzcnt,-false-deps-popcnt,-fast-11bytenop,-fast-15bytenop,-fast-7bytenop,-fast-bextr,-fast-gather,-fast-hops,-fast-lzcnt,-fast-movbe,-fast-scalar-fsqrt,-fast-scalar-shift-masks,-fast-shld-rotate,-fast-variable-crosslane-shuffle,-fast-variable-perlane-shuffle,-fast-vector-fsqrt,-fast-vector-shift-masks,+fma,-fma4,+fsgsbase,-fsrm,+fxsr,+gfni,-hreset,-idivl-to-divb,+idivq-to-divl,+invpcid,-kl,-lea-sp,-lea-uses-ag,-lvi-cfi,-lvi-load-hardening,-lwp,+lzcnt,+macrofusion,+mmx,+movbe,+movdir64b,+movdiri,-mwaitx,+nopl,-pad-short-functions,+pclmul,-pconfig,+pku,+popcnt,-prefer-128-bit,-prefer-256-bit,-prefer-mask-registers,-prefetchwt1,+prfchw,+ptwrite,+rdpid,+rdrnd,+rdseed,-retpoline,-retpoline-external-thunk,-retpoline-indirect-branches,-retpoline-indirect-calls,-rtm,+sahf,-serialize,-seses,-sgx,+sha,+shstk,+slow-3ops-lea,+slow-incdec,-slow-lea,-slow-pmaddwd,-slow-pmulld,-slow-shld,-slow-two-mem-ops,-slow-unaligned-mem-16,-slow-unaligned-mem-32,-soft-float,+sse,+sse2,+sse3,+sse4.1,+sse4.2,-sse4a,-sse-unaligned-mem,+ssse3,-tbm,-tsxldtrk,-uintr,-use-aa,-use-glm-div-sqrt-costs,+vaes,+vpclmulqdq,+vzeroupper,+waitpkg,-wbnoinvd,-widekl,+x87,-xop,+xsave,+xsavec,+xsaveopt,+xsaves" }
attributes #2 = { naked nobuiltin nounwind "frame-pointer"="all" "probe-stack"="__zig_probe_stack" "target-cpu"="x86-64" "target-features"="-16bit-mode,-32bit-mode,-3dnow,-3dnowa,+64bit,+adx,+aes,-amx-bf16,-amx-int8,-amx-tile,+avx,+avx2,-avx512bf16,-avx512bitalg,-avx512bw,-avx512cd,-avx512dq,-avx512er,-avx512f,-avx512ifma,-avx512pf,-avx512vbmi,-avx512vbmi2,-avx512vl,-avx512vnni,-avx512vp2intersect,-avx512vpopcntdq,-avxvnni,+bmi,+bmi2,-branchfusion,-cldemote,+clflushopt,+clwb,-clzero,+cmov,+cx16,+cx8,-enqcmd,-ermsb,+f16c,-false-deps-lzcnt-tzcnt,-false-deps-popcnt,-fast-11bytenop,-fast-15bytenop,-fast-7bytenop,-fast-bextr,-fast-gather,-fast-hops,-fast-lzcnt,-fast-movbe,-fast-scalar-fsqrt,-fast-scalar-shift-masks,-fast-shld-rotate,-fast-variable-crosslane-shuffle,-fast-variable-perlane-shuffle,-fast-vector-fsqrt,-fast-vector-shift-masks,+fma,-fma4,+fsgsbase,-fsrm,+fxsr,+gfni,-hreset,-idivl-to-divb,+idivq-to-divl,+invpcid,-kl,-lea-sp,-lea-uses-ag,-lvi-cfi,-lvi-load-hardening,-lwp,+lzcnt,+macrofusion,+mmx,+movbe,+movdir64b,+movdiri,-mwaitx,+nopl,-pad-short-functions,+pclmul,-pconfig,+pku,+popcnt,-prefer-128-bit,-prefer-256-bit,-prefer-mask-registers,-prefetchwt1,+prfchw,+ptwrite,+rdpid,+rdrnd,+rdseed,-retpoline,-retpoline-external-thunk,-retpoline-indirect-branches,-retpoline-indirect-calls,-rtm,+sahf,-serialize,-seses,-sgx,+sha,+shstk,+slow-3ops-lea,+slow-incdec,-slow-lea,-slow-pmaddwd,-slow-pmulld,-slow-shld,-slow-two-mem-ops,-slow-unaligned-mem-16,-slow-unaligned-mem-32,-soft-float,+sse,+sse2,+sse3,+sse4.1,+sse4.2,-sse4a,-sse-unaligned-mem,+ssse3,-tbm,-tsxldtrk,-uintr,-use-aa,-use-glm-div-sqrt-costs,+vaes,+vpclmulqdq,+vzeroupper,+waitpkg,-wbnoinvd,-widekl,+x87,-xop,+xsave,+xsavec,+xsaveopt,+xsaves" }
attributes #3 = { nofree nosync nounwind readnone willreturn }
attributes #4 = { argmemonly nofree nounwind willreturn }
attributes #5 = { nobuiltin noinline nounwind "frame-pointer"="all" "probe-stack"="__zig_probe_stack" "target-cpu"="x86-64" "target-features"="-16bit-mode,-32bit-mode,-3dnow,-3dnowa,+64bit,+adx,+aes,-amx-bf16,-amx-int8,-amx-tile,+avx,+avx2,-avx512bf16,-avx512bitalg,-avx512bw,-avx512cd,-avx512dq,-avx512er,-avx512f,-avx512ifma,-avx512pf,-avx512vbmi,-avx512vbmi2,-avx512vl,-avx512vnni,-avx512vp2intersect,-avx512vpopcntdq,-avxvnni,+bmi,+bmi2,-branchfusion,-cldemote,+clflushopt,+clwb,-clzero,+cmov,+cx16,+cx8,-enqcmd,-ermsb,+f16c,-false-deps-lzcnt-tzcnt,-false-deps-popcnt,-fast-11bytenop,-fast-15bytenop,-fast-7bytenop,-fast-bextr,-fast-gather,-fast-hops,-fast-lzcnt,-fast-movbe,-fast-scalar-fsqrt,-fast-scalar-shift-masks,-fast-shld-rotate,-fast-variable-crosslane-shuffle,-fast-variable-perlane-shuffle,-fast-vector-fsqrt,-fast-vector-shift-masks,+fma,-fma4,+fsgsbase,-fsrm,+fxsr,+gfni,-hreset,-idivl-to-divb,+idivq-to-divl,+invpcid,-kl,-lea-sp,-lea-uses-ag,-lvi-cfi,-lvi-load-hardening,-lwp,+lzcnt,+macrofusion,+mmx,+movbe,+movdir64b,+movdiri,-mwaitx,+nopl,-pad-short-functions,+pclmul,-pconfig,+pku,+popcnt,-prefer-128-bit,-prefer-256-bit,-prefer-mask-registers,-prefetchwt1,+prfchw,+ptwrite,+rdpid,+rdrnd,+rdseed,-retpoline,-retpoline-external-thunk,-retpoline-indirect-branches,-retpoline-indirect-calls,-rtm,+sahf,-serialize,-seses,-sgx,+sha,+shstk,+slow-3ops-lea,+slow-incdec,-slow-lea,-slow-pmaddwd,-slow-pmulld,-slow-shld,-slow-two-mem-ops,-slow-unaligned-mem-16,-slow-unaligned-mem-32,-soft-float,+sse,+sse2,+sse3,+sse4.1,+sse4.2,-sse4a,-sse-unaligned-mem,+ssse3,-tbm,-tsxldtrk,-uintr,-use-aa,-use-glm-div-sqrt-costs,+vaes,+vpclmulqdq,+vzeroupper,+waitpkg,-wbnoinvd,-widekl,+x87,-xop,+xsave,+xsavec,+xsaveopt,+xsaves" }
attributes #6 = { nobuiltin noreturn nounwind "frame-pointer"="all" "probe-stack"="__zig_probe_stack" "target-cpu"="x86-64" "target-features"="-16bit-mode,-32bit-mode,-3dnow,-3dnowa,+64bit,+adx,+aes,-amx-bf16,-amx-int8,-amx-tile,+avx,+avx2,-avx512bf16,-avx512bitalg,-avx512bw,-avx512cd,-avx512dq,-avx512er,-avx512f,-avx512ifma,-avx512pf,-avx512vbmi,-avx512vbmi2,-avx512vl,-avx512vnni,-avx512vp2intersect,-avx512vpopcntdq,-avxvnni,+bmi,+bmi2,-branchfusion,-cldemote,+clflushopt,+clwb,-clzero,+cmov,+cx16,+cx8,-enqcmd,-ermsb,+f16c,-false-deps-lzcnt-tzcnt,-false-deps-popcnt,-fast-11bytenop,-fast-15bytenop,-fast-7bytenop,-fast-bextr,-fast-gather,-fast-hops,-fast-lzcnt,-fast-movbe,-fast-scalar-fsqrt,-fast-scalar-shift-masks,-fast-shld-rotate,-fast-variable-crosslane-shuffle,-fast-variable-perlane-shuffle,-fast-vector-fsqrt,-fast-vector-shift-masks,+fma,-fma4,+fsgsbase,-fsrm,+fxsr,+gfni,-hreset,-idivl-to-divb,+idivq-to-divl,+invpcid,-kl,-lea-sp,-lea-uses-ag,-lvi-cfi,-lvi-load-hardening,-lwp,+lzcnt,+macrofusion,+mmx,+movbe,+movdir64b,+movdiri,-mwaitx,+nopl,-pad-short-functions,+pclmul,-pconfig,+pku,+popcnt,-prefer-128-bit,-prefer-256-bit,-prefer-mask-registers,-prefetchwt1,+prfchw,+ptwrite,+rdpid,+rdrnd,+rdseed,-retpoline,-retpoline-external-thunk,-retpoline-indirect-branches,-retpoline-indirect-calls,-rtm,+sahf,-serialize,-seses,-sgx,+sha,+shstk,+slow-3ops-lea,+slow-incdec,-slow-lea,-slow-pmaddwd,-slow-pmulld,-slow-shld,-slow-two-mem-ops,-slow-unaligned-mem-16,-slow-unaligned-mem-32,-soft-float,+sse,+sse2,+sse3,+sse4.1,+sse4.2,-sse4a,-sse-unaligned-mem,+ssse3,-tbm,-tsxldtrk,-uintr,-use-aa,-use-glm-div-sqrt-costs,+vaes,+vpclmulqdq,+vzeroupper,+waitpkg,-wbnoinvd,-widekl,+x87,-xop,+xsave,+xsavec,+xsaveopt,+xsaves" }
attributes #7 = { cold nobuiltin nounwind "frame-pointer"="all" "probe-stack"="__zig_probe_stack" "target-cpu"="x86-64" "target-features"="-16bit-mode,-32bit-mode,-3dnow,-3dnowa,+64bit,+adx,+aes,-amx-bf16,-amx-int8,-amx-tile,+avx,+avx2,-avx512bf16,-avx512bitalg,-avx512bw,-avx512cd,-avx512dq,-avx512er,-avx512f,-avx512ifma,-avx512pf,-avx512vbmi,-avx512vbmi2,-avx512vl,-avx512vnni,-avx512vp2intersect,-avx512vpopcntdq,-avxvnni,+bmi,+bmi2,-branchfusion,-cldemote,+clflushopt,+clwb,-clzero,+cmov,+cx16,+cx8,-enqcmd,-ermsb,+f16c,-false-deps-lzcnt-tzcnt,-false-deps-popcnt,-fast-11bytenop,-fast-15bytenop,-fast-7bytenop,-fast-bextr,-fast-gather,-fast-hops,-fast-lzcnt,-fast-movbe,-fast-scalar-fsqrt,-fast-scalar-shift-masks,-fast-shld-rotate,-fast-variable-crosslane-shuffle,-fast-variable-perlane-shuffle,-fast-vector-fsqrt,-fast-vector-shift-masks,+fma,-fma4,+fsgsbase,-fsrm,+fxsr,+gfni,-hreset,-idivl-to-divb,+idivq-to-divl,+invpcid,-kl,-lea-sp,-lea-uses-ag,-lvi-cfi,-lvi-load-hardening,-lwp,+lzcnt,+macrofusion,+mmx,+movbe,+movdir64b,+movdiri,-mwaitx,+nopl,-pad-short-functions,+pclmul,-pconfig,+pku,+popcnt,-prefer-128-bit,-prefer-256-bit,-prefer-mask-registers,-prefetchwt1,+prfchw,+ptwrite,+rdpid,+rdrnd,+rdseed,-retpoline,-retpoline-external-thunk,-retpoline-indirect-branches,-retpoline-indirect-calls,-rtm,+sahf,-serialize,-seses,-sgx,+sha,+shstk,+slow-3ops-lea,+slow-incdec,-slow-lea,-slow-pmaddwd,-slow-pmulld,-slow-shld,-slow-two-mem-ops,-slow-unaligned-mem-16,-slow-unaligned-mem-32,-soft-float,+sse,+sse2,+sse3,+sse4.1,+sse4.2,-sse4a,-sse-unaligned-mem,+ssse3,-tbm,-tsxldtrk,-uintr,-use-aa,-use-glm-div-sqrt-costs,+vaes,+vpclmulqdq,+vzeroupper,+waitpkg,-wbnoinvd,-widekl,+x87,-xop,+xsave,+xsavec,+xsaveopt,+xsaves" }
attributes #8 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #9 = { argmemonly nofree nounwind willreturn writeonly }
attributes #10 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.dbg.cu = !{!2}

!0 = !{i32 2, !"Debug Info Version", i32 3}
!1 = !{i32 2, !"Dwarf Version", i32 4}
!2 = distinct !DICompileUnit(language: DW_LANG_C99, file: !3, producer: "zig 0.9.1", isOptimized: false, runtimeVersion: 0, emissionKind: NoDebug, enums: !4)
!3 = !DIFile(filename: "stack-copies", directory: ".")
!4 = !{!5, !12, !17, !62, !84, !90, !96, !242, !301, !325, !332, !337, !491, !853, !876, !882, !888, !892, !897}
!5 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.builtin.OutputMode", scope: !6, file: !6, line: 458, baseType: !7, size: 8, align: 8, elements: !8)
!6 = !DIFile(filename: "builtin.zig", directory: "/nix/store/p0zs9razv7s87q8k3459d0wx120i8sp1-zig-0.9.1/lib/zig/std")
!7 = !DIBasicType(name: "u2", size: 8, encoding: DW_ATE_unsigned)
!8 = !{!9, !10, !11}
!9 = !DIEnumerator(name: "Exe", value: 0)
!10 = !DIEnumerator(name: "Lib", value: 1)
!11 = !DIEnumerator(name: "Obj", value: 2)
!12 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.builtin.LinkMode", scope: !6, file: !6, line: 466, baseType: !13, size: 8, align: 8, elements: !14)
!13 = !DIBasicType(name: "u1", size: 8, encoding: DW_ATE_unsigned)
!14 = !{!15, !16}
!15 = !DIEnumerator(name: "Static", value: 0)
!16 = !DIEnumerator(name: "Dynamic", value: 1)
!17 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.target.Tag", scope: !18, file: !18, line: 17, baseType: !19, size: 8, align: 8, elements: !20)
!18 = !DIFile(filename: "target.zig", directory: "/nix/store/p0zs9razv7s87q8k3459d0wx120i8sp1-zig-0.9.1/lib/zig/std")
!19 = !DIBasicType(name: "u6", size: 8, encoding: DW_ATE_unsigned)
!20 = !{!21, !22, !23, !24, !25, !26, !27, !28, !29, !30, !31, !32, !33, !34, !35, !36, !37, !38, !39, !40, !41, !42, !43, !44, !45, !46, !47, !48, !49, !50, !51, !52, !53, !54, !55, !56, !57, !58, !59, !60, !61}
!21 = !DIEnumerator(name: "freestanding", value: 0)
!22 = !DIEnumerator(name: "ananas", value: 1)
!23 = !DIEnumerator(name: "cloudabi", value: 2)
!24 = !DIEnumerator(name: "dragonfly", value: 3)
!25 = !DIEnumerator(name: "freebsd", value: 4)
!26 = !DIEnumerator(name: "fuchsia", value: 5)
!27 = !DIEnumerator(name: "ios", value: 6)
!28 = !DIEnumerator(name: "kfreebsd", value: 7)
!29 = !DIEnumerator(name: "linux", value: 8)
!30 = !DIEnumerator(name: "lv2", value: 9)
!31 = !DIEnumerator(name: "macos", value: 10)
!32 = !DIEnumerator(name: "netbsd", value: 11)
!33 = !DIEnumerator(name: "openbsd", value: 12)
!34 = !DIEnumerator(name: "solaris", value: 13)
!35 = !DIEnumerator(name: "windows", value: 14)
!36 = !DIEnumerator(name: "zos", value: 15)
!37 = !DIEnumerator(name: "haiku", value: 16)
!38 = !DIEnumerator(name: "minix", value: 17)
!39 = !DIEnumerator(name: "rtems", value: 18)
!40 = !DIEnumerator(name: "nacl", value: 19)
!41 = !DIEnumerator(name: "aix", value: 20)
!42 = !DIEnumerator(name: "cuda", value: 21)
!43 = !DIEnumerator(name: "nvcl", value: 22)
!44 = !DIEnumerator(name: "amdhsa", value: 23)
!45 = !DIEnumerator(name: "ps4", value: 24)
!46 = !DIEnumerator(name: "elfiamcu", value: 25)
!47 = !DIEnumerator(name: "tvos", value: 26)
!48 = !DIEnumerator(name: "watchos", value: 27)
!49 = !DIEnumerator(name: "mesa3d", value: 28)
!50 = !DIEnumerator(name: "contiki", value: 29)
!51 = !DIEnumerator(name: "amdpal", value: 30)
!52 = !DIEnumerator(name: "hermit", value: 31)
!53 = !DIEnumerator(name: "hurd", value: 32)
!54 = !DIEnumerator(name: "wasi", value: 33)
!55 = !DIEnumerator(name: "emscripten", value: 34)
!56 = !DIEnumerator(name: "uefi", value: 35)
!57 = !DIEnumerator(name: "opencl", value: 36)
!58 = !DIEnumerator(name: "glsl450", value: 37)
!59 = !DIEnumerator(name: "vulkan", value: 38)
!60 = !DIEnumerator(name: "plan9", value: 39)
!61 = !DIEnumerator(name: "other", value: 40)
!62 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.target.WindowsVersion", scope: !18, file: !18, line: 94, baseType: !63, size: 32, align: 32, elements: !64)
!63 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!64 = !{!65, !66, !67, !68, !69, !70, !71, !72, !73, !74, !75, !76, !77, !78, !79, !80, !81, !82, !83}
!65 = !DIEnumerator(name: "nt4", value: 67108864)
!66 = !DIEnumerator(name: "win2k", value: 83886080)
!67 = !DIEnumerator(name: "xp", value: 83951616)
!68 = !DIEnumerator(name: "ws2003", value: 84017152)
!69 = !DIEnumerator(name: "vista", value: 100663296)
!70 = !DIEnumerator(name: "win7", value: 100728832)
!71 = !DIEnumerator(name: "win8", value: 100794368)
!72 = !DIEnumerator(name: "win8_1", value: 100859904)
!73 = !DIEnumerator(name: "win10", value: 167772160)
!74 = !DIEnumerator(name: "win10_th2", value: 167772161)
!75 = !DIEnumerator(name: "win10_rs1", value: 167772162)
!76 = !DIEnumerator(name: "win10_rs2", value: 167772163)
!77 = !DIEnumerator(name: "win10_rs3", value: 167772164)
!78 = !DIEnumerator(name: "win10_rs4", value: 167772165)
!79 = !DIEnumerator(name: "win10_rs5", value: 167772166)
!80 = !DIEnumerator(name: "win10_19h1", value: 167772167)
!81 = !DIEnumerator(name: "win10_vb", value: 167772168)
!82 = !DIEnumerator(name: "win10_mn", value: 167772169)
!83 = !DIEnumerator(name: "win10_fe", value: 167772170)
!84 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "@typeInfo(std.target.VersionRange).Union.tag_type.?", scope: !18, file: !18, line: 221, baseType: !7, size: 8, align: 8, elements: !85)
!85 = !{!86, !87, !88, !89}
!86 = !DIEnumerator(name: "none", value: 0)
!87 = !DIEnumerator(name: "semver", value: 1)
!88 = !DIEnumerator(name: "linux", value: 2)
!89 = !DIEnumerator(name: "windows", value: 3)
!90 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.builtin.Mode", scope: !6, file: !6, line: 148, baseType: !7, size: 8, align: 8, elements: !91)
!91 = !{!92, !93, !94, !95}
!92 = !DIEnumerator(name: "Debug", value: 0)
!93 = !DIEnumerator(name: "ReleaseSafe", value: 1)
!94 = !DIEnumerator(name: "ReleaseFast", value: 2)
!95 = !DIEnumerator(name: "ReleaseSmall", value: 3)
!96 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.target.x86.Feature", scope: !97, file: !97, line: 7, baseType: !98, size: 8, align: 8, elements: !99)
!97 = !DIFile(filename: "x86.zig", directory: "/nix/store/p0zs9razv7s87q8k3459d0wx120i8sp1-zig-0.9.1/lib/zig/std/target")
!98 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned_char)
!99 = !{!100, !101, !102, !103, !104, !105, !106, !107, !108, !109, !110, !111, !112, !113, !114, !115, !116, !117, !118, !119, !120, !121, !122, !123, !124, !125, !126, !127, !128, !129, !130, !131, !132, !133, !134, !135, !136, !137, !138, !139, !140, !141, !142, !143, !144, !145, !146, !147, !148, !149, !150, !151, !152, !153, !154, !155, !156, !157, !158, !159, !160, !161, !162, !163, !164, !165, !166, !167, !168, !169, !170, !171, !172, !173, !174, !175, !176, !177, !178, !179, !180, !181, !182, !183, !184, !185, !186, !187, !188, !189, !190, !191, !192, !193, !194, !195, !196, !197, !198, !199, !200, !201, !202, !203, !204, !205, !206, !207, !208, !209, !210, !211, !212, !213, !214, !215, !216, !217, !218, !219, !220, !221, !222, !223, !224, !225, !226, !227, !228, !229, !230, !231, !232, !233, !234, !235, !236, !237, !238, !239, !240, !241}
!100 = !DIEnumerator(name: "16bit_mode", value: 0)
!101 = !DIEnumerator(name: "32bit_mode", value: 1)
!102 = !DIEnumerator(name: "3dnow", value: 2)
!103 = !DIEnumerator(name: "3dnowa", value: 3)
!104 = !DIEnumerator(name: "64bit", value: 4)
!105 = !DIEnumerator(name: "adx", value: 5)
!106 = !DIEnumerator(name: "aes", value: 6)
!107 = !DIEnumerator(name: "amx_bf16", value: 7)
!108 = !DIEnumerator(name: "amx_int8", value: 8)
!109 = !DIEnumerator(name: "amx_tile", value: 9)
!110 = !DIEnumerator(name: "avx", value: 10)
!111 = !DIEnumerator(name: "avx2", value: 11)
!112 = !DIEnumerator(name: "avx512bf16", value: 12)
!113 = !DIEnumerator(name: "avx512bitalg", value: 13)
!114 = !DIEnumerator(name: "avx512bw", value: 14)
!115 = !DIEnumerator(name: "avx512cd", value: 15)
!116 = !DIEnumerator(name: "avx512dq", value: 16)
!117 = !DIEnumerator(name: "avx512er", value: 17)
!118 = !DIEnumerator(name: "avx512f", value: 18)
!119 = !DIEnumerator(name: "avx512ifma", value: 19)
!120 = !DIEnumerator(name: "avx512pf", value: 20)
!121 = !DIEnumerator(name: "avx512vbmi", value: 21)
!122 = !DIEnumerator(name: "avx512vbmi2", value: 22)
!123 = !DIEnumerator(name: "avx512vl", value: 23)
!124 = !DIEnumerator(name: "avx512vnni", value: 24)
!125 = !DIEnumerator(name: "avx512vp2intersect", value: 25)
!126 = !DIEnumerator(name: "avx512vpopcntdq", value: 26)
!127 = !DIEnumerator(name: "avxvnni", value: 27)
!128 = !DIEnumerator(name: "bmi", value: 28)
!129 = !DIEnumerator(name: "bmi2", value: 29)
!130 = !DIEnumerator(name: "branchfusion", value: 30)
!131 = !DIEnumerator(name: "cldemote", value: 31)
!132 = !DIEnumerator(name: "clflushopt", value: 32)
!133 = !DIEnumerator(name: "clwb", value: 33)
!134 = !DIEnumerator(name: "clzero", value: 34)
!135 = !DIEnumerator(name: "cmov", value: 35)
!136 = !DIEnumerator(name: "cx16", value: 36)
!137 = !DIEnumerator(name: "cx8", value: 37)
!138 = !DIEnumerator(name: "enqcmd", value: 38)
!139 = !DIEnumerator(name: "ermsb", value: 39)
!140 = !DIEnumerator(name: "f16c", value: 40)
!141 = !DIEnumerator(name: "false_deps_lzcnt_tzcnt", value: 41)
!142 = !DIEnumerator(name: "false_deps_popcnt", value: 42)
!143 = !DIEnumerator(name: "fast_11bytenop", value: 43)
!144 = !DIEnumerator(name: "fast_15bytenop", value: 44)
!145 = !DIEnumerator(name: "fast_7bytenop", value: 45)
!146 = !DIEnumerator(name: "fast_bextr", value: 46)
!147 = !DIEnumerator(name: "fast_gather", value: 47)
!148 = !DIEnumerator(name: "fast_hops", value: 48)
!149 = !DIEnumerator(name: "fast_lzcnt", value: 49)
!150 = !DIEnumerator(name: "fast_movbe", value: 50)
!151 = !DIEnumerator(name: "fast_scalar_fsqrt", value: 51)
!152 = !DIEnumerator(name: "fast_scalar_shift_masks", value: 52)
!153 = !DIEnumerator(name: "fast_shld_rotate", value: 53)
!154 = !DIEnumerator(name: "fast_variable_crosslane_shuffle", value: 54)
!155 = !DIEnumerator(name: "fast_variable_perlane_shuffle", value: 55)
!156 = !DIEnumerator(name: "fast_vector_fsqrt", value: 56)
!157 = !DIEnumerator(name: "fast_vector_shift_masks", value: 57)
!158 = !DIEnumerator(name: "fma", value: 58)
!159 = !DIEnumerator(name: "fma4", value: 59)
!160 = !DIEnumerator(name: "fsgsbase", value: 60)
!161 = !DIEnumerator(name: "fsrm", value: 61)
!162 = !DIEnumerator(name: "fxsr", value: 62)
!163 = !DIEnumerator(name: "gfni", value: 63)
!164 = !DIEnumerator(name: "hreset", value: 64)
!165 = !DIEnumerator(name: "idivl_to_divb", value: 65)
!166 = !DIEnumerator(name: "idivq_to_divl", value: 66)
!167 = !DIEnumerator(name: "invpcid", value: 67)
!168 = !DIEnumerator(name: "kl", value: 68)
!169 = !DIEnumerator(name: "lea_sp", value: 69)
!170 = !DIEnumerator(name: "lea_uses_ag", value: 70)
!171 = !DIEnumerator(name: "lvi_cfi", value: 71)
!172 = !DIEnumerator(name: "lvi_load_hardening", value: 72)
!173 = !DIEnumerator(name: "lwp", value: 73)
!174 = !DIEnumerator(name: "lzcnt", value: 74)
!175 = !DIEnumerator(name: "macrofusion", value: 75)
!176 = !DIEnumerator(name: "mmx", value: 76)
!177 = !DIEnumerator(name: "movbe", value: 77)
!178 = !DIEnumerator(name: "movdir64b", value: 78)
!179 = !DIEnumerator(name: "movdiri", value: 79)
!180 = !DIEnumerator(name: "mwaitx", value: 80)
!181 = !DIEnumerator(name: "nopl", value: 81)
!182 = !DIEnumerator(name: "pad_short_functions", value: 82)
!183 = !DIEnumerator(name: "pclmul", value: 83)
!184 = !DIEnumerator(name: "pconfig", value: 84)
!185 = !DIEnumerator(name: "pku", value: 85)
!186 = !DIEnumerator(name: "popcnt", value: 86)
!187 = !DIEnumerator(name: "prefer_128_bit", value: 87)
!188 = !DIEnumerator(name: "prefer_256_bit", value: 88)
!189 = !DIEnumerator(name: "prefer_mask_registers", value: 89)
!190 = !DIEnumerator(name: "prefetchwt1", value: 90)
!191 = !DIEnumerator(name: "prfchw", value: 91)
!192 = !DIEnumerator(name: "ptwrite", value: 92)
!193 = !DIEnumerator(name: "rdpid", value: 93)
!194 = !DIEnumerator(name: "rdrnd", value: 94)
!195 = !DIEnumerator(name: "rdseed", value: 95)
!196 = !DIEnumerator(name: "retpoline", value: 96)
!197 = !DIEnumerator(name: "retpoline_external_thunk", value: 97)
!198 = !DIEnumerator(name: "retpoline_indirect_branches", value: 98)
!199 = !DIEnumerator(name: "retpoline_indirect_calls", value: 99)
!200 = !DIEnumerator(name: "rtm", value: 100)
!201 = !DIEnumerator(name: "sahf", value: 101)
!202 = !DIEnumerator(name: "serialize", value: 102)
!203 = !DIEnumerator(name: "seses", value: 103)
!204 = !DIEnumerator(name: "sgx", value: 104)
!205 = !DIEnumerator(name: "sha", value: 105)
!206 = !DIEnumerator(name: "shstk", value: 106)
!207 = !DIEnumerator(name: "slow_3ops_lea", value: 107)
!208 = !DIEnumerator(name: "slow_incdec", value: 108)
!209 = !DIEnumerator(name: "slow_lea", value: 109)
!210 = !DIEnumerator(name: "slow_pmaddwd", value: 110)
!211 = !DIEnumerator(name: "slow_pmulld", value: 111)
!212 = !DIEnumerator(name: "slow_shld", value: 112)
!213 = !DIEnumerator(name: "slow_two_mem_ops", value: 113)
!214 = !DIEnumerator(name: "slow_unaligned_mem_16", value: 114)
!215 = !DIEnumerator(name: "slow_unaligned_mem_32", value: 115)
!216 = !DIEnumerator(name: "soft_float", value: 116)
!217 = !DIEnumerator(name: "sse", value: 117)
!218 = !DIEnumerator(name: "sse2", value: 118)
!219 = !DIEnumerator(name: "sse3", value: 119)
!220 = !DIEnumerator(name: "sse4_1", value: 120)
!221 = !DIEnumerator(name: "sse4_2", value: 121)
!222 = !DIEnumerator(name: "sse4a", value: 122)
!223 = !DIEnumerator(name: "sse_unaligned_mem", value: 123)
!224 = !DIEnumerator(name: "ssse3", value: 124)
!225 = !DIEnumerator(name: "tbm", value: 125)
!226 = !DIEnumerator(name: "tsxldtrk", value: 126)
!227 = !DIEnumerator(name: "uintr", value: 127)
!228 = !DIEnumerator(name: "use_aa", value: 128)
!229 = !DIEnumerator(name: "use_glm_div_sqrt_costs", value: 129)
!230 = !DIEnumerator(name: "vaes", value: 130)
!231 = !DIEnumerator(name: "vpclmulqdq", value: 131)
!232 = !DIEnumerator(name: "vzeroupper", value: 132)
!233 = !DIEnumerator(name: "waitpkg", value: 133)
!234 = !DIEnumerator(name: "wbnoinvd", value: 134)
!235 = !DIEnumerator(name: "widekl", value: 135)
!236 = !DIEnumerator(name: "x87", value: 136)
!237 = !DIEnumerator(name: "xop", value: 137)
!238 = !DIEnumerator(name: "xsave", value: 138)
!239 = !DIEnumerator(name: "xsavec", value: 139)
!240 = !DIEnumerator(name: "xsaveopt", value: 140)
!241 = !DIEnumerator(name: "xsaves", value: 141)
!242 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.target.Arch", scope: !18, file: !18, line: 760, baseType: !19, size: 8, align: 8, elements: !243)
!243 = !{!244, !245, !246, !247, !248, !249, !250, !251, !252, !253, !254, !255, !256, !257, !258, !259, !260, !261, !262, !263, !264, !265, !266, !267, !268, !269, !270, !271, !272, !273, !274, !275, !276, !277, !278, !279, !280, !281, !282, !283, !284, !285, !286, !287, !288, !289, !290, !291, !292, !293, !294, !295, !296, !297, !298, !299, !300}
!244 = !DIEnumerator(name: "arm", value: 0)
!245 = !DIEnumerator(name: "armeb", value: 1)
!246 = !DIEnumerator(name: "aarch64", value: 2)
!247 = !DIEnumerator(name: "aarch64_be", value: 3)
!248 = !DIEnumerator(name: "aarch64_32", value: 4)
!249 = !DIEnumerator(name: "arc", value: 5)
!250 = !DIEnumerator(name: "avr", value: 6)
!251 = !DIEnumerator(name: "bpfel", value: 7)
!252 = !DIEnumerator(name: "bpfeb", value: 8)
!253 = !DIEnumerator(name: "csky", value: 9)
!254 = !DIEnumerator(name: "hexagon", value: 10)
!255 = !DIEnumerator(name: "m68k", value: 11)
!256 = !DIEnumerator(name: "mips", value: 12)
!257 = !DIEnumerator(name: "mipsel", value: 13)
!258 = !DIEnumerator(name: "mips64", value: 14)
!259 = !DIEnumerator(name: "mips64el", value: 15)
!260 = !DIEnumerator(name: "msp430", value: 16)
!261 = !DIEnumerator(name: "powerpc", value: 17)
!262 = !DIEnumerator(name: "powerpcle", value: 18)
!263 = !DIEnumerator(name: "powerpc64", value: 19)
!264 = !DIEnumerator(name: "powerpc64le", value: 20)
!265 = !DIEnumerator(name: "r600", value: 21)
!266 = !DIEnumerator(name: "amdgcn", value: 22)
!267 = !DIEnumerator(name: "riscv32", value: 23)
!268 = !DIEnumerator(name: "riscv64", value: 24)
!269 = !DIEnumerator(name: "sparc", value: 25)
!270 = !DIEnumerator(name: "sparcv9", value: 26)
!271 = !DIEnumerator(name: "sparcel", value: 27)
!272 = !DIEnumerator(name: "s390x", value: 28)
!273 = !DIEnumerator(name: "tce", value: 29)
!274 = !DIEnumerator(name: "tcele", value: 30)
!275 = !DIEnumerator(name: "thumb", value: 31)
!276 = !DIEnumerator(name: "thumbeb", value: 32)
!277 = !DIEnumerator(name: "i386", value: 33)
!278 = !DIEnumerator(name: "x86_64", value: 34)
!279 = !DIEnumerator(name: "xcore", value: 35)
!280 = !DIEnumerator(name: "nvptx", value: 36)
!281 = !DIEnumerator(name: "nvptx64", value: 37)
!282 = !DIEnumerator(name: "le32", value: 38)
!283 = !DIEnumerator(name: "le64", value: 39)
!284 = !DIEnumerator(name: "amdil", value: 40)
!285 = !DIEnumerator(name: "amdil64", value: 41)
!286 = !DIEnumerator(name: "hsail", value: 42)
!287 = !DIEnumerator(name: "hsail64", value: 43)
!288 = !DIEnumerator(name: "spir", value: 44)
!289 = !DIEnumerator(name: "spir64", value: 45)
!290 = !DIEnumerator(name: "kalimba", value: 46)
!291 = !DIEnumerator(name: "shave", value: 47)
!292 = !DIEnumerator(name: "lanai", value: 48)
!293 = !DIEnumerator(name: "wasm32", value: 49)
!294 = !DIEnumerator(name: "wasm64", value: 50)
!295 = !DIEnumerator(name: "renderscript32", value: 51)
!296 = !DIEnumerator(name: "renderscript64", value: 52)
!297 = !DIEnumerator(name: "ve", value: 53)
!298 = !DIEnumerator(name: "spu_2", value: 54)
!299 = !DIEnumerator(name: "spirv32", value: 55)
!300 = !DIEnumerator(name: "spirv64", value: 56)
!301 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.target.Abi", scope: !18, file: !18, line: 461, baseType: !302, size: 8, align: 8, elements: !303)
!302 = !DIBasicType(name: "u5", size: 8, encoding: DW_ATE_unsigned)
!303 = !{!86, !304, !305, !306, !307, !308, !309, !310, !311, !312, !313, !314, !315, !316, !317, !318, !319, !320, !321, !322, !323, !324}
!304 = !DIEnumerator(name: "gnu", value: 1)
!305 = !DIEnumerator(name: "gnuabin32", value: 2)
!306 = !DIEnumerator(name: "gnuabi64", value: 3)
!307 = !DIEnumerator(name: "gnueabi", value: 4)
!308 = !DIEnumerator(name: "gnueabihf", value: 5)
!309 = !DIEnumerator(name: "gnux32", value: 6)
!310 = !DIEnumerator(name: "gnuilp32", value: 7)
!311 = !DIEnumerator(name: "code16", value: 8)
!312 = !DIEnumerator(name: "eabi", value: 9)
!313 = !DIEnumerator(name: "eabihf", value: 10)
!314 = !DIEnumerator(name: "android", value: 11)
!315 = !DIEnumerator(name: "musl", value: 12)
!316 = !DIEnumerator(name: "musleabi", value: 13)
!317 = !DIEnumerator(name: "musleabihf", value: 14)
!318 = !DIEnumerator(name: "muslx32", value: 15)
!319 = !DIEnumerator(name: "msvc", value: 16)
!320 = !DIEnumerator(name: "itanium", value: 17)
!321 = !DIEnumerator(name: "cygnus", value: 18)
!322 = !DIEnumerator(name: "coreclr", value: 19)
!323 = !DIEnumerator(name: "simulator", value: 20)
!324 = !DIEnumerator(name: "macabi", value: 21)
!325 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.Thread.Mutex.State", scope: !326, file: !326, line: 63, baseType: !327, size: 32, align: 32, elements: !328)
!326 = !DIFile(filename: "Mutex.zig", directory: "/nix/store/p0zs9razv7s87q8k3459d0wx120i8sp1-zig-0.9.1/lib/zig/std/Thread")
!327 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!328 = !{!329, !330, !331}
!329 = !DIEnumerator(name: "unlocked", value: 0)
!330 = !DIEnumerator(name: "locked", value: 1)
!331 = !DIEnumerator(name: "waiting", value: 2)
!332 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.io.Mode", scope: !333, file: !333, line: 15, baseType: !13, size: 8, align: 8, elements: !334)
!333 = !DIFile(filename: "io.zig", directory: "/nix/store/p0zs9razv7s87q8k3459d0wx120i8sp1-zig-0.9.1/lib/zig/std")
!334 = !{!335, !336}
!335 = !DIEnumerator(name: "blocking", value: 0)
!336 = !DIEnumerator(name: "evented", value: 1)
!337 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.os.linux.errno.generic.E", scope: !338, file: !338, line: 1, baseType: !339, size: 16, align: 16, elements: !340)
!338 = !DIFile(filename: "generic.zig", directory: "/nix/store/p0zs9razv7s87q8k3459d0wx120i8sp1-zig-0.9.1/lib/zig/std/os/linux/errno")
!339 = !DIBasicType(name: "u16", size: 16, encoding: DW_ATE_unsigned)
!340 = !{!341, !342, !343, !344, !345, !346, !347, !348, !349, !350, !351, !352, !353, !354, !355, !356, !357, !358, !359, !360, !361, !362, !363, !364, !365, !366, !367, !368, !369, !370, !371, !372, !373, !374, !375, !376, !377, !378, !379, !380, !381, !382, !383, !384, !385, !386, !387, !388, !389, !390, !391, !392, !393, !394, !395, !396, !397, !398, !399, !400, !401, !402, !403, !404, !405, !406, !407, !408, !409, !410, !411, !412, !413, !414, !415, !416, !417, !418, !419, !420, !421, !422, !423, !424, !425, !426, !427, !428, !429, !430, !431, !432, !433, !434, !435, !436, !437, !438, !439, !440, !441, !442, !443, !444, !445, !446, !447, !448, !449, !450, !451, !452, !453, !454, !455, !456, !457, !458, !459, !460, !461, !462, !463, !464, !465, !466, !467, !468, !469, !470, !471, !472, !473, !474, !475, !476, !477, !478, !479, !480, !481, !482, !483, !484, !485, !486, !487, !488, !489, !490}
!341 = !DIEnumerator(name: "SUCCESS", value: 0)
!342 = !DIEnumerator(name: "PERM", value: 1)
!343 = !DIEnumerator(name: "NOENT", value: 2)
!344 = !DIEnumerator(name: "SRCH", value: 3)
!345 = !DIEnumerator(name: "INTR", value: 4)
!346 = !DIEnumerator(name: "IO", value: 5)
!347 = !DIEnumerator(name: "NXIO", value: 6)
!348 = !DIEnumerator(name: "2BIG", value: 7)
!349 = !DIEnumerator(name: "NOEXEC", value: 8)
!350 = !DIEnumerator(name: "BADF", value: 9)
!351 = !DIEnumerator(name: "CHILD", value: 10)
!352 = !DIEnumerator(name: "AGAIN", value: 11)
!353 = !DIEnumerator(name: "NOMEM", value: 12)
!354 = !DIEnumerator(name: "ACCES", value: 13)
!355 = !DIEnumerator(name: "FAULT", value: 14)
!356 = !DIEnumerator(name: "NOTBLK", value: 15)
!357 = !DIEnumerator(name: "BUSY", value: 16)
!358 = !DIEnumerator(name: "EXIST", value: 17)
!359 = !DIEnumerator(name: "XDEV", value: 18)
!360 = !DIEnumerator(name: "NODEV", value: 19)
!361 = !DIEnumerator(name: "NOTDIR", value: 20)
!362 = !DIEnumerator(name: "ISDIR", value: 21)
!363 = !DIEnumerator(name: "INVAL", value: 22)
!364 = !DIEnumerator(name: "NFILE", value: 23)
!365 = !DIEnumerator(name: "MFILE", value: 24)
!366 = !DIEnumerator(name: "NOTTY", value: 25)
!367 = !DIEnumerator(name: "TXTBSY", value: 26)
!368 = !DIEnumerator(name: "FBIG", value: 27)
!369 = !DIEnumerator(name: "NOSPC", value: 28)
!370 = !DIEnumerator(name: "SPIPE", value: 29)
!371 = !DIEnumerator(name: "ROFS", value: 30)
!372 = !DIEnumerator(name: "MLINK", value: 31)
!373 = !DIEnumerator(name: "PIPE", value: 32)
!374 = !DIEnumerator(name: "DOM", value: 33)
!375 = !DIEnumerator(name: "RANGE", value: 34)
!376 = !DIEnumerator(name: "DEADLK", value: 35)
!377 = !DIEnumerator(name: "NAMETOOLONG", value: 36)
!378 = !DIEnumerator(name: "NOLCK", value: 37)
!379 = !DIEnumerator(name: "NOSYS", value: 38)
!380 = !DIEnumerator(name: "NOTEMPTY", value: 39)
!381 = !DIEnumerator(name: "LOOP", value: 40)
!382 = !DIEnumerator(name: "NOMSG", value: 42)
!383 = !DIEnumerator(name: "IDRM", value: 43)
!384 = !DIEnumerator(name: "CHRNG", value: 44)
!385 = !DIEnumerator(name: "L2NSYNC", value: 45)
!386 = !DIEnumerator(name: "L3HLT", value: 46)
!387 = !DIEnumerator(name: "L3RST", value: 47)
!388 = !DIEnumerator(name: "LNRNG", value: 48)
!389 = !DIEnumerator(name: "UNATCH", value: 49)
!390 = !DIEnumerator(name: "NOCSI", value: 50)
!391 = !DIEnumerator(name: "L2HLT", value: 51)
!392 = !DIEnumerator(name: "BADE", value: 52)
!393 = !DIEnumerator(name: "BADR", value: 53)
!394 = !DIEnumerator(name: "XFULL", value: 54)
!395 = !DIEnumerator(name: "NOANO", value: 55)
!396 = !DIEnumerator(name: "BADRQC", value: 56)
!397 = !DIEnumerator(name: "BADSLT", value: 57)
!398 = !DIEnumerator(name: "BFONT", value: 59)
!399 = !DIEnumerator(name: "NOSTR", value: 60)
!400 = !DIEnumerator(name: "NODATA", value: 61)
!401 = !DIEnumerator(name: "TIME", value: 62)
!402 = !DIEnumerator(name: "NOSR", value: 63)
!403 = !DIEnumerator(name: "NONET", value: 64)
!404 = !DIEnumerator(name: "NOPKG", value: 65)
!405 = !DIEnumerator(name: "REMOTE", value: 66)
!406 = !DIEnumerator(name: "NOLINK", value: 67)
!407 = !DIEnumerator(name: "ADV", value: 68)
!408 = !DIEnumerator(name: "SRMNT", value: 69)
!409 = !DIEnumerator(name: "COMM", value: 70)
!410 = !DIEnumerator(name: "PROTO", value: 71)
!411 = !DIEnumerator(name: "MULTIHOP", value: 72)
!412 = !DIEnumerator(name: "DOTDOT", value: 73)
!413 = !DIEnumerator(name: "BADMSG", value: 74)
!414 = !DIEnumerator(name: "OVERFLOW", value: 75)
!415 = !DIEnumerator(name: "NOTUNIQ", value: 76)
!416 = !DIEnumerator(name: "BADFD", value: 77)
!417 = !DIEnumerator(name: "REMCHG", value: 78)
!418 = !DIEnumerator(name: "LIBACC", value: 79)
!419 = !DIEnumerator(name: "LIBBAD", value: 80)
!420 = !DIEnumerator(name: "LIBSCN", value: 81)
!421 = !DIEnumerator(name: "LIBMAX", value: 82)
!422 = !DIEnumerator(name: "LIBEXEC", value: 83)
!423 = !DIEnumerator(name: "ILSEQ", value: 84)
!424 = !DIEnumerator(name: "RESTART", value: 85)
!425 = !DIEnumerator(name: "STRPIPE", value: 86)
!426 = !DIEnumerator(name: "USERS", value: 87)
!427 = !DIEnumerator(name: "NOTSOCK", value: 88)
!428 = !DIEnumerator(name: "DESTADDRREQ", value: 89)
!429 = !DIEnumerator(name: "MSGSIZE", value: 90)
!430 = !DIEnumerator(name: "PROTOTYPE", value: 91)
!431 = !DIEnumerator(name: "NOPROTOOPT", value: 92)
!432 = !DIEnumerator(name: "PROTONOSUPPORT", value: 93)
!433 = !DIEnumerator(name: "SOCKTNOSUPPORT", value: 94)
!434 = !DIEnumerator(name: "OPNOTSUPP", value: 95)
!435 = !DIEnumerator(name: "PFNOSUPPORT", value: 96)
!436 = !DIEnumerator(name: "AFNOSUPPORT", value: 97)
!437 = !DIEnumerator(name: "ADDRINUSE", value: 98)
!438 = !DIEnumerator(name: "ADDRNOTAVAIL", value: 99)
!439 = !DIEnumerator(name: "NETDOWN", value: 100)
!440 = !DIEnumerator(name: "NETUNREACH", value: 101)
!441 = !DIEnumerator(name: "NETRESET", value: 102)
!442 = !DIEnumerator(name: "CONNABORTED", value: 103)
!443 = !DIEnumerator(name: "CONNRESET", value: 104)
!444 = !DIEnumerator(name: "NOBUFS", value: 105)
!445 = !DIEnumerator(name: "ISCONN", value: 106)
!446 = !DIEnumerator(name: "NOTCONN", value: 107)
!447 = !DIEnumerator(name: "SHUTDOWN", value: 108)
!448 = !DIEnumerator(name: "TOOMANYREFS", value: 109)
!449 = !DIEnumerator(name: "TIMEDOUT", value: 110)
!450 = !DIEnumerator(name: "CONNREFUSED", value: 111)
!451 = !DIEnumerator(name: "HOSTDOWN", value: 112)
!452 = !DIEnumerator(name: "HOSTUNREACH", value: 113)
!453 = !DIEnumerator(name: "ALREADY", value: 114)
!454 = !DIEnumerator(name: "INPROGRESS", value: 115)
!455 = !DIEnumerator(name: "STALE", value: 116)
!456 = !DIEnumerator(name: "UCLEAN", value: 117)
!457 = !DIEnumerator(name: "NOTNAM", value: 118)
!458 = !DIEnumerator(name: "NAVAIL", value: 119)
!459 = !DIEnumerator(name: "ISNAM", value: 120)
!460 = !DIEnumerator(name: "REMOTEIO", value: 121)
!461 = !DIEnumerator(name: "DQUOT", value: 122)
!462 = !DIEnumerator(name: "NOMEDIUM", value: 123)
!463 = !DIEnumerator(name: "MEDIUMTYPE", value: 124)
!464 = !DIEnumerator(name: "CANCELED", value: 125)
!465 = !DIEnumerator(name: "NOKEY", value: 126)
!466 = !DIEnumerator(name: "KEYEXPIRED", value: 127)
!467 = !DIEnumerator(name: "KEYREVOKED", value: 128)
!468 = !DIEnumerator(name: "KEYREJECTED", value: 129)
!469 = !DIEnumerator(name: "OWNERDEAD", value: 130)
!470 = !DIEnumerator(name: "NOTRECOVERABLE", value: 131)
!471 = !DIEnumerator(name: "RFKILL", value: 132)
!472 = !DIEnumerator(name: "HWPOISON", value: 133)
!473 = !DIEnumerator(name: "NSRNODATA", value: 160)
!474 = !DIEnumerator(name: "NSRFORMERR", value: 161)
!475 = !DIEnumerator(name: "NSRSERVFAIL", value: 162)
!476 = !DIEnumerator(name: "NSRNOTFOUND", value: 163)
!477 = !DIEnumerator(name: "NSRNOTIMP", value: 164)
!478 = !DIEnumerator(name: "NSRREFUSED", value: 165)
!479 = !DIEnumerator(name: "NSRBADQUERY", value: 166)
!480 = !DIEnumerator(name: "NSRBADNAME", value: 167)
!481 = !DIEnumerator(name: "NSRBADFAMILY", value: 168)
!482 = !DIEnumerator(name: "NSRBADRESP", value: 169)
!483 = !DIEnumerator(name: "NSRCONNREFUSED", value: 170)
!484 = !DIEnumerator(name: "NSRTIMEOUT", value: 171)
!485 = !DIEnumerator(name: "NSROF", value: 172)
!486 = !DIEnumerator(name: "NSRFILE", value: 173)
!487 = !DIEnumerator(name: "NSRNOMEM", value: 174)
!488 = !DIEnumerator(name: "NSRDESTRUCTION", value: 175)
!489 = !DIEnumerator(name: "NSRQUERYDOMAINTOOLONG", value: 176)
!490 = !DIEnumerator(name: "NSRCNAMELOOP", value: 177)
!491 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.os.linux.x86_64.SYS", scope: !492, file: !492, line: 121, baseType: !493, size: 64, align: 64, elements: !494)
!492 = !DIFile(filename: "x86_64.zig", directory: "/nix/store/p0zs9razv7s87q8k3459d0wx120i8sp1-zig-0.9.1/lib/zig/std/os/linux")
!493 = !DIBasicType(name: "usize", size: 64, encoding: DW_ATE_unsigned)
!494 = !{!495, !496, !497, !498, !499, !500, !501, !502, !503, !504, !505, !506, !507, !508, !509, !510, !511, !512, !513, !514, !515, !516, !517, !518, !519, !520, !521, !522, !523, !524, !525, !526, !527, !528, !529, !530, !531, !532, !533, !534, !535, !536, !537, !538, !539, !540, !541, !542, !543, !544, !545, !546, !547, !548, !549, !550, !551, !552, !553, !554, !555, !556, !557, !558, !559, !560, !561, !562, !563, !564, !565, !566, !567, !568, !569, !570, !571, !572, !573, !574, !575, !576, !577, !578, !579, !580, !581, !582, !583, !584, !585, !586, !587, !588, !589, !590, !591, !592, !593, !594, !595, !596, !597, !598, !599, !600, !601, !602, !603, !604, !605, !606, !607, !608, !609, !610, !611, !612, !613, !614, !615, !616, !617, !618, !619, !620, !621, !622, !623, !624, !625, !626, !627, !628, !629, !630, !631, !632, !633, !634, !635, !636, !637, !638, !639, !640, !641, !642, !643, !644, !645, !646, !647, !648, !649, !650, !651, !652, !653, !654, !655, !656, !657, !658, !659, !660, !661, !662, !663, !664, !665, !666, !667, !668, !669, !670, !671, !672, !673, !674, !675, !676, !677, !678, !679, !680, !681, !682, !683, !684, !685, !686, !687, !688, !689, !690, !691, !692, !693, !694, !695, !696, !697, !698, !699, !700, !701, !702, !703, !704, !705, !706, !707, !708, !709, !710, !711, !712, !713, !714, !715, !716, !717, !718, !719, !720, !721, !722, !723, !724, !725, !726, !727, !728, !729, !730, !731, !732, !733, !734, !735, !736, !737, !738, !739, !740, !741, !742, !743, !744, !745, !746, !747, !748, !749, !750, !751, !752, !753, !754, !755, !756, !757, !758, !759, !760, !761, !762, !763, !764, !765, !766, !767, !768, !769, !770, !771, !772, !773, !774, !775, !776, !777, !778, !779, !780, !781, !782, !783, !784, !785, !786, !787, !788, !789, !790, !791, !792, !793, !794, !795, !796, !797, !798, !799, !800, !801, !802, !803, !804, !805, !806, !807, !808, !809, !810, !811, !812, !813, !814, !815, !816, !817, !818, !819, !820, !821, !822, !823, !824, !825, !826, !827, !828, !829, !830, !831, !832, !833, !834, !835, !836, !837, !838, !839, !840, !841, !842, !843, !844, !845, !846, !847, !848, !849, !850, !851, !852}
!495 = !DIEnumerator(name: "read", value: 0)
!496 = !DIEnumerator(name: "write", value: 1)
!497 = !DIEnumerator(name: "open", value: 2)
!498 = !DIEnumerator(name: "close", value: 3)
!499 = !DIEnumerator(name: "stat", value: 4)
!500 = !DIEnumerator(name: "fstat", value: 5)
!501 = !DIEnumerator(name: "lstat", value: 6)
!502 = !DIEnumerator(name: "poll", value: 7)
!503 = !DIEnumerator(name: "lseek", value: 8)
!504 = !DIEnumerator(name: "mmap", value: 9)
!505 = !DIEnumerator(name: "mprotect", value: 10)
!506 = !DIEnumerator(name: "munmap", value: 11)
!507 = !DIEnumerator(name: "brk", value: 12)
!508 = !DIEnumerator(name: "rt_sigaction", value: 13)
!509 = !DIEnumerator(name: "rt_sigprocmask", value: 14)
!510 = !DIEnumerator(name: "rt_sigreturn", value: 15)
!511 = !DIEnumerator(name: "ioctl", value: 16)
!512 = !DIEnumerator(name: "pread", value: 17)
!513 = !DIEnumerator(name: "pwrite", value: 18)
!514 = !DIEnumerator(name: "readv", value: 19)
!515 = !DIEnumerator(name: "writev", value: 20)
!516 = !DIEnumerator(name: "access", value: 21)
!517 = !DIEnumerator(name: "pipe", value: 22)
!518 = !DIEnumerator(name: "select", value: 23)
!519 = !DIEnumerator(name: "sched_yield", value: 24)
!520 = !DIEnumerator(name: "mremap", value: 25)
!521 = !DIEnumerator(name: "msync", value: 26)
!522 = !DIEnumerator(name: "mincore", value: 27)
!523 = !DIEnumerator(name: "madvise", value: 28)
!524 = !DIEnumerator(name: "shmget", value: 29)
!525 = !DIEnumerator(name: "shmat", value: 30)
!526 = !DIEnumerator(name: "shmctl", value: 31)
!527 = !DIEnumerator(name: "dup", value: 32)
!528 = !DIEnumerator(name: "dup2", value: 33)
!529 = !DIEnumerator(name: "pause", value: 34)
!530 = !DIEnumerator(name: "nanosleep", value: 35)
!531 = !DIEnumerator(name: "getitimer", value: 36)
!532 = !DIEnumerator(name: "alarm", value: 37)
!533 = !DIEnumerator(name: "setitimer", value: 38)
!534 = !DIEnumerator(name: "getpid", value: 39)
!535 = !DIEnumerator(name: "sendfile", value: 40)
!536 = !DIEnumerator(name: "socket", value: 41)
!537 = !DIEnumerator(name: "connect", value: 42)
!538 = !DIEnumerator(name: "accept", value: 43)
!539 = !DIEnumerator(name: "sendto", value: 44)
!540 = !DIEnumerator(name: "recvfrom", value: 45)
!541 = !DIEnumerator(name: "sendmsg", value: 46)
!542 = !DIEnumerator(name: "recvmsg", value: 47)
!543 = !DIEnumerator(name: "shutdown", value: 48)
!544 = !DIEnumerator(name: "bind", value: 49)
!545 = !DIEnumerator(name: "listen", value: 50)
!546 = !DIEnumerator(name: "getsockname", value: 51)
!547 = !DIEnumerator(name: "getpeername", value: 52)
!548 = !DIEnumerator(name: "socketpair", value: 53)
!549 = !DIEnumerator(name: "setsockopt", value: 54)
!550 = !DIEnumerator(name: "getsockopt", value: 55)
!551 = !DIEnumerator(name: "clone", value: 56)
!552 = !DIEnumerator(name: "fork", value: 57)
!553 = !DIEnumerator(name: "vfork", value: 58)
!554 = !DIEnumerator(name: "execve", value: 59)
!555 = !DIEnumerator(name: "exit", value: 60)
!556 = !DIEnumerator(name: "wait4", value: 61)
!557 = !DIEnumerator(name: "kill", value: 62)
!558 = !DIEnumerator(name: "uname", value: 63)
!559 = !DIEnumerator(name: "semget", value: 64)
!560 = !DIEnumerator(name: "semop", value: 65)
!561 = !DIEnumerator(name: "semctl", value: 66)
!562 = !DIEnumerator(name: "shmdt", value: 67)
!563 = !DIEnumerator(name: "msgget", value: 68)
!564 = !DIEnumerator(name: "msgsnd", value: 69)
!565 = !DIEnumerator(name: "msgrcv", value: 70)
!566 = !DIEnumerator(name: "msgctl", value: 71)
!567 = !DIEnumerator(name: "fcntl", value: 72)
!568 = !DIEnumerator(name: "flock", value: 73)
!569 = !DIEnumerator(name: "fsync", value: 74)
!570 = !DIEnumerator(name: "fdatasync", value: 75)
!571 = !DIEnumerator(name: "truncate", value: 76)
!572 = !DIEnumerator(name: "ftruncate", value: 77)
!573 = !DIEnumerator(name: "getdents", value: 78)
!574 = !DIEnumerator(name: "getcwd", value: 79)
!575 = !DIEnumerator(name: "chdir", value: 80)
!576 = !DIEnumerator(name: "fchdir", value: 81)
!577 = !DIEnumerator(name: "rename", value: 82)
!578 = !DIEnumerator(name: "mkdir", value: 83)
!579 = !DIEnumerator(name: "rmdir", value: 84)
!580 = !DIEnumerator(name: "creat", value: 85)
!581 = !DIEnumerator(name: "link", value: 86)
!582 = !DIEnumerator(name: "unlink", value: 87)
!583 = !DIEnumerator(name: "symlink", value: 88)
!584 = !DIEnumerator(name: "readlink", value: 89)
!585 = !DIEnumerator(name: "chmod", value: 90)
!586 = !DIEnumerator(name: "fchmod", value: 91)
!587 = !DIEnumerator(name: "chown", value: 92)
!588 = !DIEnumerator(name: "fchown", value: 93)
!589 = !DIEnumerator(name: "lchown", value: 94)
!590 = !DIEnumerator(name: "umask", value: 95)
!591 = !DIEnumerator(name: "gettimeofday", value: 96)
!592 = !DIEnumerator(name: "getrlimit", value: 97)
!593 = !DIEnumerator(name: "getrusage", value: 98)
!594 = !DIEnumerator(name: "sysinfo", value: 99)
!595 = !DIEnumerator(name: "times", value: 100)
!596 = !DIEnumerator(name: "ptrace", value: 101)
!597 = !DIEnumerator(name: "getuid", value: 102)
!598 = !DIEnumerator(name: "syslog", value: 103)
!599 = !DIEnumerator(name: "getgid", value: 104)
!600 = !DIEnumerator(name: "setuid", value: 105)
!601 = !DIEnumerator(name: "setgid", value: 106)
!602 = !DIEnumerator(name: "geteuid", value: 107)
!603 = !DIEnumerator(name: "getegid", value: 108)
!604 = !DIEnumerator(name: "setpgid", value: 109)
!605 = !DIEnumerator(name: "getppid", value: 110)
!606 = !DIEnumerator(name: "getpgrp", value: 111)
!607 = !DIEnumerator(name: "setsid", value: 112)
!608 = !DIEnumerator(name: "setreuid", value: 113)
!609 = !DIEnumerator(name: "setregid", value: 114)
!610 = !DIEnumerator(name: "getgroups", value: 115)
!611 = !DIEnumerator(name: "setgroups", value: 116)
!612 = !DIEnumerator(name: "setresuid", value: 117)
!613 = !DIEnumerator(name: "getresuid", value: 118)
!614 = !DIEnumerator(name: "setresgid", value: 119)
!615 = !DIEnumerator(name: "getresgid", value: 120)
!616 = !DIEnumerator(name: "getpgid", value: 121)
!617 = !DIEnumerator(name: "setfsuid", value: 122)
!618 = !DIEnumerator(name: "setfsgid", value: 123)
!619 = !DIEnumerator(name: "getsid", value: 124)
!620 = !DIEnumerator(name: "capget", value: 125)
!621 = !DIEnumerator(name: "capset", value: 126)
!622 = !DIEnumerator(name: "rt_sigpending", value: 127)
!623 = !DIEnumerator(name: "rt_sigtimedwait", value: 128)
!624 = !DIEnumerator(name: "rt_sigqueueinfo", value: 129)
!625 = !DIEnumerator(name: "rt_sigsuspend", value: 130)
!626 = !DIEnumerator(name: "sigaltstack", value: 131)
!627 = !DIEnumerator(name: "utime", value: 132)
!628 = !DIEnumerator(name: "mknod", value: 133)
!629 = !DIEnumerator(name: "uselib", value: 134)
!630 = !DIEnumerator(name: "personality", value: 135)
!631 = !DIEnumerator(name: "ustat", value: 136)
!632 = !DIEnumerator(name: "statfs", value: 137)
!633 = !DIEnumerator(name: "fstatfs", value: 138)
!634 = !DIEnumerator(name: "sysfs", value: 139)
!635 = !DIEnumerator(name: "getpriority", value: 140)
!636 = !DIEnumerator(name: "setpriority", value: 141)
!637 = !DIEnumerator(name: "sched_setparam", value: 142)
!638 = !DIEnumerator(name: "sched_getparam", value: 143)
!639 = !DIEnumerator(name: "sched_setscheduler", value: 144)
!640 = !DIEnumerator(name: "sched_getscheduler", value: 145)
!641 = !DIEnumerator(name: "sched_get_priority_max", value: 146)
!642 = !DIEnumerator(name: "sched_get_priority_min", value: 147)
!643 = !DIEnumerator(name: "sched_rr_get_interval", value: 148)
!644 = !DIEnumerator(name: "mlock", value: 149)
!645 = !DIEnumerator(name: "munlock", value: 150)
!646 = !DIEnumerator(name: "mlockall", value: 151)
!647 = !DIEnumerator(name: "munlockall", value: 152)
!648 = !DIEnumerator(name: "vhangup", value: 153)
!649 = !DIEnumerator(name: "modify_ldt", value: 154)
!650 = !DIEnumerator(name: "pivot_root", value: 155)
!651 = !DIEnumerator(name: "_sysctl", value: 156)
!652 = !DIEnumerator(name: "prctl", value: 157)
!653 = !DIEnumerator(name: "arch_prctl", value: 158)
!654 = !DIEnumerator(name: "adjtimex", value: 159)
!655 = !DIEnumerator(name: "setrlimit", value: 160)
!656 = !DIEnumerator(name: "chroot", value: 161)
!657 = !DIEnumerator(name: "sync", value: 162)
!658 = !DIEnumerator(name: "acct", value: 163)
!659 = !DIEnumerator(name: "settimeofday", value: 164)
!660 = !DIEnumerator(name: "mount", value: 165)
!661 = !DIEnumerator(name: "umount2", value: 166)
!662 = !DIEnumerator(name: "swapon", value: 167)
!663 = !DIEnumerator(name: "swapoff", value: 168)
!664 = !DIEnumerator(name: "reboot", value: 169)
!665 = !DIEnumerator(name: "sethostname", value: 170)
!666 = !DIEnumerator(name: "setdomainname", value: 171)
!667 = !DIEnumerator(name: "iopl", value: 172)
!668 = !DIEnumerator(name: "ioperm", value: 173)
!669 = !DIEnumerator(name: "create_module", value: 174)
!670 = !DIEnumerator(name: "init_module", value: 175)
!671 = !DIEnumerator(name: "delete_module", value: 176)
!672 = !DIEnumerator(name: "get_kernel_syms", value: 177)
!673 = !DIEnumerator(name: "query_module", value: 178)
!674 = !DIEnumerator(name: "quotactl", value: 179)
!675 = !DIEnumerator(name: "nfsservctl", value: 180)
!676 = !DIEnumerator(name: "getpmsg", value: 181)
!677 = !DIEnumerator(name: "putpmsg", value: 182)
!678 = !DIEnumerator(name: "afs_syscall", value: 183)
!679 = !DIEnumerator(name: "tuxcall", value: 184)
!680 = !DIEnumerator(name: "security", value: 185)
!681 = !DIEnumerator(name: "gettid", value: 186)
!682 = !DIEnumerator(name: "readahead", value: 187)
!683 = !DIEnumerator(name: "setxattr", value: 188)
!684 = !DIEnumerator(name: "lsetxattr", value: 189)
!685 = !DIEnumerator(name: "fsetxattr", value: 190)
!686 = !DIEnumerator(name: "getxattr", value: 191)
!687 = !DIEnumerator(name: "lgetxattr", value: 192)
!688 = !DIEnumerator(name: "fgetxattr", value: 193)
!689 = !DIEnumerator(name: "listxattr", value: 194)
!690 = !DIEnumerator(name: "llistxattr", value: 195)
!691 = !DIEnumerator(name: "flistxattr", value: 196)
!692 = !DIEnumerator(name: "removexattr", value: 197)
!693 = !DIEnumerator(name: "lremovexattr", value: 198)
!694 = !DIEnumerator(name: "fremovexattr", value: 199)
!695 = !DIEnumerator(name: "tkill", value: 200)
!696 = !DIEnumerator(name: "time", value: 201)
!697 = !DIEnumerator(name: "futex", value: 202)
!698 = !DIEnumerator(name: "sched_setaffinity", value: 203)
!699 = !DIEnumerator(name: "sched_getaffinity", value: 204)
!700 = !DIEnumerator(name: "set_thread_area", value: 205)
!701 = !DIEnumerator(name: "io_setup", value: 206)
!702 = !DIEnumerator(name: "io_destroy", value: 207)
!703 = !DIEnumerator(name: "io_getevents", value: 208)
!704 = !DIEnumerator(name: "io_submit", value: 209)
!705 = !DIEnumerator(name: "io_cancel", value: 210)
!706 = !DIEnumerator(name: "get_thread_area", value: 211)
!707 = !DIEnumerator(name: "lookup_dcookie", value: 212)
!708 = !DIEnumerator(name: "epoll_create", value: 213)
!709 = !DIEnumerator(name: "epoll_ctl_old", value: 214)
!710 = !DIEnumerator(name: "epoll_wait_old", value: 215)
!711 = !DIEnumerator(name: "remap_file_pages", value: 216)
!712 = !DIEnumerator(name: "getdents64", value: 217)
!713 = !DIEnumerator(name: "set_tid_address", value: 218)
!714 = !DIEnumerator(name: "restart_syscall", value: 219)
!715 = !DIEnumerator(name: "semtimedop", value: 220)
!716 = !DIEnumerator(name: "fadvise64", value: 221)
!717 = !DIEnumerator(name: "timer_create", value: 222)
!718 = !DIEnumerator(name: "timer_settime", value: 223)
!719 = !DIEnumerator(name: "timer_gettime", value: 224)
!720 = !DIEnumerator(name: "timer_getoverrun", value: 225)
!721 = !DIEnumerator(name: "timer_delete", value: 226)
!722 = !DIEnumerator(name: "clock_settime", value: 227)
!723 = !DIEnumerator(name: "clock_gettime", value: 228)
!724 = !DIEnumerator(name: "clock_getres", value: 229)
!725 = !DIEnumerator(name: "clock_nanosleep", value: 230)
!726 = !DIEnumerator(name: "exit_group", value: 231)
!727 = !DIEnumerator(name: "epoll_wait", value: 232)
!728 = !DIEnumerator(name: "epoll_ctl", value: 233)
!729 = !DIEnumerator(name: "tgkill", value: 234)
!730 = !DIEnumerator(name: "utimes", value: 235)
!731 = !DIEnumerator(name: "vserver", value: 236)
!732 = !DIEnumerator(name: "mbind", value: 237)
!733 = !DIEnumerator(name: "set_mempolicy", value: 238)
!734 = !DIEnumerator(name: "get_mempolicy", value: 239)
!735 = !DIEnumerator(name: "mq_open", value: 240)
!736 = !DIEnumerator(name: "mq_unlink", value: 241)
!737 = !DIEnumerator(name: "mq_timedsend", value: 242)
!738 = !DIEnumerator(name: "mq_timedreceive", value: 243)
!739 = !DIEnumerator(name: "mq_notify", value: 244)
!740 = !DIEnumerator(name: "mq_getsetattr", value: 245)
!741 = !DIEnumerator(name: "kexec_load", value: 246)
!742 = !DIEnumerator(name: "waitid", value: 247)
!743 = !DIEnumerator(name: "add_key", value: 248)
!744 = !DIEnumerator(name: "request_key", value: 249)
!745 = !DIEnumerator(name: "keyctl", value: 250)
!746 = !DIEnumerator(name: "ioprio_set", value: 251)
!747 = !DIEnumerator(name: "ioprio_get", value: 252)
!748 = !DIEnumerator(name: "inotify_init", value: 253)
!749 = !DIEnumerator(name: "inotify_add_watch", value: 254)
!750 = !DIEnumerator(name: "inotify_rm_watch", value: 255)
!751 = !DIEnumerator(name: "migrate_pages", value: 256)
!752 = !DIEnumerator(name: "openat", value: 257)
!753 = !DIEnumerator(name: "mkdirat", value: 258)
!754 = !DIEnumerator(name: "mknodat", value: 259)
!755 = !DIEnumerator(name: "fchownat", value: 260)
!756 = !DIEnumerator(name: "futimesat", value: 261)
!757 = !DIEnumerator(name: "fstatat", value: 262)
!758 = !DIEnumerator(name: "unlinkat", value: 263)
!759 = !DIEnumerator(name: "renameat", value: 264)
!760 = !DIEnumerator(name: "linkat", value: 265)
!761 = !DIEnumerator(name: "symlinkat", value: 266)
!762 = !DIEnumerator(name: "readlinkat", value: 267)
!763 = !DIEnumerator(name: "fchmodat", value: 268)
!764 = !DIEnumerator(name: "faccessat", value: 269)
!765 = !DIEnumerator(name: "pselect6", value: 270)
!766 = !DIEnumerator(name: "ppoll", value: 271)
!767 = !DIEnumerator(name: "unshare", value: 272)
!768 = !DIEnumerator(name: "set_robust_list", value: 273)
!769 = !DIEnumerator(name: "get_robust_list", value: 274)
!770 = !DIEnumerator(name: "splice", value: 275)
!771 = !DIEnumerator(name: "tee", value: 276)
!772 = !DIEnumerator(name: "sync_file_range", value: 277)
!773 = !DIEnumerator(name: "vmsplice", value: 278)
!774 = !DIEnumerator(name: "move_pages", value: 279)
!775 = !DIEnumerator(name: "utimensat", value: 280)
!776 = !DIEnumerator(name: "epoll_pwait", value: 281)
!777 = !DIEnumerator(name: "signalfd", value: 282)
!778 = !DIEnumerator(name: "timerfd_create", value: 283)
!779 = !DIEnumerator(name: "eventfd", value: 284)
!780 = !DIEnumerator(name: "fallocate", value: 285)
!781 = !DIEnumerator(name: "timerfd_settime", value: 286)
!782 = !DIEnumerator(name: "timerfd_gettime", value: 287)
!783 = !DIEnumerator(name: "accept4", value: 288)
!784 = !DIEnumerator(name: "signalfd4", value: 289)
!785 = !DIEnumerator(name: "eventfd2", value: 290)
!786 = !DIEnumerator(name: "epoll_create1", value: 291)
!787 = !DIEnumerator(name: "dup3", value: 292)
!788 = !DIEnumerator(name: "pipe2", value: 293)
!789 = !DIEnumerator(name: "inotify_init1", value: 294)
!790 = !DIEnumerator(name: "preadv", value: 295)
!791 = !DIEnumerator(name: "pwritev", value: 296)
!792 = !DIEnumerator(name: "rt_tgsigqueueinfo", value: 297)
!793 = !DIEnumerator(name: "perf_event_open", value: 298)
!794 = !DIEnumerator(name: "recvmmsg", value: 299)
!795 = !DIEnumerator(name: "fanotify_init", value: 300)
!796 = !DIEnumerator(name: "fanotify_mark", value: 301)
!797 = !DIEnumerator(name: "prlimit64", value: 302)
!798 = !DIEnumerator(name: "name_to_handle_at", value: 303)
!799 = !DIEnumerator(name: "open_by_handle_at", value: 304)
!800 = !DIEnumerator(name: "clock_adjtime", value: 305)
!801 = !DIEnumerator(name: "syncfs", value: 306)
!802 = !DIEnumerator(name: "sendmmsg", value: 307)
!803 = !DIEnumerator(name: "setns", value: 308)
!804 = !DIEnumerator(name: "getcpu", value: 309)
!805 = !DIEnumerator(name: "process_vm_readv", value: 310)
!806 = !DIEnumerator(name: "process_vm_writev", value: 311)
!807 = !DIEnumerator(name: "kcmp", value: 312)
!808 = !DIEnumerator(name: "finit_module", value: 313)
!809 = !DIEnumerator(name: "sched_setattr", value: 314)
!810 = !DIEnumerator(name: "sched_getattr", value: 315)
!811 = !DIEnumerator(name: "renameat2", value: 316)
!812 = !DIEnumerator(name: "seccomp", value: 317)
!813 = !DIEnumerator(name: "getrandom", value: 318)
!814 = !DIEnumerator(name: "memfd_create", value: 319)
!815 = !DIEnumerator(name: "kexec_file_load", value: 320)
!816 = !DIEnumerator(name: "bpf", value: 321)
!817 = !DIEnumerator(name: "execveat", value: 322)
!818 = !DIEnumerator(name: "userfaultfd", value: 323)
!819 = !DIEnumerator(name: "membarrier", value: 324)
!820 = !DIEnumerator(name: "mlock2", value: 325)
!821 = !DIEnumerator(name: "copy_file_range", value: 326)
!822 = !DIEnumerator(name: "preadv2", value: 327)
!823 = !DIEnumerator(name: "pwritev2", value: 328)
!824 = !DIEnumerator(name: "pkey_mprotect", value: 329)
!825 = !DIEnumerator(name: "pkey_alloc", value: 330)
!826 = !DIEnumerator(name: "pkey_free", value: 331)
!827 = !DIEnumerator(name: "statx", value: 332)
!828 = !DIEnumerator(name: "io_pgetevents", value: 333)
!829 = !DIEnumerator(name: "rseq", value: 334)
!830 = !DIEnumerator(name: "pidfd_send_signal", value: 424)
!831 = !DIEnumerator(name: "io_uring_setup", value: 425)
!832 = !DIEnumerator(name: "io_uring_enter", value: 426)
!833 = !DIEnumerator(name: "io_uring_register", value: 427)
!834 = !DIEnumerator(name: "open_tree", value: 428)
!835 = !DIEnumerator(name: "move_mount", value: 429)
!836 = !DIEnumerator(name: "fsopen", value: 430)
!837 = !DIEnumerator(name: "fsconfig", value: 431)
!838 = !DIEnumerator(name: "fsmount", value: 432)
!839 = !DIEnumerator(name: "fspick", value: 433)
!840 = !DIEnumerator(name: "pidfd_open", value: 434)
!841 = !DIEnumerator(name: "clone3", value: 435)
!842 = !DIEnumerator(name: "close_range", value: 436)
!843 = !DIEnumerator(name: "openat2", value: 437)
!844 = !DIEnumerator(name: "pidfd_getfd", value: 438)
!845 = !DIEnumerator(name: "faccessat2", value: 439)
!846 = !DIEnumerator(name: "process_madvise", value: 440)
!847 = !DIEnumerator(name: "epoll_pwait2", value: 441)
!848 = !DIEnumerator(name: "mount_setattr", value: 442)
!849 = !DIEnumerator(name: "landlock_create_ruleset", value: 444)
!850 = !DIEnumerator(name: "landlock_add_rule", value: 445)
!851 = !DIEnumerator(name: "landlock_restrict_self", value: 446)
!852 = !DIEnumerator(name: "memfd_secret", value: 447)
!853 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "anyerror", baseType: !339, size: 16, align: 16, elements: !854)
!854 = !{!855, !856, !857, !858, !859, !860, !861, !862, !863, !864, !865, !866, !867, !868, !869, !870, !871, !872, !873, !874, !875}
!855 = !DIEnumerator(name: "(none)", value: 0)
!856 = !DIEnumerator(name: "DiskQuota", value: 1)
!857 = !DIEnumerator(name: "FileTooBig", value: 2)
!858 = !DIEnumerator(name: "InputOutput", value: 3)
!859 = !DIEnumerator(name: "NoSpaceLeft", value: 4)
!860 = !DIEnumerator(name: "AccessDenied", value: 5)
!861 = !DIEnumerator(name: "BrokenPipe", value: 6)
!862 = !DIEnumerator(name: "SystemResources", value: 7)
!863 = !DIEnumerator(name: "OperationAborted", value: 8)
!864 = !DIEnumerator(name: "NotOpenForWriting", value: 9)
!865 = !DIEnumerator(name: "WouldBlock", value: 10)
!866 = !DIEnumerator(name: "ConnectionResetByPeer", value: 11)
!867 = !DIEnumerator(name: "Unexpected", value: 12)
!868 = !DIEnumerator(name: "Expected closing ]", value: 13)
!869 = !DIEnumerator(name: "Utf8InvalidStartByte", value: 14)
!870 = !DIEnumerator(name: "TruncatedInput", value: 15)
!871 = !DIEnumerator(name: "Utf8ExpectedContinuation", value: 16)
!872 = !DIEnumerator(name: "Utf8OverlongEncoding", value: 17)
!873 = !DIEnumerator(name: "Utf8EncodesSurrogateHalf", value: 18)
!874 = !DIEnumerator(name: "Utf8CodepointTooLarge", value: 19)
!875 = !DIEnumerator(name: "TimedOut", value: 20)
!876 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.Thread.enum:338:27", scope: !877, file: !877, line: 338, baseType: !98, size: 8, align: 8, elements: !878)
!877 = !DIFile(filename: "Thread.zig", directory: "/nix/store/p0zs9razv7s87q8k3459d0wx120i8sp1-zig-0.9.1/lib/zig/std")
!878 = !{!879, !880, !881}
!879 = !DIEnumerator(name: "running", value: 0)
!880 = !DIEnumerator(name: "detached", value: 1)
!881 = !DIEnumerator(name: "completed", value: 2)
!882 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.fmt.Alignment", scope: !883, file: !883, line: 14, baseType: !7, size: 8, align: 8, elements: !884)
!883 = !DIFile(filename: "fmt.zig", directory: "/nix/store/p0zs9razv7s87q8k3459d0wx120i8sp1-zig-0.9.1/lib/zig/std")
!884 = !{!885, !886, !887}
!885 = !DIEnumerator(name: "Left", value: 0)
!886 = !DIEnumerator(name: "Center", value: 1)
!887 = !DIEnumerator(name: "Right", value: 2)
!888 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "@typeInfo(std.fmt.Specifier).Union.tag_type.?", scope: !883, file: !883, line: 286, baseType: !7, size: 8, align: 8, elements: !889)
!889 = !{!86, !890, !891}
!890 = !DIEnumerator(name: "number", value: 1)
!891 = !DIEnumerator(name: "named", value: 2)
!892 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.Thread.ResetEvent.TimedWaitResult", scope: !893, file: !893, line: 65, baseType: !13, size: 8, align: 8, elements: !894)
!893 = !DIFile(filename: "ResetEvent.zig", directory: "/nix/store/p0zs9razv7s87q8k3459d0wx120i8sp1-zig-0.9.1/lib/zig/std/Thread")
!894 = !{!895, !896}
!895 = !DIEnumerator(name: "event_set", value: 0)
!896 = !DIEnumerator(name: "timed_out", value: 1)
!897 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "std.fmt.Case", scope: !883, file: !883, line: 785, baseType: !13, size: 8, align: 8, elements: !898)
!898 = !{!899, !900}
!899 = !DIEnumerator(name: "lower", value: 0)
!900 = !DIEnumerator(name: "upper", value: 1)
