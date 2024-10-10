fn foo() -> Result<(), i64> {
    Result::<i32, i64>::Ok(0)?;
    Ok(())
}

fn main() {

}
