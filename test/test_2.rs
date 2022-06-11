fn neg_abs(mut x: i32) -> i32 {
    if x > 0 {
        x = -1 * x;
    }
    return x;
}

fn main() {
    let x = 13;
    assert!(neg_abs(x) <= 0);
}

