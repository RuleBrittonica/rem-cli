fn func<T: Default>() -> T {
    fun_name()
}

fn fun_name<T: Default>() -> T {
    T::default()
}

fn main() {

}
