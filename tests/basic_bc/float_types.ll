; This file written directly as .ll, mostly for the purpose of testing
; LLVM 11's new BFloat type

define void @takes_half(half) {
  ret void
}

define void @takes_bfloat(bfloat) {
  ret void
}

define void @takes_float(float) {
  ret void
}

define void @takes_double(double) {
  ret void
}

define void @takes_fp128(fp128) {
  ret void
}

define void @takes_x86_fp80(x86_fp80) {
  ret void
}

define void @takes_ppc_fp128(ppc_fp128) {
  ret void
}

define half* @returns_half() {
  %1 = alloca half
  ret half* %1
}

define bfloat* @returns_bfloat() {
  %1 = alloca bfloat
  ret bfloat* %1
}

define float* @returns_float() {
  %1 = alloca float
  ret float* %1
}

define double* @returns_double() {
  %1 = alloca double
  ret double* %1
}

define fp128* @returns_fp128() {
  %1 = alloca fp128
  ret fp128* %1
}

define x86_fp80* @returns_x86_fp80() {
  %1 = alloca x86_fp80
  ret x86_fp80* %1
}

define ppc_fp128* @returns_ppc_fp128() {
  %1 = alloca ppc_fp128
  ret ppc_fp128* %1
}
