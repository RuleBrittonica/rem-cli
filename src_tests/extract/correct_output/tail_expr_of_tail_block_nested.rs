fn f() -> Option<()> {
    if true {
        fun_name()
    } else {
        None
    }
}

fn fun_name() -> Option<()> {
    let a = if true {
        Some(())?
    } else {
        ()
    };
    Some(a)
}

fn main() {

}
