struct Struct<'a, T>(&'a T);
fn func<T: Debug>(i: Struct<'_, T>) {
    fun_name(i);
}

fn fun_name(i: Struct<'_, T>) {
    foo(i);
}

fn main() {

}
