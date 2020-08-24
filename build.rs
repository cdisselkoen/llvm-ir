fn main() {
    // This code mostly lifted from the build.rs in llvm-sys-featured
    // Allows us to use the LLVM_VERSION_9_OR_GREATER etc configs ourselves

    let mut versions = vec![];
    if cfg!(feature = "llvm-8") {
        versions.push(8);
    }
    if cfg!(feature = "llvm-9") {
        versions.push(9);
    }
    if cfg!(feature = "llvm-10") {
        versions.push(10);
    }
    let selected_version = if versions.len() == 0 {
        panic!("llvm-ir: Please select an LLVM version using a Cargo feature.")
    } else if versions.len() > 1 {
        panic!("llvm-ir: Multiple LLVM versions selected. Please activate only one LLVM version feature. (Got {:?})", versions);
    } else {
        versions[0]
    };

    // For convenience we set a number of configuration options to avoid
    // checking complex combinations of features all the time.
    if selected_version >= 9 {
        println!("cargo:rustc-cfg=LLVM_VERSION_9_OR_GREATER");
    }
    if selected_version >= 10 {
        println!("cargo:rustc-cfg=LLVM_VERSION_10_OR_GREATER");
    }
    if selected_version <= 9 {
        println!("cargo:rustc-cfg=LLVM_VERSION_9_OR_LOWER");
    }
    if selected_version <= 8 {
        println!("cargo:rustc-cfg=LLVM_VERSION_8_OR_LOWER");
    }
}
