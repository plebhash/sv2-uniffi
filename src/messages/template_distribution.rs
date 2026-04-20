use crate::messages::types::*;

#[derive(uniffi::Record)]
pub struct CoinbaseOutputConstraints {
    pub coinbase_output_max_additional_size: u32,
    pub coinbase_output_max_additional_sigops: u16,
}

#[derive(uniffi::Record)]
pub struct NewTemplate {
    pub template_id: u64,
    pub future_template: bool,
    pub version: u32,
    pub coinbase_tx_version: u32,
    pub coinbase_prefix: Sv2B0255,
    pub coinbase_tx_input_sequence: u32,
    pub coinbase_tx_value_remaining: u64,
    pub coinbase_tx_outputs_count: u32,
    pub coinbase_tx_outputs: Sv2B064K,
    pub coinbase_tx_locktime: u32,
    pub merkle_path: Sv2Seq0255U256,
}

#[derive(uniffi::Record)]
pub struct SetNewPrevHashTemplateDistribution {
    pub template_id: u64,
    pub prev_hash: Sv2U256,
    pub header_timestamp: u32,
    pub nbits: u32,
    pub target: Sv2U256,
}

#[derive(uniffi::Record)]
pub struct RequestTransactionData {
    pub template_id: u64,
}

#[derive(uniffi::Record)]
pub struct RequestTransactionDataSuccess {
    pub template_id: u64,
    pub excess_data: Sv2B064K,
    pub transaction_list: Sv2Seq064KB016M,
}

#[derive(uniffi::Record)]
pub struct RequestTransactionDataError {
    pub template_id: u64,
    pub error_code: String,
}

#[derive(uniffi::Record)]
pub struct SubmitSolution {
    pub template_id: u64,
    pub version: u32,
    pub header_timestamp: u32,
    pub header_nonce: u32,
    pub coinbase_tx: Sv2B064K,
}
