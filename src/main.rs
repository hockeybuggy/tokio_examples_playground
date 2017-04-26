extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_service;
extern crate tokio_proto;

use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};

pub struct LineCodec;

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove 'serialized frame'
            let line = buf.split_to(i);
            // remove trailing new line
            buf.split_to(1);

            match str::from_utf8(&line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}


use tokio_proto::pipeline::ServerProto;

pub struct LineProto;

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;

impl<T:  AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    type Request = String;
    type Response = String;

    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}

use tokio_service::Service;
use futures::{future, Future, BoxFuture};

struct Echo;

impl Service for Echo {
    type Request = String;
    type Response = String;

    type Error = io::Error;

    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}


use tokio_proto::TcpServer;


fn main() {
    let raw_addr = "0.0.0.0:2345"; // TODO get from envvar
    let addr = raw_addr.parse().unwrap();  // TODO better name?
    println!("Starting up server");
    println!("Listening on {}", addr);
    let server = TcpServer::new(LineProto, addr);
    server.serve(|| Ok(Echo));
}
