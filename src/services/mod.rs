use std::collections::HashMap;
use std::rc::Rc;

use crate::db::item_version::ContentType;
use crate::db::{item, item_version, repository};
use crate::result::ErrorV2;
use crate::result::Result;
use crate::vo;
use log::info;
use sea_orm::{
    ActiveValue, Condition, entity::prelude::*, sea_query::ExprTrait, sqlx::types::chrono,
};
use sea_orm::{EntityOrSelect, QuerySelect};
use sea_orm::{EntityTrait, QueryFilter, TransactionTrait};
use snafu::prelude::*;

pub struct RepoService {
    db: sea_orm::DbConn,
}

impl RepoService {
    pub fn new_repo_service(db: sea_orm::DbConn) -> RepoService {
        return RepoService { db };
    }

    // pub async fn create_repository(&self, name: String) -> crate::result::Result<i64> {
    //     let repo_id = crate::db::repository::create_repository(&self.db, name).await?;

    //     return Ok(repo_id);
    // }

    pub async fn repository_info(&self, repository_id: u64) -> Result<vo::RepoInfoVO> {
        let txn = self.db.begin().await.map_err(|err| ErrorV2::DBError {
            source: err,
            msg: format!("start transaction failed"),
        })?;

        let db_repository = repository::Entity::find()
            .filter(Condition::all().add(repository::Column::Id.eq(repository_id)))
            .lock_shared()
            .one(&txn)
            .await
            .map_err(|err| ErrorV2::DBError {
                source: err,
                msg: format!("start transaction failed"),
            })?;
        if db_repository.is_none() {
            return Result::Err(ErrorV2::ParamError(format!(
                "repostory_id {} can not found",
                repository_id
            )));
        }

        let mut latest_id: u64 = 0;
        let page_size = 0;

        let mut item_map = HashMap::new();
        let mut item_childs_map: HashMap<u64, Vec<_>> = HashMap::new();
        loop {
            let db_item_versions = item_version::Entity::find()
                .filter(
                    Condition::all().add(
                        item_version::Column::RepositoryId
                            .eq(repository_id)
                            .add(item_version::Column::Deleted.eq(0))
                            .add(item_version::Column::LatestVersion.eq(true))
                            .add(item_version::Column::Id.gt(latest_id)),
                    ),
                )
                .limit(100)
                .all(&txn)
                .await
                .map_err(|err| ErrorV2::DBError {
                    source: err,
                    msg: format!("start transaction failed"),
                })?;

            let db_item_version_len = db_item_versions.len();
            if db_item_version_len == 0 {
                break;
            }
            latest_id = db_item_versions.last().unwrap().id;

            for item_version in db_item_versions {
                let item = vo::ItemVO::builder()
                    .id(item_version.item_id)
                    .file_name(item_version.file_name.clone())
                    .maybe_parent_id(item_version.parent_id)
                    .item_version(
                        vo::ItemVersionVO::builder()
                            .content_version(item_version.content_version)
                            .meta_version(item_version.meta_version)
                            .build(),
                    )
                    .maybe_attrs(Option::Some(
                        vo::ItemAttrsVO::builder()
                            .capabilities(item_version.capabilities)
                            .build(),
                    ))
                    .childs(Vec::new())
                    .build();

                let parent_id = match item_version.parent_id {
                    Some(parent_id) => parent_id,
                    None => 0,
                };

                item_childs_map.entry(parent_id).or_default().push(item.id);

                item_map.insert(item.id, item);
                // if !item_childs_map.contains_key(&parent_id) {
                //     let childs = Vec::new();
                //     item_childs_map.insert(parent_id, childs);
                // }

                // let childs = item_childs_map.get_mut(&parent_id).unwrap();

                // childs.push(item);
            }

            if db_item_version_len != page_size {
                break;
            }
        }

        fn nest(
            item_id: u64,
            item_map: &mut HashMap<u64, vo::ItemVO>,
            item_childs_map: &HashMap<u64, Vec<u64>>,
        ) -> Vec<vo::ItemVO> {
            item_childs_map
                .get(&item_id)
                .map(|child_ids| {
                    child_ids
                        .iter()
                        .filter_map(|child_id| {
                            let mut item = item_map.remove(child_id)?;
                            item.childs = nest(item.id, item_map, item_childs_map);
                            Some(item)
                        })
                        .collect()
                })
                .unwrap_or_default()
        }

        let tree_item_list = nest(0, &mut item_map, &item_childs_map);

        return Result::Ok(
            vo::RepoInfoVO::builder()
                .repo_id(repository_id)
                .items(tree_item_list)
                .build(),
        );
    }

