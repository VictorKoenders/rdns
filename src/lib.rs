#![no_std]

mod common;

/// Contains structs for parsing DNS requests, which are send from the client to the server.
pub mod request;

#[cfg(test)]
mod test;
