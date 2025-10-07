use std::sync::Arc;
use tokio::sync::RwLock;
use wallmgr_core::{Config, Database};
use wallmgr_adapters::Adapter;
use wallmgr_renderers::Renderer;
use wallmgr_connectors::danbooru::DanbooruConnector;
use wallmgr_connectors::yandere::YandereConnector;
use wallmgr_connectors::safebooru::SafebooruConnector;
use wallmgr_connectors::gelbooru::GelbooruConnector;
use crate::websocket::WsChannel;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub database: Arc<RwLock<Database>>,
    pub adapter: Arc<RwLock<Option<Adapter>>>,
    pub renderer: Arc<RwLock<Option<Renderer>>>,
    pub booru_clients: Arc<BooruClients>,
    pub ws_channel: Arc<WsChannel>,
}

pub struct BooruClients {
    pub danbooru: DanbooruConnector,
    pub yandere: YandereConnector,
    pub safebooru: SafebooruConnector,
    pub gelbooru: GelbooruConnector,
}

impl AppState {
    pub fn new(config: Config, database: Database) -> Self {
        Self {
            config: Arc::new(config),
            database: Arc::new(RwLock::new(database)),
            adapter: Arc::new(RwLock::new(None)),
            renderer: Arc::new(RwLock::new(None)),
            booru_clients: Arc::new(BooruClients {
                danbooru: DanbooruConnector::new(),
                yandere: YandereConnector::new(),
                safebooru: SafebooruConnector::new(),
                gelbooru: GelbooruConnector::new(),
            }),
            ws_channel: Arc::new(WsChannel::new()),
        }
    }
}
