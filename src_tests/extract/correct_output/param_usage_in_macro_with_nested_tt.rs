macro_rules! m {
    ($val:expr) => { $val };
}

fn foo() {
    let n = 1;
    let t = 1;
    let k = fun_name(n, t);
    let m = k + 1;
}

fn fun_name(n: i32, t: i32) -> i32 {
    let k = n * m!((n) + { t });
    k
}

fn main() {

}
