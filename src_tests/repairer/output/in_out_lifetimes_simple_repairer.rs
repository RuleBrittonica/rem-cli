pub fn new_foo() {
    let mut z: &i32 = &0;
    let mut y = 2;
    z = bar_extracted(z, &mut y);
}
fn bar_extracted<'a>(z: &'a i32, y: &'a mut i32) -> &'a i32 {
    *y = *z + 1;
    &*y
}

fn main() {}



