pub mod store;
pub mod server;

pub mod setup;

#[tokio::main]
async fn main() {
    
    setup::log();

    let server = setup::server().await;

    server.run().await;
}