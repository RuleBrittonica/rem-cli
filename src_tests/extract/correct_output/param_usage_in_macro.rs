macro_rules! m {
    ($val:expr) => { $val };
}

fn foo() {
    let n = 1;
    let k = fun_name(n);
    let m = k + 1;
}

fn fun_name(n: i32) -> i32 {
    let k = n * m!(n);
    k
}

fn main() {

}
