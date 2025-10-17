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
    groupId: Option<String>,
    passwdPolicyCheck: bool,
    dropLogCheck: Option<String>,
    logoutSsoUrl: Option<String>,
}

pub(crate) async fn do_authcation(
    url: &str,
    user_id: &str,
    passwd: &str,
) -> Result<bool> {
    let client = Client::new();
    let params = [
        ("userid", user_id.to_string()),
        ("passwd", passwd.to_string()),
        ("wlanuserip", "10.99.26.50".to_string()),
        ("wlanacname", "NFT-BASE-1".to_string()),
    ];

    let full_url = format!("http://{}/quickauth.do", url);

    let resp = client.get(&full_url).query(&params).send().await?;

    if resp.status().is_success() {
        let r_data: ResponseData = resp.json().await?;
        println!("Response JSON: {:#?}", r_data);

        Ok(r_data.code == "0")
    } else {
        eprintln!("HTTP request failed: {}", resp.status());
        Ok(false)
    }
}

