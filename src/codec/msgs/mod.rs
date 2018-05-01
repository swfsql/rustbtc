use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;
use futures::{Async, Poll};
use bytes::{BufMut, BytesMut};

pub mod msg;

use codec::msgs::msg::commons::new_from_hex::NewFromHex;


use hex::ToHex;

#[derive(Debug)]
pub struct Msgs {
    pub socket: TcpStream,
    rd: BytesMut,
    wr: BytesMut,
}

impl Msgs {
    pub fn new(socket: TcpStream) -> Self {
        Msgs {
            socket,
            rd: BytesMut::new(),
            wr: BytesMut::new(),
        }
    }

    pub fn buffer(&mut self, bytes: &[u8]) {
        self.wr.reserve(bytes.len());
        self.wr.put(bytes);
    }

    pub fn poll_flush(&mut self) -> Poll<(), io::Error> {
        while !self.wr.is_empty() {
            let n = try_ready!(self.socket.poll_write(&self.wr));
            assert!(n > 0);
            let _ = self.wr.split_to(n);
        }
        Ok(Async::Ready(()))
    }

    pub fn fill_read_buf(&mut self) -> Poll<(), io::Error> {
        loop {
            self.rd.reserve(1024);
            let n = try_ready!(self.socket.read_buf(&mut self.rd));
            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }
}

impl Stream for Msgs {
    type Item = msg::Msg;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        d!("msgs poll called");

        let sock_closed = self.fill_read_buf()?.is_ready();
        d!("after read_buf");

        if self.rd.len() < 24 {
            d!("has <24");
            d!("\n{}\n{:?}", self.rd.clone().into_iter().collect::<Vec<_>>().to_hex(), self.rd.clone());
            return Ok(Async::NotReady);
        }
        d!("has >=24");
        let rd_clone = self.rd.clone();
        let header = msg::header::Header::new(
            &mut rd_clone
                .into_iter()
                .take(24)
                .collect::<Vec<_>>()
                .into_iter())
            .unwrap();
        d!("after header made:\n {:?}",&header);
        if self.rd.iter().len() < header.payload_len as usize + 24usize {
            d!("not enought bytes for payload");
            return Ok(Async::NotReady);
        }
        d!("has enought bytes for payload");
        //let rd_split =
        let msg = msg::Msg::new(
            &mut self.rd
                .split_to(header.payload_len as usize + 24usize)
                .into_iter()
                .collect::<Vec<_>>()
                .into_iter()
        ).unwrap();
        d!("finished building msg:\n{:?}", &msg);



        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::Ready(Some(msg)))
        }
    }
}
