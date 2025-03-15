
pub mod config;
pub mod io;
pub mod vault;
pub mod zettel;

pub fn config() -> config::ConfigBuilder {
    config::ConfigBuilder::new()
}
