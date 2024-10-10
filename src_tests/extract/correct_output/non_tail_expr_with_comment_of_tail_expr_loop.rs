pub fn f() {
    loop {
        if let ControlFlow::Break(_) = fun_name() {
            continue;
        }
        if false {
            break;
        }
    }
}

fn fun_name() -> ControlFlow<()> {
    // A comment
    if true {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

fn main() {

}
