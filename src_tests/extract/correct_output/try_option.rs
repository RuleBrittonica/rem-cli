fn bar() -> Option<i32> { None }
fn foo() -> Option<()> {
    let n = bar()?;
    let m = fun_name()?;
    let h = 1 + m;
    Some(())
}

fn fun_name() -> Option<i32> {
    let k = foo()?;
    let m = k + 1;
    Some(m)
}

fn main() {

}
