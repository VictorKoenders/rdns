extern crate std;

use super::*;
use std::vec::Vec;

#[test]
fn test_incoming_query() {
    let query = &[
        41, 115, // identification
        1, 0, // flags
        0, 1, // question count
        0, 0, // answer count
        0, 0, // authority count
        0, 0, // additional count
        6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109, 0, // question 1 names
        0, 1, // question 1 qtype
        0, 1, // question 1 qclass
    ];
    assert_eq!(28, query.len());

    let req = DnsRequest::new(query);

    assert_eq!(
        crate::utils::to_u16(&[41, 115]).unwrap(),
        req.identification()
    );
    assert_eq!(12..28, req.question_range);

    assert!(req.is_query());
    assert!(!req.is_reply());
    assert_eq!(QueryType::Query, req.query_type());
    assert!(!req.is_authoritative_answer());
    assert!(req.is_cache_answer());
    assert!(!req.is_truncated());
    assert_eq!(ResponseCode::NoError, req.response_code());
    assert_eq!(1, req.question_count());
    assert_eq!(0, req.answer_count());
    assert_eq!(0, req.authority_count());
    assert_eq!(0, req.additional_count());

    let questions: Vec<_> = req.questions().collect();
    assert_eq!(1, questions.len());
    let question = &questions[0];
    let names: Vec<_> = question.names().collect();
    assert_eq!("google", names[0]);
    assert_eq!("com", names[1]);
    assert_eq!(QType::A, question.qtype);
    assert_eq!(1, question.qclass);
}
