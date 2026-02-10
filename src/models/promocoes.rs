use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;


/// Tipo de desconto aplicado
#[derive(Debug, Clone, PartialEq)]
pub enum TipoDesconto {
    Percentual(f64),      // Ex: 10% de desconto
    ValorFixo(f64),       // Ex: R$ 15,00 de desconto
    FreteGratis,          // Frete grátis
}

/// Condições para aplicação do cupom
#[derive(Debug, Clone, FromRow)]
pub struct CondicoesCupom {
    pub valor_minimo_pedido: Option<f64>,
    pub valor_maximo_desconto: Option<f64>,
    pub primeira_compra_apenas: bool,
    pub categorias_validas: Option<Vec<Uuid>>,  // UUIDs de categorias
    pub produtos_validos: Option<Vec<Uuid>>,    // UUIDs de produtos específicos
    pub dias_semana_validos: Option<Vec<u8>>,   // 0=Dom, 1=Seg, ..., 6=Sáb
    pub horario_inicio: Option<String>,         // "18:00"
    pub horario_fim: Option<String>,            // "22:00"
}

/// Status do cupom
#[derive(Debug, Clone, PartialEq)]
pub enum StatusCupom {
    Ativo,
    Inativo,
    Expirado,
    Esgotado,
}

/// Cupom de desconto
#[derive(Debug, Clone)]
pub struct Cupom {
    pub uuid: Uuid,
    pub codigo: String,                    // Ex: "PIZZA20", "BEMVINDO"
    pub descricao: String,
    pub tipo_desconto: TipoDesconto,
    pub loja_uuid: Uuid,
    pub condicoes: CondicoesCupom,
    pub data_inicio: DateTime<Utc>,
    pub data_fim: DateTime<Utc>,
    pub limite_uso_total: Option<u32>,     // Limite total de usos
    pub uso_atual: u32,
    pub limite_uso_por_usuario: Option<u32>, // Quantas vezes cada usuário pode usar
    pub status: StatusCupom,
    pub criado_em: DateTime<Utc>,
}

impl Cupom {
    pub fn new(
        codigo: String,
        descricao: String,
        tipo_desconto: TipoDesconto,
        loja_uuid: Uuid,
        condicoes: CondicoesCupom,
        data_inicio: DateTime<Utc>,
        data_fim: DateTime<Utc>,
        limite_uso_total: Option<u32>,
        limite_uso_por_usuario: Option<u32>,
    ) -> Self {

        Self {
            uuid: Uuid::new_v4(),
            codigo: codigo.to_uppercase(),
            descricao,
            tipo_desconto,
            loja_uuid,
            condicoes,
            data_inicio,
            data_fim,
            limite_uso_total,
            uso_atual: 0,
            limite_uso_por_usuario,
            status: StatusCupom::Ativo,
            criado_em: Utc::now(),
        }

    }

    /// Valida se o cupom pode ser aplicado
    pub fn validar(
        &self,
        valor_pedido: f64,
        usuario_uuid: Uuid,
        usos_usuario: u32,
        data_hora: DateTime<Utc>,
    ) -> Result<(), String> {
        // Verifica status
        if self.status != StatusCupom::Ativo {
            return Err(format!("Cupom está {:?}", self.status));
        }

        // Verifica período de validade
        if data_hora < self.data_inicio {
            return Err("Cupom ainda não está válido".to_string());
        }
        if data_hora > self.data_fim {
            return Err("Cupom expirado".to_string());
        }

        // Verifica limite total
        if let Some(limite) = self.limite_uso_total {
            if self.uso_atual >= limite {
                return Err("Cupom esgotado".to_string());
            }
        }

        // Verifica limite por usuário
        if let Some(limite) = self.limite_uso_por_usuario {
            if usos_usuario >= limite {
                return Err("Você já atingiu o limite de uso deste cupom".to_string());
            }
        }

        // Verifica valor mínimo
        if let Some(minimo) = self.condicoes.valor_minimo_pedido {
            if valor_pedido < minimo {
                return Err(format!(
                    "Valor mínimo do pedido deve ser R$ {:.2}",
                    minimo
                ));
            }
        }

        Ok(())
    }

