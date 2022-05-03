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
pub struct InlineResponse200 {
    #[serde(rename = "accounts")]
    pub accounts: Vec<crate::models::Account>,
    /// Round at which the results were computed.
    #[serde(rename = "current-round")]
    pub current_round: i32,
    /// Used for pagination, when making another request provide this token with the next parameter.
    #[serde(rename = "next-token", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl InlineResponse200 {
    pub fn new(accounts: Vec<crate::models::Account>, current_round: i32) -> InlineResponse200 {
        InlineResponse200 {
            accounts,
            current_round,
            next_token: None,
        }
    }
}


