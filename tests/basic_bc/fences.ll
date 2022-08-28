; This file written directly as .ll, mostly for the purpose of testing
; fence instructions and associated `MemoryOrdering`s

define void @fences() {
  fence seq_cst
  fence acquire
  fence release
  fence acq_rel
  fence syncscope("singlethread") seq_cst
  ret void
}
