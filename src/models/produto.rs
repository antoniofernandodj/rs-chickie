use uuid::Uuid;
use sqlx::FromRow;


#[derive(Debug, Clone, FromRow)]
pub struct Produto {
    pub nome: String,
    pub descricao: String,
    pub preco: f64,
    pub categoria_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub disponivel: bool,
    pub uuid: Uuid
}


impl Produto {
    pub fn new(
        nome: String,
        descricao: String,
        preco: f64,
        categoria_uuid: Uuid,
        loja_uuid: Uuid,
    ) -> Self {
        Self {
            nome,
            descricao,
            preco,
            categoria_uuid,
            loja_uuid,
            disponivel: false,
            uuid: Uuid::new_v4(),
        }
    }
}


#[derive(Debug)]
pub struct Produtos {
    payload: Vec<Produto>,
    limit: i32,
    offset: i32,
}

impl Produtos {
    pub fn new(
        payload: Vec<Produto>,
        limit: i32,
        offset: i32,
    ) -> Self {
        Self {
            payload,
            limit,
            offset,
        }
    }
}