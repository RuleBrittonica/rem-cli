fn fallible() -> Result<(), ()> {
    if true {
        return Err(());
    }
    Ok(())
}

fn main() {

}
