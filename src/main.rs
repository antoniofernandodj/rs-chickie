mod models;
mod database;
mod repositories;
mod repositories_async;
mod services;
use chrono::Utc;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use models::{
    Adicional,
    AvaliacaoDeProduto,
    AvaliacoesDeLoja,
    AvaliacaoDeLoja,
    AvaliacoesDeProduto,
    CategoriaProdutos,
    // CategoriasProdutos,
    Usuario,
    // Usuarios,
    Loja,
    // Lojas,
    Cliente,
    Pedido,
    // Pedidos,
    // EstadoDePedido,
    Ingrediente,
    EnderecoLoja,
    // EnderecosLoja,
    Entregador,
    // Entregadores,
    Funcionario,
    Produto,
    // Funcionarios
    TipoDesconto,
    CondicoesCupom,
    // StatusCupom,
    Cupom,
    // UsoCupom,
    Promocao
};
// use uuid::Uuid;

use repositories_async::{
    UsuarioRepository,
    LojaRepository,
    ProdutoRepository,
    PedidoRepository,
    CupomRepository,
    AdicionalRepository,
    CategoriaProdutosRepository,
    FuncionarioRepository,
    AvaliacaoDeLojaRepository,
    AvaliacaoDeProdutoRepository,
    Repository as _
};

