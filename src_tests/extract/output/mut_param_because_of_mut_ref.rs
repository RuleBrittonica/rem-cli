fn foo() {
    let mut n = 1;
    fun_name(&mut n);
    let k = n;
}

fn fun_name(n: &mut i32) {
    let v = n;
    *v += 1;
}

fn main() {

}
