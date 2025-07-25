mod addr_server;
mod main_logic;

#[tokio::main]
async fn main() {
    main_logic::run();
}
