fn neg_abs(mut x: i32) -> i32 {
    if x > 0 {
        x = -1 * x;
    }
    return x;
}

fn abs(mut x: i32) -> i32 {
    if x < 0 {
        x = -1 * x;
    }
    return x;
}

fn main() {
    println!("{}", neg_abs(2));
    println!("{}", abs(-2));
}

// RUSTC_LOG=rustc_symbolic_exec=debug rustc +stage1 test_2.rs && ./test_2
