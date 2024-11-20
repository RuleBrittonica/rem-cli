fn foo() {
    loop {
        let n = 1;
        let m = fun_name();
        let h = 1 + m;
    }
}

fn fun_name() -> i32 {
    let k = 1;
    fn test() {
        return;
    }
    let m = k + 1;
    m
}

fn main() {

}
