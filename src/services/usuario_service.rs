use crate::models::Usuario;
use crate::repositories::{UsuarioRepository, Repository};
use uuid::Uuid;

pub struct UsuarioService {
    usuario_repo: UsuarioRepository,
}

impl UsuarioService {
    pub fn new(usuario_repo: UsuarioRepository) -> Self {
        Self { usuario_repo }
    }

    /// Cria um novo usuário com validações
    pub fn criar_usuario(
        &mut self,
        nome: String,
        username: String,
        email: String,
        telefone: String,
        tipo_autenticacao: String,
    ) -> Result<Usuario, String> {
        // Validar email único
        if self.usuario_repo.buscar_por_email(&email).is_some() {
            return Err("Email já cadastrado".to_string());
        }

        // Validar formato de email
        if !self.validar_email(&email) {
            return Err("Email inválido".to_string());
        }

        // Validar telefone
        if !self.validar_telefone(&telefone) {
            return Err("Telefone inválido".to_string());
        }

        // Validar nome
        if nome.trim().is_empty() {
            return Err("Nome não pode ser vazio".to_string());
        }

        let usuario = Usuario::new(nome, username, email, telefone, tipo_autenticacao);

        self.usuario_repo.criar(usuario.clone())
            .map_err(|e| format!("Erro ao criar usuário: {}", e))?;

        Ok(usuario)
    }

    /// Busca usuário por email
    pub fn buscar_por_email(&self, email: &str) -> Option<Usuario> {
        self.usuario_repo.buscar_por_email(email)
    }

    /// Busca usuário por UUID
    pub fn buscar_por_uuid(&self, uuid: Uuid) -> Option<Usuario> {
        self.usuario_repo.buscar_por_uuid(uuid)
    }

    /// Atualiza dados do usuário
    pub fn atualizar_usuario(
        &mut self,
        uuid: Uuid,
        nome: Option<String>,
        telefone: Option<String>,
    ) -> Result<Usuario, String> {
        let mut usuario = self.usuario_repo.buscar_por_uuid(uuid)
            .ok_or("Usuário não encontrado")?;

        if let Some(novo_nome) = nome {
            if novo_nome.trim().is_empty() {
                return Err("Nome não pode ser vazio".to_string());
            }
            usuario.nome = novo_nome;
        }

        if let Some(novo_telefone) = telefone {
            if !self.validar_telefone(&novo_telefone) {
                return Err("Telefone inválido".to_string());
            }
            usuario.telefone = novo_telefone;
        }

        self.usuario_repo.atualizar(usuario.clone())
            .map_err(|e| format!("Erro ao atualizar usuário: {}", e))?;

        Ok(usuario)
    }

    /// Lista todos os usuários
    pub fn listar_todos(&self) -> Vec<Usuario> {
        self.usuario_repo.listar_todos()
    }

    /// Deleta um usuário
    pub fn deletar_usuario(&mut self, uuid: Uuid) -> Result<(), String> {
        self.usuario_repo.deletar(uuid)
            .map_err(|e| format!("Erro ao deletar usuário: {}", e))
    }

    /// Valida formato de email
    fn validar_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }

    /// Valida formato de telefone (apenas números, 10-11 dígitos)
    fn validar_telefone(&self, telefone: &str) -> bool {
        let numeros: String = telefone.chars().filter(|c| c.is_numeric()).collect();
        numeros.len() >= 10 && numeros.len() <= 11
    }

    /// Autentica usuário (simulado)
    pub fn autenticar(&self, email: &str, senha: &str) -> Result<Usuario, String> {
        let usuario = self.usuario_repo.buscar_por_email(email)
            .ok_or("Usuário não encontrado")?;

        // Aqui você implementaria a verificação real de senha
        // Por enquanto, apenas retornamos o usuário
        Ok(usuario)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_email() {
        let service = UsuarioService::new(UsuarioRepository::new());
        
        assert!(service.validar_email("teste@email.com"));
        assert!(!service.validar_email("teste"));
        assert!(!service.validar_email("teste@"));
    }

    #[test]
    fn test_validar_telefone() {
        let service = UsuarioService::new(UsuarioRepository::new());
        
        assert!(service.validar_telefone("11999999999"));
        assert!(service.validar_telefone("1199999999"));
        assert!(!service.validar_telefone("119999"));
    }
}