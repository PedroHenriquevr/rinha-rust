use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(non_snake_case)] 
pub struct Clientes {
    pub id: i32,
    pub nome: String,
    pub limite: i64,
    pub saldo: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(non_snake_case)]
pub struct Transacao {
    pub id: i32,
    pub valor: i32,
    pub tipo: char,
    pub descricao: String,
    #[sqlx(rename = "realizadaEm")]
    pub realizada_em: chrono::DateTime<chrono::Utc>
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TransacaoRequestExtrato {
    pub valor: i64,
    pub tipo: String,
    pub descricao: String,
    pub realizada_em: chrono::NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(non_snake_case)]
pub struct TransacaoRequest {
    pub valor: i64,
    pub tipo: String,
    pub descricao: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(non_snake_case)]
pub struct TransacaoResponse {
    pub limite: i64,
    pub saldo: i64,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
#[allow(non_snake_case)]
pub struct SaldoModel {
    pub saldo: i64,
    pub limite: i64,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct SaldoModelwithData {
    pub total: i64,
    pub limite: i64,
    pub data_extrato: chrono::DateTime<chrono::Utc>
}