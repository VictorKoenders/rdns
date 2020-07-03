use core::ops::Range;

pub enum Error {
    EndOfSlice { additional: usize },
    InvalidLabelSequence,
}

impl From<Error> for super::question::Error {
    fn from(e: Error) -> super::question::Error {
        match e {
            Error::EndOfSlice { additional } => super::question::Error::EndOfSlice { additional },
            Error::InvalidLabelSequence => super::question::Error::InvalidLabelSequence,
        }
    }
}

impl From<Error> for super::answer::Error {
    fn from(e: Error) -> super::answer::Error {
        match e {
            Error::EndOfSlice { additional } => super::answer::Error::EndOfSlice { additional },
            Error::InvalidLabelSequence => super::answer::Error::InvalidLabelSequence,
        }
    }
}

pub struct LabelSequence<'a> {
    data: &'a [u8],
    range: Range<usize>,
}

impl<'a> LabelSequence<'a> {
    pub fn parse(data: &'a [u8], start: usize) -> Result<(LabelSequence<'a>, usize), Error> {
        let mut index = start;
        while index < data.len() {
            let part_len = data[index];

            if (part_len & 0xc0) == 0xc0 {
                // If len has the two most significant bits set, it represents a jump to some other offset in the packet
                index += 1;
                if index == data.len() {
                    return Err(Error::EndOfSlice { additional: 1 });
                }
                let start = data[index] as usize;
                return if let Some(end) = data.iter().skip(start + 1).position(|b| b == &0) {
                    Ok((
                        LabelSequence {
                            data,
                            range: start..end,
                        },
                        index + 1,
                    ))
                } else {
                    Err(Error::InvalidLabelSequence)
                };
            }

            index += 1;
            if part_len == 0 {
                return Ok((
                    LabelSequence {
                        data,
                        range: start..index,
                    },
                    index,
                ));
            }
            index += part_len as usize;
        }

        Err(Error::EndOfSlice {
            additional: index - data.len() + 1,
        })
    }

    pub fn iter(&self) -> LabelSequenceIterator<'a> {
        LabelSequenceIterator {
            data: self.data,
            offset: self.range.start,
        }
    }
}

impl<'a> IntoIterator for LabelSequence<'a> {
    type Item = StrOrSlice<'a>;
    type IntoIter = LabelSequenceIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct LabelSequenceIterator<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for LabelSequenceIterator<'a> {
    type Item = StrOrSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let len: u8 = *self.data.get(self.offset)?;
        if len == 0 {
            return None;
        }

        self.offset += 1;
        let end = self.offset + len as usize;
        assert!(self.data.len() > end);

        let slice = &self.data[self.offset..end];
        self.offset = end;

        Some(match core::str::from_utf8(slice) {
            Ok(str) => StrOrSlice::Str(str),
            Err(e) => StrOrSlice::Slice(slice, e),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StrOrSlice<'a> {
    Str(&'a str),
    Slice(&'a [u8], core::str::Utf8Error),
}

impl<'a> core::cmp::PartialEq<str> for StrOrSlice<'a> {
    fn eq(&self, other: &str) -> bool {
        match self {
            StrOrSlice::Str(s) => s == &other,
            StrOrSlice::Slice(slice, _) => slice == &other.as_bytes(),
        }
    }
}

impl<'a> core::cmp::PartialEq<[u8]> for StrOrSlice<'a> {
    fn eq(&self, other: &[u8]) -> bool {
        match self {
            StrOrSlice::Str(s) => s.as_bytes() == other,
            StrOrSlice::Slice(slice, _) => slice == &other,
        }
    }
}
