mod repo;
mod routes;

use crate::{result::Result, web::routes::routes};

pub async fn start(app_ctx: crate::AppContext) -> Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes(app_ctx)).await.unwrap();

    return Ok(());
}
