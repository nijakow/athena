
pub mod config;
pub mod io;
pub mod vault;
pub mod entity;

pub fn config() -> config::ConfigBuilder {
    config::ConfigBuilder::new()
}
