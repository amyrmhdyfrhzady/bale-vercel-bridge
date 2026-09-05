use std::sync::Arc;

use bale_signaling::BaleSignaling;
use lk_signaling::{Signaling, TokenStore};

struct EnvTokenStore;

impl TokenStore for EnvTokenStore {
    fn load(&self) -> Option<Vec<u8>> {
        std::env::var("BALE_ACCESS_TOKEN")
            .ok()
            .map(|token| token.into_bytes())
    }

    fn save(&self, _bytes: &[u8]) {
        // Token persistence will be added later.
    }

    fn clear(&self) {
        // Token clearing will be added later.
    }
}

#[tokio::main]
async fn main() {
    println!("=================================");
    println!(" Bale Vercel Bridge");
    println!(" Phase 1 - Bale Signaling PoC");
    println!("=================================");

    let token = std::env::var("BALE_ACCESS_TOKEN");

    match token {
        Ok(value) if !value.is_empty() => {
            println!("✓ BALE_ACCESS_TOKEN detected");
            println!("✓ Token length: {}", value.len());
        }
        _ => {
            eprintln!("✗ BALE_ACCESS_TOKEN is not set");
            std::process::exit(1);
        }
    }

    let store: Arc<dyn TokenStore> = Arc::new(EnvTokenStore);

    let signaling = BaleSignaling::new(store);

    println!("→ Connecting to Bale...");

    match signaling.connect().await {
        Ok(_) => {
            println!("✓ Bale connection requested");

            loop {
                if signaling.is_connected() {
                    println!("✓ Bale WebSocket connected");
                    break;
                }

                tokio::time::sleep(
                    std::time::Duration::from_secs(1)
                )
                .await;
            }
        }

        Err(error) => {
            eprintln!("✗ Bale connection failed: {error:?}");
            std::process::exit(1);
        }
    }

    println!("✓ Phase 1 signaling connection established");

    loop {
        tokio::time::sleep(
            std::time::Duration::from_secs(60)
        )
        .await;
    }
}
