struct Foo;
impl Foo {
    fn foo(&mut self) {}
}
fn foo() {
    let mut x = Foo;
    while false {
        let y = &mut x;
        fun_name(y);
    }
    let z = x;
}

fn fun_name(y: &mut Foo) {
    y.foo();
}

fn main() {

}
