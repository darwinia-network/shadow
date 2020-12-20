//! Darwinia Subscribe
use crate::darwinia::client::DarwiniaClient;
use crate::result::Error;
use crate::result::Result;
use primitives::{
    runtime::DarwiniaRuntime,
};
use std::sync::Arc;
use substrate_subxt::EventSubscription;

use mysql::*;

/// Dawrinia Subscribe
pub struct EventSubscriber {
    darwinia: Arc<DarwiniaClient>,
    sub: EventSubscription<DarwiniaRuntime>,
    stop: bool,
    db: Pool,
}

impl EventSubscriber {
    /// New redeem service
    pub async fn new(
        darwinia: Arc<DarwiniaClient>,
        db: Pool,
    ) -> Result<EventSubscriber> {
        let sub = darwinia.build_event_subscription().await?;
        Ok(EventSubscriber {
            darwinia,
            sub,
            stop: false,
            db
        })
    }

    /// start
    pub async fn start(&mut self) -> Result<EventSubscriber> {
        info!("Darwinia Event Listener Started");
        loop {
            if let Err(e) = self.process_next_event().await {
                if e.to_string() == "CodeUpdated" {
                    self.stop();
                    return Err(e);
                } else {
                    error!("{:#?}", e);
                }
            }
            if self.stop {
                return Err(Error::Shadow("Force stop".to_string()).into());
            }
        }
    }

    /// stop
    pub fn stop(&mut self) {
        info!("Darwinia Event Listener Stopped");
        self.stop = true;
    }

    /// process_next_event
    async fn process_next_event(&mut self) -> Result<()> {
        if let Some(raw) = self.sub.next().await {
            match raw {
                Ok(event) => {

                    self.handle_event(&event.module, &event.variant, event.data)
                        .await?;
                },
                Err(err) => {
                    return Err(err.into());
                }
            }
        }
        Ok(())
    }

    async fn handle_event(
        &mut self,
        module: &str,
        variant: &str,
        event_data: Vec<u8>,
    ) -> Result<()> {
        if module != "System" {
            debug!(">> Event - {}::{}", module, variant);
        }

        match (module, variant) {
            ("System", "CodeUpdated") => {
                return Err(Error::Shadow("CodeUpdated".to_string()).into());
            }

            ("EthereumRelayAuthorities", "MMRRootSigned") => {
                // if let Ok(decoded) = MMRRootSigned::<DarwiniaRuntime>::decode(&mut &event_data[..]) {
                    // write to db
                    // let sql = format!(
                    //     "INSERT INTO darwinia_signed_mmr_roots (block_number, mmr_root, signatures, created_at) VALUES ({}, '{}', '{}', '{}')",
                    //     decoded.block_number,
                    //     decoded.mmr_root,
                    //     decoded.signatures
                    //         .iter()
                    //         .map(|s| format!("{:x?}", s.1.0))
                    //         .collect::<Vec<_>>()
                    //         .join(&[][..]),
                    //
                    // );
                    // tx.query_drop(sql)?;
                // }
            }

            _ => {}
        }

        Ok(())
    }
}
