use uuid::Uuid;
use sqlx::FromRow;


#[derive(Debug, Clone, FromRow)]
pub struct Loja {
    pub nome: String,
    pub username: String,
    pub email: String,
    pub celular: String,
    pub senha_hash: String,
    pub ativa: bool,
    pub slug: String,
    
    pub telefone: Option<String>,
    pub password: Option<String>,
    pub uuid: Uuid,
    pub ativo: bool,
    pub passou_pelo_primeiro_acesso: bool,
    pub horarios_de_funcionamento: String
}

impl Loja {
    pub fn new(
        nome: String,
        username: String,
        email: String,
        celular: String,
        senha_hash: String,
        slug: String,
        horarios_de_funcionamento: Vec<String>,
    ) -> Self {

        Self {
            nome,
            username,
            email,
            celular,
            senha_hash,
            ativa: true,
            slug,

            telefone: None,
            password: None,
            uuid: Uuid::new_v4(),
            ativo: true,
            passou_pelo_primeiro_acesso: false,
            horarios_de_funcionamento,
        }
    }
}


#[derive(Debug)]
pub struct Lojas {
    limit: i32,
    offset: i32,
    payload: Vec<Loja>
}

impl Lojas {
    pub fn new(
        payload: Vec<Loja>,
        limit: i32,
        offset: i32,
    ) -> Self {
        Self {
            limit,
            offset,
            payload,
        }
    }
}