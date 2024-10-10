fn f() -> Option<()> {
    if true {
        let a = if true {
            Some(())?
        } else {
            ()
        };
        Some(a)
    } else {
        None
    }
}

fn main() {

}
