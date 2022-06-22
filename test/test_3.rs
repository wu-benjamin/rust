fn unsat_safe(x: i32) -> () {
    assert!(x == x);
}

fn sat_unsafe(x: i32) -> () {
    assert!(x < 13);
}

fn main() {
    unsat_safe(12);
    sat_unsafe(-250);
}

// RUSTC_LOG=rustc_symbolic_exec=debug rustc +stage1 test_3.rs && ./test_3
