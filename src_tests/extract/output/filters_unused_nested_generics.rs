struct Struct<T: Into<i32>, U: Debug>(T, U);
impl <T: Into<i32> + Copy, U: Debug> Struct<T, U> {
    fn func<V: Into<i32>>(&self, v: V) -> i32 {
        let t = self.0;
        fun_name(t, v)
    }
}

fn fun_name<T: Into<i32> + Copy, V: Into<i32>>(t: T, v: V) -> i32 {
    t.into() + v.into()
}

fn main() {

}
