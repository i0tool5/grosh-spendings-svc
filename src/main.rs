use std::sync::Arc;

use application::commands;
use application::queries;
use axum::{
    routing,
    Router,
};

use sea_orm::{
    ConnectOptions,
    Database,
    DatabaseConnection,
};
use tokio::signal;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use migration::{Migrator, MigratorTrait};

use infrastructure::config;
use infrastructure::repositories;
use infrastructure::rest_api;

mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let svc_cfg = config::get_config()?;
    let connection_settings = ConnectOptions::new(svc_cfg.db_url)
        .set_schema_search_path(svc_cfg.db_schema)
        .to_owned(); 
    let db: DatabaseConnection = Database::connect(connection_settings)
        .await
        .expect("failed to connect to database");

    db.ping().await.expect("failed to ping the database");

    Migrator::up(&db, None).await.expect("failed to run up migration");

    let db_ref = Arc::new(db);
    let spending_repo = Arc::new(
        repositories::spendings::Repository::new(Arc::clone(&db_ref)),
    );
    let spending_create_command_handler = commands::CreateSpendingCommandHandler::new(
        Arc::clone(&spending_repo),
    );
    let spending_remove_command_handler = commands::SpendingRemoveCommandHandler::new(
        Arc::clone(&spending_repo),
    );
    let spending_edit_command_handler = commands::SpendingEditCommandHandler::new(
        Arc::clone(&spending_repo),
    );
    let spendings_list_query_handler = queries::SpendingsListQueryHandler::new(
        db_ref,
    );

    let state = Arc::new(application::State::new(
        spending_create_command_handler,
        spending_remove_command_handler,
        spending_edit_command_handler,
        spendings_list_query_handler,
    ));
    
    let app_v1 = Router::new()
        .route("/spendings", routing::get(rest_api::spendings_list_handler))
        .route("/spendings", routing::post(rest_api::spending_create_handler))
        .route("/spendings/{id}", routing::delete(rest_api::spending_remove_handler))
        .route("/spendings/{id}", routing::put(rest_api::spending_edit_handler))
        .with_state(state);


    let v1 = Router::new()
        .nest("/v1", app_v1);

    let listener = tokio::net::TcpListener::bind(svc_cfg.bind_address).await.unwrap();
    tracing::info!("listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, v1)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {tracing::info!("shutting down the server");},
        _ = terminate => {},
    }
}
