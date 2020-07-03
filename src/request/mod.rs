pub use crate::common::{Header, Question};

use crate::common::Error as QuestionError;
use arrayref::array_ref;
use core::ops::Range;

/// Errors that can occur while parsing a request
#[derive(Debug)]
pub enum Error {
    /// Reached the end of the slice, but at least `additional` bytes were needed.
    ///
    /// The implementation should gather up to `additional` bytes and then try again. The next attempt could request even more bytes.
    EndOfSlice {
        /// The number of bytes that should be collected before trying to parse again.
        additional: usize,
    },

    /// One of the label sequences in the request are invalid. This usually indicates a malformed packet.
    InvalidLabelSequence,

    /// [TODO] Jumps are not implemented at the moment
    JumpsNotImplemented,
}

impl From<QuestionError> for Error {
    fn from(e: QuestionError) -> Self {
        match e {
            QuestionError::EndOfSlice { additional } => Self::EndOfSlice { additional },
            QuestionError::InvalidLabelSequence => Self::InvalidLabelSequence,
            QuestionError::JumpsNotImplemented => Self::JumpsNotImplemented,
        }
    }
}

/// A client-to-server DNS request
pub struct Request<'a> {
    /// The header of the request
    pub header: Header<'a>,

    data: &'a [u8],
    question_range: Range<usize>,
}

impl<'a> Request<'a> {
    /// Attempt to parse a byte slice as a valid Request.
    pub fn parse(data: &'a [u8]) -> Result<Self, Error> {
        if data.len() < Header::SIZE {
            return Err(Error::EndOfSlice {
                additional: Header::SIZE - data.len(),
            });
        }
        let header = Header(array_ref!(data, 0, Header::SIZE));

        let mut question_range = Header::SIZE..Header::SIZE;
        for _ in 0..header.question_count() {
            let (_, range) = Question::parse(&data, question_range.end)?;
            question_range.end = range.end;
        }

        Ok(Self {
            header,
            data,
            question_range,
        })
    }

    /// Return an iterator that iterates the questions in this request.
    pub fn questions(&self) -> QuestionIterator {
        QuestionIterator {
            data: self.data,
            range: self.question_range.clone(),
        }
    }
}

/// An iterator that iterates a list of questions.
pub struct QuestionIterator<'a> {
    data: &'a [u8],
    range: Range<usize>,
}

impl<'a> Iterator for QuestionIterator<'a> {
    type Item = Result<Question<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.range.start == self.range.end {
            None
        } else {
            Some(match Question::parse(self.data, self.range.start) {
                Ok((question, range)) => {
                    self.range.start = range.end;
                    Ok(question)
                }
                Err(e) => Err(e.into()),
            })
        }
    }
}

// These were test situations that crashed in the fuzzer
#[test]
fn fuzz() {
    let _ = Request::parse(&[2, 2, 38, 145, 145, 145, 145, 208, 145, 38, 145, 145, 145]);
    let _ = Request::parse(&[0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 255, 0, 255, 0, 0, 0, 0, 150]);
}
