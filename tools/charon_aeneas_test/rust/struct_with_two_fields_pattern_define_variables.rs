struct Counter { n: i32, m: i32 }
fn foo() {
    let Counter { n, m: k } = Counter { n: 1, m: 2 }
    let h = n + k;
}

fn main() {

}
