pub mod store;
pub mod server;

pub mod setup;

#[tokio::main]
async fn main() {
    let configuration = setup::Configuration::new();

    setup::start_log(&configuration);

    setup::setup_log(configuration.log_level);

    let secondary = setup::setup_secondary(
        configuration.data_dir
    );
    let primary = setup::setup_primary();

    let engine = setup::setup_engine(secondary, primary);

    let web_server = setup::setup_web_server(engine, configuration.port).await;

    web_server.run().await;
}