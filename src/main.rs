mod flypool;
use flypool::*;
use serde_json::Value;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let resp = reqwest::get(
        "https://api.ethermine.org/miner/9eb072e4cd50E041fb863F187001676D3d7288Ff/dashboard",
    )
    .await?;
    let current_statistics = reqwest::get(
        "https://api.ethermine.org/miner/9eb072e4cd50E041fb863F187001676D3d7288Ff/currentStats",
    )
    .await?;
    // let resp = reqwest::get(
    //     "https://api-ravencoin.flypool.org/miner/RQhmvdj14qBFbRpiXrEwxagYDCnEcPUbeN/dashboard",
    // )
    // .await?;
    let text = resp.text().await?;
    let cs_text = current_statistics.text().await?;
    println!("resp text {:?}", text);
    let response_json: Value = serde_json::from_str(text.as_str()).unwrap();
    let response_cs: Value = serde_json::from_str(cs_text.as_str()).unwrap();

    println!("{:?}", response_json);

    let response_struct: FlypoolDashboardEndpoint =
        serde_json::from_value(response_json["data"].clone()).unwrap();
    let response_current_statistics: FlypoolCurrentStatistics =
        serde_json::from_value(response_cs["data"].clone()).unwrap();

    println!("{:?}", response_struct);
    println!("{:?}", response_current_statistics);
    Ok(())
}
