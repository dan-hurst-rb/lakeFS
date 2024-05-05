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
pub struct StatsEvent {
    /// stats event class (e.g. \"s3_gateway\", \"openapi_request\", \"experimental-feature\", \"ui-event\")
    #[serde(rename = "class")]
    pub class: String,
    /// stats event name (e.g. \"put_object\", \"create_repository\", \"<experimental-feature-name>\")
    #[serde(rename = "name")]
    pub name: String,
    /// number of events of the class and name
    #[serde(rename = "count")]
    pub count: i32,
}

impl StatsEvent {
    pub fn new(class: String, name: String, count: i32) -> StatsEvent {
        StatsEvent {
            class,
            name,
            count,
        }
    }
}

