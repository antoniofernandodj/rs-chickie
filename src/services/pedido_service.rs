use crate::models::{Pedido, Produto, ItemPedido};
use crate::repositories::{PedidoRepository, ProdutoRepository, Repository};
use crate::services::{CupomService, LojaService, ProdutoService};
use uuid::Uuid;
use chrono::Utc;

pub struct PedidoService {
    pedido_repo: PedidoRepository,
    produto_repo: ProdutoRepository,
}

impl PedidoService {
    pub fn new(
        pedido_repo: PedidoRepository,
        produto_repo: ProdutoRepository,
    ) -> Self {
        Self {
            pedido_repo,
            produto_repo,
        }
    }

    /// Cria um novo pedido com validações completas
    pub fn criar_pedido(
        &mut self,
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
        telefone_entrega: String,
        data_hora_entrega: String,
        taxa_entrega: f64,
        observacoes: String,
        itens: Vec<ItemPedidoDto>,
        cupom_codigo: Option<String>,
        loja_service: &LojaService,
        cupom_service: &mut CupomService,
    ) -> Result<Pedido, String> {
        // 1. Validar loja está aberta
        if !loja_service.esta_aberta(loja_uuid)? {
            return Err("Loja fechada no momento".to_string());
        }

        // 2. Validar que há itens
        if itens.is_empty() {
            return Err("Pedido deve ter ao menos um item".to_string());
        }

        // 3. Validar telefone
        if !self.validar_telefone(&telefone_entrega) {
            return Err("Telefone inválido".to_string());
        }

        // 4. Criar pedido
        let mut pedido = Pedido::new(
            telefone_entrega,
            data_hora_entrega,
            loja_uuid,
            taxa_entrega,
            observacoes,
            usuario_uuid,
        );

        // 5. Adicionar itens e calcular subtotal
        let mut subtotal = 0.0;
        for item_dto in itens {
            // Buscar produto
            let produto = self.produto_repo
                .buscar_por_uuid(item_dto.produto_uuid)
                .ok_or("Produto não encontrado")?;

            // Verificar se produto está disponível
            if !produto.disponivel {
                return Err(format!("Produto '{}' indisponível", produto.nome));
            }

            // Verificar se produto pertence à loja
            if produto.loja_uuid != loja_uuid {
                return Err(format!("Produto '{}' não pertence a esta loja", produto.nome));
            }

            // Adicionar item ao pedido
            let item = pedido.adicionar_item_pedido(
                &produto,
                item_dto.quantidade,
                item_dto.observacoes,
                loja_uuid,
            );

            // Adicionar adicionais ao item
            for adicional_uuid in item_dto.adicionais_uuids {
                item.adicionar_adicional_por_uuid(adicional_uuid)
                    .map_err(|e| format!("Erro ao adicionar adicional: {}", e))?;
            }

            // Somar ao subtotal
            subtotal += item.calcular_total();
        }

        // 6. Aplicar cupom se houver
        let desconto = if let Some(codigo) = cupom_codigo {
            cupom_service.aplicar_cupom(
                &codigo,
                loja_uuid,
                usuario_uuid,
                subtotal,
                taxa_entrega,
                false, // Aqui você deveria verificar se é primeira compra
            )?
        } else {
            0.0
        };

        // 7. Calcular total final
        pedido.valor_total = subtotal + taxa_entrega - desconto;

        // Validar valor mínimo do pedido (se houver)
        if pedido.valor_total < 0.0 {
            return Err("Valor total inválido".to_string());
        }

        // 8. Salvar pedido
        self.pedido_repo.criar(pedido.clone())
            .map_err(|e| format!("Erro ao criar pedido: {}", e))?;

        Ok(pedido)
    }

    /// Busca pedido por UUID
    pub fn buscar_pedido(&self, uuid: Uuid) -> Option<Pedido> {
        self.pedido_repo.buscar_por_uuid(uuid)
    }

