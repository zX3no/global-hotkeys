use win_hotkey::*;

#[derive(Clone, Debug)]
pub enum Event {
    Next,
    Previous,
    PlayPause,
}

fn main() {
    let mut hk = Listener::<Event>::new();
    hk.register_hotkey(modifiers::SHIFT, 'A', Event::Next);
    hk.register_hotkey(modifiers::CONTROL, 'A', Event::Previous);

    loop {
        if let Some(event) = hk.listen() {
            dbg!(event);
            match event {
                Event::Next => (),
                Event::Previous => (),
                _ => (),
            }
        }
    }
}
