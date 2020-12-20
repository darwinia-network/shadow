use crate::darwinia::DarwiniaClient;
use crate::result::Result;
use jsonrpsee::client::Subscription;
use primitives::runtime::DarwiniaRuntime;
use substrate_subxt::system::System;
use substrate_subxt::RawEvent;
use substrate_subxt::sp_core::Decode;
use primitives::frame::ethereum::backing::LockRing;
use super::EventHandler;

/// Dawrinia Block Subscribe
pub struct BlockSubscriber {
    darwinia: DarwiniaClient,
    event_handler: EventHandler,
}

impl BlockSubscriber {
    /// New redeem service
    pub async fn new(
        darwinia: DarwiniaClient,
        event_handler: EventHandler,
    ) -> BlockSubscriber {
        BlockSubscriber {
            darwinia,
            event_handler,
        }
    }
}

impl BlockSubscriber {
    pub async fn start(&self) -> Result<()> {
        let mut sub: Subscription<<DarwiniaRuntime as System>::Header> = self.darwinia.client.subscribe_finalized_blocks().await?;
        while let header = sub.next().await {
            let hash = header.hash();

            debug!("Block {}", header.number);
            let events = self.darwinia.get_raw_events(hash).await;
            self.handle_events(&header, events).await?;
        }
        Ok(())
    }

    async fn handle_events(&self, header: &<DarwiniaRuntime as System>::Header, events: Result<Vec<RawEvent>>) -> Result<()> {
        match events {
            Ok(events) => {
                for event in events {
                    let module = event.module.as_str();
                    let variant = event.variant.as_str();
                    let event_data = event.data;

                    self.event_handler.handle(header, module, variant, event_data).await?;
                }
            },
            Err(err) => {
                error!("{:#?}", err);
            }
        }
        Ok(())
    }

}
