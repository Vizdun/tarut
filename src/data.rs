use druid::{im::Vector, Data, Lens};
use openssl::pkey::PKey;
use openssl::pkey::Private;

#[derive(Clone, Data)]
pub struct Message {
    pub author: String,
    pub content: String
}

#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub messages: Vector<Message>,
    pub writing_message: String,
    pub writing_address: String,
    #[data(ignore)]
    pub address: String,
    #[data(ignore)]
    pub pub_key: PKey<Private>,
}
