fn f() -> Option<()> {
    if true {
        fun_name()
    } else {
        None
    }
}

fn fun_name() -> _ {
    let a = if true {
        Some(())?
    } else {
        ()
    };
    Some(a)
}

fn main() {

}
