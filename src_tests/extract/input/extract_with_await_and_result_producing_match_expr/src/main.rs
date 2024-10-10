async fn foo() -> i32 {
    loop {
        let n = 1;
        let k = async { 1 }.await;
        if k == 42 {
            break 3;
        }
        let m = k + 1;
        let h = 1 + m;
    }
}

fn main() {

}
