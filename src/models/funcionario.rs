use uuid::Uuid;
use sqlx::FromRow;


#[derive(Debug, Clone, FromRow)]
pub struct Funcionario {
    pub loja_uuid: Uuid,
    pub cargo: String,
    pub nome: String,
    pub username: String,
    pub email: String,
    pub celular: String,

    pub senha_hash: String,
    pub telefone: Option<String>,
    pub password: Option<String>,
    pub uuid: Uuid
}

impl Funcionario {
    pub fn new(
        loja_uuid: Uuid,
        cargo: String,
        nome: String,
        username: String,
        email: String,
        celular: String,
        senha_hash: String,
    ) -> Self {

        Self {
            loja_uuid,
            cargo,
            nome,
            username,
            email,
            celular,

            senha_hash,
            telefone: None,
            password: None,
            uuid: Uuid::new_v4(),
        }

    }
}


#[derive(Debug)]
pub struct Funcionarios {
    payload: Vec<Funcionario>,
    limit: i32,
    offset: i32,
}

impl Funcionarios {
    pub fn new(
        payload: Vec<Funcionario>,
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