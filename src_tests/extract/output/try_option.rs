fn bar() -> Option<i32> { None }
fn foo() -> Option<()> {
    let n = bar()?;
    let m = fun_name()?;
    let h = 1 + m;
    Some(())
}

fn fun_name() -> Result<i32, _> {
    let k = foo()?;
    let m = k + 1;
    Ok(m)
}

fn main() {

}
