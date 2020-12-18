//! Darwinia API
use crate::result::{Result, Error};
use core::marker::PhantomData;
use primitives::{
    chain::{
        ethereum::{EthereumReceiptProofThing, EthereumRelayHeaderParcel, RedeemFor},
        proxy_type::ProxyType,
    },
    frame::{
        ethereum::{
            backing::{
                Redeem,
                RedeemCallExt,
                VerifiedProofStoreExt,
                EthereumBackingEventsDecoder
            },
            game::{AffirmationsStoreExt, EthereumRelayerGame, EthereumRelayerGameEventsDecoder},
            relay::{
                Affirm, AffirmCallExt, ConfirmedBlockNumbersStoreExt, EthereumRelay,
                PendingRelayHeaderParcelsStoreExt, SetConfirmedParcel,
                VotePendingRelayHeaderParcelCallExt,
                VotePendingRelayHeaderParcel,
                EthereumRelayEventsDecoder
            },
        },
        proxy::ProxyCallExt,
        sudo::SudoCallExt,
        bridge::relay_authorities::{
            EthereumRelayAuthoritiesEventsDecoder,
            SubmitSignedAuthorities,
            SubmitSignedAuthoritiesCallExt,
            SubmitSignedMmrRoot,
            SubmitSignedMmrRootCallExt,
        }
    },
    runtime::{DarwiniaRuntime, EcdsaMessage},
};
use std::collections::HashMap;
use substrate_subxt::{system::System, BlockNumber, Client, ClientBuilder, EventSubscription, EventsDecoder};
use web3::types::H256;
use substrate_subxt::sp_runtime::traits::Header;

// Types
type PendingRelayHeaderParcel = <DarwiniaRuntime as EthereumRelay>::PendingRelayHeaderParcel;
type RelayAffirmation = <DarwiniaRuntime as EthereumRelayerGame>::RelayAffirmation;
type AffirmationsReturn = HashMap<u64, HashMap<u32, Vec<RelayAffirmation>>>;
/// AccountId
pub type AccountId = <DarwiniaRuntime as System>::AccountId;

/// Dawrinia API
pub struct Darwinia {
    /// client
    pub client: Client<DarwiniaRuntime>,
}

impl Darwinia {
    /// New darwinia API
    pub async fn new(node_url: &str) -> Result<Darwinia> {
        let client =
            jsonrpsee::ws_client(node_url).await
                .map_err(|e| {
                    Error::FailToConnectDarwinia {
                        url: node_url.to_owned(),
                        source: e
                    }
                })?;
        let client = ClientBuilder::<DarwiniaRuntime>::new()
            .set_client(client)
            .build()
            .await?;

        Ok(Darwinia {
            client,
        })
    }

    /// Get confirmed block numbers
    pub async fn confirmed_block_numbers(&self) -> Result<Vec<u64>> {
        Ok(self.client.confirmed_block_numbers(None).await?)
    }

    /// Get the last confirmed block
    pub async fn last_confirmed(&self) -> Result<u64> {
        Ok(
            if let Some(confirmed) = self.confirmed_block_numbers().await?.iter().max() {
                *confirmed
            } else {
                0
            },
        )
    }

    /// Get pending headers
    pub async fn pending_headers(&self) -> Result<Vec<PendingRelayHeaderParcel>> {
        Ok(self.client.pending_relay_header_parcels(None).await?)
    }

    async fn get_mmr_root(&self, leaf_index: u32) -> Result<H256> {
        // Get mmr_root from block number == leaf_index + 1
        let block_number = leaf_index + 1;

        // TODO:
        let block_hash = self
            .client
            .block_hash(Some(BlockNumber::from(block_number)))
            .await?;
        let header = self.client.header(block_hash).await?;

        let mmr_root = if let Some(header) = header {
            // get digest_item from header
            let log = header
                .digest()
                .logs()
                .iter()
                .find(|&x| x.as_other().is_some());
            if let Some(digest_item) = log {
                // get mmr_root from log
                let parent_mmr_root = digest_item.as_other().unwrap().to_vec();
                let parent_mmr_root = &parent_mmr_root[4..];
                if parent_mmr_root.len() != 32 {
                    return Err(Error::Shadow(format!(
                        "Wrong parent_mmr_root len: {}",
                        parent_mmr_root.len()
                    ))
                        .into());
                }
                let mut mmr_root: [u8; 32] = [0; 32];
                mmr_root.copy_from_slice(&parent_mmr_root);
                H256(mmr_root)
            } else {
                return Err(
                    Error::Shadow("Wrong header with no parent_mmr_root".to_string()).into(),
                );
            }
        } else {
            return Err(Error::Shadow("No header fetched".to_string()).into());
        };

        Ok(mmr_root)
    }

    /// Build event subscription
    pub async fn build_event_subscription(&self) -> Result<EventSubscription<DarwiniaRuntime>> {
        let scratch = self.client.subscribe_events().await?;
        let mut decoder = EventsDecoder::<DarwiniaRuntime>::new(self.client.metadata().clone());

        // Register decoders
        decoder.with_ethereum_backing();
        decoder.with_ethereum_relayer_game();
        decoder.with_ethereum_relay();
        decoder.with_ethereum_relay_authorities();

        // Build subscriber
        let sub = EventSubscription::<DarwiniaRuntime>::new(scratch, decoder);
        Ok(sub)
    }
}
