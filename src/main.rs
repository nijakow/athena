
pub mod core;
pub mod formats;
pub mod semantic;
pub mod util;
pub mod volt;
pub mod web;


fn suggest_vault_path() -> Option<std::path::PathBuf> {
    // Suggest a vault path by looking at the environment variable ATHENA_VAULT_PATH.
    // If that does not exist, try to find ~/Vaults/Obsidian
    // If that does not exist, refer to the default path ./example
    // If none of these exist, return None
    // Use functional style

    fn try_vault_path<T: Into<std::path::PathBuf>>(path: T) -> Option<std::path::PathBuf> {
        let path = path.into();
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    let home_dir = std::env::var("HOME").ok()?;

    let vault_path = std::env::var("ATHENA_VAULT_PATH")
        .ok()
        .and_then(|path| try_vault_path(path))
        .or_else(|| try_vault_path(format!("{}/Vaults/Obsidian", home_dir)))
        .or_else(|| try_vault_path("./example"));
    
    vault_path
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let vault_path = suggest_vault_path().unwrap_or_else(|| {
        panic!("No vault path found. Please set the ATHENA_VAULT_PATH environment variable or create a vault at ~/Vaults/Obsidian or ./example");
    });

    let vault = core::config()
        .vault_path(vault_path)
        .open_vault()
        .unwrap();

    web::go(vault).await
}
