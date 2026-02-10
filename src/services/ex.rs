// Exemplo de como usar os services na main.rs

mod models;
mod repositories;
mod services;

use services::{
    UsuarioService,
    LojaService,
    ProdutoService,
    PedidoService,
    CupomService,
    AvaliacaoService,
    EntregaService,
    PagamentoService,
};

use services::pedido_service::ItemPedidoDto;
use services::pagamento_service::{MetodoPagamento, DadosCartao};

use repositories::{
    UsuarioRepository,
    LojaRepository,
    ProdutoRepository,
    PedidoRepository,
    CupomRepository,
    AdicionalRepository,
    CategoriaProdutosRepository,
    AvaliacaoDeLojaRepository,
    AvaliacaoDeProdutoRepository,
};

use crate::models::TipoDesconto;
use chrono::Utc;

fn main() {
    println!("=== Sistema de Delivery - Camada de Serviços ===\n");

    // 1. INICIALIZAR SERVIÇOS
    let mut usuario_service = UsuarioService::new(UsuarioRepository::new());
    let mut loja_service = LojaService::new(LojaRepository::new());
    let mut produto_service = ProdutoService::new(
        ProdutoRepository::new(),
        AdicionalRepository::new(),
        CategoriaProdutosRepository::new(),
    );
    let mut cupom_service = CupomService::new(CupomRepository::new());
    let mut pedido_service = PedidoService::new(
        PedidoRepository::new(),
        ProdutoRepository::new(),
    );
    let mut avaliacao_service = AvaliacaoService::new(
        AvaliacaoDeLojaRepository::new(),
        AvaliacaoDeProdutoRepository::new(),
        PedidoRepository::new(),
    );
    let entrega_service = EntregaService::new(PedidoRepository::new());
    let pagamento_service = PagamentoService::new();

    // 2. CRIAR USUÁRIO
    println!("📱 Criando usuário...");
    let usuario = usuario_service.criar_usuario(
        "João Silva".into(),
        "joao_silva".into(),
        "joao@email.com".into(),
        "11999999999".into(),
        "email".into(),
    ).expect("Erro ao criar usuário");
    println!("✅ Usuário criado: {} ({})", usuario.nome, usuario.uuid);

    // 3. CRIAR LOJA
    println!("\n🏪 Criando loja...");
    let loja = loja_service.criar_loja(
        "Pizzaria Bella".into(),
        "bella_pizza".into(),
        "contato@bella.com".into(),
        "11988887777".into(),
        "senha_hash".into(),
        "bella-pizza".into(),
        vec![
            "Seg-Sex 11:00-23:00".into(),
            "Sáb-Dom 11:00-00:00".into(),
        ],
    ).expect("Erro ao criar loja");
    println!("✅ Loja criada: {} ({})", loja.nome, loja.uuid);

    // Verificar se está aberta
    let esta_aberta = loja_service.esta_aberta(loja.uuid)
        .expect("Erro ao verificar horário");
    println!("   Status: {}", if esta_aberta { "🟢 ABERTA" } else { "🔴 FECHADA" });

    // 4. CRIAR PRODUTOS
    println!("\n🍕 Criando produtos...");
    
    // Categoria
    let categoria = produto_service.criar_categoria(
        "Pizzas".into(),
        "Pizzas artesanais".into(),
        loja.uuid,
    ).expect("Erro ao criar categoria");
    println!("✅ Categoria criada: {}", categoria.nome);

    // Produtos
    let pizza_marg = produto_service.criar_produto(
        "Pizza Margherita".into(),
        "Molho, mussarela e manjericão".into(),
        45.90,
        categoria.uuid,
        loja.uuid,
    ).expect("Erro ao criar produto");
    println!("✅ Produto: {} - R$ {:.2}", pizza_marg.nome, pizza_marg.preco);

    let pizza_calabresa = produto_service.criar_produto(
        "Pizza Calabresa".into(),
        "Calabresa, cebola e mussarela".into(),
        42.90,
        categoria.uuid,
        loja.uuid,
    ).expect("Erro ao criar produto");
    println!("✅ Produto: {} - R$ {:.2}", pizza_calabresa.nome, pizza_calabresa.preco);

    // Adicionais
    let borda_catupiry = produto_service.criar_adicional(
        "Borda Catupiry".into(),
        loja.uuid,
        "Borda recheada com catupiry".into(),
        8.0,
    ).expect("Erro ao criar adicional");
    println!("✅ Adicional: {} - R$ {:.2}", borda_catupiry.nome, borda_catupiry.preco);

    // 5. CRIAR CUPOM
    println!("\n🎟️  Criando cupom...");
    let cupom = cupom_service.criar_cupom(
        "PRIMEIRA10".into(),
        "10% de desconto na primeira compra".into(),
        TipoDesconto::Percentual(10.0),
        loja.uuid,
        models::CondicoesCupom {
            valor_minimo_pedido: Some(30.0),
            valor_maximo_desconto: Some(20.0),
            primeira_compra_apenas: true,
            categorias_validas: None,
            produtos_validos: None,
            dias_semana_validos: None,
            horario_inicio: None,
            horario_fim: None,
        },
        Utc::now(),
        Utc::now() + chrono::Duration::days(30),
        Some(100),
        Some(1),
    ).expect("Erro ao criar cupom");
    println!("✅ Cupom criado: {} - {}", cupom.codigo, cupom.descricao);

    // 6. CRIAR PEDIDO
    println!("\n🛒 Criando pedido...");
    
    let itens = vec![
        ItemPedidoDto::new(
            pizza_marg.uuid,
            2,
            "Sem cebola".into(),
            vec![borda_catupiry.uuid],
        ),
        ItemPedidoDto::new(
            pizza_calabresa.uuid,
            1,
            "Bem assada".into(),
            vec![],
        ),
    ];

    let pedido = pedido_service.criar_pedido(
        usuario.uuid,
        loja.uuid,
        "11999999999".into(),
        Utc::now().to_rfc3339(),
        8.50,
        "Entregar na portaria".into(),
        itens,
        Some("PRIMEIRA10".into()),
        &loja_service,
        &mut cupom_service,
    ).expect("Erro ao criar pedido");

    println!("✅ Pedido criado: {}", pedido.uuid);
    println!("   Valor total: R$ {:.2}", pedido.valor_total);
    println!("   Status: {}", pedido.status);

    // 7. CALCULAR TAXA DE ENTREGA
    println!("\n🚚 Calculando entrega...");
    let tempo_estimado = entrega_service.calcular_tempo_estimado(5.0, "moto");
    println!("✅ Tempo estimado: {} minutos", tempo_estimado);

    // 8. PROCESSAR PAGAMENTO
    println!("\n💳 Processando pagamento...");
    let pagamento = pagamento_service.processar_pagamento(
        pedido.uuid,
        MetodoPagamento::CartaoCredito {
            dados: DadosCartao {
                numero: "4111111111111111".into(),
                titular: "João Silva".into(),
                validade: "12/28".into(),
                cvv: "123".into(),
            }
        },
        pedido.valor_total,
    ).expect("Erro ao processar pagamento");
    println!("✅ Pagamento processado: {:?}", pagamento.status);
    println!("   ID Transação: {}", pagamento.transacao_id.unwrap());

    // 9. ATUALIZAR STATUS DO PEDIDO
    println!("\n📦 Atualizando status do pedido...");
    let _ = pedido_service.atualizar_status(pedido.uuid, "confirmado".into());
    println!("✅ Status atualizado: confirmado");
    
    let _ = pedido_service.atualizar_status(pedido.uuid, "preparando".into());
    println!("✅ Status atualizado: preparando");
    
    let _ = pedido_service.atualizar_status(pedido.uuid, "pronto".into());
    println!("✅ Status atualizado: pronto");

    // 10. ATRIBUIR ENTREGADOR
    println!("\n🏍️  Atribuindo entregador...");
    let entregador = entrega_service.criar_entregador(
        "Carlos Moto".into(),
        "11988887777".into(),
        "moto".into(),
        "ABC1234".into(),
        loja.uuid,
    ).expect("Erro ao criar entregador");
    println!("✅ Entregador criado: {}", entregador.nome);

    // 11. SIMULAR ENTREGA
    println!("\n📍 Pedido saiu para entrega...");
    let _ = entrega_service.atribuir_entregador(pedido.uuid, entregador.uuid);
    println!("✅ Pedido em rota de entrega");

    // Depois de um tempo...
    let _ = entrega_service.confirmar_entrega(pedido.uuid);
    println!("✅ Pedido entregue!");

    // 12. CRIAR AVALIAÇÕES
    println!("\n⭐ Criando avaliações...");
    
    // Primeiro precisamos atualizar o pedido para "entregue"
    let _ = pedido_service.atualizar_status(pedido.uuid, "entregue".into());
    
    let avaliacao_loja = avaliacao_service.avaliar_loja(
        usuario.uuid,
        loja.uuid,
        4.5,
        "Ótima pizzaria! Entrega rápida.".into(),
    ).expect("Erro ao avaliar loja");
    println!("✅ Loja avaliada: {:.1} estrelas", avaliacao_loja.nota);

    let avaliacao_produto = avaliacao_service.avaliar_produto(
        usuario.uuid,
        loja.uuid,
        pedido.uuid,
        5.0,
        "Pizza deliciosa! Massa perfeita.".into(),
    ).expect("Erro ao avaliar produto");
    println!("✅ Produto avaliado: {:.1} estrelas", avaliacao_produto.nota);

    // 13. ESTATÍSTICAS
    println!("\n📊 Estatísticas:");
    
    let media_loja = avaliacao_service.calcular_media_loja(loja.uuid);
    println!("   Média da loja: {:.2} estrelas", media_loja);

    let stats_pedidos = pedido_service.calcular_estatisticas(loja.uuid);
    println!("   Total de pedidos: {}", stats_pedidos.total_pedidos);
    println!("   Faturamento: R$ {:.2}", stats_pedidos.faturamento_total);
    println!("   Ticket médio: R$ {:.2}", stats_pedidos.ticket_medio);

    // 14. CALCULAR PARCELAMENTO
    println!("\n💰 Simulação de parcelamento:");
    let parcelamento = pagamento_service.calcular_parcelamento(
        pedido.valor_total,
        3,
        5.0,
    ).expect("Erro ao calcular parcelamento");
    println!("   {} x R$ {:.2}", 
        parcelamento.numero_parcelas, 
        parcelamento.valor_parcela
    );
    println!("   Total com juros: R$ {:.2}", parcelamento.valor_total);

    println!("\n✨ Fluxo completo executado com sucesso!");
}