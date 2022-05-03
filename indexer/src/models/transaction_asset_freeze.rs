/*
 * Indexer
 *
 * Algorand ledger analytics API.
 *
 * The version of the OpenAPI document: 2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionAssetFreeze : Fields for an asset freeze transaction.  Definition: data/transactions/asset.go : AssetFreezeTxnFields



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TransactionAssetFreeze {
    /// \\[fadd\\] Address of the account whose asset is being frozen or thawed.
    #[serde(rename = "address")]
    pub address: String,
    /// \\[faid\\] ID of the asset being frozen or thawed.
    #[serde(rename = "asset-id")]
    pub asset_id: i32,
    /// \\[afrz\\] The new freeze status.
    #[serde(rename = "new-freeze-status")]
    pub new_freeze_status: bool,
}

impl TransactionAssetFreeze {
    /// Fields for an asset freeze transaction.  Definition: data/transactions/asset.go : AssetFreezeTxnFields
    pub fn new(address: String, asset_id: i32, new_freeze_status: bool) -> TransactionAssetFreeze {
        TransactionAssetFreeze {
            address,
            asset_id,
            new_freeze_status,
        }
    }
}


