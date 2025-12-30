use bon::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct RepoVO {
    repo_id: u64,
    name: String,
}

#[derive(Clone, Serialize, Deserialize, Builder)]
pub struct ItemVersionVO {
    content_version: u64,
    meta_version: u64,
}

#[derive(Clone, Serialize, Deserialize, Builder)]
pub struct ItemVO {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub file_name: String,
    pub file_id: Option<u64>,
    pub item_version: ItemVersionVO,
    pub attrs: Option<ItemAttrsVO>,
    pub childs: Vec<ItemVO>,
}

#[derive(Clone, Serialize, Deserialize, Builder)]
pub struct RepoInfoVO {
    repo_id: u64,
    items: Vec<ItemVO>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModifyItemVO {
    id: u64,
    parent_id: Option<u64>,
    file_name: Option<u64>,
    file_id: Option<u64>,
    item_version: Option<ItemVersionVO>,
    attrs: Option<ModifyItemAttrsVO>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ModifyItemAttrsVO {
    capabilities: u64,
    modification_date: Option<u64>,
    creation_date: Option<u64>,
    last_use_date: Option<u64>,
    extended_attrbutes: Option<HashMap<String, Vec<u8>>>,
    file_system_flags: Option<u64>,
    tag_data: String,
    favorite_range: String,
    type_and_creator: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeleteItemVO {
    id: u64,
    base_version: ItemVersionVO,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateItemVO {
    repostory_id: u64,
    parent_id: Option<u64>,
    file_name: String,
    file_id: Option<u64>,
    content_type: u16,
    attrs: ItemAttrsVO,
}

impl CreateItemVO {
    pub fn get_repostory_id(&self) -> u64 {
        return self.repostory_id;
    }

    pub fn get_parent_id(&self) -> Option<u64> {
        return self.parent_id;
    }

    pub fn get_file_name(&self) -> String {
        return self.file_name.clone();
    }

    pub fn get_content_type(&self) -> u16 {
        return self.content_type;
    }

    pub fn get_file_id(&self) -> Option<u64> {
        return self.file_id;
    }

    pub fn get_item_attrs(&self) -> &ItemAttrsVO {
        return &self.attrs;
    }
}

#[derive(Clone, Serialize, Deserialize, Builder)]
pub struct ItemAttrsVO {
    capabilities: u64,
    modification_date: Option<i64>,
    creation_date: Option<i64>,
    last_use_date: Option<i64>,
    extended_attrbutes: Option<HashMap<String, Vec<u8>>>,
    file_system_flags: Option<u64>,
    tag_data: Option<Vec<u8>>,
    favorite_range: Option<u64>,
    type_and_creator: Option<Vec<u8>>,
}

impl ItemAttrsVO {
    pub fn get_capabilities(&self) -> u64 {
        return self.capabilities;
    }

    pub fn get_modification_date(&self) -> Option<i64> {
        return self.modification_date;
    }

    pub fn get_creation_date(&self) -> Option<i64> {
        return self.creation_date;
    }

    pub fn get_last_use_date(&self) -> Option<i64> {
        return self.last_use_date;
    }

    pub fn get_extended_attrbutes(&self) -> Option<&HashMap<String, Vec<u8>>> {
        return self.extended_attrbutes.as_ref();
    }

    pub fn get_file_system_flags(&self) -> Option<u64> {
        return self.file_system_flags;
    }

    pub fn get_tag_data(&self) -> Option<&Vec<u8>> {
        return self.tag_data.as_ref();
    }

    pub fn get_favorite_range(&self) -> Option<u64> {
        return self.favorite_range;
    }

    pub fn get_type_and_creator(&self) -> Option<&Vec<u8>> {
        return self.type_and_creator.as_ref();
    }
}
