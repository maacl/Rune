use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use iroh::{protocol::Router, Endpoint};
use iroh_blobs::util::fs;
use iroh_gossip::net::Gossip;

#[derive(Clone, Debug)]
pub(crate) struct Iroh {
    pub router: Router,
    pub gossip: Gossip,
    pub endpoint: Endpoint,
}

impl Iroh {
    pub async fn new(path: PathBuf) -> Result<Self> {
        // create dir if it doesn't already exist
        tokio::fs::create_dir_all(&path).await?;

        let key = fs::load_secret_key(path.clone().join("keypair")).await?;

        // create endpoint
        let endpoint = iroh::Endpoint::builder()
            .discovery_n0()
            .secret_key(key)
            .bind()
            .await?;

        // build the protocol router
        let mut builder = iroh::protocol::Router::builder(endpoint.clone());

        // add iroh gossip
        let gossip = iroh_gossip::net::Gossip::builder()
            .spawn(builder.endpoint().clone())
            .await?;
        builder = builder.accept(iroh_gossip::ALPN, Arc::new(gossip.clone()));

        let router = builder.spawn().await?;

        Ok(Self {
            router,
            gossip,
            endpoint,
        })
    }

    #[allow(dead_code)]
    pub(crate) async fn shutdown(self) -> Result<()> {
        self.router.shutdown().await
    }
}
