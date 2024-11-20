#[derive(Clone, Copy)]
struct Counter(i32);
fn foo() {
    let c = Counter(0);
    let n = c.0;
    let m = c.0;
}

fn main() {

}
