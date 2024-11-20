fn foo() {
    let m = 2;
    let n = 1;
    let (mut v, mut w) = fun_name(m, n);
    v += 1;
    w += 1;
}

fn fun_name(m: i32, n: i32) -> (i32, i32) {
    let mut v = m * n;
    let mut w = 3;
    (v, w)
}

fn main() {

}
