use crate::models::{Produto, Adicional, Ingrediente, CategoriaProdutos};
use crate::repositories::{
    ProdutoRepository, 
    AdicionalRepository, 
    CategoriaProdutosRepository,
    Repository
};
use uuid::Uuid;

pub struct ProdutoService {
    produto_repo: ProdutoRepository,
    adicional_repo: AdicionalRepository,
    categoria_repo: CategoriaProdutosRepository,
}

impl ProdutoService {
    pub fn new(
        produto_repo: ProdutoRepository,
        adicional_repo: AdicionalRepository,
        categoria_repo: CategoriaProdutosRepository,
    ) -> Self {
        Self {
            produto_repo,
            adicional_repo,
            categoria_repo,
        }
    }

    // ============ PRODUTOS ============

    /// Cria um novo produto com validações
    pub fn criar_produto(
        &mut self,
        nome: String,
        descricao: String,
        preco: f64,
        categoria_uuid: Uuid,
        loja_uuid: Uuid,
    ) -> Result<Produto, String> {
        // Validar categoria existe
        if self.categoria_repo.buscar_por_uuid(categoria_uuid).is_none() {
            return Err("Categoria não encontrada".to_string());
        }

        // Validar nome
        if nome.trim().is_empty() {
            return Err("Nome do produto não pode ser vazio".to_string());
        }

        // Validar preço
        if preco <= 0.0 {
            return Err("Preço deve ser maior que zero".to_string());
        }

        let produto = Produto::new(nome, descricao, preco, categoria_uuid, loja_uuid);

        self.produto_repo.criar(produto.clone())
            .map_err(|e| format!("Erro ao criar produto: {}", e))?;

        Ok(produto)
    }

    /// Busca produto por UUID
    pub fn buscar_produto(&self, uuid: Uuid) -> Option<Produto> {
        self.produto_repo.buscar_por_uuid(uuid)
    }

    /// Lista todos os produtos de uma loja
    pub fn listar_produtos_por_loja(&self, loja_uuid: Uuid) -> Vec<Produto> {
        self.produto_repo.listar_todos()
            .into_iter()
            .filter(|p| p.loja_uuid == loja_uuid)
            .collect()
    }

    /// Lista produtos de uma categoria
    pub fn listar_produtos_por_categoria(&self, categoria_uuid: Uuid) -> Vec<Produto> {
        self.produto_repo.listar_todos()
            .into_iter()
            .filter(|p| p.categoria_uuid == categoria_uuid)
            .collect()
    }

    /// Atualiza produto
    pub fn atualizar_produto(
        &mut self,
        uuid: Uuid,
        nome: Option<String>,
        descricao: Option<String>,
        preco: Option<f64>,
        disponivel: Option<bool>,
    ) -> Result<Produto, String> {
        let mut produto = self.produto_repo.buscar_por_uuid(uuid)
            .ok_or("Produto não encontrado")?;

        if let Some(novo_nome) = nome {
            if novo_nome.trim().is_empty() {
                return Err("Nome não pode ser vazio".to_string());
            }
            produto.nome = novo_nome;
        }

        if let Some(nova_descricao) = descricao {
            produto.descricao = nova_descricao;
        }

        if let Some(novo_preco) = preco {
            if novo_preco <= 0.0 {
                return Err("Preço deve ser maior que zero".to_string());
            }
            produto.preco = novo_preco;
        }

        if let Some(disponibilidade) = disponivel {
            produto.disponivel = disponibilidade;
        }

        self.produto_repo.atualizar(produto.clone())
            .map_err(|e| format!("Erro ao atualizar produto: {}", e))?;

        Ok(produto)
    }

    /// Deleta produto
    pub fn deletar_produto(&mut self, uuid: Uuid) -> Result<(), String> {
        self.produto_repo.deletar(uuid)
            .map_err(|e| format!("Erro ao deletar produto: {}", e))
    }

    /// Calcula preço final do produto com adicionais
    pub fn calcular_preco_com_adicionais(
        &self,
        produto_uuid: Uuid,
        adicionais_uuids: Vec<Uuid>,
    ) -> Result<f64, String> {
        let produto = self.produto_repo.buscar_por_uuid(produto_uuid)
            .ok_or("Produto não encontrado")?;

        let mut preco_total = produto.preco;

        for adicional_uuid in adicionais_uuids {
            let adicional = self.adicional_repo.buscar_por_uuid(adicional_uuid)
                .ok_or(format!("Adicional {} não encontrado", adicional_uuid))?;
            
            preco_total += adicional.preco;
        }

        Ok(preco_total)
    }

    // ============ CATEGORIAS ============

    /// Cria uma nova categoria
    pub fn criar_categoria(
        &mut self,
        nome: String,
        descricao: String,
        loja_uuid: Uuid,
    ) -> Result<CategoriaProdutos, String> {
        if nome.trim().is_empty() {
            return Err("Nome da categoria não pode ser vazio".to_string());
        }

        let categoria = CategoriaProdutos::new(nome, descricao, loja_uuid);

        self.categoria_repo.criar(categoria.clone())
            .map_err(|e| format!("Erro ao criar categoria: {}", e))?;

        Ok(categoria)
    }

    /// Busca categoria por UUID
    pub fn buscar_categoria(&self, uuid: Uuid) -> Option<CategoriaProdutos> {
        self.categoria_repo.buscar_por_uuid(uuid)
    }

