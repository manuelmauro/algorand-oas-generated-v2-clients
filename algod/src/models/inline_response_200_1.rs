/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse2001 {
    #[serde(rename = "asset-holding", skip_serializing_if = "Option::is_none")]
    pub asset_holding: Option<Box<crate::models::AssetHolding>>,
    #[serde(rename = "created-asset", skip_serializing_if = "Option::is_none")]
    pub created_asset: Option<Box<crate::models::AssetParams>>,
    /// The round for which this information is relevant.
    #[serde(rename = "round")]
    pub round: i32,
}

impl InlineResponse2001 {
    pub fn new(round: i32) -> InlineResponse2001 {
        InlineResponse2001 {
            asset_holding: None,
            created_asset: None,
            round,
        }
    }
}


