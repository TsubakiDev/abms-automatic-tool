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
    id: String,

    // Your account's password (Type: String)
    #[arg(short, long)]
    password: String,
}

#[tokio::main()]
async fn main() -> color_eyre::Result<()> {
    let args = Args::parse();

    println!("Authcation URL: {}", &args.authcation_url.to_string());

    let _ = auth::do_authcation(args.authcation_url.as_str(), args.id.as_str(), args.password.as_str()).await?;

    Ok(())
}
