use crate::messages::types::*;

#[derive(uniffi::Record)]
pub struct AllocateMiningJobToken {
    pub user_identifier: String,
    pub request_id: u32,
}

#[derive(uniffi::Record)]
pub struct AllocateMiningJobTokenSuccess {
    pub request_id: u32,
    pub mining_job_token: Sv2B0255,
    pub coinbase_tx_outputs: Sv2B064K,
}

#[derive(uniffi::Record)]
pub struct DeclareMiningJob {
    pub request_id: u32,
    pub mining_job_token: Sv2B0255,
    pub version: u32,
    pub coinbase_tx_prefix: Sv2B064K,
    pub coinbase_tx_suffix: Sv2B064K,
    pub wtxid_list: Sv2Seq064KU256,
    pub excess_data: Sv2B064K,
}

#[derive(uniffi::Record)]
pub struct DeclareMiningJobSuccess {
    pub request_id: u32,
    pub new_mining_job_token: Sv2B0255,
}

#[derive(uniffi::Record)]
pub struct DeclareMiningJobError {
    pub request_id: u32,
    pub error_code: String,
    pub error_details: Sv2B064K,
}

#[derive(uniffi::Record)]
pub struct ProvideMissingTransactions {
    pub request_id: u32,
    pub unknown_tx_position_list: Vec<u16>,
}

#[derive(uniffi::Record)]
pub struct ProvideMissingTransactionsSuccess {
    pub request_id: u32,
    pub transaction_list: Sv2Seq064KB016M,
}

#[derive(uniffi::Record)]
pub struct PushSolution {
    pub extranonce: Sv2B032,
    pub prev_hash: Sv2U256,
    pub nonce: u32,
    pub ntime: u32,
    pub nbits: u32,
    pub version: u32,
}
