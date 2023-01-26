use std::sync::mpsc::{channel, Receiver};

use enigo::{KeyboardControllable, Key};
use webbrowser::Browser;
use websocket::{OwnedMessage, sync::Server};

pub mod key;
pub mod mouse;

#[derive(Debug, PartialEq)]
pub enum BrowserEvent {
    KeyDown(String),
    KeyUp(String),
    MouseDown(String),
    MouseUp(String),
    MouseMove(((i32,i32),(i32,i32))),
    MouseWheel((i32, i32)),
    Open,
    Close,
}

pub fn launch() -> Receiver<BrowserEvent> {
    let (tx, rs) = channel::<BrowserEvent>();
    std::thread::spawn(move || {
        let server = Server::bind("127.0.0.1:26541").unwrap();
        for request in server.filter_map(Result::ok) {
            let client = request.accept().unwrap();

            let ip = client.peer_addr().unwrap();

            tx.send(BrowserEvent::Open).unwrap();

            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
                let message = message.unwrap();

                match message {
                    OwnedMessage::Close(_) => {
                        tx.send(BrowserEvent::Close).unwrap();
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    OwnedMessage::Text(msg) => {
                        println!("msg: {:?}", msg);
                        let (key, data) = msg.split_once(':').unwrap();
                        let be = match key {
                            "keydown" => BrowserEvent::KeyDown(data.to_string()),
                            "keyup" => BrowserEvent::KeyUp(data.to_string()),
                            "mousedown" => BrowserEvent::MouseDown(data.to_string()),
                            "mouseup" => BrowserEvent::MouseUp(data.to_string()),
                            "mousemove" => {
                                // format is relx,rely|absx,absy
                                let (rel, abs) = data.split_once('|').unwrap();
                                let (relx, rely) = rel.split_once(',').unwrap();
                                let (absx, absy) = abs.split_once(',').unwrap();
                                BrowserEvent::MouseMove(((relx.parse().unwrap(), rely.parse().unwrap()), (absx.parse().unwrap(), absy.parse().unwrap())))
                            }
                            "mousewheel" => {
                                // format is x,y
                                let (x, y) = data.split_once(',').unwrap();
                                BrowserEvent::MouseWheel((x.parse().unwrap(), y.parse().unwrap()))
                            }
                            _ => { continue }
                        };
                        tx.send(be).unwrap();
                    }
                    _ => {}
                }
            }
        }
    });
    webbrowser::open_browser(Browser::Firefox, &format!("file://{}/tests/index.html", std::env::current_dir().unwrap().to_str().unwrap())).unwrap();
    if rs.recv() == Ok(BrowserEvent::Open) {
        enigo::Enigo::new().key_click(Key::F11);
        // Full screen animation
        std::thread::sleep(std::time::Duration::from_millis(1000));
    } else {
        panic!("Expected Open event");
    }
    loop {
        if rs.recv_timeout(std::time::Duration::from_millis(500)).is_err() {
            break
        }
    }
    rs
}
