use uuid::Uuid;
use sqlx::FromRow;


#[derive(Debug, Clone, FromRow)]
pub struct Adicional {
    pub id: Uuid,
    pub nome: String,
    pub loja_uuid: Uuid,
    pub disponivel: bool,
    pub descricao: String,
    pub preco: f64,
}

impl Adicional {
    pub fn new(
        nome: String,
        loja_uuid: Uuid,
        descricao: String,
        preco: f64,
    ) -> Self {

        Self {
            nome,
            loja_uuid,
            disponivel: false,
            descricao,
            preco,
            id: Uuid::new_v4()
        }

    }
}
