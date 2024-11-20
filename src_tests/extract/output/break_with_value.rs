fn foo() -> i32 {
    loop {
        let n = 1;
        if let Some(value) = fun_name() {
            break value;
        }
        let h = 1;
    }
}

fn fun_name() -> Option<i32> {
    let k = 1;
    if k == 42 {
        return Some(3);
    }
    let m = k + 1;
    None
}

fn main() {

}
