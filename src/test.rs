// This test fails, need to figure out why
/*
extern crate std;
use std::vec::Vec;

#[test]
fn test_google_response() {
    let request: &[u8] = &[
        133, 97, 1, 32, 0, 1, 0, 0, 0, 0, 0, 1, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111, 109,
        0, 0, 1, 0, 1, 0, 0, 41, 16, 0, 0, 0, 0, 0, 0, 12, 0, 10, 0, 8, 22, 145, 22, 128, 66, 142,
        99, 205,
    ];
    let request = crate::request(request);

    let question = request
        .questions()
        .next()
        .expect("Query does not have an answer");
    let mut writer = Vec::new();
    request
        .build_reply_to(&mut writer, question)
        .write_header()
        .and_then(|res| res.send_ipv4_addresses(&[[172, 217, 20, 110]]))
        .expect("Could not build reply");

    let expected_response: &[u8] = &[
        133, 97, 129, 128, 0, 1, 0, 1, 0, 0, 0, 1, 6, 103, 111, 111, 103, 108, 101, 3, 99, 111,
        109, 0, 0, 1, 0, 1, 192, 12, 0, 1, 0, 1, 0, 0, 0, 104, 0, 4, 172, 217, 20, 110, 0, 0, 41,
        2, 0, 0, 0, 0, 0, 0, 0,
    ];

    /* Dig response:
     *
     * ; (1 server found)
     * ;; global options: +cmd
     * ;; Got answer:
     * ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 34145
     * ;; flags: qr rd ra; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 1
     *
     * ;; OPT PSEUDOSECTION:
     * ; EDNS: version: 0, flags:; udp: 512
     * ;; QUESTION SECTION:
     * ;google.com.                    IN      A
     *
     * ;; ANSWER SECTION:
     * google.com.             104     IN      A       172.217.20.110
     *
     * ;; Query time: 19 msec
     * ;; SERVER: 127.0.0.1#5553(127.0.0.1)
     * ;; WHEN: Tue May 26 09:17:57 CEST 2020
     * ;; MSG SIZE  rcvd: 55
     */
    std::println!(
        "Flags: {:08b} {:08b}",
        expected_response[2],
        expected_response[3]
    );

    assert_eq!(expected_response, writer.as_slice());
}
*/
