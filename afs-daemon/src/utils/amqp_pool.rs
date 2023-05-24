use deadpool_lapin::{Config, Pool, Runtime};

pub fn create_amqp_pool(address: String) -> Pool {
    let cfg = Config {
        url: Some(address),
        ..Default::default()
    };
    cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
}
