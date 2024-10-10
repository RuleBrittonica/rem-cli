fn bar() -> Option<i32> { None }
fn foo() -> Option<()> {
    let n = bar()?;
    let k = foo()?;
    let m = k + 1;
    let h = 1 + m;
    Some(())
}

fn main() {

}