    /// Lista categorias de uma loja
    pub fn listar_categorias_por_loja(&self, loja_uuid: Uuid) -> Vec<CategoriaProdutos> {
        self.categoria_repo.listar_todos()
            .into_iter()
            .filter(|c| c.loja_uuid == loja_uuid)
            .collect()
    }

    /// Atualiza categoria
    pub fn atualizar_categoria(
        &mut self,
        uuid: Uuid,
        nome: Option<String>,
        descricao: Option<String>,
    ) -> Result<CategoriaProdutos, String> {
        let mut categoria = self.categoria_repo.buscar_por_uuid(uuid)
            .ok_or("Categoria não encontrada")?;

        if let Some(novo_nome) = nome {
            if novo_nome.trim().is_empty() {
                return Err("Nome não pode ser vazio".to_string());
            }
            categoria.nome = novo_nome;
        }

        if let Some(nova_descricao) = descricao {
            categoria.descricao = nova_descricao;
        }

        self.categoria_repo.atualizar(categoria.clone())
            .map_err(|e| format!("Erro ao atualizar categoria: {}", e))?;

        Ok(categoria)
    }

    /// Deleta categoria
    pub fn deletar_categoria(&mut self, uuid: Uuid) -> Result<(), String> {
        // Verificar se há produtos nesta categoria
        let produtos = self.listar_produtos_por_categoria(uuid);
        if !produtos.is_empty() {
            return Err("Não é possível deletar categoria com produtos associados".to_string());
        }

        self.categoria_repo.deletar(uuid)
            .map_err(|e| format!("Erro ao deletar categoria: {}", e))
    }

    // ============ ADICIONAIS ============

    /// Cria um novo adicional
    pub fn criar_adicional(
        &mut self,
        nome: String,
        loja_uuid: Uuid,
        descricao: String,
        preco: f64,
    ) -> Result<Adicional, String> {
        if nome.trim().is_empty() {
            return Err("Nome do adicional não pode ser vazio".to_string());
        }

        if preco < 0.0 {
            return Err("Preço não pode ser negativo".to_string());
        }

        let adicional = Adicional::new(nome, loja_uuid, descricao, preco);

        self.adicional_repo.criar(adicional.clone())
            .map_err(|e| format!("Erro ao criar adicional: {}", e))?;

        Ok(adicional)
    }

    /// Busca adicional por UUID
    pub fn buscar_adicional(&self, uuid: Uuid) -> Option<Adicional> {
        self.adicional_repo.buscar_por_uuid(uuid)
    }

    /// Lista adicionais de uma loja
    pub fn listar_adicionais_por_loja(&self, loja_uuid: Uuid) -> Vec<Adicional> {
        self.adicional_repo.listar_todos()
            .into_iter()
            .filter(|a| a.loja_uuid == loja_uuid)
            .collect()
    }

    /// Atualiza adicional
    pub fn atualizar_adicional(
        &mut self,
        uuid: Uuid,
        nome: Option<String>,
        descricao: Option<String>,
        preco: Option<f64>,
    ) -> Result<Adicional, String> {
        let mut adicional = self.adicional_repo.buscar_por_uuid(uuid)
            .ok_or("Adicional não encontrado")?;

        if let Some(novo_nome) = nome {
            if novo_nome.trim().is_empty() {
                return Err("Nome não pode ser vazio".to_string());
            }
            adicional.nome = novo_nome;
        }

        if let Some(nova_descricao) = descricao {
            adicional.descricao = nova_descricao;
        }

        if let Some(novo_preco) = preco {
            if novo_preco < 0.0 {
                return Err("Preço não pode ser negativo".to_string());
            }
            adicional.preco = novo_preco;
        }

        self.adicional_repo.atualizar(adicional.clone())
            .map_err(|e| format!("Erro ao atualizar adicional: {}", e))?;

        Ok(adicional)
    }

    /// Deleta adicional
    pub fn deletar_adicional(&mut self, uuid: Uuid) -> Result<(), String> {
        self.adicional_repo.deletar(uuid)
            .map_err(|e| format!("Erro ao deletar adicional: {}", e))
    }

    /// Busca produtos disponíveis de uma loja
    pub fn listar_produtos_disponiveis(&self, loja_uuid: Uuid) -> Vec<Produto> {
        self.produto_repo.listar_todos()
            .into_iter()
            .filter(|p| p.loja_uuid == loja_uuid && p.disponivel)
            .collect()
    }

    /// Busca produtos por nome (busca parcial)
    pub fn buscar_produtos_por_nome(&self, loja_uuid: Uuid, termo: &str) -> Vec<Produto> {
        let termo_lower = termo.to_lowercase();
        self.produto_repo.listar_todos()
            .into_iter()
            .filter(|p| {
                p.loja_uuid == loja_uuid && 
                (p.nome.to_lowercase().contains(&termo_lower) ||
                 p.descricao.to_lowercase().contains(&termo_lower))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_preco_produto() {
        let mut service = ProdutoService::new(
            ProdutoRepository::new(),
            AdicionalRepository::new(),
            CategoriaProdutosRepository::new(),
        );

        let categoria = service.criar_categoria(
            "Test".into(),
            "Desc".into(),
            Uuid::new_v4(),
        ).unwrap();

        let resultado = service.criar_produto(
            "Produto".into(),
            "Desc".into(),
            -10.0,
            categoria.uuid,
            Uuid::new_v4(),
        );

        assert!(resultado.is_err());
    }
}