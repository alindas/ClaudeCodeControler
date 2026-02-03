use crate::models::HookEvent;
use axum::{
    routing::post,
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

pub struct HookServer {
    port: u16,
    events: Arc<RwLock<Vec<HookEvent>>>,
}

impl HookServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn start(&self) {
        let events = self.events.clone();

        let app = Router::new()
            .route("/hook", post(handle_hook))
            .route("/health", post(|| async { "OK" }))
            .layer(CorsLayer::permissive())
            .with_state(events);

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        axum::serve(listener, app).await.unwrap();
    }

    pub async fn get_events(&self) -> Vec<HookEvent> {
        self.events.read().await.clone()
    }

    pub async fn clear_events(&self) {
        self.events.write().await.clear();
    }
}

async fn handle_hook(
    axum::extract::State(events): axum::extract::State<Arc<RwLock<Vec<HookEvent>>>>,
    Json(event): Json<HookEvent>,
) -> &'static str {
    events.write().await.push(event);
    "OK"
}
