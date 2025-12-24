use sea_orm::{ActiveValue, entity::prelude::*, sqlx::types::chrono};
use snafu::ResultExt;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "file_content")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    #[sea_orm(default_value = "0")]
    pub deleted: u64,

    #[sea_orm(column_type = "String(StringLen::N(256))")]
    pub file_sum: String,
    pub file_size: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// struct CreateFileContentRequest {
//     file_sum: String,
//     file_size: i64,
// }

// async fn create_file_contents(
//     db: &sea_orm::DbConn,
//     create_file_contents: Vec<CreateFileContentRequest>,
// ) -> crate::result::Result<Vec<i64>> {
//     let mut models = Vec::with_capacity(create_file_contents.len());

//     for req in &create_file_contents {
//         models.push(ActiveModel {
//             id: ActiveValue::NotSet,
//             created_at: ActiveValue::Set(chrono::Utc::now()),
//             modified_at: ActiveValue::Set(chrono::Utc::now()),
//             deleted: ActiveValue::Set(0),
//             file_sum: ActiveValue::Set(req.file_sum.clone()),
//             file_size: ActiveValue::Set(req.file_size),
//         });
//     }

//     return Entity::insert_many(models)
//         .exec_with_returning_keys(db)
//         .await
//         .context(crate::result::DBSnafu {
//             msg: "batch inert file_content failed",
//         });
// }
