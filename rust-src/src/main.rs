mod addr_server;
mod backend;
mod comm;

#[tokio::main]
async fn main() {
    backend::backend().await;
}
