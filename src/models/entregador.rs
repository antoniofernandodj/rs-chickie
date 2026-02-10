use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow)]
pub struct Entregador {
    pub nome: String,
    pub celular: String,
    pub veiculo: String,
    pub placa_veiculo: String,
    pub loja_uuid: Uuid,
    pub disponivel: bool,

    pub telefone: Option<String>,
    pub email: Option<String>,
    pub uuid: Uuid,
}


impl Entregador {
    pub fn new(
        nome: String,
        celular: String,
        veiculo: String,
        placa_veiculo: String,
        loja_uuid: Uuid,
    ) -> Self {
        Self {
            nome,
            celular,
            veiculo,
            placa_veiculo,
            loja_uuid,
            disponivel: false,

            telefone: None,
            email: None,
            uuid: Uuid::new_v4(),
        }
    }
}

#[derive(Debug)]
pub struct Entregadores {
    payload: Vec<Entregador>,
    limit: i32,
    offset: i32,
}

impl Entregadores {
    pub fn new(
        payload: Vec<Entregador>,
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
