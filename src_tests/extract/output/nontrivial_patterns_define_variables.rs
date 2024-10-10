struct Counter(i32);
fn foo() {
    let n = fun_name();
    let m = n;
}

fn fun_name() -> i32 {
    let Counter(n) = Counter(0);
    n
}

fn main() {

}
