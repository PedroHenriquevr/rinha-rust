use std::sync::Arc;

use axum::{
    routing::{get, post}, Router
};

use crate::{handler::{extrato, test, transacao}, AppState};
    


pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .route("/", get(test))
    .route("/clientes/:id/transacoes", post(transacao))
    .route("/clientes/:id/extrato", get(extrato))
    .with_state(app_state)
}