mod p2p;
mod state;
mod http;

use clap::Parser;
use tracing_subscriber::EnvFilter;
use libp2p::identity::Keypair;
use std::fs;
use std::path::Path;
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
    let identity_file = format!("identity_{}.key", args.port);
    let id_keys = load_or_generate_keypair(&identity_file)?;

    p2p::run_node(args.port, id_keys).await
}

fn load_or_generate_keypair(path: &str) -> anyhow::Result<Keypair> {
    let path = Path::new(path);
    if path.exists() {
        info!("Loading identity from {:?}", path);
        let bytes = fs::read(path)?;
        let keypair = Keypair::from_protobuf_encoding(&bytes)?;
        Ok(keypair)
    } else {
        info!("Generating new identity and saving to {:?}", path);
        let keypair = Keypair::generate_ed25519();
        let bytes = keypair.to_protobuf_encoding()?;
        fs::write(path, bytes)?;
        Ok(keypair)
    }
}
