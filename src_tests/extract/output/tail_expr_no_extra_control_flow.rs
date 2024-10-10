fn fallible() -> Result<(), ()> {
    fun_name()
}

fn fun_name() -> _ {
    if true {
        return Err(());
    }
    Ok(())
}

fn main() {

}
