use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

use http::header;
use http::Request;

use std::net::SocketAddr;

fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();

    for _ in 0..10 {
        fetch_doc(&addr);
    }
}

fn fetch_doc(addr: &SocketAddr) {
    let client = TcpStream::connect(&addr)
        .and_then(|stream| {
            println!("created stream");

            let req = Request::builder()
                .uri("/main.html")
                .header(header::USER_AGENT, "tokio-learn")
                .header(header::HOST, "127.0.0.1:6142")
                .header(header::ACCEPT, "*/*")
                .body(())
                .unwrap();

            let headers = req
                .headers()
                .iter()
                .fold(String::from(""), |mut a, (k, v)| {
                    a.push_str(k.as_str());
                    a.push_str(": ");
                    a.push_str(v.to_str().unwrap());
                    a.push_str("\r\n");
                    a
                });

            let req_str = format!(
                "{} {} {:?}\r\n{}\r\n",
                req.method(),
                req.uri(),
                req.version(),
                headers
            );

            println!("{}", req_str);
            let (reader, wirter) = stream.split();

            io::write_all(wirter, req_str)
                .then(|result| {
                    println!("wrote to stream; success={:?}", result.is_ok());
                    let buf = Vec::new();
                    io::read_to_end(reader, buf)
                })
                .then(|result| {
                    println!("{}", String::from_utf8_lossy(&result.unwrap().1));
                    Ok(())
                })
        })
        .map_err(|err| {
            println!("connection error = {:?}", err);
        });

    tokio::run(client);
}
