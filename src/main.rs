
pub mod core;
pub mod formats;
pub mod util;
pub mod web;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Extract vault path from environment
    let vault_path = std::env::var("ATHENA_VAULT_PATH").unwrap_or_else(|_| "./example".to_string());

    let vault = core::config()
        .vault_path(std::path::PathBuf::from(vault_path))
        .open_vault()
        .unwrap();

    web::go(vault).await
}
