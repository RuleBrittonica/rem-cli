fn foo() -> i32 {
    'bar: loop {
        let n = 1;
        if let Some(value) = fun_name() {
            break 'bar value;
        }
        let h = 1;
    }
}

fn fun_name() -> Option<i32> {
    let k = 1;
    if k == 42 {
        return Some(4);
    }
    let m = k + 1;
    None
}

fn main() {

}
