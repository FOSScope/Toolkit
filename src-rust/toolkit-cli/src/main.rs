use fosscopetoolkit_core::config::config::config_process;

#[tokio::main]
async fn main() {
    let config = config_process();
    if config.is_err() {
        eprintln!("Error creating/loading config: {:?}", config.err().unwrap());
        std::process::exit(1);
    }
    let config = config.unwrap();
}
