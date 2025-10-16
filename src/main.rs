use clap::Parser;

mod auth;

/// ABMS Automatic authcation tool, used for China's some campus-network
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Your campus's authcation URL (Type: string)
    #[arg(short, long)]
    authcation_url: String,

    /// Your account's ID (Type: String)
    #[arg(short, long)]
    user_id: String,

    // Your account's password (Type: String)
    #[arg(short, long)]
    user_password: String,

}

#[tokio::main()]
async fn main() -> color_eyre::Result<()> {
    let args = Args::parse();

    let client = reqwest::Client::new();

    let resp_text = client.get("https://baidu.com").send().await;

    println!("{:?}", resp_text);

    Ok(())
}
