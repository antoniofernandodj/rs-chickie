use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::models::*;

/// Repositório genérico assíncrono
#[async_trait::async_trait]
pub trait Repository<T> {
    async fn criar(&self, item: T) -> Result<Uuid, String>;
    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<T>, String>;
    async fn atualizar(&self, id: Uuid, item: T) -> Result<(), String>;
    async fn deletar(&self, id: Uuid) -> Result<(), String>;
    async fn listar_todos(&self) -> Result<Vec<T>, String>;
}

// ==================== REPOSITÓRIO DE USUÁRIOS ====================
pub struct UsuarioRepository {
    pool: SqlitePool,
}

impl UsuarioRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>(
            "SELECT * FROM usuarios WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_username(&self, username: &str) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>(
            "SELECT * FROM usuarios WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_telefone(&self, telefone: &str) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>(
            "SELECT * FROM usuarios WHERE telefone = ?"
        )
        .bind(telefone)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Usuario> for UsuarioRepository {
    async fn criar(&self, item: Usuario) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO usuarios (
                uuid,
                username,
                email,
                senha_hash,
                telefone,
                criado_em,
                atualizado_em
            ) 
            VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(&item.username)
        .bind(&item.email)
        .bind(item.senha_hash)
        .bind(&item.telefone)
        .bind(item.criado_em.to_string())
        .bind("")
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<Usuario>, String> {
        sqlx::query_as::<_, Usuario>(
            "SELECT * FROM usuarios WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: Usuario) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE usuarios SET username = ?, email = ?, senha_hash = ?, telefone = ?, atualizado_em = ? 
             WHERE uuid = ?"
        )
        .bind(&item.username)
        .bind(&item.email)
        .bind(&item.senha_hash)
        .bind(&item.telefone)
        .bind("")
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Usuário não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM usuarios WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Usuário não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Usuario>, String> {
        sqlx::query_as::<_, Usuario>("SELECT * FROM usuarios")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE LOJAS ====================
pub struct LojaRepository {
    pool: SqlitePool,
}

impl LojaRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_slug(&self, slug: &str) -> Result<Option<Loja>, String> {
        sqlx::query_as::<_, Loja>(
            "SELECT * FROM lojas WHERE slug = ?"
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Loja>, String> {
        sqlx::query_as::<_, Loja>(
            "SELECT * FROM lojas WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn listar_ativas(&self) -> Result<Vec<Loja>, String> {
        sqlx::query_as::<_, Loja>(
            "SELECT * FROM lojas WHERE ativa = true"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Loja> for LojaRepository {
    async fn criar(&self, item: Loja) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO lojas (
                uuid,
                nome,
                slug,
                descricao,
                email,
                telefone,
                ativa,
                logo_url,
                banner_url, 
                horario_abertura,
                horario_fechamento,
                dias_funcionamento,
                tempo_preparo_min,
                taxa_entrega, 
                valor_minimo_pedido,
                raio_entrega_km,
                criado_em,
                atualizado_em
            ) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(&item.nome)
        .bind(&item.slug)
        .bind(&item.descricao)
        .bind(&item.email)
        .bind(&item.telefone)
        .bind(item.ativa)
        .bind(&item.logo_url)
        .bind(&item.banner_url)
        .bind(&item.horario_abertura)
        .bind(&item.horario_fechamento)
        .bind(&item.dias_funcionamento)
        .bind(item.tempo_preparo_min)
        .bind(item.taxa_entrega)
        .bind(item.valor_minimo_pedido)
        .bind(item.raio_entrega_km)
        .bind(item.criado_em)
        .bind(item.atualizado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<Loja>, String> {
        sqlx::query_as::<_, Loja>(
            "SELECT * FROM lojas WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: Loja) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE lojas SET nome = ?, slug = ?, descricao = ?, email = ?, telefone = ?, ativa = ?, 
             logo_url = ?, banner_url = ?, horario_abertura = ?, horario_fechamento = ?, 
             dias_funcionamento = ?, tempo_preparo_min = ?, taxa_entrega = ?, valor_minimo_pedido = ?, 
             raio_entrega_km = ?, atualizado_em = ? WHERE uuid = ?"
        )
        .bind(&item.nome)
        .bind(&item.slug)
        .bind(&item.descricao)
        .bind(&item.email)
        .bind(&item.telefone)
        .bind(item.ativa)
        .bind(&item.logo_url)
        .bind(&item.banner_url)
        .bind(&item.horario_abertura)
        .bind(&item.horario_fechamento)
        .bind(&item.dias_funcionamento)
        .bind(item.tempo_preparo_min)
        .bind(item.taxa_entrega)
        .bind(item.valor_minimo_pedido)
        .bind(item.raio_entrega_km)
        .bind(item.atualizado_em)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Loja não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM lojas WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Loja não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Loja>, String> {
        sqlx::query_as::<_, Loja>("SELECT * FROM lojas")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE CLIENTES ====================
pub struct ClienteRepository {
    pool: SqlitePool,
}

impl ClienteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>(
            "SELECT * FROM clientes WHERE usuario_uuid = ?"
        )
        .bind(usuario_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>(
            "SELECT * FROM clientes WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Cliente> for ClienteRepository {
    async fn criar(&self, item: Cliente) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO clientes (uuid, usuario_uuid, loja_uuid, criado_em) VALUES (?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(item.usuario_uuid.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(item.criado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<Cliente>, String> {
        sqlx::query_as::<_, Cliente>(
            "SELECT * FROM clientes WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: Cliente) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE clientes SET usuario_uuid = ?, loja_uuid = ? WHERE uuid = ?"
        )
        .bind(item.usuario_uuid.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Cliente não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM clientes WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Cliente não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Cliente>, String> {
        sqlx::query_as::<_, Cliente>("SELECT * FROM clientes")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE PRODUTOS ====================
pub struct ProdutoRepository {
    pool: SqlitePool,
}

impl ProdutoRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>(
            "SELECT * FROM produtos WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_categoria(&self, categoria_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>(
            "SELECT * FROM produtos WHERE categoria_uuid = ?"
        )
        .bind(categoria_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>(
            "SELECT * FROM produtos WHERE loja_uuid = ? AND disponivel = true"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_nome(&self, nome: &str, loja_uuid: Uuid) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>(
            "SELECT * FROM produtos WHERE loja_uuid = ? AND nome LIKE ?"
        )
        .bind(loja_uuid.to_string())
        .bind(format!("%{}%", nome))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Produto> for ProdutoRepository {
    async fn criar(&self, item: Produto) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO produtos (uuid, loja_uuid, categoria_uuid, nome, descricao, preco, 
             imagem_url, disponivel, tempo_preparo_min, destaque, criado_em, atualizado_em) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(item.categoria_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(&item.imagem_url)
        .bind(item.disponivel)
        .bind(item.tempo_preparo_min)
        .bind(item.destaque)
        .bind(item.criado_em)
        .bind(item.atualizado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<Produto>, String> {
        sqlx::query_as::<_, Produto>(
            "SELECT * FROM produtos WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: Produto) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE produtos SET loja_uuid = ?, categoria_uuid = ?, nome = ?, descricao = ?, 
             preco = ?, imagem_url = ?, disponivel = ?, tempo_preparo_min = ?, destaque = ?, 
             atualizado_em = ? WHERE uuid = ?"
        )
        .bind(item.loja_uuid.to_string())
        .bind(item.categoria_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(&item.imagem_url)
        .bind(item.disponivel)
        .bind(item.tempo_preparo_min)
        .bind(item.destaque)
        .bind(item.atualizado_em)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Produto não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM produtos WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Produto não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Produto>, String> {
        sqlx::query_as::<_, Produto>("SELECT * FROM produtos")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE CATEGORIAS ====================
pub struct CategoriaProdutosRepository {
    pool: SqlitePool,
}

impl CategoriaProdutosRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>(
            "SELECT * FROM categorias_produtos WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_nome(&self, nome: &str, loja_uuid: Uuid) -> Result<Option<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>(
            "SELECT * FROM categorias_produtos WHERE loja_uuid = ? AND nome = ?"
        )
        .bind(loja_uuid.to_string())
        .bind(nome)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<CategoriaProdutos> for CategoriaProdutosRepository {
    async fn criar(&self, item: CategoriaProdutos) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO categorias_produtos (uuid, loja_uuid, nome, descricao, ordem, criado_em) 
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.ordem)
        .bind(item.criado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>(
            "SELECT * FROM categorias_produtos WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: CategoriaProdutos) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE categorias_produtos SET loja_uuid = ?, nome = ?, descricao = ?, ordem = ? 
             WHERE uuid = ?"
        )
        .bind(item.loja_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.ordem)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Categoria não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM categorias_produtos WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Categoria não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<CategoriaProdutos>, String> {
        sqlx::query_as::<_, CategoriaProdutos>("SELECT * FROM categorias_produtos")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE PEDIDOS ====================
pub struct PedidoRepository {
    pool: SqlitePool,
}

impl PedidoRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>(
            "SELECT * FROM pedidos WHERE usuario_uuid = ?"
        )
        .bind(usuario_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>(
            "SELECT * FROM pedidos WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_status(&self, status: EstadoDePedido) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>(
            "SELECT * FROM pedidos WHERE status = ?"
        )
        .bind(status.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_pendentes(&self, loja_uuid: Uuid) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>(
            "SELECT * FROM pedidos WHERE loja_uuid = ? AND (status = ? OR status = ?)"
        )
        .bind(loja_uuid.to_string())
        .bind(EstadoDePedido::EmPreparo.to_string())
        .bind(EstadoDePedido::Criado.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Pedido> for PedidoRepository {
    async fn criar(&self, item: Pedido) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO pedidos (uuid, usuario_uuid, loja_uuid, status, total, subtotal, 
             taxa_entrega, desconto, forma_pagamento, observacoes, tempo_estimado_min, 
             criado_em, atualizado_em) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(item.usuario_uuid.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(item.status.to_string())
        .bind(item.total)
        .bind(item.subtotal)
        .bind(item.taxa_entrega)
        .bind(item.desconto)
        .bind(&item.forma_pagamento)
        .bind(&item.observacoes)
        .bind(item.tempo_estimado_min)
        .bind(item.criado_em)
        .bind(item.atualizado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<Pedido>, String> {
        sqlx::query_as::<_, Pedido>(
            "SELECT * FROM pedidos WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: Pedido) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE pedidos SET status = ?, total = ?, subtotal = ?, taxa_entrega = ?, 
             desconto = ?, forma_pagamento = ?, observacoes = ?, tempo_estimado_min = ?, 
             atualizado_em = ? WHERE uuid = ?"
        )
        .bind(item.status.to_string())
        .bind(item.total)
        .bind(item.subtotal)
        .bind(item.taxa_entrega)
        .bind(item.desconto)
        .bind(&item.forma_pagamento)
        .bind(&item.observacoes)
        .bind(item.tempo_estimado_min)
        .bind(item.atualizado_em)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Pedido não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM pedidos WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Pedido não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Pedido>, String> {
        sqlx::query_as::<_, Pedido>("SELECT * FROM pedidos")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE ADICIONAIS ====================
pub struct AdicionalRepository {
    pool: SqlitePool,
}

impl AdicionalRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>(
            "SELECT * FROM adicionais WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>(
            "SELECT * FROM adicionais WHERE loja_uuid = ? AND disponivel = true"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Adicional> for AdicionalRepository {
    async fn criar(&self, item: Adicional) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO adicionais (id, loja_uuid, nome, descricao, preco, disponivel, criado_em) 
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.id.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(item.disponivel)
        .bind(item.criado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.id)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<Adicional>, String> {
        sqlx::query_as::<_, Adicional>(
            "SELECT * FROM adicionais WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: Adicional) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE adicionais SET loja_uuid = ?, nome = ?, descricao = ?, preco = ?, 
             disponivel = ? WHERE id = ?"
        )
        .bind(item.loja_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.preco)
        .bind(item.disponivel)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Adicional não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM adicionais WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Adicional não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Adicional>, String> {
        sqlx::query_as::<_, Adicional>("SELECT * FROM adicionais")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE INGREDIENTES ====================
pub struct IngredienteRepository {
    pool: SqlitePool,
}

impl IngredienteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Ingrediente>, String> {
        sqlx::query_as::<_, Ingrediente>(
            "SELECT * FROM ingredientes WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Ingrediente>, String> {
        sqlx::query_as::<_, Ingrediente>(
            "SELECT * FROM ingredientes WHERE loja_uuid = ? AND quantidade > 0"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Ingrediente> for IngredienteRepository {
    async fn criar(&self, item: Ingrediente) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO ingredientes (uuid, loja_uuid, nome, unidade_medida, quantidade, 
             preco_unitario, criado_em, atualizado_em) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.unidade_medida)
        .bind(item.quantidade)
        .bind(item.preco_unitario)
        .bind(item.criado_em)
        .bind(item.atualizado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<Ingrediente>, String> {
        sqlx::query_as::<_, Ingrediente>(
            "SELECT * FROM ingredientes WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: Ingrediente) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE ingredientes SET loja_uuid = ?, nome = ?, unidade_medida = ?, 
             quantidade = ?, preco_unitario = ?, atualizado_em = ? WHERE uuid = ?"
        )
        .bind(item.loja_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.unidade_medida)
        .bind(item.quantidade)
        .bind(item.preco_unitario)
        .bind(item.atualizado_em)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Ingrediente não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM ingredientes WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Ingrediente não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Ingrediente>, String> {
        sqlx::query_as::<_, Ingrediente>("SELECT * FROM ingredientes")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE ENDEREÇOS DE LOJA ====================
pub struct EnderecoLojaRepository {
    pool: SqlitePool,
}

impl EnderecoLojaRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<EnderecoLoja>, String> {
        sqlx::query_as::<_, EnderecoLoja>(
            "SELECT * FROM enderecos_loja WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<EnderecoLoja> for EnderecoLojaRepository {
    async fn criar(&self, item: EnderecoLoja) -> Result<Uuid, String> {
        let id = item.get_uuid();
        sqlx::query(
            "INSERT INTO enderecos_loja (uuid, loja_uuid, cep, logradouro, numero, complemento, 
             bairro, cidade, estado, latitude, longitude) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(&item.cep)
        .bind(&item.logradouro)
        .bind(&item.numero)
        .bind(&item.complemento)
        .bind(&item.bairro)
        .bind(&item.cidade)
        .bind(&item.estado)
        .bind(item.latitude)
        .bind(item.longitude)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(id)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<EnderecoLoja>, String> {
        sqlx::query_as::<_, EnderecoLoja>(
            "SELECT * FROM enderecos_loja WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: EnderecoLoja) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE enderecos_loja SET loja_uuid = ?, cep = ?, logradouro = ?, numero = ?, 
             complemento = ?, bairro = ?, cidade = ?, estado = ?, latitude = ?, longitude = ? 
             WHERE uuid = ?"
        )
        .bind(item.loja_uuid.to_string())
        .bind(&item.cep)
        .bind(&item.logradouro)
        .bind(&item.numero)
        .bind(&item.complemento)
        .bind(&item.bairro)
        .bind(&item.cidade)
        .bind(&item.estado)
        .bind(item.latitude)
        .bind(item.longitude)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Endereço não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM enderecos_loja WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Endereço não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<EnderecoLoja>, String> {
        sqlx::query_as::<_, EnderecoLoja>("SELECT * FROM enderecos_loja")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE ENTREGADORES ====================
pub struct EntregadorRepository {
    pool: SqlitePool,
}

impl EntregadorRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        sqlx::query_as::<_, Entregador>(
            "SELECT * FROM entregadores WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_disponiveis(&self, loja_uuid: Uuid) -> Result<Vec<Entregador>, String> {
        sqlx::query_as::<_, Entregador>(
            "SELECT * FROM entregadores WHERE loja_uuid = ? AND disponivel = true"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_telefone(&self, telefone: &str) -> Result<Option<Entregador>, String> {
        sqlx::query_as::<_, Entregador>(
            "SELECT * FROM entregadores WHERE telefone = ?"
        )
        .bind(telefone)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Entregador> for EntregadorRepository {
    async fn criar(&self, item: Entregador) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO entregadores (uuid, loja_uuid, nome, telefone, veiculo, placa, 
             disponivel, criado_em) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.telefone)
        .bind(&item.veiculo)
        .bind(&item.placa)
        .bind(item.disponivel)
        .bind(item.criado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<Entregador>, String> {
        sqlx::query_as::<_, Entregador>(
            "SELECT * FROM entregadores WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: Entregador) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE entregadores SET loja_uuid = ?, nome = ?, telefone = ?, veiculo = ?, 
             placa = ?, disponivel = ? WHERE uuid = ?"
        )
        .bind(item.loja_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.telefone)
        .bind(&item.veiculo)
        .bind(&item.placa)
        .bind(item.disponivel)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Entregador não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM entregadores WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Entregador não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Entregador>, String> {
        sqlx::query_as::<_, Entregador>("SELECT * FROM entregadores")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE FUNCIONÁRIOS ====================
pub struct FuncionarioRepository {
    pool: SqlitePool,
}

impl FuncionarioRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>(
            "SELECT * FROM funcionarios WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_cargo(&self, cargo: &str, loja_uuid: Uuid) -> Result<Vec<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>(
            "SELECT * FROM funcionarios WHERE loja_uuid = ? AND cargo = ?"
        )
        .bind(loja_uuid.to_string())
        .bind(cargo)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_email(&self, email: &str) -> Result<Option<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>(
            "SELECT * FROM funcionarios WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Funcionario> for FuncionarioRepository {
    async fn criar(&self, item: Funcionario) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO funcionarios (uuid, loja_uuid, nome, email, cargo, salario, 
             data_admissao, criado_em) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.email)
        .bind(&item.cargo)
        .bind(item.salario)
        .bind(item.data_admissao)
        .bind(item.criado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>(
            "SELECT * FROM funcionarios WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: Funcionario) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE funcionarios SET loja_uuid = ?, nome = ?, email = ?, cargo = ?, 
             salario = ?, data_admissao = ? WHERE uuid = ?"
        )
        .bind(item.loja_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.email)
        .bind(&item.cargo)
        .bind(item.salario)
        .bind(item.data_admissao)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Funcionário não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM funcionarios WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Funcionário não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Funcionario>, String> {
        sqlx::query_as::<_, Funcionario>("SELECT * FROM funcionarios")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE AVALIAÇÕES DE LOJA ====================
pub struct AvaliacaoDeLojaRepository {
    pool: SqlitePool,
}

impl AvaliacaoDeLojaRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<AvaliacaoDeLoja>, String> {
        sqlx::query_as::<_, AvaliacaoDeLoja>(
            "SELECT * FROM avaliacoes_loja WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<AvaliacaoDeLoja>, String> {
        sqlx::query_as::<_, AvaliacaoDeLoja>(
            "SELECT * FROM avaliacoes_loja WHERE usuario_uuid = ?"
        )
        .bind(usuario_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn calcular_media(&self, loja_uuid: Uuid) -> Result<f64, String> {
        let result = sqlx::query(
            "SELECT AVG(nota) as media FROM avaliacoes_loja WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.try_get("media").unwrap_or(0.0))
    }
}

#[async_trait::async_trait]
impl Repository<AvaliacaoDeLoja> for AvaliacaoDeLojaRepository {
    async fn criar(&self, item: AvaliacaoDeLoja) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO avaliacoes_loja (uuid, loja_uuid, usuario_uuid, nota, comentario, criado_em) 
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(item.usuario_uuid.to_string())
        .bind(item.nota)
        .bind(&item.comentario)
        .bind(item.criado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<AvaliacaoDeLoja>, String> {
        sqlx::query_as::<_, AvaliacaoDeLoja>(
            "SELECT * FROM avaliacoes_loja WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: AvaliacaoDeLoja) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE avaliacoes_loja SET loja_uuid = ?, usuario_uuid = ?, nota = ?, 
             comentario = ? WHERE uuid = ?"
        )
        .bind(item.loja_uuid.to_string())
        .bind(item.usuario_uuid.to_string())
        .bind(item.nota)
        .bind(&item.comentario)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Avaliação não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM avaliacoes_loja WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Avaliação não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<AvaliacaoDeLoja>, String> {
        sqlx::query_as::<_, AvaliacaoDeLoja>("SELECT * FROM avaliacoes_loja")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE AVALIAÇÕES DE PRODUTO ====================
pub struct AvaliacaoDeProdutoRepository {
    pool: SqlitePool,
}

impl AvaliacaoDeProdutoRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_produto(&self, produto_uuid: Uuid) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>(
            "SELECT * FROM avaliacoes_produto WHERE produto_uuid = ?"
        )
        .bind(produto_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>(
            "SELECT * FROM avaliacoes_produto WHERE usuario_uuid = ?"
        )
        .bind(usuario_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_pedido(&self, pedido_uuid: Uuid) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>(
            "SELECT * FROM avaliacoes_produto WHERE pedido_uuid = ?"
        )
        .bind(pedido_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn calcular_media(&self, produto_uuid: Uuid) -> Result<f64, String> {
        let result = sqlx::query(
            "SELECT AVG(nota) as media FROM avaliacoes_produto WHERE produto_uuid = ?"
        )
        .bind(produto_uuid.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.try_get("media").unwrap_or(0.0))
    }
}

#[async_trait::async_trait]
impl Repository<AvaliacaoDeProduto> for AvaliacaoDeProdutoRepository {
    async fn criar(&self, item: AvaliacaoDeProduto) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO avaliacoes_produto (uuid, produto_uuid, usuario_uuid, pedido_uuid, 
             nota, comentario, criado_em) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(item.produto_uuid.to_string())
        .bind(item.usuario_uuid.to_string())
        .bind(item.pedido_uuid.to_string())
        .bind(item.nota)
        .bind(&item.comentario)
        .bind(item.criado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>(
            "SELECT * FROM avaliacoes_produto WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: AvaliacaoDeProduto) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE avaliacoes_produto SET produto_uuid = ?, usuario_uuid = ?, pedido_uuid = ?, 
             nota = ?, comentario = ? WHERE uuid = ?"
        )
        .bind(item.produto_uuid.to_string())
        .bind(item.usuario_uuid.to_string())
        .bind(item.pedido_uuid.to_string())
        .bind(item.nota)
        .bind(&item.comentario)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Avaliação não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM avaliacoes_produto WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Avaliação não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<AvaliacaoDeProduto>, String> {
        sqlx::query_as::<_, AvaliacaoDeProduto>("SELECT * FROM avaliacoes_produto")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE CUPONS ====================
pub struct CupomRepository {
    pool: SqlitePool,
}

impl CupomRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_codigo(&self, codigo: &str) -> Result<Option<Cupom>, String> {
        sqlx::query_as::<_, Cupom>(
            "SELECT * FROM cupons WHERE UPPER(codigo) = UPPER(?)"
        )
        .bind(codigo)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>(
            "SELECT * FROM cupons WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_ativos(&self, loja_uuid: Uuid) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>(
            "SELECT * FROM cupons WHERE loja_uuid = ? AND status = ?"
        )
        .bind(loja_uuid.to_string())
        .bind(StatusCupom::Ativo.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Cupom> for CupomRepository {
    async fn criar(&self, item: Cupom) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO cupons (uuid, loja_uuid, codigo, descricao, tipo_desconto, 
             valor_desconto, valor_minimo, data_validade, limite_uso, status, criado_em) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(&item.codigo)
        .bind(&item.descricao)
        .bind(item.tipo_desconto.to_string())
        .bind(item.valor_desconto)
        .bind(item.valor_minimo)
        .bind(item.data_validade)
        .bind(item.limite_uso)
        .bind(item.status.to_string())
        .bind(item.criado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<Cupom>, String> {
        sqlx::query_as::<_, Cupom>(
            "SELECT * FROM cupons WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: Cupom) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE cupons SET loja_uuid = ?, codigo = ?, descricao = ?, tipo_desconto = ?, 
             valor_desconto = ?, valor_minimo = ?, data_validade = ?, limite_uso = ?, 
             status = ? WHERE uuid = ?"
        )
        .bind(item.loja_uuid.to_string())
        .bind(&item.codigo)
        .bind(&item.descricao)
        .bind(item.tipo_desconto.to_string())
        .bind(item.valor_desconto)
        .bind(item.valor_minimo)
        .bind(item.data_validade)
        .bind(item.limite_uso)
        .bind(item.status.to_string())
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Cupom não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM cupons WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Cupom não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Cupom>, String> {
        sqlx::query_as::<_, Cupom>("SELECT * FROM cupons")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE USO DE CUPONS ====================
pub struct UsoCupomRepository {
    pool: SqlitePool,
}

impl UsoCupomRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_usuario(&self, usuario_uuid: Uuid) -> Result<Vec<UsoCupom>, String> {
        sqlx::query_as::<_, UsoCupom>(
            "SELECT * FROM uso_cupons WHERE usuario_uuid = ?"
        )
        .bind(usuario_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_cupom(&self, cupom_uuid: Uuid) -> Result<Vec<UsoCupom>, String> {
        sqlx::query_as::<_, UsoCupom>(
            "SELECT * FROM uso_cupons WHERE cupom_uuid = ?"
        )
        .bind(cupom_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn contar_usos_usuario(&self, usuario_uuid: Uuid, cupom_uuid: Uuid) -> Result<u32, String> {
        let result = sqlx::query(
            "SELECT COUNT(*) as count FROM uso_cupons WHERE usuario_uuid = ? AND cupom_uuid = ?"
        )
        .bind(usuario_uuid.to_string())
        .bind(cupom_uuid.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result.try_get::<i64, _>("count").unwrap_or(0) as u32)
    }
}

#[async_trait::async_trait]
impl Repository<UsoCupom> for UsoCupomRepository {
    async fn criar(&self, item: UsoCupom) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO uso_cupons (uuid, cupom_uuid, usuario_uuid, pedido_uuid, usado_em) 
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(item.cupom_uuid.to_string())
        .bind(item.usuario_uuid.to_string())
        .bind(item.pedido_uuid.to_string())
        .bind(item.usado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<UsoCupom>, String> {
        sqlx::query_as::<_, UsoCupom>(
            "SELECT * FROM uso_cupons WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: UsoCupom) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE uso_cupons SET cupom_uuid = ?, usuario_uuid = ?, pedido_uuid = ?, 
             usado_em = ? WHERE uuid = ?"
        )
        .bind(item.cupom_uuid.to_string())
        .bind(item.usuario_uuid.to_string())
        .bind(item.pedido_uuid.to_string())
        .bind(item.usado_em)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Uso de cupom não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM uso_cupons WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Uso de cupom não encontrado".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<UsoCupom>, String> {
        sqlx::query_as::<_, UsoCupom>("SELECT * FROM uso_cupons")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}

// ==================== REPOSITÓRIO DE PROMOÇÕES ====================
pub struct PromocaoRepository {
    pool: SqlitePool,
}

impl PromocaoRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_loja(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>(
            "SELECT * FROM promocoes WHERE loja_uuid = ?"
        )
        .bind(loja_uuid.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_ativas(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>(
            "SELECT * FROM promocoes WHERE loja_uuid = ? AND status = ?"
        )
        .bind(loja_uuid.to_string())
        .bind(StatusCupom::Ativo.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn buscar_por_prioridade(&self, loja_uuid: Uuid) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>(
            "SELECT * FROM promocoes WHERE loja_uuid = ? AND status = ? ORDER BY prioridade DESC"
        )
        .bind(loja_uuid.to_string())
        .bind(StatusCupom::Ativo.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}

#[async_trait::async_trait]
impl Repository<Promocao> for PromocaoRepository {
    async fn criar(&self, item: Promocao) -> Result<Uuid, String> {
        sqlx::query(
            "INSERT INTO promocoes (uuid, loja_uuid, nome, descricao, tipo_desconto, 
             valor_desconto, data_inicio, data_fim, prioridade, status, criado_em) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.uuid.to_string())
        .bind(item.loja_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.tipo_desconto.to_string())
        .bind(item.valor_desconto)
        .bind(item.data_inicio)
        .bind(item.data_fim)
        .bind(item.prioridade)
        .bind(item.status.to_string())
        .bind(item.criado_em)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(item.uuid)
    }

    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<Promocao>, String> {
        sqlx::query_as::<_, Promocao>(
            "SELECT * FROM promocoes WHERE uuid = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn atualizar(&self, id: Uuid, item: Promocao) -> Result<(), String> {
        let result = sqlx::query(
            "UPDATE promocoes SET loja_uuid = ?, nome = ?, descricao = ?, tipo_desconto = ?, 
             valor_desconto = ?, data_inicio = ?, data_fim = ?, prioridade = ?, status = ? 
             WHERE uuid = ?"
        )
        .bind(item.loja_uuid.to_string())
        .bind(&item.nome)
        .bind(&item.descricao)
        .bind(item.tipo_desconto.to_string())
        .bind(item.valor_desconto)
        .bind(item.data_inicio)
        .bind(item.data_fim)
        .bind(item.prioridade)
        .bind(item.status.to_string())
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Promoção não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn deletar(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM promocoes WHERE uuid = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            Err("Promoção não encontrada".to_string())
        } else {
            Ok(())
        }
    }

    async fn listar_todos(&self) -> Result<Vec<Promocao>, String> {
        sqlx::query_as::<_, Promocao>("SELECT * FROM promocoes")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}