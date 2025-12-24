use std::collections::HashMap;

use sea_orm::{FromJsonQueryResult, entity::prelude::*, sqlx::types::chrono};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "item_version")]
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
    pub item_id: u64,
    pub parent_id: Option<u64>,

    pub latest_version: bool,
    pub meta_version: u64,
    pub content_version: u64,

    pub content_type: ContentType,
    #[sea_orm(column_type = "String(StringLen::N(100))")]
    pub file_name: String,
    pub file_id: Option<u64>,

    pub capabilities: u64,
    pub modification_date: Option<i64>,
    pub creation_date: Option<i64>,
    pub last_use_date: Option<i64>,
    // pub extended_attrbutes: Option<ExtendedAttrbutes>,
    // pub file_system_flags: Option<u64>,
    // pub tag_data: Option<Vec<u8>>,
    // pub favorite_range: Option<u64>,
    // pub type_and_creator: Option<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ExtendedAttrbutes(HashMap<String, Vec<u8>>);

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum ContentType {
    #[sea_orm(num_value = 1)]
    Dict,
    #[sea_orm(num_value = 2)]
    File,
}

impl TryFrom<u16> for ContentType {
    type Error = crate::result::ErrorV2;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Result::Ok(Self::Dict),
            2 => Result::Ok(Self::File),
            _ => Result::Err(crate::result::ErrorV2::ParamError(format!(
                "unknwn content_type {}",
                value
            ))),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// pub struct CreateItemVersionRequest {
//     item_type: ItemType,
//     file_name: String,
//     file_id: Option<i64>,
// }

// async fn create_item_versions(
//     db: &sea_orm::DbConn,
//     repo_id: i64,
//     version_id: i64,
// ) -> crate::result::Result<Vec<i64>> {
//     todo!()
// }

// pub async fn create_item_version(
//     db: &sea_orm::DbConn,
//     repo_id: i64,
//     item_id: i64,
//     version_id: i64,
//     create_item: CreateItemVersionRequest,
// ) -> crate::result::Result<i64> {
//     todo!()
// }

// pub async fn query_latest_item_version(
//     db: &sea_orm::DbConn,
//     repo_id: i64,
//     item_id: i64,
// ) -> crate::result::Result<Model> {
//     let latest_item_version = Entity::find()
//         .filter(
//             Condition::all().add(Column::RepositoryId.eq(repo_id)).and(
//                 Column::ItemId
//                     .eq(item_id)
//                     .and(Column::LatestVersion.eq(true).add(Column::Deleted.eq(0))),
//             ),
//         )
//         .one(db)
//         .await
//         .context(crate::result::DBSnafu {
//             msg: "inert items failed",
//         })?;
//     todo!()
// }
