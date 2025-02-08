fn foo() -> Result<(), i64> {
    fun_name()
}

fn fun_name() -> Result<(), i64> {
    Result::<i32, i64>::Ok(0)?;
    Ok(())
}

fn main() {

}
