fn foo() -> Option<()> {
    let n = 1;
    let k = foo()?;
    let m = k + 1;
    let h = 1 + n;
    Some(())
}

fn main() {

}
