use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;


use crate::{model::{Clientes, SaldoModel, SaldoModelwithData, TransacaoRequest, TransacaoRequestExtrato}, AppState};

pub async fn test(State(data): State<Arc<AppState>>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let client = sqlx::query_as!(Clientes,"SELECT * FROM clientes").fetch_all(&data.db).await.unwrap();
    let client_response = json!(client);
    Ok(Json(client_response))
}


pub async fn transacao(Path(id): Path<i32>, 
State(data): State<Arc<AppState>>, 
Json(body): Json<TransacaoRequest>) 
-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let client_select = sqlx::query_as!(Clientes, "SELECT * FROM clientes WHERE id = $1", id)
        .fetch_one(&data.db) 
        .await;

    if client_select.is_err() {
        return Err((StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Cliente não encontrado"}))));
    }

    if body.descricao.len() > 10 || body.descricao.len() < 1 {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({"error": "Descrição inválida"}))));
    }

    let cliente = client_select.unwrap();

    let novo_saldo: i64;
    if body.tipo == "c" {
        novo_saldo = cliente.saldo + body.valor;
    } else if body.tipo == "d" {
        if cliente.saldo - body.valor < -cliente.limite {
            return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({"error": "Saldo insuficiente"}))));
        } else {
            novo_saldo = cliente.saldo - body.valor;
        }
    } else {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({"error": "Tipo de transação inválido"}))));
    }

    let client_update = sqlx::query!("UPDATE clientes SET saldo = $1 WHERE id = $2", novo_saldo, id)
        .execute(&data.db)
        .await;

    if client_update.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Erro ao salvar transação"}))));
    }

    let transacao_insert = sqlx::query!("INSERT INTO transacoes (cliente_id, valor, tipo, descricao) VALUES ($1, $2, $3, $4)", id, body.valor, body.tipo, body.descricao)
        .execute(&data.db)
        .await;

    if transacao_insert.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Erro ao salvar transação"}))));
    }

    let cliente_response  = json!({"limite": cliente.limite, "saldo": novo_saldo});

    Ok(Json(cliente_response))

}

pub async fn extrato(
    Path(id): Path<i32>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let client_saldo = sqlx::query_as!(SaldoModel,"SELECT limite, saldo FROM clientes WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if client_saldo.is_err() {
        return Err((StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Cliente não encontrado"}))));
    }

    let cliente_saldo = client_saldo.unwrap();

    let saldo_com_data = SaldoModelwithData {
        limite: cliente_saldo.limite,
        total: cliente_saldo.saldo,
        data_extrato: chrono::Utc::now(),
    };

    let ultimas_transacoes = sqlx::query_as!(TransacaoRequestExtrato, "SELECT valor, tipo, descricao, realizada_em FROM transacoes WHERE cliente_id = $1 ORDER BY realizada_em DESC LIMIT 10", id)
        .fetch_all(&data.db)
        .await
        .unwrap();

    let client_response = json!({"saldo": saldo_com_data, "ultimas_transacoes": ultimas_transacoes});
    Ok(Json(client_response))
}
