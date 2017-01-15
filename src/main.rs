extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_proto;

use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf};

pub struct LineCodec;

impl Codec for LineCodec {
    type In = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            let line = buf.drain_to(i);
            buf.drain_to(1);

            return match str::from_utf8(&line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: String, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend_from_slice(msg.as_bytes());
        buf.push(b'\n');
        Ok(())
    }
}


use tokio_proto::pipeline::ServerProto;
use tokio_core::io::{Io, Framed};

pub struct LineProto;

impl<T: Io + 'static> ServerProto<T> for LineProto {
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

struct EchoRev;

impl Service for EchoRev {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let rev: String = req.chars().rev().collect();
        future::ok(rev).boxed()
    }
}


use tokio_proto::TcpServer;

fn main() {
    println!("Hello, world!");
    let addr = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(LineProto, addr);
    server.serve(|| Ok(EchoRev));
}
