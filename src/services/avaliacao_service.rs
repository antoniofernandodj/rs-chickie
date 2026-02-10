use crate::models::{AvaliacaoDeLoja, AvaliacaoDeProduto, EstadoDePedido};
use crate::repositories::{
    AvaliacaoDeLojaRepository,
    AvaliacaoDeProdutoRepository,
    PedidoRepository,
    Repository
};
use uuid::Uuid;

pub struct AvaliacaoService {
    avaliacao_loja_repo: AvaliacaoDeLojaRepository,
    avaliacao_produto_repo: AvaliacaoDeProdutoRepository,
    pedido_repo: PedidoRepository,
}

impl AvaliacaoService {
    pub fn new(
        avaliacao_loja_repo: AvaliacaoDeLojaRepository,
        avaliacao_produto_repo: AvaliacaoDeProdutoRepository,
        pedido_repo: PedidoRepository,
    ) -> Self {
        Self {
            avaliacao_loja_repo,
            avaliacao_produto_repo,
            pedido_repo,
        }
    }

    // ============ AVALIAÇÕES DE LOJA ============

    /// Cria uma avaliação de loja
    pub fn avaliar_loja(
        &mut self,
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
        nota: f64,
        comentario: String,
    ) -> Result<AvaliacaoDeLoja, String> {
        // Validar nota
        if nota < 0.0 || nota > 5.0 {
            return Err("Nota deve estar entre 0 e 5".to_string());
        }

        // Verificar se usuário já avaliou esta loja
        if self.usuario_ja_avaliou_loja(usuario_uuid, loja_uuid) {
            return Err("Você já avaliou esta loja".to_string());
        }

        // Verificar se usuário fez pedido na loja
        if !self.usuario_fez_pedido_na_loja(usuario_uuid, loja_uuid) {
            return Err("Você precisa fazer um pedido antes de avaliar".to_string());
        }

        let avaliacao = AvaliacaoDeLoja::new(
            usuario_uuid,
            loja_uuid,
            nota,
            comentario,
        );

        self.avaliacao_loja_repo.criar(avaliacao.clone())
            .map_err(|e| format!("Erro ao criar avaliação: {}", e))?;

        Ok(avaliacao)
    }

    /// Verifica se usuário já avaliou a loja
    fn usuario_ja_avaliou_loja(&self, usuario_uuid: Uuid, loja_uuid: Uuid) -> bool {
        self.avaliacao_loja_repo.listar_todos()
            .iter()
            .any(|a| a.usuario_uuid == usuario_uuid && a.loja_uuid == loja_uuid)
    }

    /// Verifica se usuário fez pedido na loja
    fn usuario_fez_pedido_na_loja(&self, usuario_uuid: Uuid, loja_uuid: Uuid) -> bool {
        self.pedido_repo.listar_todos()
            .iter()
            .any(|p| {
                p.usuario_uuid == usuario_uuid && 
                p.loja_uuid == loja_uuid &&
                p.status == EstadoDePedido::Entregue
            })
    }

    /// Lista avaliações de uma loja
    pub fn listar_avaliacoes_loja(&self, loja_uuid: Uuid) -> Vec<&AvaliacaoDeLoja> {
        self.avaliacao_loja_repo.listar_todos()
            .into_iter()
            .filter(|a| a.loja_uuid == loja_uuid)
            .collect()
    }

    /// Calcula média de avaliações de uma loja
    pub fn calcular_media_loja(&self, loja_uuid: Uuid) -> f64 {
        let avaliacoes = self.listar_avaliacoes_loja(loja_uuid);
        
        if avaliacoes.is_empty() {
            return 0.0;
        }

        let soma: f64 = avaliacoes.iter().map(|a| a.nota).sum();
        soma / avaliacoes.len() as f64
    }

    /// Calcula distribuição de notas de uma loja
    pub fn distribuicao_notas_loja(&self, loja_uuid: Uuid) -> DistribuicaoNotas {
        let avaliacoes = self.listar_avaliacoes_loja(loja_uuid);
        
        let mut dist = DistribuicaoNotas::default();

        for avaliacao in avaliacoes {
            match avaliacao.nota as u8 {
                5 => dist.cinco_estrelas += 1,
                4 => dist.quatro_estrelas += 1,
                3 => dist.tres_estrelas += 1,
                2 => dist.duas_estrelas += 1,
                1 => dist.uma_estrela += 1,
                _ => {}
            }
        }

        dist
    }

