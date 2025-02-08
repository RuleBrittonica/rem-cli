trait I {
    fn inc(&mut self);
}
impl I for i32 {
    fn inc(&mut self) { *self += 1 }
}
fn foo() {
    let mut n = 1;
    fun_name(n);
}

fn fun_name(mut n: i32) {
    n.inc();
}

fn main() {

}
