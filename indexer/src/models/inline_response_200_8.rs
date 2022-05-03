/*
 * Indexer
 *
 * Algorand ledger analytics API.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse2008 {
    #[serde(rename = "application", skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<crate::models::Application>>,
    /// Round at which the results were computed.
    #[serde(rename = "current-round")]
    pub current_round: i32,
}

impl InlineResponse2008 {
    pub fn new(current_round: i32) -> InlineResponse2008 {
        InlineResponse2008 {
            application: None,
            current_round,
        }
    }
}


