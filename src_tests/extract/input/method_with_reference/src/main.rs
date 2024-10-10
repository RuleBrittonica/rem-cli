struct S { f: i32 }

impl S {
    fn foo(&self) -> i32 {
        self.f+self.f
    }
}

fn main() {

}
