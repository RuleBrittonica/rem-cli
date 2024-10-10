fn foo() -> i64 {
    let n = 1;
    let k = match fun_name(n) {
        Ok(value) => value,
        Err(value) => return value,
    };
    (n + k) as i64
}

fn fun_name(n: i32) -> Result<i32, i64> {
    let m = n + 1;
    return Err(1);
    let k = 2;
    Ok(k)
}

fn main() {

}
