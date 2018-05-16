use bytes::{BufMut, BytesMut};
use futures::{Async, Poll};
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

pub mod msg;

use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::header::Header;
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
        d!("pool flush called");


        // Ok([249, 190, 180, 217, g 103, e 101, t 116, h 104, e 101, a 97, d 100, e 101, r 114, s 115, 
        // - 0, - 0, - 0, - 0, - 0, - 0, 5D 93, F6 246, e0 224, e2 226])
        // b"\xf9\xbe\xb4\xd9getheaders\0\0\0\0\0\0]\xf6\xe0\xe2"





        
        {
            let mut outs = self.wr.clone();
            d!("{:?}", outs);
            while(!outs.is_empty()) {
                let header = Header::new(outs.iter().take(24)).expect(&ff!());
                let msg = msg::Msg::new(
                    outs
                        .split_to(header.payload_len as usize + 24usize)
                        .iter(),
                ).expect(&ff!());
                d!("Sending Message on Peer Socket:\n{:?}", &msg);
            }
        }
        
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
            d!(
                "\n{}\n{:?}",
                self.rd.clone().into_iter().collect::<Vec<_>>().to_hex(),
                self.rd.clone()
            );
            return Ok(Async::NotReady);
        }
        d!("has >=24");
        let header = Header::new(self.rd.iter().take(24)).expect(&ff!());
        d!("after header made");
        if self.rd.iter().len() < header.payload_len as usize + 24usize {
            d!("not enought bytes for payload");
            return Ok(Async::NotReady);
        }
        d!("has enought bytes for payload");
        //let rd_split =
        let msg = msg::Msg::new(
            self.rd
                .split_to(header.payload_len as usize + 24usize)
                .iter(),
        ).expect(&ff!());
        d!("Finished building msg recieved:\n{:?}", &msg);

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::Ready(Some(msg)))
        }
    }
}
