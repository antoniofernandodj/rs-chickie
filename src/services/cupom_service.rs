use crate::models::{Cupom, Promocao, TipoDesconto, CondicoesCupom};
use crate::repositories::{CupomRepository, Repository};
use uuid::Uuid;
use chrono::{Utc, DateTime, Datelike, NaiveTime};

pub struct CupomService {
    cupom_repo: CupomRepository,
}

impl CupomService {
    pub fn new(cupom_repo: CupomRepository) -> Self {
        Self { cupom_repo }
    }

    /// Cria um novo cupom
    pub fn criar_cupom(
        &mut self,
        codigo: String,
        descricao: String,
        tipo_desconto: TipoDesconto,
        loja_uuid: Uuid,
        condicoes: CondicoesCupom,
        data_inicio: DateTime<Utc>,
        data_fim: DateTime<Utc>,
        limite_uso_total: Option<u32>,
        limite_uso_por_usuario: Option<u32>,
    ) -> Result<Cupom, String> {
        // Validar código único
        if self.cupom_repo.buscar_por_codigo(&codigo).is_some() {
            return Err("Código de cupom já existe".to_string());
        }

        // Validar código não vazio
        if codigo.trim().is_empty() {
            return Err("Código não pode ser vazio".to_string());
        }

        // Validar datas
        if data_fim <= data_inicio {
            return Err("Data de fim deve ser posterior à data de início".to_string());
        }

        // Validar tipo de desconto
        match &tipo_desconto {
            TipoDesconto::Percentual(valor) => {
                if *valor <= 0.0 || *valor > 100.0 {
                    return Err("Percentual deve estar entre 0 e 100".to_string());
                }
            }
            TipoDesconto::ValorFixo(valor) => {
                if *valor <= 0.0 {
                    return Err("Valor fixo deve ser maior que zero".to_string());
                }
            }
            TipoDesconto::FreteGratis => {}
        }

        let cupom = Cupom::new(
            codigo,
            descricao,
            tipo_desconto,
            loja_uuid,
            condicoes,
            data_inicio,
            data_fim,
            limite_uso_total,
            limite_uso_por_usuario,
        );

        self.cupom_repo.criar(cupom.clone())
            .map_err(|e| format!("Erro ao criar cupom: {}", e))?;

        Ok(cupom)
    }

    /// Valida se um cupom pode ser usado
    pub fn validar_cupom(
        &self,
        codigo: &str,
        loja_uuid: Uuid,
        usuario_uuid: Uuid,
        valor_pedido: f64,
        primeira_compra: bool,
    ) -> Result<&Cupom, String> {
        let cupom = self.cupom_repo.buscar_por_codigo(codigo)
            .ok_or("Cupom não encontrado")?;

        // Validar loja
        if cupom.loja_uuid != loja_uuid {
            return Err("Cupom não válido para esta loja".to_string());
        }

        // Validar período de validade
        let agora = Utc::now();
        if agora < cupom.data_inicio {
            return Err("Cupom ainda não está válido".to_string());
        }
        if agora > cupom.data_fim {
            return Err("Cupom expirado".to_string());
        }

        // Validar status ativo
        if !cupom.ativo {
            return Err("Cupom inativo".to_string());
        }

        // Validar limite de uso total
        if let Some(limite) = cupom.limite_uso_total {
            if cupom.usos_realizados >= limite {
                return Err("Cupom atingiu limite de usos".to_string());
            }
        }

        // Validar primeira compra
        if cupom.condicoes.primeira_compra_apenas && !primeira_compra {
            return Err("Cupom válido apenas para primeira compra".to_string());
        }

        // Validar valor mínimo
        if let Some(valor_min) = cupom.condicoes.valor_minimo_pedido {
            if valor_pedido < valor_min {
                return Err(format!("Valor mínimo do pedido: R$ {:.2}", valor_min));
            }
        }

        // Validar dia da semana
        if let Some(ref dias_validos) = cupom.condicoes.dias_semana_validos {
            let dia_atual = agora.weekday().num_days_from_sunday() as u8;
            if !dias_validos.contains(&dia_atual) {
                return Err("Cupom não válido para este dia da semana".to_string());
            }
        }

        // Validar horário
        if let (Some(ref inicio), Some(ref fim)) = 
            (&cupom.condicoes.horario_inicio, &cupom.condicoes.horario_fim) {
            
            let hora_atual = agora.time();
            let hora_inicio = NaiveTime::parse_from_str(inicio, "%H:%M")
                .map_err(|_| "Formato de horário inválido")?;
            let hora_fim = NaiveTime::parse_from_str(fim, "%H:%M")
                .map_err(|_| "Formato de horário inválido")?;

            if hora_atual < hora_inicio || hora_atual > hora_fim {
                return Err(format!("Cupom válido apenas entre {} e {}", inicio, fim));
            }
        }

        Ok(cupom)
    }

