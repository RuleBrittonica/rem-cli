fn foo() -> Result<(), i64> {
    let n = 1;
    let m = fun_name()?;
    let h = 1 + m;
    Ok(())
}

fn fun_name() -> Result<i32, _> {
    let k = foo()?;
    if k == 42 {
        return Ok(1);
    }
    let m = k + 1;
    Ok(m)
}

fn main() {

}
