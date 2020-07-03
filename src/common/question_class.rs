/// a two octet code that specifies the class of the query.
/// For example, the QCLASS field is IN for the Internet.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum QuestionClass {
    /// the internet
    Internet,
    /// Chaos
    Chaos,
    /// Hesiod
    Hesiod,
    /// None
    None,
    /// Any
    Any,
    /// Unassigned
    Unassigned(u16),
    /// Reserved
    Reserved(u16),
}

impl From<u16> for QuestionClass {
    fn from(val: u16) -> Self {
        match val {
            1 => QuestionClass::Internet,
            3 => QuestionClass::Chaos,
            4 => QuestionClass::Hesiod,
            254 => QuestionClass::None,
            255 => QuestionClass::Any,
            x if x >= 0xFF00 => QuestionClass::Reserved(x),
            x => QuestionClass::Unassigned(x),
        }
    }
}
