fn main() {
    test_incr();
}

fn ref_incr(x: &mut i32) {
    fun_name(x);
}

fn fun_name(x: &mut i32) {
    *x = *x + 1;
}

fn test_incr() {
    let mut y = 0i32;
    ref_incr(&mut y);
    assert!(y == 1);
}