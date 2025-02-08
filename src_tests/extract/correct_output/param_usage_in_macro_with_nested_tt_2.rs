macro_rules! m {
    ($val:expr) => { $val };
}

struct S(i32);
impl S {
    fn foo(&self) {
        let n = 1;
        let k = self.fun_name(n);
        let m = k + 1;
    }

    fn fun_name(&self, n: i32) -> i32 {
        let k = n * m!((n) + { self.0 });
        k
    }
}

fn main() {

}
