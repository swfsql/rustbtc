/*
TODO:


- testar o server/client do fibers em VPN


- mais estruturas das mensagens;
- comunicação TCP/IP
-
-

*/

//

#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{}
}
use errors::*;

extern crate hex;

extern crate btc;
use btc::commons::new_from_hex::NewFromHex;
// use btc::commons::into_bytes::IntoBytes;
use hex::FromHex;

fn run() -> Result<()> {
    println!("change");

    {
        let msg_ping_hex = "".to_string() + "F9BEB4D9" + "70696E670000000000000000"
            + "0800000088EA8176" + "0094102111e2af4d";
        let msg_ping_vec: Vec<u8> =
            Vec::from_hex(msg_ping_hex).chain_err(|| "Falha no hex -> Vec<u8>")?;
        let mut msg_ping_it = msg_ping_vec.into_iter();
        let msg_ping = btc::msg::Msg::new(msg_ping_it.by_ref())
            .chain_err(|| "Falha no hex -> Msg no teste 2")?;
        println!("{:?\n}", msg_ping);

        let tx = if let Some(btc::msg::payload::Payload::Tx(tx)) = msg_ping.payload {
            Some(tx)
        } else {
            None
        };
        print!("{:?}\n\n", tx);
    }

    {
        let msg_pong_hex = "".to_string() + "F9BEB4D9" + "706F6E670000000000000000"
            + "08000000EAF3B51D" + "0194102111e2af4d";
        let msg_pong = btc::msg::Msg::new_from_hex(&msg_pong_hex)
            .chain_err(|| "Falha no hex -> Msg no teste 3")?;
        println!("{:?}", msg_pong);
    }

    {
        let msg_version_hex = "".to_string() +
    "F9BEB4D976657273696F6E0000000000640000003B648D5A" +
    "62EA0000010000000000000011B2D05000000000010000000000000000000000000000000000FFFF000000000000010000000000000000000000000000000000FFFF0000000000003B2EB35D8CE617650F2F5361746F7368693A302E372E322FC03E0300";
        let msg_version = btc::msg::Msg::new_from_hex(&msg_version_hex)
            .chain_err(|| "Falha no hex -> Msg no teste 4")?;
        println!("{:?}", msg_version);
    }

    {
        let msg_verack_hex = "F9BEB4D976657261636B000000000000000000005DF6E0E2";
        let msg_verack = btc::msg::Msg::new_from_hex(msg_verack_hex)
            .chain_err(|| "Falha no hex -> Msg no teste 5")?;
        println!("{:?}", msg_verack);
    }

    Ok(())
}

quick_main!(run);
