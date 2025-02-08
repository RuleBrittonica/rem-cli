struct Struct<T: Into<i32>, U: Debug>(T, U);
impl <T: Into<i32> + Copy, U: Debug> Struct<T, U> {
    fn func<V: Into<i32>>(&self, v: V) -> i32 {
        let t = self.0;
        t.into() + v.into()
    }
}

fn main() {

}
