struct S { f: i32 }

impl S {
    fn foo(&mut self) {
        self.fun_name();
    }

    fn fun_name(&mut self) {
        self.f += 1;
    }
}

fn main() {

}
