macro_rules! m {
    ($val:expr) => { $val };
}

fn foo() {
    let n = 1;
    let t = 1;
    let k = n * m!((n) + { t });
    let m = k + 1;
}

fn main() {

}
