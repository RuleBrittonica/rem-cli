#[derive(Clone, Copy)]
struct Counter(i32);
fn foo() {
    let c = Counter(0);
    fun_name(&c);
    let m = c.0;
}

fn fun_name(c: &Counter) {
    let n = c.0;
}

fn main() {

}
