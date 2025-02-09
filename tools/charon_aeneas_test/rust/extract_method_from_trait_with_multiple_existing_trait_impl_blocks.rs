struct Struct(i32);
trait Trait {
    fn bar(&self) -> i32;
}
trait TraitBefore {
    fn before(&self) -> i32;
}
trait TraitAfter {
    fn after(&self) -> i32;
}

impl TraitBefore for Struct {
    fn before(&self) -> i32 {
        42
    }
}

impl Struct {
    fn foo(){}
}

impl TraitAfter for Struct {
    fn after(&self) -> i32 {
        42
    }
}

impl Trait for Struct {
    fn bar(&self) -> i32 {
        self.0 + 2
    }
}

fn main() {

}
