use pdu_io::{PduSerialize, PduDeserialize, SizeOf};

use crate::pdu::PDU;

pub const STUN_LOOKUP_ID: u8 = 200;
pub const STUN_RESPONSE_ID: u8 = 201;

const STUN_LOOKUP_SIZE: usize = 1;
const STUN_RESPONSE_SIZE: usize = 1 + 4;

#[derive(PduSerialize, PduDeserialize)]
pub struct StunResponsePdu {
    pub pdu_type: u8,
    pub address: u32,
}

impl StunResponsePdu {
    pub fn new(address: u32) -> Self {
        StunResponsePdu {
            pdu_type: STUN_RESPONSE_ID,
            address,
        }
    }
}

impl From<StunResponsePdu> for PDU {
    fn from(pdu: StunResponsePdu) -> Self {
        Self::StunResponse(pdu)
    }
}

#[derive(PduSerialize, PduDeserialize)]
pub struct StunLookupPdu {
    pub pdu_type: u8,
}

impl StunLookupPdu {
    pub fn new() -> Self {
        StunLookupPdu {
            pdu_type: STUN_LOOKUP_ID,
        }
    }
}

impl From<StunLookupPdu> for PDU {
    fn from(pdu: StunLookupPdu) -> Self {
        Self::StunLookup(pdu)
    }
}
