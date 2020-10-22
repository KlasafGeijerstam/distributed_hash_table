use std::net::{Ipv4Addr, SocketAddr};

use pdu_io::{PduSerialize, PduDeserialize, SizeOf};

use crate::pdu::PDU;

pub const NET_ALIVE_ID: u8 = 0;
pub const NET_GET_NODE_ID: u8 = 1;
pub const NET_GET_NODE_RESPONSE_ID: u8 = 2;
pub const NET_JOIN_ID: u8 = 3;
pub const NET_JOIN_RESPONSE_ID: u8 = 4;
pub const NET_CLOSE_CONNECTION_ID: u8 = 5;
pub const NET_NEW_RANGE_ID: u8 = 6;
pub const NET_LEAVING_ID: u8 = 7;
pub const NET_NEW_RANGE_RESPONSE_ID: u8 = 8;


const NET_ALIVE_SIZE: usize = 1;
const NET_GET_NODE_SIZE: usize = 1;
const NET_GET_NODE_RESPONSE_SIZE: usize = 1 + 4 + 2;
const NET_JOIN_SIZE: usize = 1 + 4 + 2 + 1 + 4 + 2;
const NET_JOIN_RESPONSE_SIZE: usize = 1 + 4 + 2 + 1 + 1;
const NET_CLOSE_CONNECTION_SIZE: usize = 1;
const NET_NEW_RANGE_SIZE: usize = 1 + 1 + 1;
const NET_LEAVING_SIZE: usize = 1 + 4 + 2;
const NET_NEW_RANGE_RESPONSE_SIZE: usize = 1;

#[derive(PduSerialize, PduDeserialize)]
pub struct NetGetNodeResponsePdu {
    pub pdu_type: u8,
    pub address: u32,
    pub port: u16,
}

impl NetGetNodeResponsePdu {
    pub fn new(address: u32, port: u16) -> Self {
        NetGetNodeResponsePdu {
            pdu_type: NET_GET_NODE_RESPONSE_ID,
            address,
            port,
        }
    }

    pub fn get_addr(&self) -> SocketAddr {
        let ip: Ipv4Addr = self.address.into();
        (ip, self.port).into()
    }
}

impl From<NetGetNodeResponsePdu> for PDU {
    fn from(pdu: NetGetNodeResponsePdu) -> Self {
        Self::NetGetNodeResponse(pdu)
    }
}

#[derive(PduSerialize, PduDeserialize)]
pub struct NetAlivePdu {
    pub pdu_type: u8,
}

impl NetAlivePdu {
    pub fn new() -> Self {
        NetAlivePdu {
            pdu_type: NET_ALIVE_ID,
        }
    }
}

impl From<NetAlivePdu> for PDU {
    fn from(pdu: NetAlivePdu) -> Self {
        Self::NetAlive(pdu)
    }
}

#[derive(PduSerialize, PduDeserialize)]
pub struct NetGetNodePdu {
    pub pdu_type: u8,
}

impl NetGetNodePdu {
    pub fn new() -> Self {
        NetGetNodePdu {
            pdu_type: NET_GET_NODE_ID,
        }
    }
}


impl From<NetGetNodePdu> for PDU {
    fn from(pdu: NetGetNodePdu) -> Self {
        Self::NetGetNode(pdu)
    }
}

#[derive(PduSerialize, PduDeserialize)]
pub struct NetCloseConnectionPdu {
    pub pdu_type: u8,
}

impl NetCloseConnectionPdu {
    pub fn new() -> Self {
        NetCloseConnectionPdu {
            pdu_type: NET_CLOSE_CONNECTION_ID,
        }
    }
}

impl From<NetCloseConnectionPdu> for PDU {
    fn from(pdu: NetCloseConnectionPdu) -> Self {
        Self::NetCloseConnection(pdu)
    }
}


