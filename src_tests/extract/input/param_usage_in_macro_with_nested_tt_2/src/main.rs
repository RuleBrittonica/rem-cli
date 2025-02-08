macro_rules! m {
    ($val:expr) => { $val };
}

struct S(i32);
impl S {
    fn foo(&self) {
        let n = 1;
        let k = n * m!((n) + { self.0 });
        let m = k + 1;
    }
}

fn main() {

}
