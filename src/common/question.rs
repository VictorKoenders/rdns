use super::{LabelSequence, QuestionClass, QuestionType};
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
}

impl From<Error> for crate::request::Error {
    fn from(e: Error) -> crate::request::Error {
        match e {
            Error::InvalidLabelSequence => crate::request::Error::InvalidLabelSequence,
            Error::EndOfSlice { additional } => crate::request::Error::EndOfSlice { additional },
        }
    }
}
