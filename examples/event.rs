use win_hotkey::*;

#[derive(Clone, Debug)]
pub enum Event {
    Next,
    Previous,
    PlayPause,
}

fn main() {
    let mut hk = Listener::<Event>::new();
    hk.register_hotkey(modifiers::CONTROL, 'A' as u32, Event::Previous);
    hk.register_hotkey(modifiers::CONTROL, 'S' as u32, Event::Next);
    hk.register_hotkey(modifiers::SHIFT, keys::CAPS_LOCK, Event::PlayPause);

    loop {
        if let Some(event) = hk.listen() {
            match event {
                Event::Next => println!("Next"),
                Event::Previous => println!("Previous"),
                Event::PlayPause => println!("Play/Pause"),
            }
        }
    }
}
