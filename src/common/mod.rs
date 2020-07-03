mod header;
mod question;
mod question_class;
mod question_type;

pub use self::header::{Header, Opcode, ResponseCode};
pub use self::question::{Error, LabelSequence, LabelSequenceIterator, Question, StrOrSlice};
pub use self::question_class::QuestionClass;
pub use self::question_type::QuestionType;