    /// Atualiza avaliação de loja
    pub fn atualizar_avaliacao_loja(
        &mut self,
        avaliacao_uuid: Uuid,
        nota: Option<f64>,
        comentario: Option<String>,
    ) -> Result<&AvaliacaoDeLoja, String> {
        let avaliacao = self.avaliacao_loja_repo.buscar_por_id(avaliacao_uuid)
            .ok_or("Avaliação não encontrada")?;

        if let Some(nova_nota) = nota {
            if nova_nota < 0.0 || nova_nota > 5.0 {
                return Err("Nota deve estar entre 0 e 5".to_string());
            }
            avaliacao.nota = nova_nota;
        }

        if let Some(novo_comentario) = comentario {
            avaliacao.descricao = novo_comentario;
        }

        self.avaliacao_loja_repo
            .atualizar(avaliacao.uuid, avaliacao.clone())
            .map_err(|e| format!("Erro ao atualizar avaliação: {}", e))?;

        Ok(avaliacao)
    }

    /// Deleta avaliação de loja
    pub fn deletar_avaliacao_loja(
        &mut self,
        avaliacao_uuid: Uuid
    ) -> Result<(), String> {
        self.avaliacao_loja_repo.deletar(avaliacao_uuid)
            .map_err(|e| format!("Erro ao deletar avaliação: {}", e))
    }

    // ============ AVALIAÇÕES DE PRODUTO ============

    /// Cria uma avaliação de produto
    pub fn avaliar_produto(
        &mut self,
        usuario_uuid: Uuid,
        loja_uuid: Uuid,
        produto_uuid: Uuid,
        nota: f64,
        comentario: String,
    ) -> Result<AvaliacaoDeProduto, String> {
        // Validar nota
        if nota < 0.0 || nota > 5.0 {
            return Err("Nota deve estar entre 0 e 5".to_string());
        }

        // Verificar se pedido existe e pertence ao usuário
        let pedido = self.pedido_repo.buscar_por_id(produto_uuid)
            .ok_or("Pedido não encontrado")?;

        if pedido.usuario_uuid != usuario_uuid {
            return Err("Pedido não pertence ao usuário".to_string());
        }

        if pedido.status != EstadoDePedido::Entregue
         {
            return Err("Só é possível avaliar pedidos entregues".to_string());
        }

        // Verificar se usuário já avaliou este pedido
        if self.usuario_ja_avaliou_pedido(usuario_uuid, produto_uuid) {
            return Err("Você já avaliou este pedido".to_string());
        }

        let avaliacao = AvaliacaoDeProduto::new(
            usuario_uuid,
            loja_uuid,
            produto_uuid,
            nota,
            comentario,
        );

        self.avaliacao_produto_repo.criar(avaliacao.clone())
            .map_err(|e| format!("Erro ao criar avaliação: {}", e))?;

        Ok(avaliacao)
    }

    /// Verifica se usuário já avaliou o pedido
    fn usuario_ja_avaliou_pedido(&self, usuario_uuid: Uuid, pedido_uuid: Uuid) -> bool {
        self.avaliacao_produto_repo.listar_todos()
            .iter()
            .any(|a| a.usuario_uuid == usuario_uuid && a.produto_uuid == pedido_uuid)
    }

    /// Lista avaliações de produto de uma loja
    pub fn listar_avaliacoes_produto(&self, loja_uuid: Uuid) -> Vec<&AvaliacaoDeProduto> {
        self.avaliacao_produto_repo.listar_todos()
            .into_iter()
            .filter(|a| a.loja_uuid == loja_uuid)
            .collect()
    }

    /// Calcula média de avaliações de produto
    pub fn calcular_media_produto(&self, loja_uuid: Uuid) -> f64 {
        let avaliacoes = self.listar_avaliacoes_produto(loja_uuid);
        
        if avaliacoes.is_empty() {
            return 0.0;
        }

        let soma: f64 = avaliacoes.iter().map(|a| a.nota).sum();
        soma / avaliacoes.len() as f64
    }

