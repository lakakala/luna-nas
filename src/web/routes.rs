use super::repo;
use axum::routing::{get, post};

#[derive(Clone)]
pub struct AppState {
    app_ctx: crate::AppContext,
}

impl AppState {
    pub fn get_app_ctx(&self) -> crate::AppContext {
        return self.app_ctx.clone();
    }
}

pub fn routes(app_ctx: crate::AppContext) -> axum::Router {
    return axum::Router::new()
        // .route("/repos", get(super::repo::get_repos))
        .route("/get_repo_info", post(repo::get_repoinfo))
        .route("/create_item", post(repo::create_item))
        .with_state(AppState { app_ctx });
}
