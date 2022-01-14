use super::payload;
use crate::error::Result;
use minicbor::bytes::ByteVec;
use minicbor::{Decode, Encode};
use zarb_crypto::bls;
use zarb_crypto::bls::signatory::BLSSignatory;
use zarb_crypto::signatory::Signatory;
use zarb_crypto::stamp::Stamp;

pub struct Transaction {
    pub stamp: Stamp,
    pub sequence: i32,
    pub fee: i64,
    pub memo: String,
    pub payload: Box<dyn payload::Payload>,
    pub signatory: Option<Box<dyn Signatory>>,
}

#[derive(Encode, Decode)]
#[cbor(map)]
pub struct RawTransaction {
    #[n(1)]
    pub version: i32,
    #[n(2)]
    pub stamp: Stamp,
    #[n(3)]
    pub sequence: i32,
    #[n(4)]
    pub fee: i64,
    #[n(5)]
    pub payload_type: payload::Type,
    #[n(6)]
    pub payload: ByteVec,
    #[n(7)]
    pub memo: String,
    #[n(20)]
    pub public_key: Option<ByteVec>,
    #[n(21)]
    pub signature: Option<ByteVec>,
}

impl Transaction {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let raw: RawTransaction = minicbor::decode(data)?;
        let payload = Box::new(match raw.payload_type {
            payload::Type::Send => {
                minicbor::decode::<payload::send::SendPayload>(raw.payload.as_ref())?
            }
            _ => minicbor::decode::<payload::send::SendPayload>(raw.payload.as_ref())?,
        });

        let signatory: Option<Box<dyn Signatory>> = match raw.signature {
            Some(data) => {
                let sig = bls::signature::BLSSignature::from_bytes(data.as_ref())?;
                match raw.public_key {
                    Some(data) => {
                        let pub_key = bls::public_key::BLSPublicKey::from_bytes(data.as_ref())?;

                        Some(Box::new(BLSSignatory { pub_key, sig }))
                    }
                    None => None,
                }
            }
            None => None,
        };
        Ok(Transaction {
            stamp: raw.stamp,
            sequence: raw.sequence,
            fee: raw.fee,
            memo: raw.memo,
            payload,
            signatory,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn check_signature(&self) -> Result<()> {
        let ok = self.signatory.as_ref().unwrap().as_ref().verify(&[1]);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        let buf1 = hex::decode(
            "a901010244e4f59ccd03186e041903e80501065833a3015501d75c059a4157d78f9b86741164037392de0fa53102550194f782f332649a4234b79216277e0b1594836313031903e8076c746573742073656e642d7478145860a4de42541ddeebfa6c4c8f008d2a64e6a2c8069096a5ad2fd807089a2f3ca8b71554365a01a2a3d5eee73f814b2aaeee0a49496e9222bc5cb4e9ffec219b4dca5091844ac1752286a524ca89928187ea60d0bdd6f10047d06f204bac5c215967155830b1c1b312df0ac1877c8daeb35eaf53c5008fb1de9654c698bab851b73d8730204c5c93c13c7d5d6b29ee439d1bdb7118",
        )
        .unwrap()
        .to_vec();
        let trx = Transaction::from_bytes(buf1.as_slice()).unwrap();
        //let mut buf2 = Vec::new();
        //minicbor::encode(&trx, &mut buf2).unwrap();
        //assert_eq!(buf1, buf2);
    }
}