#[derive(PduSerialize, PduDeserialize)]
pub struct NetJoinPdu {
    pub pdu_type: u8,
    pub src_address: u32,
    pub src_port: u16,
    pub max_span: u8,
    pub max_address: u32,
    pub max_port: u16,
}

impl NetJoinPdu {
    pub fn new(
        src_address: u32,
        src_port: u16,
        max_span: u8,
        max_address: u32,
        max_port: u16,
    ) -> Self {
        NetJoinPdu {
            pdu_type: NET_JOIN_ID,
            src_address,
            src_port,
            max_span,
            max_address,
            max_port,
        }
    }

    pub fn get_max_socket_addr(&self) -> SocketAddr {
        let ip: Ipv4Addr = self.max_address.into();
        (ip, self.max_port).into()
    }

    pub fn get_src_socket_addr(&self) -> SocketAddr {
        let ip: Ipv4Addr = self.src_address.into();
        (ip, self.src_port).into()
    }
}

impl From<NetJoinPdu> for PDU {
    fn from(pdu: NetJoinPdu) -> Self {
        Self::NetJoin(pdu)
    }
}


#[derive(PduSerialize, PduDeserialize)]
pub struct NetJoinResponsePdu {
    pub pdu_type: u8,
    pub next_address: u32,
    pub next_port: u16,
    pub range_start: u8,
    pub range_end: u8,
}

impl NetJoinResponsePdu {
    pub fn new(next_address: u32, next_port: u16, range_start: u8, range_end: u8) -> Self {
        NetJoinResponsePdu {
            pdu_type: NET_JOIN_RESPONSE_ID,
            next_address,
            next_port,
            range_start,
            range_end,
        }
    }

    pub fn get_next_addr(&self) -> SocketAddr {
        let ip: Ipv4Addr = self.next_address.into();
        (ip, self.next_port).into()
    }
}

impl From<NetJoinResponsePdu> for PDU {
    fn from(pdu: NetJoinResponsePdu) -> Self {
        Self::NetJoinResponse(pdu)
    }
}

#[derive(PduSerialize, PduDeserialize)]
pub struct NetNewRangePdu {
    pub pdu_type: u8,
    pub range_start: u8,
    pub range_end: u8,
}

impl NetNewRangePdu {
    pub fn new(range_start: u8, range_end: u8) -> Self {
        NetNewRangePdu {
            pdu_type: NET_NEW_RANGE_ID,
            range_start,
            range_end,
        }
    }
}

impl From<NetNewRangePdu> for PDU {
    fn from(pdu: NetNewRangePdu) -> Self {
        Self::NetNewRange(pdu)
    }
}


#[derive(PduSerialize, PduDeserialize)]
pub struct NetNewRangeResponsePdu {
    pub pdu_type: u8,
}

impl NetNewRangeResponsePdu {
    pub fn new() -> Self {
        NetNewRangeResponsePdu {
            pdu_type: NET_NEW_RANGE_RESPONSE_ID,
        }
    }
}

impl From<NetNewRangeResponsePdu> for PDU {
    fn from(pdu: NetNewRangeResponsePdu) -> Self {
        Self::NetNewRangeResponse(pdu)
    }
}


#[derive(PduSerialize, PduDeserialize)]
pub struct NetLeavingPdu {
    pub pdu_type: u8,
    pub new_address: u32,
    pub new_port: u16,
}

impl NetLeavingPdu {
    pub fn new(new_address: u32, new_port: u16) -> Self {
        NetLeavingPdu {
            pdu_type: NET_LEAVING_ID,
            new_address,
            new_port,
        }
    }

    pub fn get_new_addr(&self) -> SocketAddr {
        let ip: Ipv4Addr = self.new_address.into();
        (ip, self.new_port).into()
    }
}

impl From<NetLeavingPdu> for PDU {
    fn from(pdu: NetLeavingPdu) -> Self {
        Self::NetLeaving(pdu)
    }
}
#[cfg(test)]
mod net_tests {
    use super::*;
    use pdu_io::ParsePdu;

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
}
