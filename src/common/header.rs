use byteorder::{ByteOrder, NetworkEndian};

/// DNS header. See [RFC1035](https://tools.ietf.org/html/rfc1035#section-4.1.1) for more information.
///
/// The header section is always present.
/// The header includes fields that specify which of the remaining sections are present,
/// and also specify whether the message is a query or a response,
/// a standard query or some other opcode, etc.
pub struct Header<'a>(pub &'a [u8; Header::SIZE]);

impl<'a> Header<'a> {
    /// Headers are always 12 bytes.
    pub const SIZE: usize = 12;

    /// A 16 bit identifier assigned by the program that generates any kind of query.  This identifier is copied the corresponding reply and can be used by the requester to match up replies to outstanding queries.
    pub fn id(&self) -> u16 {
        NetworkEndian::read_u16(&self.0[..2])
    }

    /// Clients send a query to the server. The server responds with a response.
    pub fn is_query(&self) -> bool {
        !self.is_response()
    }

    /// Clients send a query to the server. The server responds with a response.
    pub fn is_response(&self) -> bool {
        self.0[2] & 0b1000_0000 > 0
    }

    /// A four bit field that specifies kind of query in this message. This value is set by the originator of a query and copied into the response.
    pub fn opcode(&self) -> Opcode {
        let opcode: u8 = (self.0[2] & 0b0111_1000) >> 3;
        opcode.into()
    }

    /// Authoritative Answer - this bit is valid in responses, and specifies that the responding name server is an authority for the domain name in question section.
    ///
    /// Note that the contents of the answer section may have multiple owner names because of aliases.  The AA bit corresponds to the name which matches the query name, or the first owner name in the answer section.
    pub fn is_authoritative_answer(&self) -> bool {
        (self.0[2] & 0b0000_0100) > 0
    }

    /// Specifies that this message was truncated due to length greater than that permitted on the transmission channel.
    pub fn is_truncated(&self) -> bool {
        (self.0[2] & 0b0000_0010) > 0
    }

    /// Set by the sender of the request if the server should attempt to resolve the query recursively if it does not have an answer readily available.
    pub fn is_recursion_desired(&self) -> bool {
        (self.0[2] & 0b0000_0001) > 0
    }

    /// Set by the server to indicate whether or not recursive queries are allowed.
    pub fn is_recursion_available(&self) -> bool {
        (self.0[3] & 0b1000_0000) > 0
    }

    /// Originally reserved for later use, but now used for DNSSEC queries.
    pub fn z(&self) -> u8 {
        (self.0[3] & 0b0111_0000) >> 4
    }

    /// Response code - this 4 bit field is set as part of responses.
    /// Set by the server to indicate the status of the response, i.e. whether or not it was successful or failed, and in the latter case providing details about the cause of the failure.
    pub fn response_code(&self) -> ResponseCode {
        (self.0[3] & 0b0000_1111).into()
    }

    /// an unsigned 16 bit integer specifying the number of entries in the question section.
    pub fn question_count(&self) -> u16 {
        NetworkEndian::read_u16(&self.0[4..6])
    }

    /// an unsigned 16 bit integer specifying the number of resource records in the answer section.
    pub fn answer_count(&self) -> u16 {
        NetworkEndian::read_u16(&self.0[6..8])
    }

    /// an unsigned 16 bit integer specifying the number of name server resource records in the authority records section.
    pub fn authority_count(&self) -> u16 {
        NetworkEndian::read_u16(&self.0[8..10])
    }

    /// an unsigned 16 bit integer specifying the number of resource records in the additional records section.
    pub fn additional_count(&self) -> u16 {
        NetworkEndian::read_u16(&self.0[10..12])
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
/// A four bit field that specifies kind of query in this message. This value is set by the originator of a query and copied into the response.
pub enum Opcode {
    /// a standard query (QUERY)
    Standard,
    /// an inverse query (IQUERY)
    Inverse,
    /// a server status request (STATUS)
    Status,
    /// reserved for future use
    Reserved(u8),
}
impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            0 => Opcode::Standard,
            1 => Opcode::Inverse,
            2 => Opcode::Status,
            x => Opcode::Reserved(x),
        }
    }
}

/// Response code - this 4 bit field is set as part of responses.
/// Set by the server to indicate the status of the response, i.e. whether or not it was successful or failed, and in the latter case providing details about the cause of the failure.
pub enum ResponseCode {
    /// No error condition
    NoError,

    /// Format error - The name server was unable to interpret the query.
    FormatError,

    /// Server failure - The name server was unable to process this query due to a problem with the name server.
    ServerFailure,

    /// Name Error - Meaningful only for responses from an authoritative name server, this code signifies that the domain name referenced in the query does not exist.
    NameError,

    /// Not Implemented - The name server does not support the requested kind of query.
    NotImplemented,

    /// Refused - The name server refuses to perform the specified operation for policy reasons.
    /// For example, a name server may not wish to provide the information to the particular requester, or a name server may not wish to perform a particular operation (e.g., zone transfer) for particular data.
    Refused,

    /// Reserved for future use.
    Reserved(u8),
}

impl From<u8> for ResponseCode {
    fn from(u: u8) -> Self {
        match u {
            x => ResponseCode::Reserved(x),
        }
    }
}
