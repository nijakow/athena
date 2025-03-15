use core::vault;

use actix_web::{App, HttpServer};


pub mod core;
pub mod web;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let vault = core::config()
        .vault_path(std::path::PathBuf::from("vault"))
        .open_vault()
        .unwrap();

    web::go(vault).await
}
