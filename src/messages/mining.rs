use crate::messages::types::*;

#[derive(uniffi::Record)]
pub struct OpenStandardMiningChannel {
    pub request_id: u32,
    pub user_identity: String,
    pub nominal_hash_rate: f32,
    pub max_target: Sv2U256,
}

#[derive(uniffi::Record)]
pub struct OpenStandardMiningChannelSuccess {
    pub request_id: u32,
    pub channel_id: u32,
    pub target: Sv2U256,
    pub extranonce_prefix: Sv2B032,
    pub group_channel_id: u32,
}

#[derive(uniffi::Record)]
pub struct OpenExtendedMiningChannel {
    pub request_id: u32,
    pub user_identity: String,
    pub nominal_hash_rate: f32,
    pub max_target: Sv2U256,
    pub min_extranonce_size: u16,
}

#[derive(uniffi::Record)]
pub struct OpenExtendedMiningChannelSuccess {
    pub request_id: u32,
    pub channel_id: u32,
    pub group_channel_id: u32,
    pub target: Sv2U256,
    pub extranonce_size: u16,
    pub extranonce_prefix: Sv2B032,
}

#[derive(uniffi::Record)]
pub struct OpenMiningChannelError {
    pub request_id: u32,
    pub error_code: String,
}

#[derive(uniffi::Record)]
pub struct UpdateChannel {
    pub channel_id: u32,
    pub nominal_hash_rate: f32,
    pub maximum_target: Sv2U256,
}

#[derive(uniffi::Record)]
pub struct UpdateChannelError {
    pub channel_id: u32,
    pub error_code: String,
}

#[derive(uniffi::Record)]
pub struct CloseChannel {
    pub channel_id: u32,
    pub reason_code: String,
}

#[derive(uniffi::Record)]
pub struct SetExtranoncePrefix {
    pub channel_id: u32,
    pub extranonce_prefix: Sv2B032,
}

#[derive(uniffi::Record)]
pub struct SubmitSharesStandard {
    pub channel_id: u32,
    pub sequence_number: u32,
    pub job_id: u32,
    pub nonce: u32,
    pub ntime: u32,
    pub version: u32,
}

#[derive(uniffi::Record)]
pub struct SubmitSharesExtended {
    pub channel_id: u32,
    pub sequence_number: u32,
    pub job_id: u32,
    pub nonce: u32,
    pub ntime: u32,
    pub version: u32,
    pub extranonce: Sv2B032,
}

#[derive(uniffi::Record)]
pub struct SubmitSharesSuccess {
    pub channel_id: u32,
    pub last_sequence_number: u32,
    pub new_submits_accepted_count: u32,
    pub new_shares_sum: u64,
}

#[derive(uniffi::Record)]
pub struct SubmitSharesError {
    pub channel_id: u32,
    pub sequence_number: u32,
    pub error_code: String,
}

#[derive(uniffi::Record)]
pub struct NewMiningJob {
    pub channel_id: u32,
    pub job_id: u32,
    pub min_ntime: Option<u32>,
    pub version: u32,
    pub merkle_root: Sv2U256,
}

#[derive(uniffi::Record)]
pub struct NewExtendedMiningJob {
    pub channel_id: u32,
    pub job_id: u32,
    pub min_ntime: Option<u32>,
    pub version: u32,
    pub version_rolling_allowed: bool,
    pub merkle_path: Sv2Seq0255U256,
    pub coinbase_tx_prefix: Sv2B064K,
    pub coinbase_tx_suffix: Sv2B064K,
}

#[derive(uniffi::Record)]
pub struct SetNewPrevHashMining {
    pub channel_id: u32,
    pub job_id: u32,
    pub prev_hash: Sv2U256,
    pub min_ntime: u32,
    pub nbits: u32,
}

#[derive(uniffi::Record)]
pub struct SetCustomMiningJob {
    pub channel_id: u32,
    pub request_id: u32,
    pub mining_job_token: Sv2B0255,
    pub version: u32,
    pub prev_hash: Sv2U256,
    pub min_ntime: u32,
    pub nbits: u32,
    pub coinbase_tx_version: u32,
    pub coinbase_prefix: Sv2B0255,
    pub coinbase_tx_input_nsequence: u32,
    pub coinbase_tx_outputs: Sv2B064K,
    pub coinbase_tx_locktime: u32,
    pub merkle_path: Sv2Seq0255U256,
}

#[derive(uniffi::Record)]
pub struct SetCustomMiningJobSuccess {
    pub channel_id: u32,
    pub request_id: u32,
    pub job_id: u32,
}

#[derive(uniffi::Record)]
pub struct SetCustomMiningJobError {
    pub channel_id: u32,
    pub request_id: u32,
    pub error_code: String,
}

#[derive(uniffi::Record)]
pub struct SetTarget {
    pub channel_id: u32,
    pub maximum_target: Sv2U256,
}

#[derive(uniffi::Record)]
pub struct SetGroupChannel {
    pub group_channel_id: u32,
    pub channel_ids: Vec<u32>,
}
