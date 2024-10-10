fn foo() -> Result<(), i64> {
    let n = 1;
    let k = foo()?;
    if k == 42 {
        return Ok(1);
    }
    let m = k + 1;
    let h = 1 + m;
    Ok(())
}

fn main() {

}
