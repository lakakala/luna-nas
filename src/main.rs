use std::sync::Arc;

use snafu::ResultExt;

mod db;
mod prelude;
mod result;
mod services;
mod utils;
mod vo;
mod web;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    println!("Hello, world!");

    let app_ctx = AppContext::new().await.unwrap();

    web::start(app_ctx).await;
}

#[derive(Clone)]
struct AppContext {
    inner: Arc<InnerAppContext>,
}

struct InnerAppContext {
    db: sea_orm::DbConn,
    repo_service: Arc<services::RepoService>,
}

impl AppContext {
    pub async fn new() -> result::Result<AppContext> {
        let db = sea_orm::Database::connect(
            // "sqlite:///home/kexin/projects/luna-nas/data/nas.db?mode=rwc",
            "mysql://root:mysql@localhost:3306/luna_nas",
        )
        .await
        .map_err(|err| result::ErrorV2::DBError {
            source: err,
            msg: format!("connect to db failed"),
        })?;

        return Ok(AppContext {
            inner: Arc::new(InnerAppContext {
                db: db.clone(),
                repo_service: Arc::new(services::RepoService::new_repo_service(db)),
            }),
        });
    }

    pub fn get_repo_service(&self) -> Arc<services::RepoService> {
        return self.inner.repo_service.clone();
    }
}
