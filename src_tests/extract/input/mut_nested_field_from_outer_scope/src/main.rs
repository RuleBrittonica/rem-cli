struct P { n: i32}
struct C { p: P }
fn foo() {
    let mut c = C { p: P { n: 0 } };
    let mut v = C { p: P { n: 0 } };
    let u = C { p: P { n: 0 } };
    c.p.n += u.p.n;
    let r = &mut v.p.n;
    let m = c.p.n + v.p.n + u.p.n;
}

fn main() {

}