    /// Atualiza avaliação de produto
    pub fn atualizar_avaliacao_produto(
        &mut self,
        avaliacao_uuid: Uuid,
        nota: Option<f64>,
        comentario: Option<String>,
    ) -> Result<AvaliacaoDeProduto, String> {
        let mut avaliacao = self.avaliacao_produto_repo.buscar_por_id(avaliacao_uuid)
            .ok_or("Avaliação não encontrada")?;

        if let Some(nova_nota) = nota {
            if nova_nota < 0.0 || nova_nota > 5.0 {
                return Err("Nota deve estar entre 0 e 5".to_string());
            }
            avaliacao.nota = nova_nota;
        }

        if let Some(novo_comentario) = comentario {
            avaliacao.descricao = novo_comentario;
        }

        self.avaliacao_produto_repo.atualizar(avaliacao.uuid, avaliacao.clone())
            .map_err(|e| format!("Erro ao atualizar avaliação: {}", e))?;

        Ok(avaliacao)
    }

    /// Deleta avaliação de produto
    pub fn deletar_avaliacao_produto(&mut self, avaliacao_uuid: Uuid) -> Result<(), String> {
        self.avaliacao_produto_repo.deletar(avaliacao_uuid)
            .map_err(|e| format!("Erro ao deletar avaliação: {}", e))
    }

    /// Lista avaliações de um usuário
    pub fn listar_avaliacoes_usuario(&self, usuario_uuid: Uuid) -> AvaliacoesUsuario {
        let avaliacoes_loja: Vec<&AvaliacaoDeLoja> = self.avaliacao_loja_repo.listar_todos()
            .into_iter()
            .filter(|a| a.usuario_uuid == usuario_uuid)
            .collect();

        let avaliacoes_produto: Vec<&AvaliacaoDeProduto> = self.avaliacao_produto_repo.listar_todos()
            .into_iter()
            .filter(|a| a.usuario_uuid == usuario_uuid)
            .collect();

        let avaliacoes_loja = avaliacoes_loja.iter().map(|i| i.clone().to_owned()).collect();
        let avaliacoes_produto = avaliacoes_produto.iter().map(|i| i.clone().to_owned()).collect();

        AvaliacoesUsuario {
            avaliacoes_loja,
            avaliacoes_produto,
        }

    }

    /// Lista melhores avaliações de uma loja
    pub fn melhores_avaliacoes_loja(&self, loja_uuid: Uuid, limite: usize) -> Vec<AvaliacaoDeLoja> {
        let mut avaliacoes = self.listar_avaliacoes_loja(loja_uuid);
        
        // Ordenar por nota (decrescente) e depois por data
        avaliacoes.sort_by(|a, b| {
            b.nota.partial_cmp(&a.nota)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.criado_em.cmp(&a.criado_em))
        });

        avaliacoes.into_iter().take(limite).collect()
    }

    /// Lista piores avaliações de uma loja
    pub fn piores_avaliacoes_loja(&self, loja_uuid: Uuid, limite: usize) -> Vec<AvaliacaoDeLoja> {
        let mut avaliacoes = self.listar_avaliacoes_loja(loja_uuid);
        
        // Ordenar por nota (crescente)
        avaliacoes.sort_by(|a, b| {
            a.nota.partial_cmp(&b.nota)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        avaliacoes.into_iter().take(limite).collect()
    }
}

/// Distribuição de notas
#[derive(Debug, Clone, Default)]
pub struct DistribuicaoNotas {
    pub cinco_estrelas: usize,
    pub quatro_estrelas: usize,
    pub tres_estrelas: usize,
    pub duas_estrelas: usize,
    pub uma_estrela: usize,
}

impl DistribuicaoNotas {
    pub fn total(&self) -> usize {
        self.cinco_estrelas + 
        self.quatro_estrelas + 
        self.tres_estrelas + 
        self.duas_estrelas + 
        self.uma_estrela
    }

    pub fn percentual_cinco_estrelas(&self) -> f64 {
        if self.total() == 0 {
            return 0.0;
        }
        (self.cinco_estrelas as f64 / self.total() as f64) * 100.0
    }
}

/// Avaliações de um usuário
#[derive(Debug, Clone)]
pub struct AvaliacoesUsuario {
    pub avaliacoes_loja: Vec<AvaliacaoDeLoja>,
    pub avaliacoes_produto: Vec<AvaliacaoDeProduto>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_nota() {
        let mut service = AvaliacaoService::new(
            AvaliacaoDeLojaRepository::new(),
            AvaliacaoDeProdutoRepository::new(),
            PedidoRepository::new(),
        );

        let resultado = service.avaliar_loja(
            Uuid::new_v4(),
            Uuid::new_v4(),
            6.0, // Inválido
            "Teste".into(),
        );

        assert!(resultado.is_err());
    }
}