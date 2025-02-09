fn foo() -> Result<(), i64> {
    let n = 1;
    let k = foo()?;
    let m = k + 1;
    let h = 1 + m;
    Ok(())
}

fn main() {

}
