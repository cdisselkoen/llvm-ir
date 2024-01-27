fn main() {
    let mut versions = vec![];
    if cfg!(feature = "llvm-9") {
        versions.push(9);
    }
    if cfg!(feature = "llvm-10") {
        versions.push(10);
    }
    if cfg!(feature = "llvm-11") {
        versions.push(11);
    }
    if cfg!(feature = "llvm-12") {
        versions.push(12);
    }
    if cfg!(feature = "llvm-13") {
        versions.push(13);
    }
    if cfg!(feature = "llvm-14") {
        versions.push(14);
    }
    if cfg!(feature = "llvm-15") {
        versions.push(15);
    }
    if cfg!(feature = "llvm-16") {
        versions.push(16);
    }
    if cfg!(feature = "llvm-17") {
        versions.push(17);
    }
    match versions.len() {
        0 => panic!("llvm-ir: Please select an LLVM version using a Cargo feature."),
        1 => {},
        _ => panic!("llvm-ir: Multiple LLVM versions selected. Please activate only one LLVM version feature. (Got {:?})", versions),
    };
}
