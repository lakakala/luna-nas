use sea_orm::{
    ActiveValue, Condition, entity::prelude::*, sea_query::ExprTrait, sqlx::types::chrono,
};
use snafu::prelude::*;
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u64,
    // pub created_at: chrono::DateTime<chrono::Utc>,
    // pub modified_at: chrono::DateTime<chrono::Utc>,
    pub created_at: i64,
    pub modified_at: i64,
    #[sea_orm(default_value = "0")]
    pub deleted: u64,

    pub repository_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// pub struct CreateItemRequest {
//     file_name: String,
// }

// pub async fn create_item( db: &sea_orm::DbConn, )

// pub async fn create_items(
//     db: &sea_orm::DbConn,
//     repo_id: i64,
//     version_id: i64,
//     parent_id: Option<i64>,
//     create_items: Vec<CreateItemRequest>,
// ) -> crate::result::Result<Vec<i64>> {
//     let mut models = Vec::with_capacity(create_items.len());

//     for create_item_req in &create_items {
//         models.push(ActiveModel {
//             id: ActiveValue::NotSet,
//             created_at: ActiveValue::NotSet,
//             modified_at: ActiveValue::NotSet,
//             deleted: ActiveValue::NotSet,
//             repository_id: ActiveValue::Set(repo_id),
//             parent_id: match parent_id {
//                 Some(v) => ActiveValue::Set(v),
//                 None => ActiveValue::NotSet,
//             },
//             version: ActiveValue::Set(version_id),
//         });
//     }

//     return Entity::insert_many(models)
//         .exec_with_returning_keys(db)
//         .await
//         .context(crate::result::DBSnafu {
//             msg: "inert items failed",
//         });
// }

// pub async fn query_item_by_id(
//     db: &sea_orm::DbConn,
//     repo_id: i64,
//     item_id: i64,
// ) -> crate::result::Result<Model> {
//     let item = Entity::find()
//         .filter(
//             Condition::all()
//                 .add(Column::RepositoryId.eq(repo_id))
//                 .and(Column::Id.eq(item_id).and(Column::Deleted.eq(0))),
//         )
//         .one(db)
//         .await
//         .context(crate::result::DBSnafu {
//             msg: "inert items failed",
//         })?;

//     todo!()
// }
