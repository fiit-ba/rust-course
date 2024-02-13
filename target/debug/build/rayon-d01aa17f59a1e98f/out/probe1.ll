; ModuleID = 'probe1.8cc8e6c2-cgu.0'
source_filename = "probe1.8cc8e6c2-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>" = type { %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>" }
%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>" = type { { i32, i32 }, i64, i8, [7 x i8] }

@alloc22 = private unnamed_addr constant <{ [27 x i8] }> <{ [27 x i8] c"assertion failed: step != 0" }>, align 1
@alloc23 = private unnamed_addr constant <{ [89 x i8] }> <{ [89 x i8] c"/rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/iter/adapters/step_by.rs" }>, align 1
@alloc24 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc23, [16 x i8] c"Y\00\00\00\00\00\00\00\15\00\00\00\09\00\00\00" }>, align 8

; core::iter::traits::iterator::Iterator::rev
; Function Attrs: inlinehint nonlazybind uwtable
define void @_ZN4core4iter6traits8iterator8Iterator3rev17h3d4365da7672ddb7E(ptr sret(%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>") %0, ptr %self) unnamed_addr #0 {
start:
  %_2 = alloca %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>", align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_2, ptr align 8 %self, i64 24, i1 false)
; call core::iter::adapters::rev::Rev<T>::new
  call void @"_ZN4core4iter8adapters3rev12Rev$LT$T$GT$3new17h04204b920f3d2264E"(ptr sret(%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>") %0, ptr %_2)
  ret void
}

; core::iter::traits::iterator::Iterator::step_by
; Function Attrs: inlinehint nonlazybind uwtable
define void @_ZN4core4iter6traits8iterator8Iterator7step_by17h7b7dc37ac00c169aE(ptr sret(%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>") %0, i32 %self.0, i32 %self.1, i64 %step) unnamed_addr #0 {
start:
; call core::iter::adapters::step_by::StepBy<I>::new
  call void @"_ZN4core4iter8adapters7step_by15StepBy$LT$I$GT$3new17he4a98538c3cf885cE"(ptr sret(%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>") %0, i32 %self.0, i32 %self.1, i64 %step)
  ret void
}

; core::iter::adapters::rev::Rev<T>::new
; Function Attrs: nonlazybind uwtable
define void @"_ZN4core4iter8adapters3rev12Rev$LT$T$GT$3new17h04204b920f3d2264E"(ptr sret(%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>") %0, ptr %iter) unnamed_addr #1 {
start:
  %_2 = alloca %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>", align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_2, ptr align 8 %iter, i64 24, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %0, ptr align 8 %_2, i64 24, i1 false)
  ret void
}

; core::iter::adapters::step_by::StepBy<I>::new
; Function Attrs: nonlazybind uwtable
define void @"_ZN4core4iter8adapters7step_by15StepBy$LT$I$GT$3new17he4a98538c3cf885cE"(ptr sret(%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>") %0, i32 %iter.0, i32 %iter.1, i64 %step) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %1 = alloca { ptr, i32 }, align 8
  %_4 = icmp ne i64 %step, 0
  %_3 = xor i1 %_4, true
  br i1 %_3, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_8 = sub i64 %step, 1
  %2 = getelementptr inbounds { i32, i32 }, ptr %0, i32 0, i32 0
  store i32 %iter.0, ptr %2, align 8
  %3 = getelementptr inbounds { i32, i32 }, ptr %0, i32 0, i32 1
  store i32 %iter.1, ptr %3, align 4
  %4 = getelementptr inbounds %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>", ptr %0, i32 0, i32 1
  store i64 %_8, ptr %4, align 8
  %5 = getelementptr inbounds %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>", ptr %0, i32 0, i32 2
  store i8 1, ptr %5, align 8
  ret void

bb1:                                              ; preds = %start
; invoke core::panicking::panic
  invoke void @_ZN4core9panicking5panic17h545818946343732bE(ptr align 1 @alloc22, i64 27, ptr align 8 @alloc24) #4
          to label %unreachable unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %6 = load ptr, ptr %1, align 8
  %7 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  %8 = load i32, ptr %7, align 8
  %9 = insertvalue { ptr, i32 } undef, ptr %6, 0
  %10 = insertvalue { ptr, i32 } %9, i32 %8, 1
  resume { ptr, i32 } %10

cleanup:                                          ; preds = %bb1
  %11 = landingpad { ptr, i32 }
          cleanup
  %12 = extractvalue { ptr, i32 } %11, 0
  %13 = extractvalue { ptr, i32 } %11, 1
  %14 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 0
  store ptr %12, ptr %14, align 8
  %15 = getelementptr inbounds { ptr, i32 }, ptr %1, i32 0, i32 1
  store i32 %13, ptr %15, align 8
  br label %bb3

unreachable:                                      ; preds = %bb1
  unreachable
}

; probe1::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe15probe17h204efdfc4969c80eE() unnamed_addr #1 {
start:
  %_3 = alloca { i32, i32 }, align 4
  %_2 = alloca %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>", align 8
  %_1 = alloca %"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>", align 8
  store i32 0, ptr %_3, align 4
  %0 = getelementptr inbounds { i32, i32 }, ptr %_3, i32 0, i32 1
  store i32 10, ptr %0, align 4
  %1 = getelementptr inbounds { i32, i32 }, ptr %_3, i32 0, i32 0
  %2 = load i32, ptr %1, align 4
  %3 = getelementptr inbounds { i32, i32 }, ptr %_3, i32 0, i32 1
  %4 = load i32, ptr %3, align 4
; call core::iter::traits::iterator::Iterator::step_by
  call void @_ZN4core4iter6traits8iterator8Iterator7step_by17h7b7dc37ac00c169aE(ptr sret(%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>") %_2, i32 %2, i32 %4, i64 2)
; call core::iter::traits::iterator::Iterator::rev
  call void @_ZN4core4iter6traits8iterator8Iterator3rev17h3d4365da7672ddb7E(ptr sret(%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>") %_1, ptr %_2)
  ret void
}

; Function Attrs: argmemonly nocallback nofree nounwind willreturn
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #2

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17h545818946343732bE(ptr align 1, i64, ptr align 8) unnamed_addr #3

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { argmemonly nocallback nofree nounwind willreturn }
attributes #3 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #4 = { noreturn }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
