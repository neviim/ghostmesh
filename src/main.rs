mod p2p;
mod state;
mod http;

use clap::Parser;
use tracing_subscriber::EnvFilter;
use libp2p::identity;
use tracing::info;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 0)]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    let args = Args::parse();
    // let identity_file = format!("identity_{}.key", args.port); // Removed
    let id_keys = load_or_generate_keypair(args.port)?; // Updated call site

    p2p::run_node(args.port, id_keys).await
}

fn load_or_generate_keypair(port: u16) -> anyhow::Result<identity::Keypair> {
    let dir = std::path::Path::new(".key");
    if !dir.exists() {
        std::fs::create_dir(dir)?;
    }
    
    let file_path = dir.join(format!("identity_{}.key", port));
    
    if file_path.exists() {
        info!("Loading identity from {:?}", file_path);
        let bytes = std::fs::read(&file_path)?;
        return identity::Keypair::from_protobuf_encoding(&bytes).map_err(|e| anyhow::anyhow!("{:?}", e));
    }

    info!("Generating new identity and saving to {:?}", file_path);
    let keypair = identity::Keypair::generate_ed25519();
    let bytes = keypair.to_protobuf_encoding()?;
    std::fs::write(&file_path, bytes)?;

    Ok(keypair)
}
