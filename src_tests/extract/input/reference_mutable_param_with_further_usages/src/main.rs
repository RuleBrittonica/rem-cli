pub struct Foo {
    field: u32,
}

pub fn testfn(arg: &mut Foo) {
    arg.field = 8;
    // Simulating access after the extracted portion
    arg.field = 16;
}

fn main() {

}
