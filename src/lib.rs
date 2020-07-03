#![no_std]

pub mod data;
#[cfg(test)]
mod test;
pub(crate) mod utils;

pub use data::Error as RequestError;

pub fn parse_request(data: &[u8]) -> Result<data::DnsRequest, RequestError> {
    data::DnsRequest::new(data)
}
