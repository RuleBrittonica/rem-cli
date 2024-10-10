fn parent(factor: i32) {
    let v = &[1, 2, 3];

    fun_name(v, factor);
}

fn fun_name(v: &[i32; 3], factor: i32) {
    v.iter().map(|it| it * factor);
}

fn main() {

}
