fn f() -> Option<()> {
    if true {
        let a = fun_name()?;;
        Some(a)
    } else {
        None
    }
}

fn fun_name() -> Result<(), _> {
    Ok(if true {
        Some(())?
    } else {
        ()
    })
}

fn main() {

}
