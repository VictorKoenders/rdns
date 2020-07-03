extern crate std;

use super::{
    question::{DnsQuestion, DnsQuestionIterator},
    DnsResponseBuilderNoHeader,
};
use core::ops::Range;

pub struct DnsRequest<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) question_range: Range<usize>,
}

impl<'a> DnsRequest<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        let mut result = DnsRequest {
            data,
            question_range: (0..0),
        };

        let start = 12;
        let mut end = start;
        for _ in 0..result.question_count() {
            end += DnsQuestionIterator::calculate_len(&data[end..]);
        }
        result.question_range = start..end;
        result
    }

    fn get_u16(&self, offset: usize) -> u16 {
        self.data
            .get(offset..offset + 2)
            .and_then(crate::utils::to_u16)
            .unwrap_or_default()
    }
    fn is_bit_set(&self, offset: usize, mask: u8) -> bool {
        self.data.get(offset).unwrap_or(&0) & mask == mask
    }

    pub fn identification(&self) -> u16 {
        self.get_u16(0)
    }
    pub fn is_query(&self) -> bool {
        !self.is_bit_set(2, 0b1000_0000)
    }

    pub fn is_reply(&self) -> bool {
        self.is_bit_set(2, 0b1000_0000)
    }

    pub fn query_type(&self) -> QueryType {
        self.data
            .get(2)
            .map(|high| {
                let bits = (high >> 3) & 0xF;
                QueryType::from(bits)
            })
            .unwrap_or_default()
    }

    pub fn is_authoritative_answer(&self) -> bool {
        self.is_bit_set(2, 0b0000_0100)
    }

    pub fn is_cache_answer(&self) -> bool {
        !self.is_bit_set(2, 0b0000_0100)
    }

    pub fn is_truncated(&self) -> bool {
        self.is_bit_set(2, 0b0000_0010)
    }
    pub fn recursion_desired(&self) -> bool {
        self.is_bit_set(2, 0b0000_0001)
    }
    pub fn recursion_available(&self) -> bool {
        self.is_bit_set(3, 0b1000_0000)
    }
    pub fn response_code(&self) -> ResponseCode {
        self.data
            .get(3)
            .map(|low| ResponseCode::from(low & 0xF))
            .unwrap_or_default()
    }
    pub fn question_count(&self) -> u16 {
        self.get_u16(4)
    }
    pub fn answer_count(&self) -> u16 {
        self.get_u16(6)
    }
    pub fn authority_count(&self) -> u16 {
        self.get_u16(8)
    }
    pub fn additional_count(&self) -> u16 {
        self.get_u16(10)
    }
    pub fn questions(&self) -> impl Iterator<Item = DnsQuestion<'a>> {
        let question_range = self.question_range.clone();
        DnsQuestionIterator(&self.data[question_range])
    }

    pub fn build_reply_to<'b, T: std::io::Write>(
        self,
        writer: &'b mut T,
        question: DnsQuestion<'a>,
    ) -> DnsResponseBuilderNoHeader<'a, 'b, T> {
        DnsResponseBuilderNoHeader::new(self, question, writer)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum QueryType {
    Query,
    InverseQuery,
    Status,
    Unassigned,
    Notify,
    Update,
    DnsStatefulOperation,
    Unknown(u8),
}

impl Default for QueryType {
    fn default() -> Self {
        QueryType::Unknown(u8::max_value())
    }
}

impl From<u8> for QueryType {
    fn from(val: u8) -> QueryType {
        match val {
            0 => QueryType::Query,
            1 => QueryType::InverseQuery,
            2 => QueryType::Status,
            3 => QueryType::Unassigned,
            4 => QueryType::Notify,
            5 => QueryType::Update,
            6 => QueryType::DnsStatefulOperation,
            x => QueryType::Unknown(x),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ResponseCode {
    NoError,
    FormErr,
    ServFail,
    NXDomain,
    NotImp,
    Refused,
    YXDomain,
    YXRRSet,
    NXRRSet,
    NotAuth,
    NotZone,
    DSOTYPENI,
    Unassigned(u8),
}

impl Default for ResponseCode {
    fn default() -> Self {
        ResponseCode::Unassigned(u8::max_value())
    }
}

impl From<u8> for ResponseCode {
    fn from(val: u8) -> ResponseCode {
        match val {
            0 => ResponseCode::NoError,
            1 => ResponseCode::FormErr,
            2 => ResponseCode::ServFail,
            3 => ResponseCode::NXDomain,
            4 => ResponseCode::NotImp,
            5 => ResponseCode::Refused,
            6 => ResponseCode::YXDomain,
            7 => ResponseCode::YXRRSet,
            8 => ResponseCode::NXRRSet,
            9 => ResponseCode::NotAuth,
            10 => ResponseCode::NotZone,
            11 => ResponseCode::DSOTYPENI,
            x => ResponseCode::Unassigned(x),
        }
    }
}
