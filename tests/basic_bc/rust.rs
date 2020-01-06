// testing some LLVM IR emitted by rustc

pub fn rust_loop(a: isize, b: isize, v: &mut Vec<isize>) -> isize {
    let mut sum = 0;
    for i in v.iter() {
        sum += if i % 3 == 1 { i + a } else { i + b };
    }
    for i in 0 .. 5 {
        v[i] = (i + 2) as isize;
    }
    sum
}
