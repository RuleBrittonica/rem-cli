fn foo() {
    loop {
        let n = 1;
        let k = match fun_name(n) {
            Some(value) => value,
            None => break,
        };
        let h = 1 + k;
    }
}

fn fun_name(n: i32) -> Option<i32> {
    let m = n + 1;
    return None;
    let k = 2;
    Some(k)
}

fn main() {

}
