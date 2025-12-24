use sea_orm::{ActiveValue, EntityOrSelect, QuerySelect, entity::prelude::*};
use snafu::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "repository")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u64,

    // pub created_at: chrono::DateTime<chrono::Utc>,
    // pub modified_at: chrono::DateTime<chrono::Utc>,
    pub created_at: i64,
    pub modified_at: i64,
    pub deleted: u64,

    pub version: u64,
    #[sea_orm(column_name = "repository_name")]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// pub async fn create_repository(db: &sea_orm::DbConn, name: String) -> crate::result::Result<i64> {
//     let repo = ActiveModel {
//         id: ActiveValue::NotSet,
//         created_at: ActiveValue::Set(chrono::Utc::now()),
//         modified_at: ActiveValue::Set(chrono::Utc::now()),
//         deleted: ActiveValue::Set(0),
//         version: ActiveValue::Set(1),
//         name: ActiveValue::Set(name),
//     };

//     let result = Entity::insert(repo)
//         .exec(db)
//         .await
//         .context(crate::result::DBSnafu {
//             msg: "inert repository failed",
//         })?;

//     return Ok(result.last_insert_id);
// }

// pub async fn query_repo_version_with_lock(
//     db: &sea_orm::DbConn,
//     repo_id: i64,
// ) -> crate::result::Result<i64> {
//     let repo = Entity::find_by_id(repo_id)
//         .column(Column::Version)
//         .one(db)
//         .await
//         .context(crate::result::DBSnafu {
//             msg: "inert repository failed",
//         })?;

//     return match repo {
//         Some(repo) => crate::result::Result::Ok(repo.version),
//         None => crate::result::Result::Ok(1),
//     };
// }
