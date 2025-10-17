use std::process::Command;

use color_eyre::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ResponseData {
    code: String,
    rec: Option<String>,
    message: Option<String>,
    wlanacIp: Option<String>,
    wlanacIpv6: Option<String>,
    version: Option<String>,
    usertime: Option<String>,
    reccode: Option<String>,
    logouturl: Option<String>,
    selfTicket: Option<String>,
    macChange: bool,
    groupId: Option<i64>,
    passwdPolicyCheck: bool,
    dropLogCheck: Option<String>,
    logoutSsoUrl: Option<String>,
}

pub(crate) async fn do_authcation(
    url: &str,
    user_id: &str,
    passwd: &str,
    wan_interface: &str,
) -> Result<bool> {
    let client = Client::new();

    let cmd = format!(
        "ifconfig {} | grep 'inet addr:' | grep -oE '([0-9]{{1,3}}\\.){{3}}[0-9]{{1,3}}' | head -n 1",
        wan_interface
    );
    let output = Command::new("sh").arg("-c").arg(cmd).output()?;
    let wlan_ip = String::from_utf8(output.stdout)?.trim().to_string();

    let params = [
        ("userid", user_id.to_string()),
        ("passwd", passwd.to_string()),
        ("wlanuserip", wlan_ip),
        ("wlanacname", "NFV-BASE-1".to_string()),
    ];

    let full_url = format!("http://{}/quickauth.do", url);
    let resp = client.get(&full_url).query(&params).send().await?;

    let status = resp.status();
    let body = resp.text().await?;

    if status.is_success() {
        match serde_json::from_str::<ResponseData>(&body) {
            Ok(r_data) => {
                if r_data.code == "0" {
                    println!("Authentication Successful.");
                    Ok(true)
                } else {
                    eprintln!(
                        "Authentication failed: code = {}, message = {:?}, rec = {:?}",
                        r_data.code, r_data.message, r_data.rec
                    );
                    Ok(false)
                }
            }
            Err(err) => {
                eprintln!("Failed to parse response JSON: {}", err);
                eprintln!("Raw response body: {}", body);
                Ok(false)
            }
        }
    } else {
        eprintln!("HTTP request failed: {}", status);
        eprintln!("Response body: {}", body);
        Ok(false)
    }
}
