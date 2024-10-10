fn foo() -> Option<()> {
    let n = 1;
    fun_name()?;
    let h = 1 + n;
    Some(())
}

fn fun_name() -> Option<()> {
    let k = foo()?;
    let m = k + 1;
    Some(())
}

fn main() {

}