    /// Calcula o valor do desconto
    pub fn calcular_desconto(&self, valor_pedido: f64, valor_frete: f64) -> f64 {
        let desconto_bruto = match &self.tipo_desconto {
            TipoDesconto::Percentual(percentual) => {
                valor_pedido * (percentual / 100.0)
            }
            TipoDesconto::ValorFixo(valor) => *valor,
            TipoDesconto::FreteGratis => valor_frete,
        };

        // Aplica limite máximo de desconto se existir
        if let Some(maximo) = self.condicoes.valor_maximo_desconto {
            desconto_bruto.min(maximo)
        } else {
            desconto_bruto
        }
    }

    /// Registra o uso do cupom
    pub fn registrar_uso(&mut self) {
        self.uso_atual += 1;
        
        // Atualiza status se atingiu limite
        if let Some(limite) = self.limite_uso_total {
            if self.uso_atual >= limite {
                self.status = StatusCupom::Esgotado;
            }
        }
    }

    pub fn ativar(&mut self) {
        if self.status == StatusCupom::Inativo {
            self.status = StatusCupom::Ativo;
        }
    }

    pub fn desativar(&mut self) {
        self.status = StatusCupom::Inativo;
    }
}

/// Histórico de uso de cupom por usuário
#[derive(Debug, Clone)]
pub struct UsoCupom {
    pub uuid: Uuid,
    pub cupom_uuid: Uuid,
    pub usuario_uuid: Uuid,
    pub pedido_uuid: Uuid,
    pub valor_desconto: f64,
    pub usado_em: DateTime<Utc>,
}

impl UsoCupom {
    pub fn new(
        cupom_uuid: Uuid,
        usuario_uuid: Uuid,
        pedido_uuid: Uuid,
        valor_desconto: f64,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            cupom_uuid,
            usuario_uuid,
            pedido_uuid,
            valor_desconto,
            usado_em: Utc::now(),
        }
    }
}

/// Promoção automática (sem código)
#[derive(Debug, Clone)]
pub struct Promocao {
    pub uuid: Uuid,
    pub nome: String,
    pub descricao: String,
    pub tipo_desconto: TipoDesconto,
    pub loja_uuid: Uuid,
    pub condicoes: CondicoesCupom,
    pub data_inicio: DateTime<Utc>,
    pub data_fim: DateTime<Utc>,
    pub status: StatusCupom,
    pub prioridade: u8,  // Para resolver conflitos entre promoções
    pub criado_em: DateTime<Utc>,
}

impl Promocao {
    pub fn new(
        nome: String,
        descricao: String,
        tipo_desconto: TipoDesconto,
        loja_uuid: Uuid,
        condicoes: CondicoesCupom,
        data_inicio: DateTime<Utc>,
        data_fim: DateTime<Utc>,
        prioridade: u8,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            nome,
            descricao,
            tipo_desconto,
            loja_uuid,
            condicoes,
            data_inicio,
            data_fim,
            status: StatusCupom::Ativo,
            prioridade,
            criado_em: Utc::now(),
        }
    }

    /// Verifica se a promoção é aplicável ao pedido
    pub fn eh_aplicavel(
        &self,
        valor_pedido: f64,
        data_hora: DateTime<Utc>,
        dia_semana: u8,
    ) -> bool {
        // Verifica status e período
        if self.status != StatusCupom::Ativo {
            return false;
        }
        if data_hora < self.data_inicio || data_hora > self.data_fim {
            return false;
        }

        // Verifica valor mínimo
        if let Some(minimo) = self.condicoes.valor_minimo_pedido {
            if valor_pedido < minimo {
                return false;
            }
        }

        // Verifica dia da semana
        if let Some(ref dias) = self.condicoes.dias_semana_validos {
            if !dias.contains(&dia_semana) {
                return false;
            }
        }

        true
    }

    /// Calcula o desconto da promoção
    pub fn calcular_desconto(&self, valor_pedido: f64, valor_frete: f64) -> f64 {
        let desconto_bruto = match &self.tipo_desconto {
            TipoDesconto::Percentual(percentual) => {
                valor_pedido * (percentual / 100.0)
            }
            TipoDesconto::ValorFixo(valor) => *valor,
            TipoDesconto::FreteGratis => valor_frete,
        };

        if let Some(maximo) = self.condicoes.valor_maximo_desconto {
            desconto_bruto.min(maximo)
        } else {
            desconto_bruto
        }
    }
}
