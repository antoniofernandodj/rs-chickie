use uuid::Uuid;
use sqlx::FromRow;


#[derive(Debug, Clone, FromRow)]
pub struct CategoriaProdutos {
    pub nome: String,
    pub descricao: String,
    pub loja_uuid: Uuid,

    pub uuid: Uuid
}

impl CategoriaProdutos {
    pub fn new(
        nome: String,
        descricao: String,
        loja_uuid: Uuid,
    ) -> Self {
        Self {
            nome,
            descricao,
            loja_uuid,
            uuid: Uuid::new_v4(),
        }
    }
}


#[derive(Debug)]
pub struct CategoriasProdutos {
    payload: Vec<CategoriaProdutos>,
    limit: i32,
    offset: i32,
    length: i32,
}


impl CategoriasProdutos {
    pub fn new(
        payload: Vec<CategoriaProdutos>,
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