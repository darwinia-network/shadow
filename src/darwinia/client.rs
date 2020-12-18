//! Darwinia API
use crate::result::{Result, Error};
use primitives::{
    frame::{
        ethereum::{
            backing::{
                EthereumBackingEventsDecoder
            },
            game::EthereumRelayerGameEventsDecoder,
            relay::{
                ConfirmedBlockNumbersStoreExt, EthereumRelay,
                PendingRelayHeaderParcelsStoreExt,
                EthereumRelayEventsDecoder
            },
        },
        bridge::relay_authorities::{
            EthereumRelayAuthoritiesEventsDecoder,
        }
    },
    runtime::DarwiniaRuntime,
};
use web3::types::H256 as EthH256;
use substrate_subxt::sp_runtime::traits::Header;
use substrate_subxt::sp_core::{twox_128, H256};
use substrate_subxt::sp_core::storage::StorageKey;
use substrate_subxt::events::Raw;
use substrate_subxt::{BlockNumber, Client, ClientBuilder, EventSubscription, RawEvent, EventsDecoder};

// Types
type PendingRelayHeaderParcel = <DarwiniaRuntime as EthereumRelay>::PendingRelayHeaderParcel;

/// Dawrinia API
pub struct Client {
    /// client
    pub client: Client<DarwiniaRuntime>,
}

impl Client {
    /// New darwinia API
    pub async fn new(node_url: &str) -> Result<Client> {
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

        Ok(Client {
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

    async fn get_mmr_root(&self, leaf_index: u32) -> Result<EthH256> {
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

    pub async fn get_raw_events(&self, header_hash: H256) -> Result<Vec<RawEvent>> {
        let mut events = vec![];

        let mut storage_key = twox_128(b"System").to_vec();
        storage_key.extend(twox_128(b"Events").to_vec());
        let keys = vec![StorageKey(storage_key)];

        let change_sets = self.client.query_storage(keys, header_hash, None).await?;
        for change_set in change_sets {
            for (_key, data) in change_set.changes {
                if let Some(data) = data {
                    let decoder = EventsDecoder::<DarwiniaRuntime>::new(self.client.metadata().clone());
                    let raw_events = decoder.decode_events(&mut &data.0[..])?;
                    for (_, raw) in raw_events {
                        match raw {
                            Raw::Event(event) => {
                                events.push(event);
                            },
                            Raw::Error(err) => {
                                error!("{:#?}", err);
                            }
                        }
                    }
                }
            }
        }

        Ok(events)
    }
}
