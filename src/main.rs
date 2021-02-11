use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FlypoolDashboardStatistics {
    time: u64,
    reported_hashrate: f64,
    current_hashrate: f64,
    valid_shares: u64,
    invalid_shares: u64,
    stale_shares: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FlypoolDashboardWorker {
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
struct FlypoolDashboardCurrentStatistics {
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
struct FlypoolDashboardSettings {
    email: String,
    monitor: u8,
    min_payout: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FlypoolDashboard {
    statistics: Vec<FlypoolDashboardStatistics>,
    workers: Vec<FlypoolDashboardWorker>,
    current_statistics: FlypoolDashboardCurrentStatistics,
    settings: FlypoolDashboardSettings,
}

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let resp = reqwest::get(
        "https://api.ethermine.org/miner/a510a00561f916df8be07028d51c0426abc290ac/dashboard",
    )
    .await?;
    // let resp = reqwest::get(
    //     "https://api-ravencoin.flypool.org/miner/RQhmvdj14qBFbRpiXrEwxagYDCnEcPUbeN/dashboard",
    // )
    // .await?;
    let text = resp.text().await?;
    println!("resp text {:?}", text);
    let response_json: Value = serde_json::from_str(text.as_str()).unwrap();

    println!("{:?}", response_json);

    let response_struct: FlypoolDashboard =
        serde_json::from_value(response_json["data"].clone()).unwrap();

    println!("{:?}", response_struct);
    Ok(())
}
