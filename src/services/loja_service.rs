use crate::models::{Loja, EnderecoLoja};
use crate::repositories::{LojaRepository, Repository};
use uuid::Uuid;
use chrono::{Datelike, NaiveTime, Utc, Weekday};

pub struct LojaService {
    loja_repo: LojaRepository,
}

impl LojaService {
    pub fn new(loja_repo: LojaRepository) -> Self {
        Self { loja_repo }
    }

    /// Cria uma nova loja com validações
    pub fn criar_loja(
        &mut self,
        nome: String,
        username: String,
        email: String,
        telefone: String,
        senha: String,
        slug: String,
        horarios_de_funcionamento: Vec<String>,
    ) -> Result<Loja, String> {
        // Validar email único
        if self.loja_repo.buscar_por_email(&email).is_some() {
            return Err("Email já cadastrado".to_string());
        }

        // Validar slug único
        if self.loja_repo.buscar_por_slug(&slug).is_some() {
            return Err("Slug já em uso".to_string());
        }

        // Validar nome
        if nome.trim().is_empty() {
            return Err("Nome da loja não pode ser vazio".to_string());
        }

        // Validar telefone
        if !self.validar_telefone(&telefone) {
            return Err("Telefone inválido".to_string());
        }

        let loja = Loja::new(
            nome,
            username,
            email,
            telefone,
            senha, // Em produção, fazer hash da senha
            slug,
            horarios_de_funcionamento,
        );

        self.loja_repo.criar(loja.clone())
            .map_err(|e| format!("Erro ao criar loja: {}", e))?;

        Ok(loja)
    }

    /// Busca loja por UUID
    pub fn buscar_por_uuid(&self, uuid: Uuid) -> Option<&Loja> {
        self.loja_repo.buscar_por_id(uuid)
    }

    /// Busca loja por slug
    pub fn buscar_por_slug(&self, slug: &str) -> Option<&Loja> {
        self.loja_repo.buscar_por_slug(slug)
    }

    /// Busca loja por email
    pub fn buscar_por_email(&self, email: &str) -> Option<&Loja> {
        self.loja_repo.buscar_por_email(email)
    }

    /// Verifica se a loja está aberta no momento
    pub fn esta_aberta(&self, loja_uuid: Uuid) -> Result<bool, String> {
        let loja = self.loja_repo.buscar_por_id(loja_uuid)
            .ok_or("Loja não encontrada")?;

        let agora = Utc::now();
        let dia_semana = agora.weekday();
        let hora_atual = agora.time();

        // Verificar se há horário para o dia atual
        for horario in &loja.horarios_de_funcionamento {
            if self.horario_valido_para_dia(horario, dia_semana, hora_atual) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Verifica se um horário é válido para o dia e hora atuais
    fn horario_valido_para_dia(&self, horario: &str, dia: Weekday, hora: NaiveTime) -> bool {
        // Exemplo: "Seg-Sex 08:00-18:00" ou "Sáb 08:00-12:00"
        let partes: Vec<&str> = horario.split_whitespace().collect();
        if partes.len() < 2 {
            return false;
        }

        let dias_str = partes[0];
        let horario_str = partes[1];

        // Verificar se o dia atual está no range
        if !self.dia_no_range(dias_str, dia) {
            return false;
        }

        // Verificar se a hora atual está no range
        self.hora_no_range(horario_str, hora)
    }

    /// Verifica se o dia está no range especificado
    fn dia_no_range(&self, dias_str: &str, dia: Weekday) -> bool {
        // Simplificado - pode melhorar o parsing
        match dias_str {
            "Seg-Sex" => matches!(dia, Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri),
            "Sáb" | "Sab" => dia == Weekday::Sat,
            "Dom" => dia == Weekday::Sun,
            "Seg-Dom" => true,
            _ => false,
        }
    }

    /// Verifica se a hora está no range
    fn hora_no_range(&self, horario_str: &str, hora: NaiveTime) -> bool {
        let partes: Vec<&str> = horario_str.split('-').collect();
        if partes.len() != 2 {
            return false;
        }

        let inicio = match NaiveTime::parse_from_str(partes[0], "%H:%M") {
            Ok(t) => t,
            Err(_) => return false,
        };

        let fim = match NaiveTime::parse_from_str(partes[1], "%H:%M") {
            Ok(t) => t,
            Err(_) => return false,
        };

        hora >= inicio && hora <= fim
    }

    /// Atualiza informações da loja
    pub fn atualizar_loja(
        &mut self,
        uuid: Uuid,
        nome: Option<String>,
        telefone: Option<String>,
        horarios: Option<Vec<String>>,
    ) -> Result<Loja, String> {
        let mut loja = self.loja_repo.buscar_por_id(uuid)
            .ok_or("Loja não encontrada")?;

        if let Some(novo_nome) = nome {
            if novo_nome.trim().is_empty() {
                return Err("Nome não pode ser vazio".to_string());
            }
            loja.nome = novo_nome;
        }

        if let Some(novo_telefone) = telefone {
            if !self.validar_telefone(&novo_telefone) {
                return Err("Telefone inválido".to_string());
            }
            loja.telefone = novo_telefone;
        }

        if let Some(novos_horarios) = horarios {
            loja.horarios_de_funcionamento = novos_horarios;
        }

        self.loja_repo.atualizar(loja.clone())
            .map_err(|e| format!("Erro ao atualizar loja: {}", e))?;

        Ok(loja)
    }

    /// Lista todas as lojas
    pub fn listar_todas(&self) -> Vec<Loja> {
        self.loja_repo.listar_todos()
    }

    /// Lista lojas abertas no momento
    pub fn listar_abertas(&self) -> Vec<Loja> {
        self.listar_todas()
            .into_iter()
            .filter(|loja| self.esta_aberta(loja.uuid).unwrap_or(false))
            .collect()
    }

    /// Deleta uma loja
    pub fn deletar_loja(&mut self, uuid: Uuid) -> Result<(), String> {
        self.loja_repo.deletar(uuid)
            .map_err(|e| format!("Erro ao deletar loja: {}", e))
    }

    /// Valida formato de telefone
    fn validar_telefone(&self, telefone: &str) -> bool {
        let numeros: String = telefone.chars().filter(|c| c.is_numeric()).collect();
        numeros.len() >= 10 && numeros.len() <= 11
    }

    /// Calcula estatísticas da loja
    pub fn obter_estatisticas(&self, loja_uuid: Uuid) -> Result<EstatisticasLoja, String> {
        let _loja = self.loja_repo.buscar_por_uuid(loja_uuid)
            .ok_or("Loja não encontrada")?;

        // Aqui você integraria com outros repositórios para calcular:
        // - Total de pedidos
        // - Faturamento
        // - Avaliação média
        // - etc.

        Ok(EstatisticasLoja {
            total_pedidos: 0,
            faturamento_total: 0.0,
            avaliacao_media: 0.0,
            total_clientes: 0,
        })
    }
}

/// Estrutura para estatísticas da loja
#[derive(Debug, Clone)]
pub struct EstatisticasLoja {
    pub total_pedidos: usize,
    pub faturamento_total: f64,
    pub avaliacao_media: f64,
    pub total_clientes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_telefone() {
        let service = LojaService::new(LojaRepository::new());
        
        assert!(service.validar_telefone("11999999999"));
        assert!(service.validar_telefone("1199999999"));
        assert!(!service.validar_telefone("119999"));
    }
}