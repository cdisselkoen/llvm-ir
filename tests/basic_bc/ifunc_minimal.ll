@strstr = alias ptr (ptr, ptr), ptr @__libc_strstr

@__libc_strstr = ifunc ptr (ptr, ptr), ptr @__libc_strstr_ifunc

define internal ptr @__libc_strstr_ifunc() {
  ret ptr null
}
