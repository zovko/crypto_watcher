mod flypool;
use flypool::*;
use serde_json::Value;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let resp = reqwest::get(
        "https://api.ethermine.org/miner/9eb072e4cd50E041fb863F187001676D3d7288Ff/dashboard",
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
