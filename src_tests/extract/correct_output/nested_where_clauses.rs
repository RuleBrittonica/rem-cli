struct Struct<T>(T) where T: Into<i32>;
impl <T> Struct<T> where T: Into<i32> + Copy {
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
