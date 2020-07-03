use crate::common::*;
use crate::request::Request;

#[test]
fn dig_request() {
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

    assert_eq!(0, request.header.answer_count());
    assert_eq!(0, request.header.authority_count());
    assert_eq!(0, request.header.additional_count());
}

#[test]
fn google_response() {
    let input = include_bytes!("response_packet.txt");
    let request = Request::parse(input).expect("Could not parse response_packet.txt");

    assert!(request.header.is_response());
    assert_eq!(Opcode::Standard, request.header.opcode());
    assert!(request.header.is_recursion_desired());
    assert!(request.header.is_recursion_available());
    assert_eq!(0b000, request.header.z());
    assert_eq!(1, request.header.question_count());

    let question = request.questions().next().unwrap().unwrap();
    let mut label_iter = question.label_sequence.iter();
    assert_eq!(Some(StrOrSlice::Str("google")), label_iter.next());
    assert_eq!(Some(StrOrSlice::Str("com")), label_iter.next());
    assert_eq!(None, label_iter.next());

    assert_eq!(QuestionType::A, question.r#type);
    assert_eq!(QuestionClass::Internet, question.class);

    assert_eq!(1, request.header.answer_count());

    let answer = request.answers().next().unwrap().unwrap();
    let mut label_iter = answer.label_sequence.iter();
    assert_eq!(Some(StrOrSlice::Str("google")), label_iter.next());
    assert_eq!(Some(StrOrSlice::Str("com")), label_iter.next());
    assert_eq!(None, label_iter.next());
    let (ttl, ip) = match answer.r#type {
        AnswerType::A { ttl, ip } => (ttl, ip),
        // x => panic!("Expected AnswerType::A, got {:?}", x),
    };

    assert_eq!(201, ttl);
    assert_eq!([172, 217, 168, 206], ip);

    assert_eq!(0, request.header.authority_count());
    assert_eq!(0, request.header.additional_count());
}
