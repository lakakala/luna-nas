use crate::result::{Resp, Result};
use crate::{services, vo};
use axum::extract::{Json, Path, State};

#[axum::debug_handler]
pub(super) async fn get_repoinfo(Path(repo_id): Path<String>) -> Resp<vo::RepoInfoVO> {
    todo!()
    // return Resp::from(Result::Ok(vo::RepoInfoVO { repo_id: 1 }));
}

#[axum::debug_handler]
pub(super) async fn create_item(
    State(app_state): State<super::routes::AppState>,
    Json(create_item_req): Json<vo::CreateItemVO>,
) -> Resp<vo::ItemVO> {
    let app_ctx = app_state.get_app_ctx();

    return Resp::from(
        app_ctx
            .get_repo_service()
            .create_item(create_item_req)
            .await,
    );
}

#[axum::debug_handler]
pub(super) async fn modify_item(Json(modify_item): Json<vo::ModifyItemVO>) -> Resp<()> {
    // return Resp::from(Result::Ok(RepoInfoVO { repo_id: 1 }));
    todo!()
}

#[axum::debug_handler]
pub(super) async fn delete_item(Json(delete_item): Json<vo::DeleteItemVO>) -> Resp<()> {
    // return Resp::from(Result::Ok(RepoInfoVO { repo_id: 1 }));
    todo!()
}
