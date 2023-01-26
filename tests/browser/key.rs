use std::sync::mpsc::Receiver;

use enigo::{Key, Enigo, KeyboardControllable};

use super::BrowserEvent;

pub fn run(recv: &Receiver<BrowserEvent>) {
    press(recv, Key::F1);
    press(recv, Key::Control);
    press(recv, Key::Backspace);
    press(recv, Key::PageUp);
}

fn press(recv: &Receiver<BrowserEvent>, key: Key) {
    Enigo::new().key_down(key);
    let ev = recv.recv().unwrap();
    if let BrowserEvent::KeyDown(pressed) = ev {
        assert_eq!(format!("{:?}", key).to_lowercase(), pressed.to_lowercase());
    } else {
        panic!("Event wasn't KeyDown after mouse::press. {:?}", ev);
    }
    Enigo::new().key_up(key);
    let ev = recv.recv().unwrap();
    if let BrowserEvent::KeyUp(pressed) = ev {
        assert_eq!(format!("{:?}", key).to_lowercase(), pressed.to_lowercase());
    } else {
        panic!("Event wasn't KeyUp after mouse::press. {:?}", ev);
    }
}
