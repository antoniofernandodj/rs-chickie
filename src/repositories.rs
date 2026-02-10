use std::collections::HashMap;
use uuid::Uuid;
use crate::models::*;

/// Repositório genérico em memória
pub trait Repository<T> {
    fn criar(&mut self, item: T) -> Result<Uuid, String>;
    fn buscar_por_id(&self, id: Uuid) -> Option<&T>;
    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut T>;
    fn atualizar(&mut self, id: Uuid, item: T) -> Result<(), String>;
    fn deletar(&mut self, id: Uuid) -> Result<(), String>;
    fn listar_todos(&self) -> Vec<&T>;
}

// ==================== REPOSITÓRIO DE USUÁRIOS ====================
pub struct UsuarioRepository {
    dados: HashMap<Uuid, Usuario>,
}

impl UsuarioRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_email(&self, email: &str) -> Option<&Usuario> {
        self.dados.values().find(|u| u.email == email)
    }

    pub fn buscar_por_username(&self, username: &str) -> Option<&Usuario> {
        self.dados.values().find(|u| u.username == username)
    }

    pub fn buscar_por_telefone(&self, telefone: &str) -> Option<&Usuario> {
        self.dados.values().find(|u| u.telefone == Some(telefone.into()))
    }
}

impl Repository<Usuario> for UsuarioRepository {
    fn criar(&mut self, item: Usuario) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&Usuario> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut Usuario> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: Usuario) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Usuário não encontrado".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Usuário não encontrado".to_string())
    }

    fn listar_todos(&self) -> Vec<&Usuario> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE LOJAS ====================
pub struct LojaRepository {
    dados: HashMap<Uuid, Loja>,
}

impl LojaRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_slug(&self, slug: &str) -> Option<&Loja> {
        self.dados.values().find(|l| l.slug == slug)
    }

    pub fn buscar_por_email(&self, email: &str) -> Option<&Loja> {
        self.dados.values().find(|l| l.email == email)
    }

    pub fn listar_ativas(&self) -> Vec<&Loja> {
        self.dados
            .values()
            .filter(|l| l.ativa)
            .collect()
    }
}

impl Repository<Loja> for LojaRepository {
    fn criar(&mut self, item: Loja) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&Loja> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut Loja> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: Loja) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Loja não encontrada".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Loja não encontrada".to_string())
    }

    fn listar_todos(&self) -> Vec<&Loja> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE CLIENTES ====================
pub struct ClienteRepository {
    dados: HashMap<Uuid, Cliente>,
}

impl ClienteRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Vec<&Cliente> {
        self.dados
            .values()
            .filter(|c| c.usuario_uuid == usuario_uuid)
            .collect()
    }

    pub fn buscar_por_loja(&self, loja_uuid: Uuid) -> Vec<&Cliente> {
        self.dados
            .values()
            .filter(|c| c.loja_uuid == loja_uuid)
            .collect()
    }
}

impl Repository<Cliente> for ClienteRepository {
    fn criar(&mut self, item: Cliente) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&Cliente> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut Cliente> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: Cliente) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Cliente não encontrado".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Cliente não encontrado".to_string())
    }

    fn listar_todos(&self) -> Vec<&Cliente> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE PRODUTOS ====================
pub struct ProdutoRepository {
    dados: HashMap<Uuid, Produto>,
}

impl ProdutoRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_loja(&self, loja_uuid: Uuid) -> Vec<&Produto> {
        self.dados
            .values()
            .filter(|p| p.loja_uuid == loja_uuid)
            .collect()
    }

    pub fn buscar_por_categoria(&self, categoria_uuid: Uuid) -> Vec<&Produto> {
        self.dados
            .values()
            .filter(|p| p.categoria_uuid == categoria_uuid)
            .collect()
    }

    pub fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Vec<&Produto> {
        self.dados
            .values()
            .filter(|p| p.loja_uuid == loja_uuid && p.disponivel)
            .collect()
    }

    pub fn buscar_por_nome(&self, nome: &str, loja_uuid: Uuid) -> Vec<&Produto> {
        self.dados
            .values()
            .filter(|p| {
                p.loja_uuid == loja_uuid && 
                p.nome.to_lowercase().contains(&nome.to_lowercase())
            })
            .collect()
    }
}

impl Repository<Produto> for ProdutoRepository {
    fn criar(&mut self, item: Produto) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&Produto> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut Produto> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: Produto) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Produto não encontrado".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Produto não encontrado".to_string())
    }

    fn listar_todos(&self) -> Vec<&Produto> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE CATEGORIAS ====================
pub struct CategoriaProdutosRepository {
    dados: HashMap<Uuid, CategoriaProdutos>,
}

