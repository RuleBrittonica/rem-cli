struct C { n: i32 }
fn foo() {
    let mut c = C { n: 0 };
    fun_name(&mut c);
    let m = c.n + 1;
}

fn fun_name(c: &mut C) {
    c.n += 1;
}

fn main() {

}
