macro_rules! m {
    ($val:expr) => { $val };
}
fn main() {
    let bar = "bar";
    fun_name(bar);
}

fn fun_name(bar: &str) {
    m!(bar);
}

fn main() {

}
