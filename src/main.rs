mod controllers;
mod errors;
mod models;
use crate::controllers::task::{create_task, delete_task, get_all_tasks, get_task, update_task};

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};

use dotenv::dotenv;
use sqlx::postgres::{PgPoolOptions};
use std::{env, net::SocketAddr};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER not found");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not found");
    let server = env::var("POSTGRES_SERVER").expect("POSTGRES_SERVER not found");
    let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT not found");
    let db = env::var("POSTGRES_DB").expect("POSTGRES_DB not found");

    let database_url = format!("postgres://{}:{}@{}:{}/{}", user, password, server, port, db);
    println!("{}", database_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=trace")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/tasks", get(get_all_tasks))
        .route("/task", post(create_task))
        .route(
            "/task/:id",
            get(get_task).put(update_task).delete(delete_task),
        )
        .layer(cors)
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");

    Ok(())
}

async fn root() -> &'static str {
    "Hello, world!"
}
