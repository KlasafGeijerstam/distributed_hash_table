mod net_pdus;
pub use net_pdus::*;

mod stun_pdus;
pub use stun_pdus::*;

mod val_pdus;
pub use val_pdus::*;

const SSN_LENGTH: usize = 12;

pub enum PDU {
    NetAlive(NetAlivePdu),
    NetGetNode(NetGetNodePdu),
    NetGetNodeResponse(NetGetNodeResponsePdu),
    NetJoin(NetJoinPdu),
    NetJoinResponse(NetJoinResponsePdu),
    NetCloseConnection(NetCloseConnectionPdu),
    NetNewRange(NetNewRangePdu),
    NetNewRangeResponse(NetNewRangeResponsePdu),
    NetLeaving(NetLeavingPdu),
    ValInsert(ValInsertPdu),
    ValRemove(ValRemovePdu),
    ValLookup(ValLookupPdu),
    ValLookupResponse(ValLookupResponsePdu),
    StunLookup(StunLookupPdu),
    StunResponse(StunResponsePdu),
}

impl std::fmt::Debug for PDU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(match self {
            Self::NetAlive(_) => "NetAlive",
            Self::NetGetNode(_) => "NetGetNode",
            Self::NetGetNodeResponse(_) => "NetGetNodeResponse",
            Self::NetJoin(_) => "NetJoin",
            Self::NetJoinResponse(_) => "NetJoinResponse",
            Self::NetCloseConnection(_) => "NetCloseConnection",
            Self::NetNewRange(_) => "NetNewRange",
            Self::NetNewRangeResponse(_) => "NetNewRangeResponse",
            Self::NetLeaving(_) => "NetLeaving",
            Self::ValInsert(_) => "ValInsert",
            Self::ValRemove(_) => "ValRemove",
            Self::ValLookup(_) => "ValLookup",
            Self::ValLookupResponse(_) => "ValLookupResponse",
            Self::StunLookup(_) => "StunLookup",
            Self::StunResponse(_) => "StunResponse",
        })
        .finish()
    }
}

impl PDU {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Self::NetAlive(p) => Vec::from(p),
            Self::NetGetNode(p) => Vec::from(p),
            Self::NetGetNodeResponse(p) => Vec::from(p),
            Self::NetJoin(p) => Vec::from(p),
            Self::NetJoinResponse(p) => Vec::from(p),
            Self::NetCloseConnection(p) => Vec::from(p),
            Self::NetNewRange(p) => Vec::from(p),
            Self::NetNewRangeResponse(p) => Vec::from(p),
            Self::NetLeaving(p) => Vec::from(p),
            Self::ValInsert(p) => Vec::from(p),
            Self::ValRemove(p) => Vec::from(p),
            Self::ValLookup(p) => Vec::from(p),
            Self::ValLookupResponse(p) => Vec::from(p),
            Self::StunLookup(p) => Vec::from(p),
            Self::StunResponse(p) => Vec::from(p),
        }
    }
}
