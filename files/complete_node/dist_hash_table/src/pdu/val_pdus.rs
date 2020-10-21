use pdu_io::{ParsePdu, util::*};
use crate::pdu::{PDU, SSN_LENGTH};

pub const VAL_INSERT_ID: u8 = 100;
pub const VAL_REMOVE_ID: u8 = 101;
pub const VAL_LOOKUP_ID: u8 = 102;
pub const VAL_LOOKUP_RESPONSE_ID: u8 = 103;


const VAL_REMOVE_SIZE: usize = 1 + SSN_LENGTH;
const VAL_LOOKUP_SIZE: usize = 1 + SSN_LENGTH + 4 + 2;

pub struct ValInsertPdu {
    pub pdu_type: u8,
    pub ssn: String,
    pub name_length: u8,
    pub name: String,
    pub email_length: u8,
    pub email: String,
}

impl ValInsertPdu {
    pub fn new(ssn: String, name: String, email: String) -> Self {
        ValInsertPdu {
            pdu_type: VAL_INSERT_ID,
            name_length: name.len() as u8,
            email_length: email.len() as u8,
            ssn,
            name,
            email,
        }
    }
}

impl From<ValInsertPdu> for Vec<u8> {
    fn from(pdu: ValInsertPdu) -> Self {
        let mut v = Vec::new();
        v.push(pdu.pdu_type);
        v.extend(pdu.ssn.chars().take(SSN_LENGTH).map(|x| x as u8));
        v.push(pdu.name_length);
        v.extend(pdu.name.chars().map(|x| x as u8));
        v.push(pdu.email_length);
        v.extend(pdu.email.chars().map(|x| x as u8));
        v
    }
}

impl ParsePdu for ValInsertPdu {
    fn try_parse(buffer: &[u8]) -> Option<(Self, usize)> {
        if buffer.len() < 1 + SSN_LENGTH + 1 {
            return None;
        }

        let mut buffer = &buffer[..];
        let pdu_type = read_be_u8(&mut buffer);

        let (ssn, mut buffer) = buffer.split_at(SSN_LENGTH);
        let ssn = ssn.iter().map(|&x| x as char).collect();
        let name_length = read_be_u8(&mut buffer);

        if buffer.len() < name_length as usize {
            return None;
        }

        let (name, mut buffer) = buffer.split_at(name_length as usize);
        let name: String = name.iter().map(|&x| x as char).collect();

        let email_length = read_be_u8(&mut buffer);

        if buffer.len() < email_length as usize {
            return None;
        }

        let email = buffer
            .iter()
            .take(email_length as usize)
            .map(|&x| x as char)
            .collect();

        let pdu = ValInsertPdu {
            pdu_type,
            ssn,
            name_length,
            name,
            email_length,
            email,
        };
        //       Type   SSN    Name length  Name           Email length   Email
        let size = 1 + SSN_LENGTH + 1 + name_length as usize + 1 + email_length as usize;
        Some((pdu, size))
    }
}

impl From<ValInsertPdu> for PDU {
    fn from(pdu: ValInsertPdu) -> Self {
        Self::ValInsert(pdu)
    }
}

pub struct ValLookupPdu {
    pub pdu_type: u8,
    pub ssn: String,
    pub sender_address: u32,
    pub sender_port: u16,
}

impl ValLookupPdu {
    pub fn new(ssn: String, sender_address: u32, sender_port: u16) -> Self {
        ValLookupPdu {
            pdu_type: VAL_LOOKUP_ID,
            sender_address,
            sender_port,
            ssn,
        }
    }
}

impl From<ValLookupPdu> for Vec<u8> {
    fn from(pdu: ValLookupPdu) -> Self {
        let mut v = Vec::new();
        v.push(pdu.pdu_type);
        v.extend(pdu.ssn.chars().take(SSN_LENGTH).map(|x| x as u8));
        v.extend_from_slice(&pdu.sender_address.to_be_bytes());
        v.extend_from_slice(&pdu.sender_port.to_be_bytes());
        v
    }
}

