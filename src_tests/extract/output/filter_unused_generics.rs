fn func<T: Debug, U: Copy>(i: T, u: U) {
    bar(u);
    fun_name(i);
}

fn fun_name<T: Debug>(i: T) {
    foo(i);
}

fn main() {

}
