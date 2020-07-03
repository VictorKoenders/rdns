#![no_std]

pub mod data;
#[cfg(test)]
mod test;
pub(crate) mod utils;

pub fn request(data: &[u8]) -> data::DnsRequest {
    data::DnsRequest::new(data)
}