impl CategoriaProdutosRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_loja(&self, loja_uuid: Uuid) -> Vec<&CategoriaProdutos> {
        self.dados
            .values()
            .filter(|c| c.loja_uuid == loja_uuid)
            .collect()
    }

    pub fn buscar_por_nome(&self, nome: &str, loja_uuid: Uuid) -> Option<&CategoriaProdutos> {
        self.dados
            .values()
            .find(|c| c.loja_uuid == loja_uuid && c.nome == nome)
    }
}

impl Repository<CategoriaProdutos> for CategoriaProdutosRepository {
    fn criar(&mut self, item: CategoriaProdutos) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&CategoriaProdutos> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut CategoriaProdutos> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: CategoriaProdutos) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Categoria não encontrada".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Categoria não encontrada".to_string())
    }

    fn listar_todos(&self) -> Vec<&CategoriaProdutos> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE PEDIDOS ====================
pub struct PedidoRepository {
    dados: HashMap<Uuid, Pedido>,
}

impl PedidoRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Vec<&Pedido> {
        self.dados
            .values()
            .filter(|p| p.usuario_uuid == usuario_uuid)
            .collect()
    }

    pub fn buscar_por_loja(&self, loja_uuid: Uuid) -> Vec<&Pedido> {
        self.dados
            .values()
            .filter(|p| p.loja_uuid == loja_uuid)
            .collect()
    }

    pub fn buscar_por_status(&self, status: EstadoDePedido) -> Vec<&Pedido> {
        self.dados
            .values()
            .filter(|p| p.status == status)
            .collect()
    }

    pub fn buscar_pendentes(&self, loja_uuid: Uuid) -> Vec<&Pedido> {
        self.dados
            .values()
            .filter(|p| {
                p.loja_uuid == loja_uuid && 
                (p.status == EstadoDePedido::EmPreparo || p.status == EstadoDePedido::Criado)
            })
            .collect()
    }
}

impl Repository<Pedido> for PedidoRepository {
    fn criar(&mut self, item: Pedido) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&Pedido> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut Pedido> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: Pedido) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Pedido não encontrado".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Pedido não encontrado".to_string())
    }

    fn listar_todos(&self) -> Vec<&Pedido> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE ADICIONAIS ====================
pub struct AdicionalRepository {
    dados: HashMap<Uuid, Adicional>,
}

impl AdicionalRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_loja(&self, loja_uuid: Uuid) -> Vec<&Adicional> {
        self.dados
            .values()
            .filter(|a| a.loja_uuid == loja_uuid)
            .collect()
    }

    pub fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Vec<&Adicional> {
        self.dados
            .values()
            .filter(|a| a.loja_uuid == loja_uuid && a.disponivel)
            .collect()
    }
}

impl Repository<Adicional> for AdicionalRepository {
    fn criar(&mut self, item: Adicional) -> Result<Uuid, String> {
        let id = item.id;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&Adicional> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut Adicional> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: Adicional) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Adicional não encontrado".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Adicional não encontrado".to_string())
    }

    fn listar_todos(&self) -> Vec<&Adicional> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE INGREDIENTES ====================
pub struct IngredienteRepository {
    dados: HashMap<Uuid, Ingrediente>,
}

impl IngredienteRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_loja(&self, loja_uuid: Uuid) -> Vec<&Ingrediente> {
        self.dados
            .values()
            .filter(|i| i.loja_uuid == loja_uuid)
            .collect()
    }

    pub fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Vec<&Ingrediente> {
        self.dados
            .values()
            .filter(|i| i.loja_uuid == loja_uuid && i.quantidade > 0.0)
            .collect()
    }
}

impl Repository<Ingrediente> for IngredienteRepository {
    fn criar(&mut self, item: Ingrediente) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&Ingrediente> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut Ingrediente> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: Ingrediente) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Ingrediente não encontrado".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Ingrediente não encontrado".to_string())
    }

    fn listar_todos(&self) -> Vec<&Ingrediente> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE ENDEREÇOS DE LOJA ====================
pub struct EnderecoLojaRepository {
    dados: HashMap<Uuid, EnderecoLoja>,
}

impl EnderecoLojaRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_loja(&self, loja_uuid: Uuid) -> Vec<&EnderecoLoja> {
        self.dados
            .values()
            .filter(|e| e.loja_uuid == loja_uuid)
            .collect()
    }
}

impl Repository<EnderecoLoja> for EnderecoLojaRepository {
    fn criar(&mut self, item: EnderecoLoja) -> Result<Uuid, String> {
        let id = item.get_uuid();
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&EnderecoLoja> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut EnderecoLoja> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: EnderecoLoja) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Endereço não encontrado".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Endereço não encontrado".to_string())
    }

    fn listar_todos(&self) -> Vec<&EnderecoLoja> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE ENTREGADORES ====================
pub struct EntregadorRepository {
    dados: HashMap<Uuid, Entregador>,
}

