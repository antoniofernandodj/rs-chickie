use crate::models::{Entregador, Pedido, EnderecoLoja};
use crate::repositories::{PedidoRepository, Repository};
use uuid::Uuid;
use chrono::Utc;

pub struct EntregaService {
    pedido_repo: PedidoRepository,
}

impl EntregaService {
    pub fn new(pedido_repo: PedidoRepository) -> Self {
        Self { pedido_repo }
    }

    /// Cria um novo entregador
    pub fn criar_entregador(
        &self,
        nome: String,
        telefone: String,
        tipo_veiculo: String,
        placa_veiculo: String,
        loja_uuid: Uuid,
    ) -> Result<Entregador, String> {
        // Validar nome
        if nome.trim().is_empty() {
            return Err("Nome não pode ser vazio".to_string());
        }

        // Validar telefone
        if !self.validar_telefone(&telefone) {
            return Err("Telefone inválido".to_string());
        }

        // Validar tipo de veículo
        let tipos_validos = vec!["moto", "bicicleta", "carro", "patinete"];
        if !tipos_validos.contains(&tipo_veiculo.to_lowercase().as_str()) {
            return Err("Tipo de veículo inválido".to_string());
        }

        // Validar placa
        if !self.validar_placa(&placa_veiculo) {
            return Err("Placa inválida".to_string());
        }

        let entregador = Entregador::new(
            nome,
            telefone,
            tipo_veiculo,
            placa_veiculo,
            loja_uuid,
        );

        Ok(entregador)
    }

    /// Atribui um entregador a um pedido
    pub fn atribuir_entregador(
        &mut self,
        pedido_uuid: Uuid,
        entregador_uuid: Uuid,
    ) -> Result<Pedido, String> {
        let mut pedido = self.pedido_repo.buscar_por_uuid(pedido_uuid)
            .ok_or("Pedido não encontrado")?;

        // Verificar se pedido está pronto para entrega
        if pedido.status != "pronto" {
            return Err("Pedido não está pronto para entrega".to_string());
        }

        // Atribuir entregador (em um sistema real, você teria um campo para isso)
        // pedido.entregador_uuid = Some(entregador_uuid);
        pedido.status = "saiu_para_entrega".to_string();
        pedido.atualizado_em = Utc::now();

        self.pedido_repo.atualizar(pedido.clone())
            .map_err(|e| format!("Erro ao atribuir entregador: {}", e))?;

        Ok(pedido)
    }

    /// Calcula taxa de entrega baseada na distância
    pub fn calcular_taxa_entrega(
        &self,
        endereco_origem: &EnderecoLoja,
        endereco_destino: &EnderecoEntrega,
    ) -> f64 {
        // Simplificado - em produção usaria API de geolocalização
        let distancia_km = self.calcular_distancia_aproximada(
            endereco_origem,
            endereco_destino,
        );

        // Taxa base + valor por km
        let taxa_base = 5.0;
        let valor_por_km = 2.0;
        let taxa_minima = 3.0;

        let taxa = taxa_base + (distancia_km * valor_por_km);
        taxa.max(taxa_minima)
    }

    /// Calcula distância aproximada (simplificado)
    fn calcular_distancia_aproximada(
        &self,
        _origem: &EnderecoLoja,
        _destino: &EnderecoEntrega,
    ) -> f64 {
        // Em produção: usar API do Google Maps ou similar
        // Por enquanto retorna um valor fixo para teste
        5.0 // km
    }

    /// Calcula tempo estimado de entrega
    pub fn calcular_tempo_estimado(
        &self,
        distancia_km: f64,
        tipo_veiculo: &str,
    ) -> u32 {
        // Velocidade média por tipo de veículo (km/h)
        let velocidade = match tipo_veiculo.to_lowercase().as_str() {
            "moto" => 40.0,
            "carro" => 35.0,
            "bicicleta" => 15.0,
            "patinete" => 12.0,
            _ => 30.0,
        };

        // Tempo em minutos + buffer
        let tempo_viagem = (distancia_km / velocidade) * 60.0;
        let buffer = 10.0; // minutos de buffer

        (tempo_viagem + buffer) as u32
    }

    /// Lista entregas em andamento
    pub fn listar_entregas_em_andamento(&self, loja_uuid: Uuid) -> Vec<Pedido> {
        self.pedido_repo.listar_todos()
            .into_iter()
            .filter(|p| {
                p.loja_uuid == loja_uuid && 
                p.status == "saiu_para_entrega"
            })
            .collect()
    }

    /// Lista entregas por entregador
    pub fn listar_entregas_entregador(
        &self,
        entregador_uuid: Uuid,
    ) -> Vec<Pedido> {
        // Em um sistema real, filtraria por entregador_uuid
        // Por enquanto retorna vazio
        vec![]
    }

