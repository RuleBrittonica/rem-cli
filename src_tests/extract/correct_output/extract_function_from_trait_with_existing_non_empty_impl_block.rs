struct Struct(i32);
trait Trait {
    fn bar(&self) -> i32;
}

impl Struct {
    fn foo() {}
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        let three_squared = fun_name();
        self.0 + three_squared
    }
}

fn fun_name() -> i32 {
    3 * 3
}

fn main() {

}
