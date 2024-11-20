struct Counter(i32);
fn foo() {
    let Counter(n) = Counter(0);
    let m = n;
}

fn main() {

}
