struct P { n: i32}
struct C { p: P }
fn foo() {
    let mut c = C { p: P { n: 0 } };
    let mut v = C { p: P { n: 0 } };
    let u = C { p: P { n: 0 } };
    fun_name(&mut c, &u, &mut v);
    let m = c.p.n + v.p.n + u.p.n;
}

fn fun_name(c: &mut C, u: &C, v: &mut C) {
    c.p.n += u.p.n;
    let r = &mut v.p.n;
}

fn main() {

}