    /// Calcula o desconto de um cupom
    pub fn calcular_desconto(
        &self,
        cupom: &Cupom,
        valor_pedido: f64,
        valor_frete: f64,
    ) -> f64 {
        let desconto_bruto = match cupom.tipo_desconto {
            TipoDesconto::Percentual(percentual) => {
                valor_pedido * (percentual / 100.0)
            }
            TipoDesconto::ValorFixo(valor) => valor,
            TipoDesconto::FreteGratis => valor_frete,
        };

        // Aplicar valor máximo de desconto se houver
        if let Some(max) = cupom.condicoes.valor_maximo_desconto {
            desconto_bruto.min(max)
        } else {
            desconto_bruto
        }
    }

    /// Aplica um cupom e retorna o valor do desconto
    pub fn aplicar_cupom(
        &mut self,
        codigo: &str,
        loja_uuid: Uuid,
        usuario_uuid: Uuid,
        valor_pedido: f64,
        valor_frete: f64,
        primeira_compra: bool,
    ) -> Result<f64, String> {
        // Validar cupom
        let cupom = self.validar_cupom(
            codigo,
            loja_uuid,
            usuario_uuid,
            valor_pedido,
            primeira_compra,
        )?;

        // Calcular desconto
        let desconto = self.calcular_desconto(cupom, valor_pedido, valor_frete);

        // Incrementar contador de usos
        // Nota: Em um sistema real, você também registraria o uso por usuário
        let mut cupom_mut = cupom.clone();
        cupom_mut.usos_realizados += 1;
        
        self.cupom_repo.atualizar(cupom_mut)
            .map_err(|e| format!("Erro ao atualizar cupom: {}", e))?;

        Ok(desconto)
    }

    /// Busca cupom por código
    pub fn buscar_por_codigo(&self, codigo: &str) -> Option<Cupom> {
        self.cupom_repo.buscar_por_codigo(codigo)
    }

    /// Lista cupons ativos de uma loja
    pub fn listar_cupons_ativos(&self, loja_uuid: Uuid) -> Vec<Cupom> {
        self.cupom_repo.buscar_ativos(loja_uuid)
    }

    /// Lista todos os cupons de uma loja
    pub fn listar_cupons_loja(&self, loja_uuid: Uuid) -> Vec<Cupom> {
        self.cupom_repo.listar_todos()
            .into_iter()
            .filter(|c| c.loja_uuid == loja_uuid)
            .collect()
    }

    /// Ativa ou desativa um cupom
    pub fn alterar_status_cupom(&mut self, codigo: &str, ativo: bool) -> Result<(), String> {
        let mut cupom = self.cupom_repo.buscar_por_codigo(codigo)
            .ok_or("Cupom não encontrado")?;

        cupom.ativo = ativo;

        self.cupom_repo.atualizar(cupom)
            .map_err(|e| format!("Erro ao atualizar status: {}", e))
    }

    /// Verifica promoções aplicáveis automaticamente
    pub fn buscar_promocoes_aplicaveis(
        &self,
        loja_uuid: Uuid,
        valor_pedido: f64,
    ) -> Vec<Promocao> {
        let agora = Utc::now();
        let dia_atual = agora.weekday().num_days_from_sunday() as u8;
        let hora_atual = agora.time();

        // Aqui você buscaria promoções do repositório
        // Por enquanto retorna vazio
        vec![]
    }

    /// Busca o melhor desconto entre cupom e promoções
    pub fn melhor_desconto(
        &self,
        cupom_codigo: Option<&str>,
        loja_uuid: Uuid,
        usuario_uuid: Uuid,
        valor_pedido: f64,
        valor_frete: f64,
    ) -> Option<(String, f64)> {
        let mut melhor: Option<(String, f64)> = None;

        // Verificar cupom
        if let Some(codigo) = cupom_codigo {
            if let Ok(cupom) = self.validar_cupom(
                codigo,
                loja_uuid,
                usuario_uuid,
                valor_pedido,
                false,
            ) {
                let desconto = self.calcular_desconto(cupom, valor_pedido, valor_frete);
                melhor = Some((codigo.to_string(), desconto));
            }
        }

        // Verificar promoções automáticas
        let promocoes = self.buscar_promocoes_aplicaveis(loja_uuid, valor_pedido);
        for promo in promocoes {
            // Calcular desconto da promoção
            // Comparar com melhor desconto atual
        }

        melhor
    }

    /// Deleta um cupom
    pub fn deletar_cupom(&mut self, codigo: &str) -> Result<(), String> {
        let cupom = self.cupom_repo.buscar_por_codigo(codigo)
            .ok_or("Cupom não encontrado")?;

        self.cupom_repo.deletar(cupom.uuid)
            .map_err(|e| format!("Erro ao deletar cupom: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_percentual() {
        let mut service = CupomService::new(CupomRepository::new());
        
        let resultado = service.criar_cupom(
            "TESTE".into(),
            "Desconto teste".into(),
            TipoDesconto::Percentual(150.0), // Inválido
            Uuid::new_v4(),
            CondicoesCupom::default(),
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
            None,
            None,
        );

        assert!(resultado.is_err());
    }
}