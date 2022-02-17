use crate::{LISTEN_ADDRESS, TARGET_ADDRESS};

// TODO add bench mark to proxy server
// TODO benchmark TCP
pub async fn main_loop() {
    while let Ok((client_stream, _client_addr)) = tokio::net::TcpListener::bind(LISTEN_ADDRESS)
        .await
        .unwrap()
        .accept()
        .await
    {
        // TODO: abort coroutine when client cancel?
        tokio::spawn(proxy_connection(client_stream));
    }
}

async fn proxy_connection(mut client_stream: tokio::net::TcpStream) {
    // or tokio::io::split(client_stream)
    let (mut client_read_fd, mut client_write_fd) = client_stream.split();
    let target_stream = tokio::net::TcpStream::connect(TARGET_ADDRESS)
        .await
        .unwrap();
    let (mut target_read_fd, mut target_write_fd) = tokio::io::split(target_stream);
    // why tokio proxy example would call stream shutdown() https://github.com/tokio-rs/tokio/blob/master/examples/proxy.rs
    let client_to_target_fut = tokio::io::copy(&mut client_read_fd, &mut target_write_fd);
    let target_to_client_fut = tokio::io::copy(&mut target_read_fd, &mut client_write_fd);
    tokio::try_join!(client_to_target_fut, target_to_client_fut).unwrap();
}
