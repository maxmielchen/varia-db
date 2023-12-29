pub mod store;
pub mod server;

pub mod setup;

#[tokio::main]
async fn main() {
    setup::log(log::Level::Info);

    let secondary = setup::secondary("data/secondary".to_string());
    let primary = setup::primary();

    let engine = setup::engine(secondary, primary);

    let server = setup::server(engine, 8080).await;

    server.run().await;
}