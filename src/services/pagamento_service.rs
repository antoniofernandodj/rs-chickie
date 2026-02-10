use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct PagamentoService {
    // Em produção, incluiria clientes de APIs de pagamento
}

impl PagamentoService {
    pub fn new() -> Self {
        Self {}
    }

    /// Processa um pagamento
    pub fn processar_pagamento(
        &self,
        pedido_uuid: Uuid,
        metodo_pagamento: MetodoPagamento,
        valor: f64,
    ) -> Result<Pagamento, String> {
        // Validar valor
        if valor <= 0.0 {
            return Err("Valor deve ser maior que zero".to_string());
        }

        // Simular processamento baseado no método
        match metodo_pagamento {
            MetodoPagamento::CartaoCredito { ref dados } => {
                self.processar_cartao_credito(dados, valor)?
            }
            MetodoPagamento::CartaoDebito { ref dados } => {
                self.processar_cartao_debito(dados, valor)?
            }
            MetodoPagamento::Pix { ref chave } => {
                self.processar_pix(chave, valor)?
            }
            MetodoPagamento::Dinheiro => {
                self.processar_dinheiro(valor)?
            }
            MetodoPagamento::ValeRefeicao { ref dados } => {
                self.processar_vale_refeicao(dados, valor)?
            }
        }

        // Criar registro de pagamento
        let pagamento = Pagamento {
            uuid: Uuid::new_v4(),
            pedido_uuid,
            metodo: metodo_pagamento.clone(),
            valor,
            status: StatusPagamento::Aprovado,
            transacao_id: Some(format!("TXN{}", Uuid::new_v4())),
            criado_em: Utc::now(),
            atualizado_em: Utc::now(),
        };

        Ok(pagamento)
    }

    /// Processa pagamento com cartão de crédito
    fn processar_cartao_credito(
        &self,
        dados: &DadosCartao,
        valor: f64,
    ) -> Result<(), String> {
        // Validar dados do cartão
        if !self.validar_numero_cartao(&dados.numero) {
            return Err("Número de cartão inválido".to_string());
        }

        if !self.validar_cvv(&dados.cvv) {
            return Err("CVV inválido".to_string());
        }

        if !self.validar_validade(&dados.validade) {
            return Err("Data de validade inválida ou expirada".to_string());
        }

        // Em produção: integrar com gateway de pagamento
        // Por enquanto, simula sucesso
        Ok(())
    }

    /// Processa pagamento com cartão de débito
    fn processar_cartao_debito(
        &self,
        dados: &DadosCartao,
        valor: f64,
    ) -> Result<(), String> {
        // Similar ao cartão de crédito
        self.processar_cartao_credito(dados, valor)
    }

    /// Processa pagamento via PIX
    fn processar_pix(
        &self,
        chave: &str,
        valor: f64,
    ) -> Result<(), String> {
        // Validar chave PIX
        if chave.trim().is_empty() {
            return Err("Chave PIX inválida".to_string());
        }

        // Em produção: gerar QR code, verificar pagamento
        Ok(())
    }

    /// Processa pagamento em dinheiro
    fn processar_dinheiro(&self, valor: f64) -> Result<(), String> {
        // Pagamento em dinheiro é sempre aprovado
        Ok(())
    }

    /// Processa pagamento com vale refeição
    fn processar_vale_refeicao(
        &self,
        dados: &DadosCartao,
        valor: f64,
    ) -> Result<(), String> {
        // Similar a cartão, mas com validações específicas
        self.processar_cartao_credito(dados, valor)
    }

    /// Calcula valor de parcelamento
    pub fn calcular_parcelamento(
        &self,
        valor_total: f64,
        numero_parcelas: u8,
        juros_percentual: f64,
    ) -> Result<DetalhesParcelamento, String> {
        if numero_parcelas == 0 || numero_parcelas > 12 {
            return Err("Número de parcelas deve estar entre 1 e 12".to_string());
        }

        if juros_percentual < 0.0 {
            return Err("Taxa de juros não pode ser negativa".to_string());
        }

        // Calcular valor com juros
        let juros_total = valor_total * (juros_percentual / 100.0);
        let valor_com_juros = valor_total + juros_total;
        let valor_parcela = valor_com_juros / numero_parcelas as f64;

        Ok(DetalhesParcelamento {
            numero_parcelas,
            valor_parcela,
            valor_total: valor_com_juros,
            juros_percentual,
            juros_valor: juros_total,
        })
    }

    /// Calcula taxas de processamento
    pub fn calcular_taxa_processamento(
        &self,
        valor: f64,
        metodo: &MetodoPagamento,
    ) -> f64 {
        match metodo {
            MetodoPagamento::CartaoCredito { .. } => valor * 0.0399, // 3.99%
            MetodoPagamento::CartaoDebito { .. } => valor * 0.0199,  // 1.99%
            MetodoPagamento::Pix { .. } => 0.0,                      // Sem taxa
            MetodoPagamento::Dinheiro => 0.0,                        // Sem taxa
            MetodoPagamento::ValeRefeicao { .. } => valor * 0.0299,  // 2.99%
        }
    }

    /// Solicita estorno de pagamento
    pub fn solicitar_estorno(
        &self,
        pagamento_uuid: Uuid,
        motivo: String,
    ) -> Result<Estorno, String> {
        if motivo.trim().is_empty() {
            return Err("Motivo do estorno é obrigatório".to_string());
        }

        // Em produção: integrar com gateway para processar estorno
        let estorno = Estorno {
            uuid: Uuid::new_v4(),
            pagamento_uuid,
            valor_estornado: 0.0, // Deveria buscar do pagamento original
            motivo,
            status: StatusEstorno::Pendente,
            criado_em: Utc::now(),
        };

        Ok(estorno)
    }

