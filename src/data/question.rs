use super::SliceExt;

pub struct DnsQuestionIterator<'a>(pub &'a [u8]);

impl DnsQuestionIterator<'_> {
    pub fn calculate_name_length(slice: &[u8]) -> usize {
        let mut total_len = 0;
        loop {
            let name_len = match slice.get(total_len) {
                Some(n) => *n as usize,
                None => return 0,
            };
            if name_len == 0 {
                break;
            }
            let name_len = name_len + 1;
            total_len += name_len;
        }
        total_len
    }

    pub fn calculate_len(slice: &[u8]) -> usize {
        Self::calculate_name_length(slice)
            + 1 // names has an extra 0-byte
            + 2 // qtype is 2 bytes
            + 2 // qclass is 2 bytes
    }
}

impl<'a> Iterator for DnsQuestionIterator<'a> {
    type Item = DnsQuestion<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let name_len = Self::calculate_name_length(self.0);
        let (names, remaining) = self.0.try_split_at(name_len)?;
        let (zero, remaining) = remaining.split_first()?;
        if zero != &0 {
            return None;
        }

        let (qtype, remaining) = remaining.try_split_at(2)?;
        let (qclass, remaining) = remaining.try_split_at(2)?;
        self.0 = remaining;

        let qtype = QType::from(crate::utils::to_u16(qtype)?);
        let qclass = crate::utils::to_u16(qclass)?;
        Some(DnsQuestion {
            names,
            qtype,
            qclass,
        })
    }
}

pub struct DnsQuestion<'a> {
    pub(crate) names: &'a [u8],
    pub qtype: QType,
    pub qclass: u16,
}

impl<'a> DnsQuestion<'a> {
    pub fn names(&self) -> impl Iterator<Item = &'a str> {
        DnsQuestionNameIter(self.names)
    }
}

struct DnsQuestionNameIter<'a>(&'a [u8]);

impl<'a> Iterator for DnsQuestionNameIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        let (len, remaining) = self.0.split_first()?;
        let (str, remaining) = remaining.try_split_at(*len as usize)?;
        self.0 = remaining;
        core::str::from_utf8(str).ok()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum QType {
    A,
    NS,
    CName,
    SOA,
    WKS,
    PTR,
    MX,
    SRV,
    AAAA,
    Any,
    Unknown(u16),
}

impl From<u16> for QType {
    fn from(v: u16) -> Self {
        match v {
            1 => QType::A,
            2 => QType::NS,
            5 => QType::CName,
            6 => QType::SOA,
            11 => QType::WKS,
            12 => QType::PTR,
            15 => QType::MX,
            28 => QType::AAAA,
            33 => QType::SRV,
            x => QType::Unknown(x),
        }
    }
}

impl QType {
    pub(crate) fn to_u16(self) -> [u8; 2] {
        match self {
            QType::A => [0, 1],
            _ => unimplemented!(),
        }
    }
}
