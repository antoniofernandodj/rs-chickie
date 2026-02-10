use uuid::Uuid;
use crate::models::{Adicional, Produto};
use sqlx::FromRow;


#[derive(Debug, Clone, FromRow)]
struct AdicionalDeItemDePedido {
    nome: String,
    descricao: String,
    loja_uuid: Uuid,
    item_uuid: Uuid,
    preco: f64,
    uuid: Uuid
}

impl AdicionalDeItemDePedido {
    pub fn new(
        nome: String,
        descricao: String,
        loja_uuid: Uuid,
        item_uuid: Uuid,
        preco: f64,
    ) -> Self {
        Self {
            nome,
            descricao,
            loja_uuid,
            item_uuid,
            preco,
            uuid: Uuid::new_v4(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct ItemPedido {
    quantidade: i32,
    observacoes: String,
    pedido_uuid: String,
    loja_uuid: Uuid,
    produto_nome: String,
    produto_descricao: String,
    valor: f64,
    uuid: Uuid,
    adicionais: Vec<AdicionalDeItemDePedido>,
}

impl ItemPedido {
    pub fn new(
        quantidade: i32,
        observacoes: String,
        pedido_uuid: String,
        loja_uuid: Uuid,
        produto_nome: String,
        produto_descricao: String,
        valor: f64,
    ) -> Self {

        Self {
            quantidade,
            observacoes,
            pedido_uuid,
            loja_uuid,
            produto_nome,
            produto_descricao,
            valor,
            uuid: Uuid::new_v4(),
            adicionais: Vec::new(),
        }

    }

    pub fn adicionar_adicional(
        &mut self,
        adicional: &Adicional
    ) -> Result<Uuid, String> {

        if self.loja_uuid != adicional.loja_uuid {
            return Err("Não foi possível adicionar item".to_string())
        }

        let adicional = AdicionalDeItemDePedido::new(
            adicional.nome.clone(),
            adicional.descricao.clone(),
            adicional.loja_uuid,
            self.uuid,
            adicional.preco,
        );

        let uuid = adicional.uuid.clone();

        self.adicionais.push(adicional);

        Ok(uuid)
    }

}



#[derive(Debug, PartialEq, Clone)]
pub enum EstadoDePedido {
    Criado,
    AguardandoConfirmacaoDeLoja,
    ConfirmadoPelaLoja,
    EmPreparo,
    ProntoParaRetirada,
    SaiuParaEntrega,
    Entregue
}


#[derive(Debug, Clone)]
pub struct Pedido {
    pub celular: String,
    pub data_hora: String,
    pub loja_uuid: Uuid,
    pub frete: f64,
    pub comentarios: String,

    pub status: EstadoDePedido,
    pub usuario_uuid: Uuid,
    pub uuid: Uuid,
    pub itens: Vec<ItemPedido>,
}


impl Pedido {
    pub fn new(
        celular: String,
        data_hora: String,
        loja_uuid: Uuid,
        frete: f64,
        comentarios: String,
        usuario_uuid: Uuid,
    ) -> Self {

        Self {
            celular,
            data_hora,
            loja_uuid,
            frete,
            comentarios,

            status: EstadoDePedido::Criado,
            usuario_uuid,
            uuid: Uuid::new_v4(),
            itens: Vec::<ItemPedido>::new()
        }

    }

    pub fn adicionar_item_pedido(
        &mut self,
        produto: &Produto,
        quantidade: i32,
        observacoes: String,
        loja_uuid: Uuid,
    ) -> &mut ItemPedido {

        let item_de_pedido = ItemPedido::new(
            quantidade,
            observacoes,
            self.uuid.to_string(),
            loja_uuid,
            produto.nome.clone(),
            produto.descricao.clone(),
            produto.preco.clone(),
        );

        let uuid = item_de_pedido.uuid; 


        self.itens.push(item_de_pedido);

        let item =
            self
                .itens
                .iter_mut()
                .find(|i| i.uuid == uuid)
                .expect("Item não encontrado");

        return item
    }

    pub fn localizar_pedido(&mut self, item_uuid: Uuid) -> &mut ItemPedido {

        let item =
            self
                .itens
                .iter_mut()
                .find(|i| i.uuid == item_uuid)
                .expect("Item não encontrado");

        return item

    }

}


#[derive(Debug)]
pub struct Pedidos {
    payload: Vec<Pedido>,
    limit: i32,
    offset: i32
}

impl Pedidos {
    pub fn new(
        payload: Vec<Pedido>,
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