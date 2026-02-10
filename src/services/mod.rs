pub mod usuario_service;
pub mod loja_service;
pub mod pedido_service;
pub mod produto_service;
pub mod pagamento_service;
pub mod cupom_service;
pub mod avaliacao_service;
pub mod entrega_service;

pub use usuario_service::UsuarioService;
pub use loja_service::LojaService;
pub use pedido_service::PedidoService;
pub use produto_service::ProdutoService;
pub use pagamento_service::PagamentoService;
pub use cupom_service::CupomService;
pub use avaliacao_service::AvaliacaoService;
pub use entrega_service::EntregaService;