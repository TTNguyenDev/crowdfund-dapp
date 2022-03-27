use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{ U128};
use near_sdk::{env, near_bindgen, setup_alloc, Balance, CryptoHash, Promise, PanicOnDefault, Timestamp};
use near_sdk::collections::{UnorderedSet, UnorderedMap, LookupMap};
use near_sdk::AccountId;

setup_alloc!();

use crate::utils::*;
pub use crate::metadata::*;
pub use crate::internal::*;
pub use crate::enumeration::*;
pub use crate::core::*;
pub use crate::events::*;

mod utils;
mod metadata;
mod internal;
mod enumeration;
mod core;
mod events;

pub type ProjectId = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    // mapping user ID với danh sách dự án của người này
    pub projects_per_owner: LookupMap<AccountId, UnorderedSet<ProjectId>>,
    // mapping project ID với project struct tương ứng (có thể lưu metadata của project struct off-chain)
    pub projects_by_id: LookupMap<ProjectId, Project>,
    // mapping project ID với project metadata
    pub project_metadata_by_id: UnorderedMap<ProjectId, ProjectMetadata>,
    // mapping project ID với danh sách supporters
    pub supporters_per_project: LookupMap<ProjectId, UnorderedSet<AccountId>>,
    // mapping project ID với danh sách voters
    pub voters_per_project: LookupMap<ProjectId, UnorderedSet<AccountId>>
}

#[near_bindgen]
impl Contract {
    #[init]

    pub fn new() -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            projects_per_owner: LookupMap::new(b"a".to_vec()),
            projects_by_id: LookupMap::new(b"b".to_vec()),
            project_metadata_by_id: UnorderedMap::new(b"c".to_vec()),
            supporters_per_project: LookupMap::new(b"d".to_vec()),
            voters_per_project: LookupMap::new(b"e".to_vec())
        }
    }
}