    /// Lista pedidos de um usuário
    pub fn listar_pedidos_usuario(&self, usuario_uuid: Uuid) -> Vec<Pedido> {
        self.pedido_repo.listar_todos()
            .into_iter()
            .filter(|p| p.usuario_uuid == usuario_uuid)
            .collect()
    }

    /// Lista pedidos de uma loja
    pub fn listar_pedidos_loja(&self, loja_uuid: Uuid) -> Vec<Pedido> {
        self.pedido_repo.listar_todos()
            .into_iter()
            .filter(|p| p.loja_uuid == loja_uuid)
            .collect()
    }

    /// Lista pedidos por status
    pub fn listar_pedidos_por_status(&self, loja_uuid: Uuid, status: &str) -> Vec<Pedido> {
        self.listar_pedidos_loja(loja_uuid)
            .into_iter()
            .filter(|p| p.status == status)
            .collect()
    }

    /// Atualiza status do pedido
    pub fn atualizar_status(
        &mut self,
        pedido_uuid: Uuid,
        novo_status: String,
    ) -> Result<Pedido, String> {
        let mut pedido = self.pedido_repo.buscar_por_uuid(pedido_uuid)
            .ok_or("Pedido não encontrado")?;

        // Validar transição de status
        self.validar_transicao_status(&pedido.status, &novo_status)?;

        pedido.status = novo_status;
        pedido.atualizado_em = Utc::now();

        self.pedido_repo.atualizar(pedido.clone())
            .map_err(|e| format!("Erro ao atualizar pedido: {}", e))?;

        Ok(pedido)
    }

    /// Valida se a transição de status é permitida
    fn validar_transicao_status(&self, status_atual: &str, novo_status: &str) -> Result<(), String> {
        let transicoes_validas = match status_atual {
            "pendente" => vec!["confirmado", "cancelado"],
            "confirmado" => vec!["preparando", "cancelado"],
            "preparando" => vec!["pronto", "cancelado"],
            "pronto" => vec!["saiu_para_entrega"],
            "saiu_para_entrega" => vec!["entregue"],
            "entregue" => vec![],
            "cancelado" => vec![],
            _ => vec![],
        };

        if transicoes_validas.contains(&novo_status) {
            Ok(())
        } else {
            Err(format!(
                "Transição de '{}' para '{}' não permitida",
                status_atual,
                novo_status
            ))
        }
    }

    /// Cancela um pedido
    pub fn cancelar_pedido(
        &mut self,
        pedido_uuid: Uuid,
        motivo: String,
    ) -> Result<Pedido, String> {
        let mut pedido = self.pedido_repo.buscar_por_uuid(pedido_uuid)
            .ok_or("Pedido não encontrado")?;

        // Validar se pode ser cancelado
        if pedido.status == "entregue" {
            return Err("Pedido já entregue não pode ser cancelado".to_string());
        }

        if pedido.status == "cancelado" {
            return Err("Pedido já está cancelado".to_string());
        }

        pedido.status = "cancelado".to_string();
        pedido.atualizado_em = Utc::now();

        self.pedido_repo.atualizar(pedido.clone())
            .map_err(|e| format!("Erro ao cancelar pedido: {}", e))?;

        Ok(pedido)
    }

    /// Calcula tempo estimado de entrega
    pub fn calcular_tempo_estimado(&self, loja_uuid: Uuid) -> Result<u32, String> {
        // Buscar pedidos ativos da loja
        let pedidos_ativos = self.listar_pedidos_loja(loja_uuid)
            .into_iter()
            .filter(|p| !matches!(p.status.as_str(), "entregue" | "cancelado"))
            .count();

        // Calcular tempo base + tempo por pedido na fila
        let tempo_base = 30; // minutos
        let tempo_por_pedido = 10; // minutos

        Ok(tempo_base + (pedidos_ativos as u32 * tempo_por_pedido))
    }

