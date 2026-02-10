use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Usuario {
    pub nome: String,
    pub username: String,
    pub email: String,
    pub celular: String,
    pub criado_em: DateTime<Utc>,
    pub atualizado_em: Option<DateTime<Utc>>,
    
    pub modo_de_cadastro: String,
    
    pub telefone: Option<String>,
    pub senha_hash: Option<String>,
    pub uuid: Uuid,
    pub ativo: bool,
    pub passou_pelo_primeiro_acesso: bool
}

impl Usuario {
    pub fn new(
        nome: String,
        username: String,
        email: String,
        celular: String,
        modo_de_cadastro: String,
    ) -> Self {

        Self {
            nome,
            username,
            email,
            celular,
            criado_em: Utc::now(),
            atualizado_em: None,
            modo_de_cadastro,

            telefone: None,
            senha_hash: None,
            uuid: Uuid::new_v4(),
            ativo: true,
            passou_pelo_primeiro_acesso: false,
        }
    }
}


#[derive(Debug)]
pub struct Usuarios {
    payload: Vec<Usuario>,
    limit: i32,
    offset: i32,
    length: i32,
}


impl Usuarios {
    pub fn new(
        payload: Vec<Usuario>,
        limit: i32,
        offset: i32,
    ) -> Self {
        let length = payload.len() as i32;

        Self {
            payload,
            limit,
            offset,
            length,
        }
    }
}