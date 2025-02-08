use std::ops::ControlFlow;

fn foo() {
    loop {
        let mut n = 1;
        if let ControlFlow::Break(_) = fun_name(n) {
            break;
        }
        let h = 1;
    }
}

fn fun_name(n: i32) -> ControlFlow<()> {
    let m = n + 1;
    if m == 42 {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

fn main() {

}