// use crate::repositories::Repository as _;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let pool = database::criar_pool().await.unwrap();

    // Inicializar repositórios
    let mut usuario_repo =
        UsuarioRepository::new(pool);
    let mut loja_repo =
        LojaRepository::new(pool);
    let mut produto_repo =
        ProdutoRepository::new(pool);
    let mut pedido_repo =
        PedidoRepository::new(pool);
    let mut cupom_repo =
        CupomRepository::new(pool);
    let mut adicional_repo =
        AdicionalRepository::new(pool);
    let mut avaliacoes_de_produto_repo =
        AvaliacaoDeProdutoRepository::new(pool);
    let mut avaliacoes_de_loja_repo =
        AvaliacaoDeLojaRepository::new(pool);
    let mut funcionario_repo =
        FuncionarioRepository::new(pool);
    let mut categorias_de_produtos_repo = 
        CategoriaProdutosRepository::new(pool);

    let usuario: Usuario = Usuario::new(
        String::from("Antonio Silva"),
        String::from("antonio"),
        String::from("antonio@email.com"),
        String::from("11999999999"),
        String::from("email"),
    );

    usuario_repo.criar(usuario.clone()).unwrap();

    // Buscar usuário por email
    if let Some(u) = usuario_repo.buscar_por_email("antonio@email.com") {
        println!("Usuário encontrado: {:?}", u.nome);
    }

    println!("usuario: {:?}", usuario);

    let horarios_de_funcionamento = vec![
        String::from("Seg-Sex 08:00-18:00"),
        String::from("Sáb 08:00-12:00"),
    ];

    let loja: Loja = Loja::new(
        String::from("Padaria Central"),
        String::from("padaria_central"),
        String::from("contato@padaria.com"),
        String::from("11988887777"),
        String::from("hash_da_senha"),
        String::from("slug"),
        horarios_de_funcionamento,
    );

    loja_repo.criar(loja.clone()).unwrap();

    println!("loja: {:?}", loja);

    let adicional_bacon: Adicional = Adicional::new(
        String::from("Bacon"),
        loja.uuid,
        String::from("Bacon crocante"),
        5.0,
    );

    let adicional_cheddar: Adicional = Adicional::new(
        String::from("Cheddar"),
        loja.uuid,
        String::from("Cheddar amarelo"),
        3.0,
    );

    adicional_repo.criar(adicional_bacon.clone()).unwrap();
    adicional_repo.criar(adicional_cheddar.clone()).unwrap();

    println!("adicional: {:?}, {:?}", adicional_bacon, adicional_cheddar);

    let categoria_bebidas: CategoriaProdutos = CategoriaProdutos::new(
        String::from("Bebidas"),
        String::from("Produtos da categoria de bebidas"),
        loja.uuid,
    );

    let categoria_pizzas: CategoriaProdutos = CategoriaProdutos::new(
        String::from("Pizzas"),
        String::from("Produtos da categoria de pizzas"),
        loja.uuid,
    );

    let categoria_hamburgueres: CategoriaProdutos = CategoriaProdutos::new(
        String::from("Hambúrgueres"),
        String::from("Produtos da categoria de hamburgueres"),
        loja.uuid,
    );

    categorias_de_produtos_repo.criar(categoria_bebidas.clone()).unwrap();
    categorias_de_produtos_repo.criar(categoria_pizzas.clone()).unwrap();
    categorias_de_produtos_repo.criar(categoria_hamburgueres.clone()).unwrap();

    let produto_pizza = Produto::new(
        "Pizza Calabresa".into(),
        "Pizza de calabresa com cebola".into(),
        39.90,
        categoria_pizzas.uuid,
        loja.uuid,
    );

    let produto_coca_cola = Produto::new(
        "Coca-Cola Lata".into(),
        "Refrigerante 350ml".into(),
        6.5,
        categoria_pizzas.uuid,
        loja.uuid,
    );

    let produto_hamburger = Produto::new(
        "Hambúrguer artesanal".into(),
        "Pão brioche, carne 180g, queijo".into(),
        32.00,
        categoria_hamburgueres.uuid,
        loja.uuid,
    );

    produto_repo.criar(produto_pizza.clone()).unwrap();
    produto_repo.criar(produto_coca_cola.clone()).unwrap();
    produto_repo.criar(produto_hamburger.clone()).unwrap();

    println!("categoria: {:?}", categoria_bebidas);
    println!("categoria: {:?}", categoria_pizzas);
    
    println!("{:?}", produto_pizza);

    let cliente: Cliente = Cliente::new(
        usuario.uuid,
        loja.uuid,
    );

    println!("cliente: {:?}", cliente);

    let mut pedido_1: Pedido = Pedido::new(
        String::from("11999998888"),
        String::from("2026-02-08T14:30:00"),
        loja.uuid,
        8.50,
        String::from("Sem cebola, por favor"),
        usuario.uuid,
    );

    pedido_1.adicionar_item_pedido(
        &produto_coca_cola,
        2,
        String::from("Sem açúcar"),
        loja.uuid,
    );

    println!("pedido: {:?}", pedido_1);

    let ingrediente: Ingrediente = Ingrediente::new(
        String::from("Tomate"),
        String::from("Tomate fresco em rodelas"),
        loja.uuid,
        12.5,
    );

    println!("ingrediente: {:?}", ingrediente);

    let endereco_loja: EnderecoLoja = EnderecoLoja::new(
        "SP".into(),
        "São Paulo".into(),
        "Rua das Flores".into(),
        "100".into(),
        "Centro".into(),
        Some("01000-000".into()),
        Some("Apto 12".into()),
        loja.uuid,
    );

    println!("endereco_loja: {:?}", endereco_loja);

    let entregador: Entregador = Entregador::new(
        String::from("Carlos Lima"),
        String::from("11988887777"),
        String::from("Moto"),
        String::from("ABC1D23"),
        loja.uuid,
    );

    println!("entregador: {:?}", entregador);

    let funcionario: Funcionario = Funcionario::new(
        loja.uuid,
        String::from("Atendente"),
        String::from("Maria Silva"),
        String::from("maria"),
        String::from("maria@email.com"),
        String::from("11977776666"),
        String::from("hash_da_senha"),
    );

    funcionario_repo.criar(funcionario.clone()).unwrap();

    println!("funcionario: {:?}", funcionario);

    let mut pedido_2: Pedido = Pedido::new(
        "11999998888".into(),
        "2026-02-08T19:10:00".into(),
        loja.uuid,
        7.50,
        "Entregar na portaria".into(),
        usuario.uuid,
    );

    println!("{:?}", pedido_2);

    let item = pedido_2.adicionar_item_pedido(
        &produto_hamburger,
        2,
        "Sem cebola".into(),
        loja.uuid
    );

    println!("item criado: {:?}", item);

    let adicional_1_uuid = item
        .adicionar_adicional(&adicional_bacon)
        .expect("Não foi possível");

    let adicional_2_uuid = item
        .adicionar_adicional(&adicional_cheddar)
        .expect("Não foi possível");

    println!("UUID adicional 1: {:?}", adicional_1_uuid);
    println!("UUID adicional 2: {:?}", adicional_2_uuid);

    pedido_repo.criar(pedido_1.clone()).unwrap();
    pedido_repo.criar(pedido_2.clone()).unwrap();

    let avaliacao_loja: AvaliacaoDeLoja = AvaliacaoDeLoja::new(
        usuario.uuid,
        loja.uuid,
        4.5,
        String::from("Loja muito boa"),
    );

    let avaliacao_produto: AvaliacaoDeProduto = AvaliacaoDeProduto::new(
        usuario.uuid,
        loja.uuid,
        pedido_1.uuid,
        4.8,
        String::from("Produto excelente"),
    );

    avaliacoes_de_produto_repo.criar(avaliacao_produto.clone()).unwrap();
    avaliacoes_de_loja_repo.criar(avaliacao_loja.clone()).unwrap();

    println!("avaliacoes_de_loja: {:?}", avaliacao_loja);
    println!("avaliacoes_de_produto: {:?}", avaliacao_produto);

    // Cupom de boas-vindas - 20% off, primeira compra
    let cupom_bemvindo = Cupom::new(
        "BEMVINDO".to_string(),
        "20% de desconto na primeira compra".to_string(),
        TipoDesconto::Percentual(20.0),
        loja.uuid,
        CondicoesCupom {
            valor_minimo_pedido: Some(30.0),
            valor_maximo_desconto: Some(25.0),
            primeira_compra_apenas: true,
            categorias_validas: None,
            produtos_validos: None,
            dias_semana_validos: None,
            horario_inicio: None,
            horario_fim: None,
        },
        Utc::now(),
        Utc::now() + chrono::Duration::days(30),
        Some(100),  // 100 usos no total
        Some(1),    // 1 uso por usuário
    );

    // Cupom frete grátis final de semana
    let cupom_frete = Cupom::new(
        "FRETEGRATIS".to_string(),
        "Frete grátis aos finais de semana".to_string(),
        TipoDesconto::FreteGratis,
        loja.uuid,
        CondicoesCupom {
            valor_minimo_pedido: Some(50.0),
            valor_maximo_desconto: None,
            primeira_compra_apenas: false,
            categorias_validas: None,
            produtos_validos: None,
            dias_semana_validos: Some(vec![0, 6]), // Domingo e Sábado
            horario_inicio: None,
            horario_fim: None,
        },
        Utc::now(),
        Utc::now() + chrono::Duration::days(90),
        None,
        Some(4),  // 4 usos por usuário
    );

    // Promoção automática - Happy Hour
    let promo_happy_hour = Promocao::new(
        "Happy Hour".to_string(),
        "15% de desconto das 18h às 20h".to_string(),
        TipoDesconto::Percentual(15.0),
        loja.uuid,
        CondicoesCupom {
            valor_minimo_pedido: Some(25.0),
            valor_maximo_desconto: Some(20.0),
            primeira_compra_apenas: false,
            categorias_validas: None,
            produtos_validos: None,
            dias_semana_validos: Some(vec![1, 2, 3, 4, 5]), // Seg a Sex
            horario_inicio: Some("18:00".to_string()),
            horario_fim: Some("20:00".to_string()),
        },
        Utc::now(),
        Utc::now() + chrono::Duration::days(365),
        1,  // Prioridade 1
    );

    println!("{:?}", cupom_bemvindo);
    println!("{:?}", promo_happy_hour);
    println!("{:?}", cupom_frete);

    cupom_repo.criar(cupom_bemvindo).unwrap();
    cupom_repo.criar(cupom_frete).unwrap();


    // Buscar cupom por código
    if let Some(c) = cupom_repo.buscar_por_codigo("BEMVINDO") {
        println!("Cupom: {} - {}", c.codigo, c.descricao);
    }

    // Listar todos os cupons ativos da loja
    let cupons_ativos = cupom_repo.buscar_ativos(loja.uuid);
    println!("Cupons ativos: {}", cupons_ativos.len());

    for (n, i) in usuario_repo.listar_todos().iter().enumerate() {
        println!("\n{}:{:?}", n, i);
    };

    for (n, i) in loja_repo.listar_todos().iter().enumerate() {
        println!("\n{}:{:?}", n, i);
    };

    for (n, i) in produto_repo.listar_todos().iter().enumerate() {
        println!("\n{}:{:?}", n, i);
    };

    for (n, i) in pedido_repo.listar_todos().iter().enumerate() {
        println!("\n{}:{:?}", n, i);
    };

    for (n, i) in cupom_repo.listar_todos().iter().enumerate() {
        println!("\n{}:{:?}", n, i);
    };


}
