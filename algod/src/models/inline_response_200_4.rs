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
pub struct InlineResponse2004 {
    /// The type of hash function used to create the proof, must be one of: * sumhash  * sha512_256
    #[serde(rename = "hashtype")]
    pub hashtype: Hashtype,
    /// Index of the transaction in the block's payset.
    #[serde(rename = "idx")]
    pub idx: i32,
    /// Merkle proof of transaction membership.
    #[serde(rename = "proof")]
    pub proof: String,
    /// Hash of SignedTxnInBlock for verifying proof.
    #[serde(rename = "stibhash")]
    pub stibhash: String,
    /// Represents the depth of the tree that is being proven, i.e. the number of edges from a leaf to the root.
    #[serde(rename = "treedepth")]
    pub treedepth: i32,
}

impl InlineResponse2004 {
    pub fn new(hashtype: Hashtype, idx: i32, proof: String, stibhash: String, treedepth: i32) -> InlineResponse2004 {
        InlineResponse2004 {
            hashtype,
            idx,
            proof,
            stibhash,
            treedepth,
        }
    }
}

/// The type of hash function used to create the proof, must be one of: * sumhash  * sha512_256
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Hashtype {
    #[serde(rename = "sumhash")]
    Sumhash,
    #[serde(rename = "sha512_256")]
    Sha512256,
}

impl Default for Hashtype {
    fn default() -> Hashtype {
        Self::Sumhash
    }
}