    /// Verifica status de pagamento
    pub fn verificar_status_pagamento(
        &self,
        transacao_id: &str,
    ) -> Result<StatusPagamento, String> {
        // Em produção: consultar gateway de pagamento
        Ok(StatusPagamento::Aprovado)
    }

    /// Gera comprovante de pagamento
    pub fn gerar_comprovante(
        &self,
        pagamento: &Pagamento,
    ) -> String {
        format!(
            "COMPROVANTE DE PAGAMENTO\n\
             \n\
             ID: {}\n\
             Valor: R$ {:.2}\n\
             Método: {:?}\n\
             Status: {:?}\n\
             Data: {}\n\
             \n\
             Transação: {}",
            pagamento.uuid,
            pagamento.valor,
            pagamento.metodo,
            pagamento.status,
            pagamento.criado_em.format("%d/%m/%Y %H:%M"),
            pagamento.transacao_id.as_ref().unwrap_or(&"N/A".to_string())
        )
    }

    // ============ VALIDAÇÕES ============

    /// Valida número do cartão (Algoritmo de Luhn)
    fn validar_numero_cartao(&self, numero: &str) -> bool {
        let numeros: String = numero.chars().filter(|c| c.is_numeric()).collect();
        
        if numeros.len() < 13 || numeros.len() > 19 {
            return false;
        }

        // Algoritmo de Luhn simplificado
        let mut soma = 0;
        let mut dobrar = false;

        for c in numeros.chars().rev() {
            let mut digito = c.to_digit(10).unwrap();
            
            if dobrar {
                digito *= 2;
                if digito > 9 {
                    digito -= 9;
                }
            }
            
            soma += digito;
            dobrar = !dobrar;
        }

        soma % 10 == 0
    }

    /// Valida CVV
    fn validar_cvv(&self, cvv: &str) -> bool {
        let numeros: String = cvv.chars().filter(|c| c.is_numeric()).collect();
        numeros.len() == 3 || numeros.len() == 4
    }

    /// Valida data de validade do cartão
    fn validar_validade(&self, validade: &str) -> bool {
        // Formato: MM/AA
        let partes: Vec<&str> = validade.split('/').collect();
        
        if partes.len() != 2 {
            return false;
        }

        let mes = match partes[0].parse::<u32>() {
            Ok(m) => m,
            Err(_) => return false,
        };

        let ano = match partes[1].parse::<u32>() {
            Ok(a) => 2000 + a, // Assume formato de 2 dígitos
            Err(_) => return false,
        };

        if mes < 1 || mes > 12 {
            return false;
        }

        // Verificar se não está expirado
        let agora = Utc::now();
        let ano_atual = agora.year() as u32;
        let mes_atual = agora.month();

        if ano > ano_atual {
            return true;
        }

        if ano == ano_atual && mes >= mes_atual {
            return true;
        }

        false
    }
}

impl Default for PagamentoService {
    fn default() -> Self {
        Self::new()
    }
}

// ============ ESTRUTURAS ============

#[derive(Debug, Clone)]
pub enum MetodoPagamento {
    CartaoCredito { dados: DadosCartao },
    CartaoDebito { dados: DadosCartao },
    Pix { chave: String },
    Dinheiro,
    ValeRefeicao { dados: DadosCartao },
}

#[derive(Debug, Clone)]
pub struct DadosCartao {
    pub numero: String,
    pub titular: String,
    pub validade: String, // MM/AA
    pub cvv: String,
}

#[derive(Debug, Clone)]
pub struct Pagamento {
    pub uuid: Uuid,
    pub pedido_uuid: Uuid,
    pub metodo: MetodoPagamento,
    pub valor: f64,
    pub status: StatusPagamento,
    pub transacao_id: Option<String>,
    pub criado_em: DateTime<Utc>,
    pub atualizado_em: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusPagamento {
    Pendente,
    Processando,
    Aprovado,
    Recusado,
    Estornado,
    Cancelado,
}

#[derive(Debug, Clone)]
pub struct DetalhesParcelamento {
    pub numero_parcelas: u8,
    pub valor_parcela: f64,
    pub valor_total: f64,
    pub juros_percentual: f64,
    pub juros_valor: f64,
}

#[derive(Debug, Clone)]
pub struct Estorno {
    pub uuid: Uuid,
    pub pagamento_uuid: Uuid,
    pub valor_estornado: f64,
    pub motivo: String,
    pub status: StatusEstorno,
    pub criado_em: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum StatusEstorno {
    Pendente,
    Aprovado,
    Recusado,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_cvv() {
        let service = PagamentoService::new();
        
        assert!(service.validar_cvv("123"));
        assert!(service.validar_cvv("1234"));
        assert!(!service.validar_cvv("12"));
        assert!(!service.validar_cvv("12345"));
    }

    #[test]
    fn test_calcular_parcelamento() {
        let service = PagamentoService::new();
        
        let resultado = service.calcular_parcelamento(100.0, 10, 10.0);
        
        assert!(resultado.is_ok());
        let detalhes = resultado.unwrap();
        assert_eq!(detalhes.numero_parcelas, 10);
        assert_eq!(detalhes.valor_total, 110.0);
    }

    #[test]
    fn test_validar_validade_cartao() {
        let service = PagamentoService::new();
        
        // Este teste pode falhar dependendo da data atual
        assert!(service.validar_validade("12/30")); // Válido até 2030
        assert!(!service.validar_validade("01/20")); // Expirado
    }
}
