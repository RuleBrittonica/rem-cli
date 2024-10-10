fn foo() {
    let n = 1;
    let (k, m) = fun_name(n);
    let h = k + m;
}

fn fun_name(n: i32) -> (i32, i32) {
    let k = n * n;
    let m = k + 2;
    (k, m)
}

fn main() {

}
