fn foo() -> Option<()> {
    let n = 1;
    fun_name()?;
    let h = 1 + n;
    Some(())
}

fn fun_name() -> Result<(), _> {
    let k = foo()?;
    let m = k + 1;
    Ok(())
}

fn main() {

}
