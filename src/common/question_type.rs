/// a two octet code which specifies the type of the query.
/// The values for this field include all codes valid for a TYPE field, together with some more general codes which can match more than one type of RR.
///
/// This list is taken from https://en.wikipedia.org/wiki/List_of_DNS_record_types
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum QuestionType {
    /// Returns a 32-bit IPv4 address, most commonly used to map hostnames to an IP address of the host, but it is also used for DNSBLs, storing subnet masks in RFC 1101, etc.
    A,
    /// Delegates a DNS zone to use the given authoritative name servers
    NS,
    /// Alias of one name to another: the DNS lookup will continue by retrying the lookup with the new name.
    CNAME,
    /// Specifies authoritative information about a DNS zone, including the primary name server, the email of the domain administrator, the domain serial number, and several timers relating to refreshing the zone.
    SOA,
    /// Pointer to a canonical name. Unlike a CNAME, DNS processing stops and just the name is returned.
    /// The most common use is for implementing reverse DNS lookups, but other uses include such things as DNS-SD.
    PTR,
    /// Providing Minimal-Sized Responses to DNS Queries That Have QTYPE=ANY
    HINFO,
    /// Maps a domain name to a list of message transfer agents for that domain
    MX,
    /// Originally for arbitrary human-readable text in a DNS record.
    /// Since the early 1990s, however, this record more often carries machine-readable data, such as specified by RFC 1464, opportunistic encryption, Sender Policy Framework, DKIM, DMARC, DNS-SD, etc.
    TXT,
    /// Information about the responsible person(s) for the domain. Usually an email address with the @ replaced by a .
    RP,
    /// Location of database servers of an AFS cell.
    /// This record is commonly used by AFS clients to contact AFS cells outside their local domain.
    /// A Subtype of this record is used by the obsolete DCE/DFS file system.
    AFSDB,
    /// Signature record used in SIG(0) (RFC 2931) and TKEY (RFC 2930). RFC 3755 designated RRSIG as the replacement for SIG for use within DNSSEC.
    SIG,
    /// Used only for SIG(0) (RFC 2931) and TKEY (RFC 2930).
    /// RFC 3445 eliminated their use for application keys and limited their use to DNSSEC.
    //  RFC 3755 designates DNSKEY as the replacement within DNSSEC.
    //  RFC 4025 designates IPSECKEY as the replacement for use with IPsec.
    KEY,
    /// Returns a 128-bit IPv6 address, most commonly used to map hostnames to an IP address of the host.
    AAAA,
    /// Specifies a geographical location associated with a domain name
    LOC,
    /// Generalized service location record, used for newer protocols instead of creating protocol-specific records such as MX.
    SRV,
    /// Allows regular-expression-based rewriting of domain names which can then be used as URIs, further domain names to lookups, etc.
    NAPTR,
    /// Used with some cryptographic systems (not including DNSSEC) to identify a key management agent for the associated domain-name.
    /// Note that this has nothing to do with DNS Security.
    /// It is Informational status, rather than being on the IETF standards-track.
    /// It has always had limited deployment, but is still in use.
    KX,
    /// Stores PKIX, SPKI, PGP, etc.
    CERT,
    /// Alias for a name and all its subnames, unlike CNAME, which is an alias for only the exact name.
    /// Like a CNAME record, the DNS lookup will continue by retrying the lookup with the new name.
    DNAME,
    /// Specify lists of address ranges, e.g. in CIDR format, for various address families. Experimental.
    APL,
    /// The record used to identify the DNSSEC signing key of a delegated zone
    DS,
    /// Resource record for publishing SSH public host key fingerprints in the DNS System, in order to aid in verifying the authenticity of the host.
    /// RFC 6594 defines ECC SSH keys and SHA-256 hashes.
    /// See the IANA SSHFP RR parameters registry for details.
    SSHFP,
    /// Key record that can be used with IPsec
    IPSECKEY,
    /// Signature for a DNSSEC-secured record set. Uses the same format as the SIG record.
    RRSIG,
    /// Part of DNSSEC—used to prove a name does not exist. Uses the same format as the (obsolete) NXT record.
    NSEC,
    /// The key record used in DNSSEC. Uses the same format as the KEY record.
    DNSKEY,
    /// Used in conjunction with the FQDN option to DHCP
    DHCID,
    /// An extension to DNSSEC that allows proof of nonexistence for a name without permitting zonewalking
    NSEC3,
    /// Parameter record for use with NSEC3
    NSEC3PARAM,
    /// A record for DANE.
    /// RFC 6698 defines "The TLSA DNS resource record is used to associate a TLS server certificate or public key with the domain name where the record is found, thus forming a 'TLSA certificate association'".
    TLSA,
    /// Associates an S/MIME certificate with a domain name for sender authentication.
    SMIMEA,
    /// Method of separating the end-point identifier and locator roles of IP addresses.
    HIP,
    /// Child copy of DS record, for transfer to parent
    CDS,
    /// Child copy of DNSKEY record, for transfer to parent
    CDNSKEY,
    /// A DNS-based Authentication of Named Entities (DANE) method for publishing and locating OpenPGP public keys in DNS for a specific email address using an OPENPGPKEY DNS resource record.
    OPENPGPKEY,
    /// Specify a synchronization mechanism between a child and a parent DNS zone.
    /// Typical example is declaring the same NS records in the parent and the child zone
    CSYNC,
    /// Assigned by IANA although the RFC is in draft status.
    ZONEMD,
    /// A 48-bit IEEE Extended Unique Identifier.
    EUI48,
    /// A 64-bit IEEE Extended Unique Identifier.
    EUI64,
    /// A method of providing keying material to be used with TSIG that is encrypted under the public key in an accompanying KEY RR.
    TKEY,
    /// Can be used to authenticate dynamic updates as coming from an approved client, or to authenticate responses as coming from an approved recursive name server similar to DNSSEC.
    TSIG,
    /// Can be used for publishing mappings from hostnames to URIs.
    URI,
    /// DNS Certification Authority Authorization, constraining acceptable CAs for a host/domain
    CAA,
    /// Part of a deployment proposal for DNSSEC without a signed DNS root. See the IANA database and Weiler Spec for details. Uses the same format as the DS record.
    TA,
    /// For publishing DNSSEC trust anchors outside of the DNS delegation chain. Uses the same format as the DS record. RFC 5074 describes a way of using these records.
    DLV,
    /// Unknown question type
    Unknown(u16),
}

