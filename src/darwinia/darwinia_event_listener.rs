//! Darwinia Subscribe
use crate::darwinia::darwinia::Darwinia;
use crate::result::Error;
use crate::result::Result;
use primitives::{
    frame::bridge::relay_authorities::{AuthoritiesSetSigned, NewAuthorities, NewMMRRoot},
    runtime::DarwiniaRuntime,
};
use std::sync::Arc;
use substrate_subxt::sp_core::Decode;
use substrate_subxt::EventSubscription;

/// Dawrinia Subscribe
pub struct DarwiniaEventListener {
    darwinia: Arc<Darwinia>,
    sub: EventSubscription<DarwiniaRuntime>,
    stop: bool,
}

impl DarwiniaEventListener {
    /// New redeem service
    pub async fn new(
        darwinia: Arc<Darwinia>,
    ) -> Result<DarwiniaEventListener> {
        let sub = darwinia.build_event_subscription().await?;
        Ok(DarwiniaEventListener {
            darwinia,
            sub,
            stop: false,
        })
    }

    /// start
    pub async fn start(&mut self) -> Result<DarwiniaEventListener> {
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
        // if module != "System" {
            debug!(">> Event - {}::{}", module, variant);
        // }

        match (module, variant) {
            ("System", "CodeUpdated") => {
                return Err(Error::Shadow("CodeUpdated".to_string()).into());
            }

            // call ethereum_relay_authorities.request_authority and then sudo call
            // EthereumRelayAuthorities.add_authority will emit the event
            ("EthereumRelayAuthorities", "NewAuthorities") => {
            }

            // authority set changed will emit this event
            ("EthereumRelayAuthorities", "AuthoritiesSetSigned") => {
            }

            // call ethereum_backing.lock will emit the event
            ("EthereumRelayAuthorities", "NewMMRRoot") => {
            }

            _ => {}
        }

        Ok(())
    }
}