fn foo() {
    loop {
        let n = 1;
        let m = match fun_name() {
            Some(value) => value,
            None => break,
        };
        let h = 1 + m;
    }
}

fn fun_name() -> Option<i32> {
    let k = 1;
    loop {
        break;
    }
    if k == 42 {
        return None;
    }
    let m = k + 1;
    Some(m)
}

fn main() {

}
