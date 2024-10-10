fn main() {
    fun_name().await;
}

async fn fun_name() {
    function_call("a", some_function().await);
}

async fn some_function() {

}

fn main() {

}
