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
pub struct PullRequest {
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "creation_date")]
    pub creation_date: String,
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "source_branch")]
    pub source_branch: String,
    #[serde(rename = "destination_branch")]
    pub destination_branch: String,
    /// the commit id of merged PRs
    #[serde(rename = "merged_commit_id", skip_serializing_if = "Option::is_none")]
    pub merged_commit_id: Option<String>,
}

impl PullRequest {
    pub fn new(status: Status, title: String, description: String, id: String, creation_date: String, author: String, source_branch: String, destination_branch: String) -> PullRequest {
        PullRequest {
            status,
            title,
            description,
            id,
            creation_date,
            author,
            source_branch,
            destination_branch,
            merged_commit_id: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "merged")]
    Merged,
}

impl Default for Status {
    fn default() -> Status {
        Self::Open
    }
}

