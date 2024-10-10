fn func<T, U>(i: T, u: U) where T: Debug, U: Copy {
    bar(u);
    fun_name(i);
}

fn fun_name<T>(i: T) where T: Debug {
    foo(i);
}

fn main() {

}
