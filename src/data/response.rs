extern crate std;
use super::{DnsQuestion, DnsRequest};

struct DnsResponseBuilder<'a, 'b, T> {
    source: DnsRequest<'a>,
    source_question: DnsQuestion<'a>,
    writer: &'b mut T,
}

pub struct DnsResponseBuilderNoHeader<'a, 'b, T>(DnsResponseBuilder<'a, 'b, T>);

// TODO: Port this to no_std when rust supports this
// https://github.com/rust-lang/rust/issues/48331
impl<'a, 'b, T> DnsResponseBuilderNoHeader<'a, 'b, T>
where
    T: std::io::Write,
{
    pub(crate) fn new(
        source: DnsRequest<'a>,
        source_question: DnsQuestion<'a>,
        writer: &'b mut T,
    ) -> Self {
        Self(DnsResponseBuilder {
            source,
            source_question,
            writer,
        })
    }

    pub fn write_header(self) -> std::io::Result<DnsResponseBuilderAfterHeader<'a, 'b, T>> {
        self.0
            .writer
            .write_all(&crate::utils::from_u16(self.0.source.identification()))?;

        // TODO: These need to be configurable
        let high: u8 = 0b1000_0001; // reply and recursion desired
        let low: u8 = 0b1000_0000; // recursion available

        self.0.writer.write_all(&[high, low])?;

        Ok(DnsResponseBuilderAfterHeader(self.0))
    }
}

pub struct DnsResponseBuilderAfterHeader<'a, 'b, T>(DnsResponseBuilder<'a, 'b, T>);

// TODO: Port this to no_std when rust supports this
// https://github.com/rust-lang/rust/issues/48331
impl<'a, 'b, T> DnsResponseBuilderAfterHeader<'a, 'b, T>
where
    T: std::io::Write,
{
    pub fn send_ipv4_addresses(self, addresses: &[[u8; 4]]) -> std::io::Result<()> {
        let DnsResponseBuilder {
            writer,
            source,
            source_question,
        } = self.0;
        let _ = source;

        writer.write_all(&[0, 1])?; // We got 1 question
        writer.write_all(&crate::utils::from_u16(addresses.len() as u16))?;
        writer.write_all(&[0, 0])?; // no authorities
        writer.write_all(&[0, 1])?; // We have 1 additional parameter

        const TTL: u32 = 0;

        for address in addresses {
            writer.write_all(source_question.names)?;
            writer.write_all(&[0u8])?; // extra 0byte at the end of the names
            writer.write_all(&source_question.qtype.to_u16())?;
            writer.write_all(&crate::utils::from_u16(source_question.qclass))?;

            // expected: 192, 12, 0, 1, 0, 1,

            writer.write_all(&crate::utils::from_u32(TTL))?;
            writer.write_all(&[0, 4])?; // size of ipv4
            writer.write_all(address)?;
        }
        // missing: 0, 0, 41, 2, 0, 0, 0, 0, 0, 0, 0
        Ok(())
    }
}
