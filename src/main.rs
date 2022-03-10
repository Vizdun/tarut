use data::{AppData, Message};
use delegate::Delegate;
use druid::{
    im::{vector, Vector},
    AppLauncher, LocalizedString, WindowDesc,
};

use openssl::{pkey::PKey, rsa::Rsa};
use ui::ui_builder;

mod data;
mod delegate;
mod receive;
mod ui;

pub fn main() {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("list-demo-window-title").with_placeholder("List Demo"));

    let messages: Vector<Message> = vector![];

    let keypair = Rsa::generate(2048).unwrap();
    let keypair = PKey::from_rsa(keypair).unwrap();

    let data = AppData {
        messages,
        writing_message: String::new(),
        writing_address: String::new(),
        address: String::from("xwygb6sktyd2hnadoyuko3otqhj2ahvxxwx76tgyztye2at5iun7a6id.onion:5890"),
        pub_key: keypair,
    };

    let launcher = AppLauncher::with_window(main_window);

    let event_sink = launcher.get_external_handle();
    let address = data.address.clone();

    std::thread::spawn(move || receive::receive(event_sink, address));

    launcher
        .delegate(Delegate {})
        .launch(data)
        .expect("launch failed");
}
