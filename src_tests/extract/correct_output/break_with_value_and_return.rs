fn foo() -> i64 {
    loop {
        let n = 1;
        let m = match fun_name() {
            Ok(value) => value,
            Err(value) => break value,
        };
        let h = 1 + m;
    }
}

fn fun_name() -> Result<i32, i64> {
    let k = 1;
    if k == 42 {
        return Err(3);
    }
    let m = k + 1;
    Ok(m)
}

fn main() {

}
