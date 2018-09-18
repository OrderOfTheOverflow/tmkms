use super::{BlockID, Ed25519Signature, PartsSetHeader, TendermintSignable, Time};
use bytes::BufMut;
use chrono::{DateTime, Utc};
use hex::encode_upper;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, PartialEq, Message)]
pub struct Proposal {
    #[prost(sint64, tag = "1")]
    height: i64,
    #[prost(sint64)]
    round: i64,
    #[prost(message)]
    timestamp: Option<Time>,
    #[prost(message)]
    block_parts_header: Option<PartsSetHeader>,
    #[prost(sint64)]
    pol_round: i64,
    #[prost(message)]
    pol_block_id: Option<BlockID>,
    #[prost(message)]
    signature: Option<Vec<u8>>,
}

pub const AMINO_NAME: &str = "tendermint/socketpv/SignProposalMsg";

#[derive(Clone, PartialEq, Message)]
#[amino_name = "tendermint/socketpv/SignProposalMsg"]
pub struct SignProposalMsg {
    #[prost(message, tag = "1")]
    proposal: Option<Proposal>,
}

impl TendermintSignable for SignProposalMsg {
    fn sign_bytes<B>(&mut self, sign_bytes: &mut B)
    where
        B: BufMut,
    {
        unimplemented!()
    }
    fn set_signature(&mut self, sig: &Ed25519Signature) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;
    use std::error::Error;

    #[test]
    fn test_serialization() {
        let dt = "2018-02-11T07:09:22.765Z".parse::<DateTime<Utc>>().unwrap();
        let t = Time {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        };
        let proposal = Proposal {
            height: 12345,
            round: 23456,
            timestamp: Some(t),
            block_parts_header: Some(PartsSetHeader {
                total: 111,
                hash: "blockparts".as_bytes().to_vec(),
            }),
            pol_round: -1,
            pol_block_id: None,
            signature: None,
        };
        let mut got = vec![];

        let _have = SignProposalMsg {
            proposal: Some(proposal),
        }.encode(&mut got);
        let want = vec![
            0x31, 0x5d, 0x48, 0x70, 0x4, 0xa, 0x2b, 0x8, 0xf2, 0xc0, 0x1, 0x10, 0xc0, 0xee, 0x2,
            0x1a, 0xe, 0x9, 0x22, 0xec, 0x7f, 0x5a, 0x0, 0x0, 0x0, 0x0, 0x15, 0x40, 0xf9, 0x98,
            0x2d, 0x22, 0xf, 0x8, 0xde, 0x1, 0x12, 0xa, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x70, 0x61,
            0x72, 0x74, 0x73, 0x28, 0x1,
        ];

        assert_eq!(got, want)
    }

    #[test]
    fn test_deserialization() {
        let dt = "2018-02-11T07:09:22.765Z".parse::<DateTime<Utc>>().unwrap();
        let t = Time {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        };
        let proposal = Proposal {
            height: 12345,
            round: 23456,
            timestamp: Some(t),
            block_parts_header: Some(PartsSetHeader {
                total: 111,
                hash: "blockparts".as_bytes().to_vec(),
            }),
            pol_round: -1,
            pol_block_id: None,
            signature: None,
        };
        let want = SignProposalMsg {
            proposal: Some(proposal),
        };

        let data = vec![
            0x31, 0x5d, 0x48, 0x70, 0x4, 0xa, 0x2b, 0x8, 0xf2, 0xc0, 0x1, 0x10, 0xc0, 0xee, 0x2,
            0x1a, 0xe, 0x9, 0x22, 0xec, 0x7f, 0x5a, 0x0, 0x0, 0x0, 0x0, 0x15, 0x40, 0xf9, 0x98,
            0x2d, 0x22, 0xf, 0x8, 0xde, 0x1, 0x12, 0xa, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x70, 0x61,
            0x72, 0x74, 0x73, 0x28, 0x1,
        ];

        match SignProposalMsg::decode(&data) {
            Ok(have) => assert_eq!(have, want),
            Err(err) => assert!(false, err.description().to_string()),
        }
    }
    // TODO Serialization with Signature should be fairly easy as the signature is just
    // an Option<bytes> now
}
