pub struct Foo {
    field: u32,
}

pub fn testfn(arg: &mut Foo) {
    fun_name(arg);
}

fn fun_name(arg: &mut Foo) {
    arg.field = 8;
}

fn main() {

}
