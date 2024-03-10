mod handler;
mod model;
mod route;

use std::{collections::HashMap, sync::Arc};

use axum::http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::CorsLayer;

use crate::route::create_router;


pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    let pool = match PgPoolOptions::new().max_connections(10).connect(&database_url).await {
        Ok(pool) => {
            println!("Connected to database");
            pool
        }
        Err(e) => {
            println!("Error connecting to database: {}", e);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:9999".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true) 
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

        let mut limites = HashMap::new();
        let clients = sqlx::query!("SELECT * FROM clientes")
            .fetch_all(&pool)
            .await
            .unwrap();

        for cliente in clients {
            limites.insert(cliente.id as i32, cliente.limite as i64);
        }

    let app = create_router(Arc::new(AppState { db: pool })).layer(cors);
    println!("🚀 Starting server at http://localhost:{}", port);
    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind(port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}