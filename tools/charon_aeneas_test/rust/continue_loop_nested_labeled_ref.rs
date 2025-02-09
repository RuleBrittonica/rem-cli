use std::ops::ControlFlow;

fn foo() {
    'bar: loop {
        loop {
            if let ControlFlow::Break(_) = fun_name() {
                continue 'bar;
            }
        }
    }
}

fn fun_name() -> ControlFlow<()> {
    return ControlFlow::Break(());
    ControlFlow::Continue(())
}

fn main() {

}