    /// Calcula estatísticas de pedidos
    pub fn calcular_estatisticas(&self, loja_uuid: Uuid) -> EstatisticasPedidos {
        let pedidos = self.listar_pedidos_loja(loja_uuid);

        let total = pedidos.len();
        let entregues = pedidos.iter().filter(|p| p.status == "entregue").count();
        let cancelados = pedidos.iter().filter(|p| p.status == "cancelado").count();
        let em_andamento = pedidos.iter()
            .filter(|p| !matches!(p.status.as_str(), "entregue" | "cancelado"))
            .count();

        let faturamento_total: f64 = pedidos.iter()
            .filter(|p| p.status == "entregue")
            .map(|p| p.valor_total)
            .sum();

        let ticket_medio = if entregues > 0 {
            faturamento_total / entregues as f64
        } else {
            0.0
        };

        EstatisticasPedidos {
            total_pedidos: total,
            pedidos_entregues: entregues,
            pedidos_cancelados: cancelados,
            pedidos_em_andamento: em_andamento,
            faturamento_total,
            ticket_medio,
        }
    }

    /// Adiciona observação ao pedido
    pub fn adicionar_observacao(
        &mut self,
        pedido_uuid: Uuid,
        observacao: String,
    ) -> Result<Pedido, String> {
        let mut pedido = self.pedido_repo.buscar_por_uuid(pedido_uuid)
            .ok_or("Pedido não encontrado")?;

        if !pedido.observacoes.is_empty() {
            pedido.observacoes.push_str("\n");
        }
        pedido.observacoes.push_str(&observacao);
        pedido.atualizado_em = Utc::now();

        self.pedido_repo.atualizar(pedido.clone())
            .map_err(|e| format!("Erro ao adicionar observação: {}", e))?;

        Ok(pedido)
    }

    /// Valida formato de telefone
    fn validar_telefone(&self, telefone: &str) -> bool {
        let numeros: String = telefone.chars().filter(|c| c.is_numeric()).collect();
        numeros.len() >= 10 && numeros.len() <= 11
    }

    /// Deleta um pedido (apenas se cancelado ou muito antigo)
    pub fn deletar_pedido(&mut self, pedido_uuid: Uuid) -> Result<(), String> {
        let pedido = self.pedido_repo.buscar_por_uuid(pedido_uuid)
            .ok_or("Pedido não encontrado")?;

        if pedido.status != "cancelado" {
            return Err("Apenas pedidos cancelados podem ser deletados".to_string());
        }

        self.pedido_repo.deletar(pedido_uuid)
            .map_err(|e| format!("Erro ao deletar pedido: {}", e))
    }

    /// Lista pedidos recentes (últimos 30 dias)
    pub fn listar_pedidos_recentes(&self, loja_uuid: Uuid, dias: i64) -> Vec<Pedido> {
        let data_limite = Utc::now() - chrono::Duration::days(dias);
        
        self.listar_pedidos_loja(loja_uuid)
            .into_iter()
            .filter(|p| p.criado_em >= data_limite)
            .collect()
    }
}

/// DTO para criar item de pedido
#[derive(Debug, Clone)]
pub struct ItemPedidoDto {
    pub produto_uuid: Uuid,
    pub quantidade: u32,
    pub observacoes: String,
    pub adicionais_uuids: Vec<Uuid>,
}

impl ItemPedidoDto {
    pub fn new(
        produto_uuid: Uuid,
        quantidade: u32,
        observacoes: String,
        adicionais_uuids: Vec<Uuid>,
    ) -> Self {
        Self {
            produto_uuid,
            quantidade,
            observacoes,
            adicionais_uuids,
        }
    }
}

/// Estatísticas de pedidos
#[derive(Debug, Clone)]
pub struct EstatisticasPedidos {
    pub total_pedidos: usize,
    pub pedidos_entregues: usize,
    pub pedidos_cancelados: usize,
    pub pedidos_em_andamento: usize,
    pub faturamento_total: f64,
    pub ticket_medio: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_transicao_status() {
        let service = PedidoService::new(
            PedidoRepository::new(),
            ProdutoRepository::new(),
        );

        // Transição válida
        assert!(service.validar_transicao_status("pendente", "confirmado").is_ok());
        
        // Transição inválida
        assert!(service.validar_transicao_status("entregue", "pendente").is_err());
    }
}