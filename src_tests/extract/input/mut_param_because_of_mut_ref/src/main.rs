fn foo() {
    let mut n = 1;
    let v = &mut n;
    *v += 1;
    let k = n;
}

fn main() {

}
