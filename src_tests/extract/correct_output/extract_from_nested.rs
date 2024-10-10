fn main() {
    let x = true;
    let tuple = match x {
        true => (fun_name(), true)
        _ => (0, false)
    };
}

fn fun_name() -> i32 {
    2 + 2
}

fn main() {

}