    /// Confirma entrega de pedido
    pub fn confirmar_entrega(
        &mut self,
        pedido_uuid: Uuid,
    ) -> Result<Pedido, String> {
        let mut pedido = self.pedido_repo.buscar_por_uuid(pedido_uuid)
            .ok_or("Pedido não encontrado")?;

        if pedido.status != "saiu_para_entrega" {
            return Err("Pedido não está em rota de entrega".to_string());
        }

        pedido.status = "entregue".to_string();
        pedido.atualizado_em = Utc::now();

        self.pedido_repo.atualizar(pedido.clone())
            .map_err(|e| format!("Erro ao confirmar entrega: {}", e))?;

        Ok(pedido)
    }

    /// Calcula estatísticas de entregador
    pub fn calcular_estatisticas_entregador(
        &self,
        entregador_uuid: Uuid,
    ) -> EstatisticasEntregador {
        let entregas = self.listar_entregas_entregador(entregador_uuid);

        let total_entregas = entregas.len();
        let entregas_concluidas = entregas.iter()
            .filter(|e| e.status == "entregue")
            .count();

        EstatisticasEntregador {
            total_entregas,
            entregas_concluidas,
            taxa_conclusao: if total_entregas > 0 {
                (entregas_concluidas as f64 / total_entregas as f64) * 100.0
            } else {
                0.0
            },
        }
    }

    /// Busca entregador disponível
    pub fn buscar_entregador_disponivel(
        &self,
        loja_uuid: Uuid,
    ) -> Option<Uuid> {
        // Em um sistema real:
        // 1. Buscar entregadores da loja
        // 2. Verificar quais não estão em entrega
        // 3. Retornar o mais próximo ou com menos entregas pendentes
        None
    }

    /// Otimiza rota de entregas
    pub fn otimizar_rota(
        &self,
        pedidos_uuids: Vec<Uuid>,
    ) -> Vec<Uuid> {
        // Em produção: usar algoritmo de otimização de rotas
        // Por enquanto retorna a mesma ordem
        pedidos_uuids
    }

    /// Valida formato de telefone
    fn validar_telefone(&self, telefone: &str) -> bool {
        let numeros: String = telefone.chars().filter(|c| c.is_numeric()).collect();
        numeros.len() >= 10 && numeros.len() <= 11
    }

    /// Valida formato de placa (formato brasileiro)
    fn validar_placa(&self, placa: &str) -> bool {
        let placa_limpa: String = placa.chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        
        // Formato antigo: ABC1234 ou Mercosul: ABC1D23
        placa_limpa.len() == 7
    }

    /// Calcula área de cobertura
    pub fn verificar_area_cobertura(
        &self,
        endereco_loja: &EnderecoLoja,
        endereco_entrega: &EnderecoEntrega,
        raio_max_km: f64,
    ) -> bool {
        let distancia = self.calcular_distancia_aproximada(
            endereco_loja,
            endereco_entrega,
        );

        distancia <= raio_max_km
    }

    /// Lista pedidos prontos para coleta
    pub fn listar_prontos_para_coleta(&self, loja_uuid: Uuid) -> Vec<Pedido> {
        self.pedido_repo.listar_todos()
            .into_iter()
            .filter(|p| p.loja_uuid == loja_uuid && p.status == "pronto")
            .collect()
    }
}

/// Endereço de entrega simplificado
#[derive(Debug, Clone)]
pub struct EnderecoEntrega {
    pub estado: String,
    pub cidade: String,
    pub rua: String,
    pub numero: String,
    pub bairro: String,
    pub cep: Option<String>,
    pub complemento: Option<String>,
}

impl EnderecoEntrega {
    pub fn new(
        estado: String,
        cidade: String,
        rua: String,
        numero: String,
        bairro: String,
        cep: Option<String>,
        complemento: Option<String>,
    ) -> Self {
        Self {
            estado,
            cidade,
            rua,
            numero,
            bairro,
            cep,
            complemento,
        }
    }
}

/// Estatísticas de entregador
#[derive(Debug, Clone)]
pub struct EstatisticasEntregador {
    pub total_entregas: usize,
    pub entregas_concluidas: usize,
    pub taxa_conclusao: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validar_placa() {
        let service = EntregaService::new(PedidoRepository::new());
        
        assert!(service.validar_placa("ABC1234"));
        assert!(service.validar_placa("ABC1D23"));
        assert!(!service.validar_placa("AB123"));
    }

    #[test]
    fn test_calcular_tempo_estimado() {
        let service = EntregaService::new(PedidoRepository::new());
        
        let tempo_moto = service.calcular_tempo_estimado(10.0, "moto");
        let tempo_bicicleta = service.calcular_tempo_estimado(10.0, "bicicleta");
        
        assert!(tempo_bicicleta > tempo_moto);
    }
}