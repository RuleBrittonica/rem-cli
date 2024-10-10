fn foo() {
    let mut n = 1;
    fun_name(&mut n);
    let m = n + 1;
}

fn fun_name(n: &mut i32) {
    *n += 1;
}

fn main() {

}
