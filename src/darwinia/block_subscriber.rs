use crate::darwinia::Client;
use crate::result::Result;
use jsonrpsee::client::Subscription;
use primitives::runtime::DarwiniaRuntime;
use substrate_subxt::system::System;

/// Dawrinia Block Subscribe
pub struct BlockSubscriber {
    darwinia: Client,
}

impl BlockSubscriber {
    /// New redeem service
    pub async fn new(
        darwinia: Client,
    ) -> BlockSubscriber {
        BlockSubscriber {
            darwinia,
        }
    }
}

impl BlockSubscriber {
    pub async fn start(&self) -> Result<()> {
        let mut sub: Subscription<<DarwiniaRuntime as System>::Header> = self.darwinia.client.subscribe_finalized_blocks().await?;
        while let header = sub.next().await {
            let hash = header.hash();

            let events = self.darwinia.get_raw_events(hash).await?;

            println!("{:?}", events.len());
        }
        Ok(())
    }


}