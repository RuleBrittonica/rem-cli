fn foo() -> Option<()> {
    let n = 1;
    let k = foo()?;
    if k == 42 {
        return None;
    }
    let m = k + 1;
    let h = 1 + m;
    Some(())
}

fn main() {

}
