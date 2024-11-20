async fn foo() -> i32 {
    loop {
        let n = 1;
        let m = match fun_name().await {
            Ok(value) => value,
            Err(value) => break value,
        };
        let h = 1 + m;
    }
}

async fn fun_name() -> Result<i32, i32> {
    let k = async { 1 }.await;
    if k == 42 {
        return Err(3);
    }
    let m = k + 1;
    Ok(m)
}

fn main() {

}