impl EntregadorRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_loja(&self, loja_uuid: Uuid) -> Vec<&Entregador> {
        self.dados
            .values()
            .filter(|e| e.loja_uuid == loja_uuid)
            .collect()
    }

    pub fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Vec<&Entregador> {
        self.dados
            .values()
            .filter(|e| e.loja_uuid == loja_uuid && e.disponivel)
            .collect()
    }

    pub fn buscar_por_telefone(&self, telefone: &str) -> Option<&Entregador> {
        self.dados.values().find(|e| e.telefone == Some(telefone.to_string()))
    }
}

impl Repository<Entregador> for EntregadorRepository {
    fn criar(&mut self, item: Entregador) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&Entregador> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut Entregador> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: Entregador) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Entregador não encontrado".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Entregador não encontrado".to_string())
    }

    fn listar_todos(&self) -> Vec<&Entregador> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE FUNCIONÁRIOS ====================
pub struct FuncionarioRepository {
    dados: HashMap<Uuid, Funcionario>,
}

impl FuncionarioRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_loja(&self, loja_uuid: Uuid) -> Vec<&Funcionario> {
        self.dados
            .values()
            .filter(|f| f.loja_uuid == loja_uuid)
            .collect()
    }

    pub fn buscar_por_cargo(&self, cargo: &str, loja_uuid: Uuid) -> Vec<&Funcionario> {
        self.dados
            .values()
            .filter(|f| f.loja_uuid == loja_uuid && f.cargo == cargo)
            .collect()
    }

    pub fn buscar_por_email(&self, email: &str) -> Option<&Funcionario> {
        self.dados.values().find(|f| f.email == email)
    }
}

impl Repository<Funcionario> for FuncionarioRepository {
    fn criar(&mut self, item: Funcionario) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&Funcionario> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut Funcionario> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: Funcionario) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Funcionário não encontrado".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Funcionário não encontrado".to_string())
    }

    fn listar_todos(&self) -> Vec<&Funcionario> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE AVALIAÇÕES DE LOJA ====================
pub struct AvaliacaoDeLojaRepository {
    dados: HashMap<Uuid, AvaliacaoDeLoja>,
}

impl AvaliacaoDeLojaRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_loja(&self, loja_uuid: Uuid) -> Vec<&AvaliacaoDeLoja> {
        self.dados
            .values()
            .filter(|a| a.loja_uuid == loja_uuid)
            .collect()
    }

    pub fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Vec<&AvaliacaoDeLoja> {
        self.dados
            .values()
            .filter(|a| a.usuario_uuid == usuario_uuid)
            .collect()
    }

    pub fn calcular_media(&self, loja_uuid: Uuid) -> f64 {
        let avaliacoes = self.buscar_por_loja(loja_uuid);
        if avaliacoes.is_empty() {
            return 0.0;
        }
        let soma: f64 = avaliacoes.iter().map(|a| a.nota).sum();
        soma / avaliacoes.len() as f64
    }
}

impl Repository<AvaliacaoDeLoja> for AvaliacaoDeLojaRepository {
    fn criar(&mut self, item: AvaliacaoDeLoja) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&AvaliacaoDeLoja> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut AvaliacaoDeLoja> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: AvaliacaoDeLoja) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Avaliação não encontrada".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Avaliação não encontrada".to_string())
    }

    fn listar_todos(&self) -> Vec<&AvaliacaoDeLoja> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE AVALIAÇÕES DE PRODUTO ====================
pub struct AvaliacaoDeProdutoRepository {
    dados: HashMap<Uuid, AvaliacaoDeProduto>,
}

impl AvaliacaoDeProdutoRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_produto(&self, produto_uuid: Uuid) -> Vec<&AvaliacaoDeProduto> {
        self.dados
            .values()
            .filter(|a| a.produto_uuid == produto_uuid)
            .collect()
    }

    pub fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Vec<&AvaliacaoDeProduto> {
        self.dados
            .values()
            .filter(|a| a.usuario_uuid == usuario_uuid)
            .collect()
    }

    pub fn buscar_por_pedido(&self, pedido_uuid: Uuid) -> Vec<&AvaliacaoDeProduto> {
        self.dados
            .values()
            .filter(|a| a.produto_uuid == pedido_uuid)
            .collect()
    }

    pub fn calcular_media(&self, produto_uuid: Uuid) -> f64 {
        let avaliacoes = self.buscar_por_produto(produto_uuid);
        if avaliacoes.is_empty() {
            return 0.0;
        }
        let soma: f64 = avaliacoes.iter().map(|a| a.nota).sum();
        soma / avaliacoes.len() as f64
    }
}

