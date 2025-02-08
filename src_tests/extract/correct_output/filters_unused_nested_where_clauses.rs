struct Struct<T, U>(T, U) where T: Into<i32>, U: Debug;
impl <T, U> Struct<T, U> where T: Into<i32> + Copy, U: Debug {
    fn func<V>(&self, v: V) -> i32 where V: Into<i32> {
        let t = self.0;
        fun_name(t, v)
    }
}

fn fun_name<T, V>(t: T, v: V) -> i32 where T: Into<i32> + Copy, V: Into<i32> {
    t.into() + v.into()
}

fn main() {

}
