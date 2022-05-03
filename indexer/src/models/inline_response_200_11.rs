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
pub struct InlineResponse20011 {
    #[serde(rename = "asset")]
    pub asset: Box<crate::models::Asset>,
    /// Round at which the results were computed.
    #[serde(rename = "current-round")]
    pub current_round: i32,
}

impl InlineResponse20011 {
    pub fn new(asset: crate::models::Asset, current_round: i32) -> InlineResponse20011 {
        InlineResponse20011 {
            asset: Box::new(asset),
            current_round,
        }
    }
}


