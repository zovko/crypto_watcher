use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EthermineDashboardStatistics {
    time: u64,
    reported_hashrate: f64,
    current_hashrate: f64,
    valid_shares: u64,
    invalid_shares: u64,
    stale_shares: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EthermineDashboardWorker {
    worker: String,
    time: u64,
    last_seen: u64,
    reported_hashrate: f64,
    current_hashrate: f64,
    valid_shares: u64,
    invalid_shares: u64,
    stale_shares: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EthermineDashboardCurrentStatistics {
    time: u64,
    last_seen: u64,
    reported_hashrate: f64,
    current_hashrate: f64,
    valid_shares: u64,
    invalid_shares: u64,
    stale_shares: u64,
    active_workers: u32,
    unpaid: u64,
    #[serde(default)]
    unconfirmed: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EthermineDashboardSettings {
    email: String,
    monitor: u8,
    min_payout: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EthermineDashboard {
    statistics: Vec<EthermineDashboardStatistics>,
    workers: Vec<EthermineDashboardWorker>,
    current_statistics: EthermineDashboardCurrentStatistics,
    settings: EthermineDashboardSettings,
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let resp = reqwest::get(
        "https://api.ethermine.org/miner/a510a00561f916df8be07028d51c0426abc290ac/dashboard",
    )
    .await?;
    let response_json: Value = serde_json::from_str(resp.text().await?.as_str()).unwrap();

    println!("{:?}", response_json);

    let response_struct: EthermineDashboard =
        serde_json::from_value(response_json["data"].clone()).unwrap();

    println!("{:?}", response_struct);
    Ok(())
}
