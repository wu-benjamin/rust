fn minus_one_safe(x: i32) -> i32 {
    if x > 0 {
        return x - 1;
    }
    return x;
}

fn minus_one_unsafe(x: i32) -> i32 {
    return x - 1;
}

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
    println!("{}", minus_one_safe(3));
    println!("{}", minus_one_unsafe(3));
    println!("{}", neg_abs(2));
    println!("{}", abs(-2));
}
