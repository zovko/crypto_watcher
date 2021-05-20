use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlypoolStatistics {
    pub time: u64,
    pub reported_hashrate: f64,
    pub current_hashrate: f64,
    pub valid_shares: u64,
    pub invalid_shares: u64,
    pub stale_shares: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlypoolWorker {
    pub worker: String,
    pub time: u64,
    pub last_seen: u64,
    pub reported_hashrate: f64,
    pub current_hashrate: f64,
    pub valid_shares: u64,
    pub invalid_shares: u64,
    pub stale_shares: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlypoolCurrentStatistics {
    pub time: u64,
    pub last_seen: u64,
    pub reported_hashrate: f64,
    pub current_hashrate: f64,
    pub valid_shares: u64,
    pub invalid_shares: u64,
    pub stale_shares: u64,
    pub active_workers: u32,
    pub unpaid: u64,
    pub unconfirmed: Option<f32>,
    pub coins_per_min: Option<f32>,
    pub usd_per_min: Option<f32>,
    pub btc_per_min: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlypoolSettings {
    pub email: String,
    pub monitor: u8,
    pub min_payout: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlypoolDashboardEndpoint {
    pub statistics: Vec<FlypoolStatistics>,
    pub workers: Vec<FlypoolWorker>,
    pub current_statistics: FlypoolCurrentStatistics,
    pub settings: FlypoolSettings,
}

