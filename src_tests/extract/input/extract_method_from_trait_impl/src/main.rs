struct Struct(i32);
trait Trait {
    fn bar(&self) -> i32;
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        self.0 + 2
    }
}

fn main() {

}
