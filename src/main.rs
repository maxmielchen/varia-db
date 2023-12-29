pub mod store;
pub mod server;

pub mod setup;

#[tokio::main]
async fn main() {
    setup::setup_log(log::Level::Info);

    let secondary = setup::setup_secondary("data/secondary".to_string());
    let primary = setup::setup_primary();

    let engine = setup::setup_engine(secondary, primary);

    let web_server = setup::setup_web_server(engine, 8080).await;

    web_server.run().await;
}