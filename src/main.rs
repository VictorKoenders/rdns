use std::net::{Ipv4Addr, UdpSocket};

use rdns::data::QType;

fn main() {
    let addr = (Ipv4Addr::new(0, 0, 0, 0), 53);
    let socket = UdpSocket::bind(addr).unwrap();

    println!("Bound on {:?}", socket.local_addr().unwrap());
    let mut buffer = [0u8; 1024];
    'outer: loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, addr)) => {
                println!("{:?}", &buffer[..size]);
                let request = rdns::parse_request(&buffer[..size]).unwrap();
                println!("[{:?}] Found {} request(s)", addr, request.question_count());
                for question in request.questions() {
                    print!(" - {:?} record of ", question.qtype);
                    for (i, part) in question.names().enumerate() {
                        if i != 0 {
                            print!(" - ");
                        }
                        print!("{}", part);
                    }
                    println!();
                }

                if let Some(question) = request.questions().next() {
                    if question.qtype == QType::A {
                        let mut response = Vec::new();

                        if let Err(e) = request
                            .build_reply_to(&mut response, question)
                            .write_header()
                            .and_then(|res| {
                                res.send_ipv4_addresses(&[Ipv4Addr::new(127, 0, 0, 1).octets()])
                            })
                        {
                            eprintln!("Could not build reply: {:?}", e);
                            continue 'outer;
                        }
                        println!("Responding with {:?}", response);
                        if let Err(e) = socket.send_to(&response, addr) {
                            eprintln!("Could not send reply: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Could not read from UDP: {:?}", e);
            }
        }
    }
}
