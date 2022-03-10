use std::io::prelude::*;

use druid::{ExtEventSink, Target};
use openssl::{hash::MessageDigest, pkey::PKey, sign::Verifier};

use crate::{data::Message, delegate::RECEIVE_MESSAGE};

pub fn receive(event_sink: ExtEventSink, address: String) {
    let mut stream = socks::Socks5Stream::connect(
        "127.0.0.1:9050",
        address.as_str(),
    )
    .unwrap();

    let mut hbuf = [0b11110000];
    stream.write(&mut hbuf).unwrap();

    loop {
        let mut nbuf = [0u8; 1];

        stream.read(&mut nbuf).unwrap();

        // breaks if tcp dies
        if nbuf[0] == 0 {
            break;
        }

        let mut sign_buf = [0u8; 256];
        stream.read(&mut sign_buf).unwrap();
        let mut pubkey_buf = [0u8; 294];
        stream.read(&mut pubkey_buf).unwrap();

        let mut msg_buf: Vec<u8> = Vec::new();

        let mut buf = [0u8];

        for _ in 0..nbuf[0] {
            stream.read(&mut buf).unwrap();
            msg_buf.push(buf[0]);
        }

        let keypair = PKey::public_key_from_der(&pubkey_buf).unwrap();

        let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
        verifier.update(&msg_buf).unwrap();

        if !verifier.verify(&sign_buf).unwrap() {
            panic!()
        }

        let mut hasher = openssl::sha::Sha256::new();

        hasher.update(&pubkey_buf[33..]);

        let hash = hasher.finish();

        event_sink
            .submit_command(
                RECEIVE_MESSAGE,
                Message {
                    author: bs58::encode(hash).into_string(),
                    content: String::from_utf8_lossy(&msg_buf).to_string(),
                },
                Target::Auto,
            )
            .unwrap();
    }
}
