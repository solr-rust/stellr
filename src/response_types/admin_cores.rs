//! Utility Structs for deserialising solr responses
//!
//! This module contains a set of structs that you can use to deserialise common solr responses.
//!
//! As the SolrResponseHeader is frequently reused, this is split out into it's own struct, and
//! composed into other types (eg. in SolrSelectType and SolrUpdateType).

use super::SolrResponseHeader;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Debug, Display};

/// Standard structure for an update response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolrAdminCoresType {
    pub response_header: SolrResponseHeader,
    pub debug: Option<String>,
    pub init_failures: HashMap<String, String>,
    pub status: HashMap<String, SolrAdminCoresDetails>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolrAdminCoresDetails {
    name: String,
    instance_dir: String,
    data_dir: String,
    config: String,
    schema: String,
    start_time: DateTime<Utc>,
    uptime: usize,
    last_published: String,
    config_version: usize,
    cloud: SolrAdminCoresCloudInfo,
    index: SolrAdminCoresIndexInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolrAdminCoresCloudInfo {
    collection: String,
    shard: String,
    replica: String,
    replica_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolrAdminCoresIndexInfo {
    num_docs: usize,
    max_doc: usize,
    deleted_docs: usize,
    index_heap_usage_bytes: usize,
    version: usize,
    segment_count: usize,
    current: bool,
    has_deletions: bool,
    directory: String,
    segments_file: String,
    segments_file_size_in_bytes: usize,
    user_data: SolrAdminCoresUserData,
    last_modified: DateTime<Utc>,
    size_in_bytes: usize,
    size: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolrAdminCoresUserData {
    commit_command_ver: String,
    #[serde(rename = "commitTimeMSec")]
    commit_time_msec: String,
}

impl fmt::Display for SolrAdminCoresType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn core_admin_deserialize() {
        let test_data = "tests/sample_data/core_admin.json";
        let test_content =
            std::fs::read_to_string(test_data).expect("Missing core_admin test data");
        let result_struct = serde_json::from_str::<SolrAdminCoresType>(&test_content)
            .expect("Failed to deserialize core admin test data");
        eprintln!(">>> {:#?}", result_struct);
        assert_eq!(
            result_struct.status["techproducts_shard2_replica_n6"].index.segment_count,
            1,
        );
        assert_eq!(
            result_struct.status["techproducts_shard2_replica_n6"].index.user_data.commit_time_msec,
            "1611511283890",
        );
    }
}
