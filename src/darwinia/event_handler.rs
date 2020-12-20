use crate::result::Result;
use primitives::runtime::DarwiniaRuntime;
use substrate_subxt::sp_core::Decode;
use substrate_subxt::system::System;
use primitives::frame::{
    ethereum::backing::LockRing,
    bridge::relay_authorities::MMRRootSigned,
};
use super::database::DatabaseService;

pub struct EventHandler {
    db: DatabaseService
}

impl EventHandler {

    pub fn new(db: DatabaseService) -> EventHandler {
        EventHandler { db }
    }
    
    pub async fn handle(
        &self,
        header: &<DarwiniaRuntime as System>::Header,
        module: &str,
        variant: &str,
        event_data: Vec<u8>,
    ) -> Result<()> {
        if module != "System" {
            debug!(">> Event - {}::{}", module, variant);
        }

        match (module, variant) {
            ("EthereumBacking", "LockRing") => {
                if let Ok(decoded) = LockRing::<DarwiniaRuntime>::decode(&mut &event_data[..]) {
                    let account_id: &[u8] = decoded.account_id.as_ref();
                    let account_id = format!("0x{}", hex::encode(account_id));
                    let ecdsa_address = format!("0x{}", hex::encode(decoded.ecdsa_address));
                    let asset_type = decoded.asset_type;
                    let amount = decoded.amount / u128::pow(10, 9);

                    self.db.save_lock(header.number, account_id, ecdsa_address, asset_type, amount).await?;
                }
            },

            ("EthereumRelayAuthorities", "MMRRootSigned") => {
                 if let Ok(decoded) = MMRRootSigned::<DarwiniaRuntime>::decode(&mut &event_data[..]) {
                    self.db.save_signed_mmr_root(
                        header.number, 
                        decoded.block_number, 
                        format!("{:x?}", decoded.mmr_root), 
                        decoded.signatures.iter().map(|s| hex::encode(s.1.0)).collect::<Vec<_>>()
                    ).await?;
                 }
            }

            _ => {}
        }
        Ok(())
    }


}
