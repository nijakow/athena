
pub mod config;
pub mod entity;
pub mod vault;

pub fn config() -> config::ConfigBuilder {
    config::ConfigBuilder::new()
}
