use std::ops::ControlFlow;

fn foo() {
    loop {
        let mut n = 1;
        if let ControlFlow::Break(_) = fun_name(&mut n) {
            break;
        }
        let h = 1 + n;
    }
}

fn fun_name(n: &mut i32) -> ControlFlow<()> {
    let m = *n + 1;
    return ControlFlow::Break(());
    *n += m;
    ControlFlow::Continue(())
}

fn main() {

}
