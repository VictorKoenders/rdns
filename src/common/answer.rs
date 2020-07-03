use super::{LabelSequence, QuestionClass, QuestionType};
use byteorder::{ByteOrder, NetworkEndian};
use core::ops::Range;

pub enum Error {
    EndOfSlice { additional: usize },
    InvalidLabelSequence,
    Answer(AnswerError),
}

#[derive(Debug)]
pub enum AnswerError {
    InvalidTypeData {
        r#type: QuestionType,
        expected: &'static [usize],
        parsed: usize,
    },
    UnimplementedConversion(QuestionType),
}

impl From<Error> for crate::request::Error {
    fn from(e: Error) -> crate::request::Error {
        match e {
            Error::EndOfSlice { additional } => crate::request::Error::EndOfSlice { additional },
            Error::InvalidLabelSequence => crate::request::Error::InvalidLabelSequence,
            Error::Answer(answer) => crate::request::Error::Answer(answer),
        }
    }
}

pub struct Answer<'a> {
    pub label_sequence: LabelSequence<'a>,
    pub r#type: AnswerType,
    pub class: QuestionClass,
}

impl<'a> Answer<'a> {
    pub fn parse(data: &'a [u8], start: usize) -> Result<(Answer<'a>, Range<usize>), Error> {
        let (label_sequence, offset) = LabelSequence::parse(data, start)?;
        if offset + 4 > data.len() {
            return Err(Error::EndOfSlice {
                additional: offset + 4 - data.len(),
            }
            .into());
        }
        let r#type = NetworkEndian::read_u16(&data[offset..offset + 2]);
        let class = NetworkEndian::read_u16(&data[offset + 2..offset + 4]);

        let (r#type, end) = AnswerType::parse(r#type, data, offset + 4)?;

        Ok((
            Answer {
                label_sequence,
                r#type,
                class: class.into(),
            },
            start..end,
        ))
    }
}

#[derive(Debug)]
pub enum AnswerType {
    A { ttl: u32, ip: [u8; 4] },
}

impl AnswerType {
    pub fn parse(r#type: u16, data: &[u8], start: usize) -> Result<(AnswerType, usize), Error> {
        match QuestionType::from(r#type) {
            QuestionType::A => {
                if start + 6 > data.len() {
                    return Err(Error::EndOfSlice {
                        additional: (start + 6) - data.len(),
                    });
                }
                let ttl = NetworkEndian::read_u32(&data[start..]);
                let len = NetworkEndian::read_u16(&data[start + 4..]) as usize;
                let start = start + 6;
                if start + len > data.len() {
                    return Err(Error::EndOfSlice {
                        additional: (start + len) - data.len(),
                    });
                }
                if len != 4 {
                    return Err(Error::Answer(AnswerError::InvalidTypeData {
                        r#type: QuestionType::A,
                        expected: &[4],
                        parsed: len,
                    }));
                }
                let mut ip = [0u8; 4];
                ip.copy_from_slice(&data[start..start + len]);
                Ok((AnswerType::A { ttl, ip }, start + len))
            }
            x => Err(Error::Answer(AnswerError::UnimplementedConversion(x))),
        }
    }
}
