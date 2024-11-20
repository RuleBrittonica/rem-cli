fn foo() -> i32 {
    'bar: loop {
        let n = 1;
        let k = 1;
        if k == 42 {
            break 'bar 4;
        }
        let m = k + 1;
        let h = 1;
    }
}

fn main() {

}
