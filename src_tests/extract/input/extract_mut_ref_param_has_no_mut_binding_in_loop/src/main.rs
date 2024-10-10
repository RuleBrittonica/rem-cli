struct Foo;
impl Foo {
    fn foo(&mut self) {}
}
fn foo() {
    let mut x = Foo;
    while false {
        let y = &mut x;
        y.foo();
    }
    let z = x;
}

fn main() {

}