    pub async fn create_item(
        &self,
        create_item: vo::CreateItemVO,
    ) -> crate::result::Result<vo::ItemVO> {
        let content_type = item_version::ContentType::try_from(create_item.get_content_type())?;

        let file_id = create_item.get_file_id();

        if content_type == ContentType::File {
            if file_id.is_none() {
                return Result::Err(ErrorV2::ParamError(format!(
                    "content_type = file file_id must not empty"
                )));
            }
        }

        let txn = self.db.begin().await.map_err(|err| ErrorV2::DBError {
            source: err,
            msg: format!("start transaction failed"),
        })?;

        let repository = repository::Entity::find()
            .filter(
                Condition::all()
                    .add(repository::Column::Id.eq(create_item.get_repostory_id()))
                    .add(repository::Column::Deleted.eq(0)),
            )
            .columns([repository::Column::Id, repository::Column::Version])
            .lock_exclusive()
            .one(&txn)
            .await
            .map_err(|err| ErrorV2::DBError {
                source: err,
                msg: format!("start transaction failed"),
            })?;

        let repo_version = if let Some(repo) = repository {
            repo.version
        } else {
            return Result::Err(ErrorV2::ParamError(format!(
                "repostory_id {} can not found",
                create_item.get_repostory_id()
            )));
        };

        if create_item.get_parent_id().is_some() {
            let parent_id = create_item.get_parent_id().unwrap();

            let db_parent_item_version = item_version::Entity::find()
                .filter(
                    Condition::all()
                        .add(item_version::Column::Id.eq(parent_id))
                        .add(item_version::Column::LatestVersion.eq(true)),
                )
                .lock_exclusive()
                .column(item_version::Column::Id)
                .one(&txn)
                .await
                .map_err(|err| ErrorV2::DBError {
                    source: err,
                    msg: format!("start transaction failed"),
                })?;

            if db_parent_item_version.is_none() {
                return Result::Err(ErrorV2::ParamError(format!(
                    "parent_id {} can not found",
                    parent_id
                )));
            }
        }

        let condition = Condition::all()
            .add(item_version::Column::FileName.eq(create_item.get_file_name()))
            .add(item_version::Column::Deleted.eq(0))
            .add(item_version::Column::LatestVersion.eq(true))
            .add(match create_item.get_parent_id() {
                Some(parent_id) => item_version::Column::ParentId.eq(parent_id),
                None => item_version::Column::ParentId.is_null(),
            });

        let db_item_version = item_version::Entity::find()
            .filter(condition)
            .lock_exclusive()
            .column(item_version::Column::Id)
            .one(&txn)
            .await
            .map_err(|err| ErrorV2::DBError {
                source: err,
                msg: format!("start transaction failed"),
            })?;

        if db_item_version.is_some() {
            return Result::Err(ErrorV2::ParamError(format!(
                "file_name {} alread exist",
                create_item.get_file_name(),
            )));
        }

        let item_insert_result = item::Entity::insert(item::ActiveModel {
            id: ActiveValue::NotSet,
            created_at: ActiveValue::Set(chrono::Utc::now().timestamp_millis()),
            modified_at: ActiveValue::Set(chrono::Utc::now().timestamp_millis()),
            deleted: ActiveValue::Set(0),
            repository_id: ActiveValue::Set(create_item.get_repostory_id()),
        })
        .exec(&txn)
        .await
        .map_err(|err| ErrorV2::DBError {
            source: err,
            msg: format!("start transaction failed"),
        })?;

        let item_attrs = create_item.get_item_attrs();

        let item_version_insert_result = item_version::Entity::insert(item_version::ActiveModel {
            id: ActiveValue::NotSet,
            created_at: ActiveValue::Set(chrono::Utc::now().timestamp_millis()),
            modified_at: ActiveValue::Set(chrono::Utc::now().timestamp_millis()),
            deleted: ActiveValue::Set(0),
            repository_id: ActiveValue::Set(create_item.get_repostory_id()),
            item_id: ActiveValue::Set(item_insert_result.last_insert_id),
            parent_id: ActiveValue::set(create_item.get_parent_id()),
            latest_version: ActiveValue::Set(true),
            content_type: ActiveValue::Set(content_type),
            file_name: ActiveValue::Set(create_item.get_file_name()),
            file_id: ActiveValue::Set(create_item.get_file_id()),
            meta_version: ActiveValue::Set(repo_version),
            content_version: ActiveValue::Set(repo_version),
            modification_date: ActiveValue::Set(item_attrs.get_modification_date()),
            creation_date: ActiveValue::Set(item_attrs.get_creation_date()),
            last_use_date: ActiveValue::Set(item_attrs.get_last_use_date()),
            // extended_attrbutes: ActiveValue::NotSet,
            // file_system_flags: ActiveValue::NotSet,
            // tag_data: ActiveValue::NotSet,
            // favorite_range: ActiveValue::NotSet,
            // type_and_creator: ActiveValue::NotSet,
            capabilities: ActiveValue::Set(item_attrs.get_capabilities()),
        })
        .exec(&txn)
        .await
        .map_err(|err| ErrorV2::DBError {
            source: err,
            msg: format!("start transaction failed"),
        })?;

        txn.commit().await.map_err(|err| ErrorV2::DBError {
            source: err,
            msg: format!("start transaction failed"),
        })?;

        info!(
            "create item success item_id {} item_version_id {}",
            item_insert_result.last_insert_id, item_version_insert_result.last_insert_id
        );

        return Result::Ok(
            vo::ItemVO::builder()
                .id(item_insert_result.last_insert_id)
                .maybe_parent_id(create_item.get_parent_id())
                .maybe_file_id(create_item.get_file_id())
                .file_name(create_item.get_file_name())
                .item_version(
                    vo::ItemVersionVO::builder()
                        .content_version(repo_version)
                        .meta_version(repo_version)
                        .build(),
                )
                .attrs(
                    vo::ItemAttrsVO::builder()
                        .capabilities(item_attrs.get_capabilities())
                        .maybe_creation_date(item_attrs.get_creation_date())
                        .maybe_modification_date(item_attrs.get_modification_date())
                        .maybe_last_use_date(item_attrs.get_last_use_date())
                        .build(),
                )
                .childs(Vec::new())
                .build(),
        );
    }

    // pub async fn add_file(
    //     &self,
    //     repo_id: i64,
    //     version: i64,
    //     parent_id: i64,
    // ) -> crate::result::Result<i64> {
    //     let repo_curr_version =
    //         crate::db::repository::query_repo_version_with_lock(&self.db, repo_id).await?;

    //     if version != repo_curr_version {
    //         return Result::Err(crate::result::Error::VersionConflict { msg: format!("") });
    //     }

    //     let parent_item = crate::db::item::query_item_by_id(&self.db, repo_id, parent_id).await?;

    //     let parent_item_version =
    //         crate::db::item_version::query_latest_item_version(&self.db, repo_id, parent_id)
    //             .await?;

    //     if parent_item_version.item_type != crate::db::item_version::ItemType::Dict {
    //         return Result::Err(crate::result::Error::VersionConflict { msg: format!("") });
    //     }

    //     // crate::db::item::create_items(db, repo_id, version_id, parent_id, create_items)
    //     todo!()
    // }
}

fn convert_u64_to_datetime(i64_date_time: Option<i64>) -> Option<chrono::DateTime<chrono::Utc>> {
    match i64_date_time {
        Some(i64_date_time) => chrono::DateTime::from_timestamp_secs(i64_date_time),
        None => None,
    }
}
