use super::{QuestionClass, QuestionType};
use byteorder::{ByteOrder, NetworkEndian};
use core::ops::Range;

pub struct Question<'a> {
    pub label_sequence: LabelSequence<'a>,
    pub r#type: QuestionType,
    pub class: QuestionClass,
}

impl<'a> Question<'a> {
    pub fn parse(data: &'a [u8], start: usize) -> Result<(Question<'a>, Range<usize>), Error> {
        let (label_sequence, end_of_label_sequence) = LabelSequence::parse(data, start)?;

        // We need a length of <label_sequence.len()> + 2x u16 (4 bytes total)
        if end_of_label_sequence + 4 > data.len() {
            return Err(Error::EndOfSlice {
                additional: (end_of_label_sequence + 4) - data.len(),
            });
        }

        let type_start = end_of_label_sequence;
        let class_start = type_start + 2;
        let end = class_start + 2;

        let type_num = NetworkEndian::read_u16(&data[type_start..type_start + 2]);
        let class_num = NetworkEndian::read_u16(&data[class_start..class_start + 2]);

        Ok((
            Question {
                label_sequence,
                r#type: type_num.into(),
                class: class_num.into(),
            },
            start..end,
        ))
    }
}

pub enum Error {
    InvalidLabelSequence,
    EndOfSlice { additional: usize },
    JumpsNotImplemented,
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
                return Err(Error::JumpsNotImplemented);
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
