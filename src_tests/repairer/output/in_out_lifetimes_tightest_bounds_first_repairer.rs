pub fn new_foo() {
    let mut z: &i32 = &0;
    let mut y = 2;
    z = bar_extracted(z, &mut y);
}
fn bar_extracted<'lt0>(z: &'lt0 i32, y: &'lt0 mut i32) -> &'lt0 i32 {
    *y = *z + 1;
    &*y
}
fn main() {}
