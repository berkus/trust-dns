// Copyright 2015-2017 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! class of DNS operations, in general always IN for internet

use std::convert::From;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};

use serialize::binary::*;
use error::*;

/// The DNS Record class
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[allow(dead_code)]
pub enum DNSClass {
    /// Internet
    IN,
    /// Chaos
    CH,
    /// Hesiod
    HS,
    /// QCLASS NONE
    NONE,
    /// QCLASS * (ANY)
    ANY,
    /// Special class for OPT Version, it was overloaded for EDNS - RFC 6891
    OPT(u16),
}

impl DNSClass {
    /// Convert from &str to DNSClass
    ///
    /// ```
    /// use trust_dns_proto::rr::dns_class::DNSClass;
    ///
    /// let var: DNSClass = DNSClass::from_str("IN").unwrap();
    /// assert_eq!(DNSClass::IN, var);
    /// ```
    pub fn from_str(str: &str) -> ProtoResult<Self> {
        match str {
            "IN" => Ok(DNSClass::IN),
            "CH" => Ok(DNSClass::CH),
            "HS" => Ok(DNSClass::HS),
            "NONE" => Ok(DNSClass::NONE),
            "ANY" | "*" => Ok(DNSClass::ANY),
            _ => Err(ProtoErrorKind::UnknownDnsClassStr(str.to_string()).into()),
        }
    }


    /// Convert from u16 to DNSClass
    ///
    /// ```
    /// use trust_dns_proto::rr::dns_class::DNSClass;
    ///
    /// let var = DNSClass::from_u16(1).unwrap();
    /// assert_eq!(DNSClass::IN, var);
    /// ```
    pub fn from_u16(value: u16) -> ProtoResult<Self> {
        match value {
            1 => Ok(DNSClass::IN),
            3 => Ok(DNSClass::CH),
            4 => Ok(DNSClass::HS),
            254 => Ok(DNSClass::NONE),
            255 => Ok(DNSClass::ANY),
            _ => Err(ProtoErrorKind::UnknownDnsClassValue(value).into()),
        }
    }

    /// Return the OPT version from value
    pub fn for_opt(value: u16) -> Self {
        DNSClass::OPT(value)
    }
}

impl BinSerializable<DNSClass> for DNSClass {
    fn read(decoder: &mut BinDecoder) -> ProtoResult<Self> {
        Self::from_u16(try!(decoder.read_u16()))
    }

    fn emit(&self, encoder: &mut BinEncoder) -> ProtoResult<()> {
        encoder.emit_u16((*self).into())
    }
}

// TODO make these a macro or annotation

/// Convert from DNSClass to &str
///
/// ```
/// use trust_dns_proto::rr::dns_class::DNSClass;
///
/// let var: &'static str = DNSClass::IN.into();
/// assert_eq!("IN", var);
/// ```
impl From<DNSClass> for &'static str {
    fn from(rt: DNSClass) -> &'static str {
        match rt {
            DNSClass::IN => "IN",
            DNSClass::CH => "CH",
            DNSClass::HS => "HS",
            DNSClass::NONE => "NONE",
            DNSClass::ANY => "ANY",
            DNSClass::OPT(_) => "OPT",
        }
    }
}

/// Convert from DNSClass to u16
///
/// ```
/// use trust_dns_proto::rr::dns_class::DNSClass;
///
/// let var: u16 = DNSClass::IN.into();
/// assert_eq!(1, var);
/// ```
impl From<DNSClass> for u16 {
    fn from(rt: DNSClass) -> Self {
        match rt {
            DNSClass::IN => 1,
            DNSClass::CH => 3,
            DNSClass::HS => 4,
            DNSClass::NONE => 254,
            DNSClass::ANY => 255,
            DNSClass::OPT(version) => version,
        }
    }
}

impl PartialOrd<DNSClass> for DNSClass {
    fn partial_cmp(&self, other: &DNSClass) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DNSClass {
    fn cmp(&self, other: &Self) -> Ordering {
        u16::from(*self).cmp(&u16::from(*other))
    }
}

impl Display for DNSClass {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(Into::<&str>::into(*self))
    }
}

#[test]
fn test_order() {
    let ordered = vec![
        DNSClass::IN,
        DNSClass::CH,
        DNSClass::HS,
        DNSClass::NONE,
        DNSClass::ANY,
    ];
    let mut unordered = vec![
        DNSClass::NONE,
        DNSClass::HS,
        DNSClass::CH,
        DNSClass::IN,
        DNSClass::ANY,
    ];

    unordered.sort();

    assert_eq!(unordered, ordered);
}
