use anyhow::Result;
use semver::Version;

use crate::json_utils::unwrap_json_string;

pub async fn check_for_updates() -> Result<()> {
    let result = reqwest::Client::builder()
        .user_agent("Kyocera Printer Monitor")
        .build()?
        .get("https://api.github.com/repos/machineonamission/kyocera-printer-monitor/releases/latest")
        .send()
        .await?
        .text()
        .await?;
    // dbg!(&result);
    let json = serde_json::from_str::<serde_json::Value>(&result)?;
    let tag_name = unwrap_json_string(&json["tag_name"], "tag_name")?;
    let new_ver = Version::parse(tag_name.strip_prefix('v').unwrap_or(tag_name))?; // 1.. is to remove the v
    let current_ver = Version::parse(env!("CARGO_PKG_VERSION"))?;
    if new_ver > current_ver {
        println!(
            "🔔 A new version of Kyocera Printer Monitor is available! \
        You are using v{current_ver}, but v{new_ver} is available. Get it here:\n\
        https://github.com/machineonamission/kyocera-printer-monitor/releases/latest"
        );
    }

    Ok(())
}