impl ParsePdu for ValLookupPdu {
    fn try_parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let size = VAL_LOOKUP_SIZE;
        if buffer.len() < size {
            return None;
        }

        let mut buffer = &buffer[..];
        let pdu_type = read_be_u8(&mut buffer);
        let (ssn, mut buffer) = buffer.split_at(SSN_LENGTH);
        let pdu = ValLookupPdu {
            pdu_type,
            ssn: ssn.iter().map(|&x| x as char).collect(),
            sender_address: read_be_u32(&mut buffer),
            sender_port: read_be_u16(&mut buffer),
        };
        Some((pdu, size))
    }
}

impl From<ValLookupPdu> for PDU {
    fn from(pdu: ValLookupPdu) -> Self {
        Self::ValLookup(pdu)
    }
}

pub struct ValLookupResponsePdu {
    pub pdu_type: u8,
    pub ssn: String,
    pub name_length: u8,
    pub name: String,
    pub email_length: u8,
    pub email: String,
}

impl ValLookupResponsePdu {
    pub fn new(ssn: String, name: String, email: String) -> Self {
        ValLookupResponsePdu {
            pdu_type: VAL_LOOKUP_RESPONSE_ID,
            name_length: name.len() as u8,
            email_length: email.len() as u8,
            ssn,
            name,
            email,
        }
    }
}

impl From<ValLookupResponsePdu> for Vec<u8> {
    fn from(pdu: ValLookupResponsePdu) -> Self {
        let mut v = Vec::new();
        v.push(pdu.pdu_type);
        v.extend(pdu.ssn.chars().take(SSN_LENGTH).map(|x| x as u8));
        v.push(pdu.name_length);
        v.extend(pdu.name.chars().map(|x| x as u8));
        v.push(pdu.email_length);
        v.extend(pdu.email.chars().map(|x| x as u8));
        v
    }
}

impl ParsePdu for ValLookupResponsePdu {
    fn try_parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let (pdu, s) = ValInsertPdu::try_parse(buffer)?;
        let pdu = ValLookupResponsePdu {
            pdu_type: VAL_LOOKUP_ID,
            ssn: pdu.ssn,
            name_length: pdu.name_length,
            name: pdu.name,
            email_length: pdu.email_length,
            email: pdu.email,
        };
        Some((ValLookupResponsePdu { ..pdu }, s))
    }
}

impl From<ValLookupResponsePdu> for PDU {
    fn from(pdu: ValLookupResponsePdu) -> Self {
        Self::ValLookupResponse(pdu)
    }
}

pub struct ValRemovePdu {
    pub pdu_type: u8,
    pub ssn: String,
}

impl ValRemovePdu {
    pub fn new(ssn: String) -> Self {
        ValRemovePdu {
            pdu_type: VAL_REMOVE_ID,
            ssn,
        }
    }
}

impl From<ValRemovePdu> for Vec<u8> {
    fn from(pdu: ValRemovePdu) -> Self {
        let mut v = Vec::new();
        v.push(pdu.pdu_type);
        v.extend(pdu.ssn.chars().take(SSN_LENGTH).map(|x| x as u8));
        v
    }
}

impl ParsePdu for ValRemovePdu {
    fn try_parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let size = VAL_REMOVE_SIZE;
        if buffer.len() < size {
            return None;
        }

        let mut buffer = &buffer[..];

        let pdu = ValRemovePdu {
            pdu_type: read_be_u8(&mut buffer),
            ssn: buffer[..SSN_LENGTH].iter().map(|&x| x as char).collect(),
        };
        Some((pdu, size))
    }
}

impl From<ValRemovePdu> for PDU {
    fn from(pdu: ValRemovePdu) -> Self {
        Self::ValRemove(pdu)
    }
}
