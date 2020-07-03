use crate::common::*;
use crate::request::Request;

#[test]
fn dig() {
    let input = include_bytes!("query_packet.txt");
    let request = Request::parse(input).expect("Could not parse query_packet.txt");

    assert!(request.header.is_query());
    assert_eq!(Opcode::Standard, request.header.opcode());
    assert!(request.header.is_recursion_desired());
    assert!(!request.header.is_recursion_available());
    assert_eq!(0b010, request.header.z());
    assert_eq!(1, request.header.question_count());

    let question = request.questions().next().unwrap().unwrap();
    let mut label_iter = question.label_sequence.iter();
    assert_eq!(Some(StrOrSlice::Str("google")), label_iter.next());
    assert_eq!(Some(StrOrSlice::Str("com")), label_iter.next());
    assert_eq!(None, label_iter.next());

    assert_eq!(QuestionType::A, question.r#type);
    assert_eq!(QuestionClass::Internet, question.class);
}
