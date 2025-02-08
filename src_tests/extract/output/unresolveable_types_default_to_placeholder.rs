fn foo() {
    let a = __unresolved;
    let _ = fun_name(a);
}

fn fun_name(a: _) -> _ {
    a
}

fn main() {

}
