async fn foo() -> Result<(), ()> {
    async {}.await;
    Err(())?
}

fn main() {

}
