; This file written directly as .ll, mostly copied from LLVM's
; compatibility.ll, but with function definitions instead of declarations

; -- Return attributes
define zeroext i64 @f.zeroext() {
  unreachable
}
define signext i64 @f.signext() {
  unreachable
}
define inreg i32* @f.inreg() {
  unreachable
}
define noalias i32* @f.noalias() {
  unreachable
}
define nonnull i32* @f.nonnull() {
  unreachable
}
define dereferenceable(4) i32* @f.dereferenceable4() {
  unreachable
}
define dereferenceable(8) i32* @f.dereferenceable8() {
  unreachable
}
define dereferenceable_or_null(4) i32* @f.dereferenceable4_or_null() {
  unreachable
}
define dereferenceable_or_null(8) i32* @f.dereferenceable8_or_null() {
  unreachable
}

; -- Parameter attributes
define void @f.param.zeroext(i8 zeroext) {
  ret void
}
define void @f.param.signext(i8 signext) {
  ret void
}
define void @f.param.inreg(i8 inreg) {
  ret void
}
define void @f.param.byval({ i8, i8 }* byval({i8, i8})) {
  ret void
}
define void @f.param.inalloca(i8* inalloca(i8)) {
  ret void
}
define void @f.param.sret(i8* sret(i8)) {
  ret void
}
define void @f.param.noalias(i8* noalias) {
  ret void
}
define void @f.param.nocapture(i8* nocapture) {
  ret void
}
define void @f.param.nest(i8* nest) {
  ret void
}
define i8* @f.param.returned(i8* returned) {
  ret i8* %0
}
define void @f.param.nonnull(i8* nonnull) {
  ret void
}
define void @f.param.dereferenceable(i8* dereferenceable(4)) {
  ret void
}
define void @f.param.dereferenceable_or_null(i8* dereferenceable_or_null(4)) {
  ret void
}

; -- Function attributes
define void @f.alignstack4() alignstack(4) {
  ret void
}
define void @f.alignstack8() alignstack(8) {
  ret void
}
define void @f.alwaysinline() alwaysinline {
  ret void
}
define void @f.cold() cold {
  ret void
}
define void @f.convergent() convergent {
  ret void
}
define void @f.inlinehint() inlinehint {
  ret void
}
define void @f.jumptable() unnamed_addr jumptable {
  ret void
}
define void @f.minsize() minsize {
  ret void
}
define void @f.naked() naked {
  ret void
}
define void @f.nobuiltin() nobuiltin {
  ret void
}
define void @f.noduplicate() noduplicate {
  ret void
}
define void @f.noimplicitfloat() noimplicitfloat {
  ret void
}
define void @f.noinline() noinline {
  ret void
}
define void @f.nonlazybind() nonlazybind {
  ret void
}
define void @f.noredzone() noredzone {
  ret void
}
define void @f.noreturn() noreturn {
  ret void
}
define void @f.nounwind() nounwind {
  ret void
}
define void @f.optnone() noinline optnone {
  ret void
}
define void @f.optsize() optsize {
  ret void
}
define void @f.readnone() readnone {
  ret void
}
define void @f.readonly() readonly {
  ret void
}
define void @f.returns_twice() returns_twice {
  ret void
}
define void @f.safestack() safestack {
  ret void
}
define void @f.sanitize_address() sanitize_address {
  ret void
}
define void @f.sanitize_memory() sanitize_memory {
  ret void
}
define void @f.sanitize_thread() sanitize_thread {
  ret void
}
define void @f.ssp() ssp {
  ret void
}
define void @f.sspreq() sspreq {
  ret void
}
define void @f.sspstrong() sspstrong {
  ret void
}
define void @f.thunk() "thunk" {
  ret void
}
define void @f.uwtable() uwtable {
  ret void
}
define void @f.kvpair() "cpu"="cortex-a8" {
  ret void
}
define void @f.norecurse() norecurse {
  ret void
}
define void @f.inaccessiblememonly() inaccessiblememonly {
  ret void
}
define void @f.inaccessiblemem_or_argmemonly() inaccessiblemem_or_argmemonly {
  ret void
}
define void @f.strictfp() strictfp {
  ret void
}


; -- new memory(...) attribute in LLVM 16

define void @f.default_none() memory(none) {
  ret void
}

define void @f.default_read() memory(read) {
  ret void
}

define void @f.default_write() memory(write) {
  ret void
}

define void @f.default_readwrite() memory(readwrite) {
  ret void
}

define void @f.default_none_arg_readwrite() memory(none, argmem: readwrite) {
  ret void
}

define void @f.default_readwrite_arg_none() memory(readwrite, argmem: none) {
  ret void
}

define void @f.arg_read() memory(argmem: read) {
  ret void
}

define void @f.inaccessiblemem_read() memory(inaccessiblemem: read) {
  ret void
}

define void @f.default_read_inaccessiblemem_write_arg_none() memory(read, inaccessiblemem: write, argmem: none) {
  ret void
}
