use uuid::Uuid;
use sqlx::FromRow;


#[derive(Debug, FromRow)]
pub struct Ingrediente {
    pub nome: String,
    pub descricao: String,
    pub preco_base: f64,
    pub loja_uuid: Uuid,
    pub quantidade: f64,
    pub uuid: Uuid
}


impl Ingrediente {
    pub fn new(
        nome: String,
        descricao: String,
        loja_uuid: Uuid,
        preco_base: f64,
    ) -> Self {

        Self {
            nome,
            preco_base,
            descricao,
            loja_uuid,
            quantidade: 0.0,
            uuid: Uuid::new_v4(),
        }

    }
}