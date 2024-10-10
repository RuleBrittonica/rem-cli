struct Struct<'a, T>(&'a T);
fn func<T: Debug>(i: Struct<'_, T>) {
    foo(i);
}

fn main() {

}
