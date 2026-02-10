use uuid::Uuid;
use sqlx::FromRow;


#[derive(Debug, Clone, FromRow)]
pub struct AvaliacaoDeLoja {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub nota: f64,
    pub descricao: String,
    pub uuid: Uuid
}

impl AvaliacaoDeLoja {
    pub fn new(
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
        nota: f64,
        descricao: String,
    ) -> Self {
        Self {
            usuario_uuid,
            loja_uuid,
            nota,
            descricao,
            uuid: Uuid::new_v4(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct AvaliacaoDeProduto {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub produto_uuid: Uuid,
    pub nota: f64,
    pub descricao: String,
    pub uuid: Uuid
}

impl AvaliacaoDeProduto {
    pub fn new(
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
        produto_uuid: Uuid,
        nota: f64,
        descricao: String,
    ) -> Self {
        Self {
            usuario_uuid,
            loja_uuid,
            produto_uuid,
            nota,
            descricao,
            uuid: Uuid::new_v4(),
        }
    }
}


#[derive(Debug)]
pub struct AvaliacoesDeProduto {
    payload: Vec<AvaliacaoDeProduto>,
    limit: i32,
    offset: i32,
    length: i32,
}

impl AvaliacoesDeProduto {
    pub fn new(
        payload: Vec<AvaliacaoDeProduto>,
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


#[derive(Debug)]
pub struct AvaliacoesDeLoja {
    payload: Vec<AvaliacaoDeLoja>,
    limit: i32,
    offset: i32,
    length: i32,
}


impl AvaliacoesDeLoja {
    pub fn new(
        payload: Vec<AvaliacaoDeLoja>,
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
