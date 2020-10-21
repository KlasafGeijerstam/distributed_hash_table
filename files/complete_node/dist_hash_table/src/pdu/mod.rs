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

#[cfg(test)]
mod serialization_test {
    use crate::pdu::PDU::*;
    use crate::pdu::*;
    #[test]
    fn test_net_alive() {
        let a = NetAlivePdu::new();
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), NET_ALIVE_SIZE);
        let (a, b) = NetAlivePdu::try_parse(&b).unwrap();
        assert_eq!(b, NET_ALIVE_SIZE);
    }

    #[test]
    fn test_net_get_node() {
        let a = NetGetNodePdu::new();
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), NET_GET_NODE_SIZE);
        let (a, b) = NetGetNodePdu::try_parse(&b).unwrap();
        assert_eq!(b, NET_GET_NODE_SIZE);
    }

    #[test]
    fn test_net_get_node_response() {
        let a = NetGetNodeResponsePdu::new(123456, 1234);
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), NET_GET_NODE_RESPONSE_SIZE);
        let (a, b) = NetGetNodeResponsePdu::try_parse(&b).unwrap();
        assert_eq!(b, NET_GET_NODE_RESPONSE_SIZE);
        assert_eq!(123456, a.address);
        assert_eq!(1234, a.port);
    }

    #[test]
    fn test_net_join() {
        let a = NetJoinPdu::new(1010, 1011, 10, 1122, 1123);
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), NET_JOIN_SIZE);
        let (a, b) = NetJoinPdu::try_parse(&b).unwrap();
        assert_eq!(b, NET_JOIN_SIZE);
        assert_eq!(1010, a.src_address);
        assert_eq!(1011, a.src_port);
        assert_eq!(10, a.max_span);
        assert_eq!(1122, a.max_address);
        assert_eq!(1123, a.max_port);
    }

    #[test]
    fn test_net_join_response() {
        let a = NetJoinResponsePdu::new(123456, 1234, 10, 20);
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), NET_JOIN_RESPONSE_SIZE);
        let (a, b) = NetJoinResponsePdu::try_parse(&b).unwrap();
        assert_eq!(b, NET_JOIN_RESPONSE_SIZE);
        assert_eq!(123456, a.next_address);
        assert_eq!(1234, a.next_port);
        assert_eq!(10, a.range_start);
        assert_eq!(20, a.range_end);
    }

    #[test]
    fn test_net_close_connection() {
        let a = NetCloseConnectionPdu::new();
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), NET_CLOSE_CONNECTION_SIZE);
        let (a, b) = NetCloseConnectionPdu::try_parse(&b).unwrap();
        assert_eq!(b, NET_CLOSE_CONNECTION_SIZE);
    }

    #[test]
    fn test_net_new_range() {
        let a = NetNewRangePdu::new(1, 255);
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), NET_NEW_RANGE_SIZE);
        let (a, b) = NetNewRangePdu::try_parse(&b).unwrap();
        assert_eq!(b, NET_NEW_RANGE_SIZE);
        assert_eq!(a.range_start, 1);
        assert_eq!(a.range_end, 255);
    }

    #[test]
    fn test_net_new_range_response() {
        let a = NetNewRangeResponsePdu::new();
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), NET_NEW_RANGE_RESPONSE_SIZE);
        let (a, b) = NetNewRangeResponsePdu::try_parse(&b).unwrap();
        assert_eq!(b, NET_NEW_RANGE_RESPONSE_SIZE);
        assert_eq!(a.pdu_type, NET_NEW_RANGE_RESPONSE_ID);
    }

    #[test]
    fn test_net_leaving() {
        let a = NetLeavingPdu::new(12345, 255);
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), NET_LEAVING_SIZE);
        let (a, b) = NetLeavingPdu::try_parse(&b).unwrap();
        assert_eq!(b, NET_LEAVING_SIZE);
        assert_eq!(a.new_address, 12345);
        assert_eq!(a.new_port, 255);
    }

    #[test]
    fn test_val_insert() {
        let ssn = "111111111111".to_owned();
        let name = "Test".to_owned();
        let email = "Emai".to_owned();
        let a = ValInsertPdu::new(ssn.clone(), name.clone(), email.clone());
        let b: Vec<u8> = a.into();
        let len = 1 + SSN_LENGTH + 1 + name.len() + 1 + email.len();
        assert_eq!(b.len(), len);
        let (a, b) = ValInsertPdu::try_parse(&b).unwrap();
        assert_eq!(b, len);
        assert_eq!(a.ssn, ssn);
        assert_eq!(a.name_length, name.len() as u8);
        assert_eq!(a.name, name);
        assert_eq!(a.email_length, email.len() as u8);
        assert_eq!(a.email, email);
    }

    #[test]
    fn test_val_remove() {
        let ssn = "111111111111".to_owned();
        let a = ValRemovePdu::new(ssn.clone());
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), VAL_REMOVE_SIZE);
        let (a, b) = ValRemovePdu::try_parse(&b).unwrap();
        assert_eq!(b, VAL_REMOVE_SIZE);
        assert_eq!(a.ssn, ssn);
    }

    #[test]
    fn test_val_lookup() {
        let ssn = "111111111111".to_owned();
        let a = ValLookupPdu::new(ssn.clone(), 12345, 1234);
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), VAL_LOOKUP_SIZE);
        let (a, b) = ValLookupPdu::try_parse(&b).unwrap();
        assert_eq!(b, VAL_LOOKUP_SIZE);
        assert_eq!(a.ssn, ssn);
        assert_eq!(a.sender_address, 12345);
        assert_eq!(a.sender_port, 1234);
    }

    #[test]
    fn test_val_lookup_response() {
        let ssn = "111111111111".to_owned();
        let name = "Test".to_owned();
        let email = "Emai".to_owned();
        let a = ValLookupResponsePdu::new(ssn.clone(), name.clone(), email.clone());
        let b: Vec<u8> = a.into();
        let len = 1 + SSN_LENGTH + 1 + name.len() + 1 + email.len();
        assert_eq!(b.len(), len);
        let (a, b) = ValLookupResponsePdu::try_parse(&b).unwrap();
        assert_eq!(b, len);
        assert_eq!(a.ssn, ssn);
        assert_eq!(a.name_length, name.len() as u8);
        assert_eq!(a.name, name);
        assert_eq!(a.email_length, email.len() as u8);
        assert_eq!(a.email, email);
    }

    #[test]
    fn test_stun_lookup() {
        let a = StunLookupPdu::new();
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), STUN_LOOKUP_SIZE);
        let (a, b) = StunLookupPdu::try_parse(&b).unwrap();
        assert_eq!(b, STUN_LOOKUP_SIZE);
    }

    #[test]
    fn test_stun_response() {
        let a = StunResponsePdu::new(12345);
        let b: Vec<u8> = a.into();
        assert_eq!(b.len(), STUN_RESPONSE_SIZE);
        let (a, b) = StunResponsePdu::try_parse(&b).unwrap();
        assert_eq!(b, STUN_RESPONSE_SIZE);
        assert_eq!(a.address, 12345);
    }
}
