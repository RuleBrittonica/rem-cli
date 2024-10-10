async fn foo() -> Result<(), ()> {
    fun_name().await
}

async fn fun_name() -> Result<(), ()> {
    async {}.await;
    Err(())?
}

fn main() {

}
