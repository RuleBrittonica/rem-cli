fn fallible() -> Result<(), ()> {
    fun_name()
}

fn fun_name() -> Result<(), ()> {
    if true {
        return Err(());
    }
    Ok(())
}

fn main() {

}
