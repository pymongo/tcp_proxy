use tcp_proxy::tokio_proxy::main_loop;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    main_loop().await;
}
