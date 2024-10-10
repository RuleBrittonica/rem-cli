struct C { n: i32 }
fn foo() {
    let mut c = C { n: 0 };
    c.n += 1;
    let m = c.n + 1;
}

fn main() {

}
