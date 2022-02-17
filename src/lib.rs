// why SocketAddrV4::new not a const fn?
pub(crate) const LISTEN_ADDRESS: &str = "127.0.0.1:28529";
/// proxy to arangodb web server
pub(crate) const TARGET_ADDRESS: &str = "127.0.0.1:8529";

mod epoll_proxy;
pub mod tokio_proxy;
