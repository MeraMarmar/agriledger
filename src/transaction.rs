use super::*;
use rsa::{RsaPublicKey};

#[derive(Debug, Clone,)]
pub struct Transaction {
    pub dapp: App,
    pub from: RsaPublicKey,
    pub to: RsaPublicKey,
    pub order: Order,
    pub time: u128,
}
impl Transaction {
    pub fn new(dapp: App, from: &RsaPublicKey, to: &RsaPublicKey, order: Order) -> Self {
        Self {
            dapp,
            from:from.to_owned(),
            to: to.to_owned(),
            order,
            time : now()
        }
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(bincode::serialize(&self.dapp).unwrap());
        bytes.extend(bincode::serialize(&self.from).unwrap());
        bytes.extend(bincode::serialize(&self.to).unwrap());
        bytes.extend(bincode::serialize(&self.order).unwrap());
        bytes.extend(bincode::serialize(&self.time).unwrap());
        
        bytes
    }

    fn hash(&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}
