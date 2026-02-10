use uuid::Uuid;
use sqlx::FromRow;


#[derive(Debug, FromRow)]
pub struct Cliente {
    pub usuario_uuid: Uuid,
    pub loja_uuid: Uuid,
    pub uuid: Uuid,
}

impl Cliente {
    pub fn new(
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
    ) -> Self {
        Self {
            usuario_uuid,
            loja_uuid,
            uuid: Uuid::new_v4()
        }
    }
}
