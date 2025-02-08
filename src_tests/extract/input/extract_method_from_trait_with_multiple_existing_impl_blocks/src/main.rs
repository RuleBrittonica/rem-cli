struct Struct(i32);
struct StructBefore(i32);
struct StructAfter(i32);
trait Trait {
    fn bar(&self) -> i32;
}

impl StructBefore {
    fn foo(){}
}

impl Struct {
    fn foo(){}
}

impl StructAfter {
    fn foo(){}
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        self.0 + 2
    }
}

fn main() {

}
