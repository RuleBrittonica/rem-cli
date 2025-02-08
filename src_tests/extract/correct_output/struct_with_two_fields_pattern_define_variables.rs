struct Counter { n: i32, m: i32 }
fn foo() {
    let (n, k) = fun_name();
    let h = n + k;
}

fn fun_name() -> (i32, i32) {
    let Counter { n, m: k } = Counter { n: 1, m: 2 }
    (n, k)
}

fn main() {

}
