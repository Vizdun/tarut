use std::io::prelude::*;

use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target};

use crate::data::{AppData, Message};

pub struct Delegate;

pub const SEND_MESSAGE: Selector = Selector::new("message.send");
pub const RECEIVE_MESSAGE: Selector<Message> = Selector::new("message.receive");

impl AppDelegate<AppData> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppData,
        _env: &Env,
    ) -> Handled {
        if cmd.is(SEND_MESSAGE) {
            let msg = data.writing_message.clone();
            let mut stream =
                socks::Socks5Stream::connect("127.0.0.1:9050", data.address.as_str()).unwrap();

            use openssl::hash::MessageDigest;
            use openssl::sign::Signer;

            let mut signer = Signer::new(MessageDigest::sha256(), &data.pub_key).unwrap();
            signer.update(msg.as_bytes()).unwrap();
            let signature = signer.sign_to_vec().unwrap();

            let message = [
                &[0b00001111, msg.as_bytes().len() as u8][..],
                &signature[..],
                &data.pub_key.public_key_to_der().unwrap()[..],
                &msg.as_bytes()[..],
            ]
            .concat();

            stream.write_all(&message).unwrap();

            data.writing_message = String::new();

            Handled::Yes
        } else if let Some(msg) = cmd.get(RECEIVE_MESSAGE) {
            data.messages.push_back(msg.clone());
            Handled::Yes
        } else {
            Handled::No
        }
    }
}
