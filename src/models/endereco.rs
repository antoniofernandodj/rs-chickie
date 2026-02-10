use uuid::Uuid;
use sqlx::FromRow;


#[derive(Debug)]
struct EnderecoBase {
    pub uf: String,
    pub cidade: String,
    pub logradouro: String,
    pub numero: String,
    pub bairro: String,
    pub cep: Option<String>,
    pub complemento: Option<String>,
    pub uuid: Uuid,
}

impl EnderecoBase {
    fn new(
        uf: String,
        cidade: String,
        logradouro: String,
        numero: String,
        bairro: String,
        cep: Option<String>,
        complemento: Option<String>,
    ) -> Self {

        Self {
            uf,
            cidade,
            logradouro,
            numero,
            bairro,
            cep,
            complemento,
            uuid: Uuid::new_v4(),
        }

    }
}


#[derive(Debug)]
pub struct EnderecoLoja {
    pub endereco: EnderecoBase,
    pub loja_uuid: Uuid,
}

impl EnderecoLoja {

    pub fn get_uuid(&self) -> Uuid {
        return self.endereco.uuid.clone();
    }

    pub fn new(
        uf: String,
        cidade: String,
        logradouro: String,
        numero: String,
        bairro: String,
        cep: Option<String>,
        complemento: Option<String>,
        loja_uuid: Uuid,
    ) -> Self {

        let endereco = EnderecoBase::new(
            uf,
            cidade,
            logradouro,
            numero,
            bairro,
            cep,
            complemento,
        );

        Self {
            endereco,
            loja_uuid,
        }
    }
}


#[derive(Debug)]
pub struct EnderecoUsuario {
    pub endereco: EnderecoBase,
    pub usuario_uuid: Uuid,
}

impl EnderecoUsuario {
    pub fn new(
        uf: String,
        cidade: String,
        logradouro: String,
        numero: String,
        bairro: String,
        cep: Option<String>,
        complemento: Option<String>,
        usuario_uuid: Uuid,
    ) -> Self {

        let endereco = EnderecoBase::new(
            uf,
            cidade,
            logradouro,
            numero,
            bairro,
            cep,
            complemento,
        );

        Self {
            endereco,
            usuario_uuid,
        }

    }
}



#[derive(Debug)]
pub struct EnderecoEntrega {
    pub endereco: EnderecoBase,
    pub pedido_uuid: Option<String>
}

impl EnderecoEntrega {
    pub fn new(
        uf: String,
        cidade: String,
        logradouro: String,
        numero: String,
        bairro: String,
        cep: Option<String>,
        complemento: Option<String>,
        pedido_uuid: Option<String>,
    ) -> Self {

        let endereco = EnderecoBase::new(
            uf,
            cidade,
            logradouro,
            numero,
            bairro,
            cep,
            complemento,
        );

        Self {
            endereco,
            pedido_uuid,
        }

    }
}



#[derive(Debug)]
pub struct EnderecosLoja {
    payload: Vec<EnderecoLoja>,
    limit: i32,
    offset: i32,
}

impl EnderecosLoja {
    pub fn new(
        payload: Vec<EnderecoLoja>,
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


#[derive(Debug)]
pub struct EnderecosEntrega {
    payload: Vec<EnderecoEntrega>,
    limit: i32,
    offset: i32
}

impl EnderecosEntrega {
    pub fn new(
        payload: Vec<EnderecoEntrega>,
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



#[derive(Debug)]
pub struct EnderecosUsuario {
    payload: Vec<EnderecoUsuario>,
    limit: i32,
    offset: i32,
}

impl EnderecosUsuario {
    pub fn new(
        payload: Vec<EnderecoUsuario>,
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