impl From<u16> for QuestionType {
    fn from(val: u16) -> Self {
        use QuestionType::*;
        match val {
            1 => A,
            2 => NS,
            5 => CNAME,
            6 => SOA,
            12 => PTR,
            13 => HINFO,
            15 => MX,
            16 => TXT,
            17 => RP,
            18 => AFSDB,
            24 => SIG,
            25 => KEY,
            28 => AAAA,
            29 => LOC,
            33 => SRV,
            35 => NAPTR,
            36 => KX,
            37 => CERT,
            39 => DNAME,
            42 => APL,
            43 => DS,
            44 => SSHFP,
            45 => IPSECKEY,
            46 => RRSIG,
            47 => NSEC,
            48 => DNSKEY,
            49 => DHCID,
            50 => NSEC3,
            51 => NSEC3PARAM,
            52 => TLSA,
            53 => SMIMEA,
            55 => HIP,
            59 => CDS,
            60 => CDNSKEY,
            61 => OPENPGPKEY,
            62 => CSYNC,
            63 => ZONEMD,
            108 => EUI48,
            109 => EUI64,
            249 => TKEY,
            250 => TSIG,
            256 => URI,
            257 => CAA,
            32768 => TA,
            32769 => DLV,
            x => Unknown(x),
        }
    }
}
