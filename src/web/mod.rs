use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use tokio::time::{interval, Duration};

use crate::core::vault;

pub mod routes;
pub mod pages;


async fn run_periodic_task(vault: Arc<vault::Vault>) {
    let mut interval = interval(Duration::from_secs(60)); // Run every 60 seconds

    loop {
        interval.tick().await;

        println!("Running periodic task on the vault...");
        vault.tick();
    }
}

pub async fn go(vault: vault::Vault) -> std::io::Result<()> {
    let vault_data = Arc::new(vault);

    {
        let vault_clone = Arc::clone(&vault_data);
        tokio::spawn(async move {
            run_periodic_task(vault_clone).await;
        });
    }

    {
        let vault_data = web::Data::new(vault_data);

        HttpServer::new(move || {
            App::new()
                .app_data(vault_data.clone())
                .route("/", web::get().to(routes::list_entities))
                .route("/web/{file}", web::get().to(routes::web_file))
                .route("/entity/{id}", web::get().to(routes::process_entity))
                .route("/entity/{id}", web::post().to(routes::post_entity))
                .route("/raw/{id}", web::get().to(routes::download_entity))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
}
