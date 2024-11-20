struct Struct(i32);
trait Trait {
    fn bar(&self) -> i32;
}

impl Struct {
    fn foo() {}
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        let three_squared = 3 * 3;
        self.0 + three_squared
    }
}

fn main() {

}