impl Repository<AvaliacaoDeProduto> for AvaliacaoDeProdutoRepository {
    fn criar(&mut self, item: AvaliacaoDeProduto) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&AvaliacaoDeProduto> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut AvaliacaoDeProduto> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: AvaliacaoDeProduto) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Avaliação não encontrada".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Avaliação não encontrada".to_string())
    }

    fn listar_todos(&self) -> Vec<&AvaliacaoDeProduto> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE CUPONS ====================
pub struct CupomRepository {
    dados: HashMap<Uuid, Cupom>,
}

impl CupomRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_codigo(&self, codigo: &str) -> Option<&Cupom> {
        self.dados
            .values()
            .find(|c| c.codigo.to_uppercase() == codigo.to_uppercase())
    }

    pub fn buscar_por_loja(&self, loja_uuid: Uuid) -> Vec<&Cupom> {
        self.dados
            .values()
            .filter(|c| c.loja_uuid == loja_uuid)
            .collect()
    }

    pub fn buscar_ativos(&self, loja_uuid: Uuid) -> Vec<&Cupom> {
        self.dados
            .values()
            .filter(|c| {
                c.loja_uuid == loja_uuid && c.status == StatusCupom::Ativo
            })
            .collect()
    }
}

impl Repository<Cupom> for CupomRepository {
    fn criar(&mut self, item: Cupom) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&Cupom> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut Cupom> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: Cupom) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Cupom não encontrado".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Cupom não encontrado".to_string())
    }

    fn listar_todos(&self) -> Vec<&Cupom> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE USO DE CUPONS ====================
pub struct UsoCupomRepository {
    dados: HashMap<Uuid, UsoCupom>,
}

impl UsoCupomRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Vec<&UsoCupom> {
        self.dados
            .values()
            .filter(|u| u.usuario_uuid == usuario_uuid)
            .collect()
    }

    pub fn buscar_por_cupom(&self, cupom_uuid: Uuid) -> Vec<&UsoCupom> {
        self.dados
            .values()
            .filter(|u| u.cupom_uuid == cupom_uuid)
            .collect()
    }

    pub fn contar_usos_usuario(&self, usuario_uuid: Uuid, cupom_uuid: Uuid) -> u32 {
        self.dados
            .values()
            .filter(|u| u.usuario_uuid == usuario_uuid && u.cupom_uuid == cupom_uuid)
            .count() as u32
    }
}

impl Repository<UsoCupom> for UsoCupomRepository {
    fn criar(&mut self, item: UsoCupom) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&UsoCupom> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut UsoCupom> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: UsoCupom) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Uso de cupom não encontrado".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Uso de cupom não encontrado".to_string())
    }

    fn listar_todos(&self) -> Vec<&UsoCupom> {
        self.dados.values().collect()
    }
}

// ==================== REPOSITÓRIO DE PROMOÇÕES ====================
pub struct PromocaoRepository {
    dados: HashMap<Uuid, Promocao>,
}

impl PromocaoRepository {
    pub fn new() -> Self {
        Self {
            dados: HashMap::new(),
        }
    }

    pub fn buscar_por_loja(&self, loja_uuid: Uuid) -> Vec<&Promocao> {
        self.dados
            .values()
            .filter(|p| p.loja_uuid == loja_uuid)
            .collect()
    }

    pub fn buscar_ativas(&self, loja_uuid: Uuid) -> Vec<&Promocao> {
        self.dados
            .values()
            .filter(|p| {
                p.loja_uuid == loja_uuid && p.status == StatusCupom::Ativo
            })
            .collect()
    }

    pub fn buscar_por_prioridade(&self, loja_uuid: Uuid) -> Vec<&Promocao> {
        let mut promocoes = self.buscar_ativas(loja_uuid);
        promocoes.sort_by(|a, b| b.prioridade.cmp(&a.prioridade));
        promocoes
    }
}

impl Repository<Promocao> for PromocaoRepository {
    fn criar(&mut self, item: Promocao) -> Result<Uuid, String> {
        let id = item.uuid;
        self.dados.insert(id, item);
        Ok(id)
    }

    fn buscar_por_id(&self, id: Uuid) -> Option<&Promocao> {
        self.dados.get(&id)
    }

    fn buscar_por_id_mut(&mut self, id: Uuid) -> Option<&mut Promocao> {
        self.dados.get_mut(&id)
    }

    fn atualizar(&mut self, id: Uuid, item: Promocao) -> Result<(), String> {
        if self.dados.contains_key(&id) {
            self.dados.insert(id, item);
            Ok(())
        } else {
            Err("Promoção não encontrada".to_string())
        }
    }

    fn deletar(&mut self, id: Uuid) -> Result<(), String> {
        self.dados
            .remove(&id)
            .map(|_| ())
            .ok_or("Promoção não encontrada".to_string())
    }

    fn listar_todos(&self) -> Vec<&Promocao> {
        self.dados.values().collect()
    }
}
