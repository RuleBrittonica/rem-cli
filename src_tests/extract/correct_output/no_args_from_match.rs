fn foo() {
    let v: i32 = fun_name();
}

fn fun_name() -> i32 {
    match Some(1) {
        Some(x) => x,
        None => 0,
    }
}

fn main() {

}
