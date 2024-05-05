/*
 * lakeFS API
 *
 * lakeFS HTTP API
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: services@treeverse.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryCreation {
    #[serde(rename = "name")]
    pub name: String,
    /// Filesystem URI to store the underlying data in (e.g. \"s3://my-bucket/some/path/\")
    #[serde(rename = "storage_namespace")]
    pub storage_namespace: String,
    #[serde(rename = "default_branch", skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(rename = "sample_data", skip_serializing_if = "Option::is_none")]
    pub sample_data: Option<bool>,
    #[serde(rename = "read_only", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

impl RepositoryCreation {
    pub fn new(name: String, storage_namespace: String) -> RepositoryCreation {
        RepositoryCreation {
            name,
            storage_namespace,
            default_branch: None,
            sample_data: None,
            read_only: None,
        }
    }
}

