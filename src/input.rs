use console::{Key, Term};
use std::{sync::mpsc, thread};

pub enum Input {
    Esc,
    Quit,
    Unknown,

    Up,
    Down,
    Left,
    Right,
    Space,
}

pub fn init_term() -> mpsc::Receiver<Key> {
    let (tx, rx) = mpsc::channel();

    let term = Term::stdout();
    thread::spawn(move || loop {
        let key = term.read_key().expect("Should always get a key");
        tx.send(key).expect("Receiver should be present");
    });

    rx
}

pub fn read_term(rx: &mpsc::Receiver<Key>) -> Input {
    let mut key = rx.try_recv().unwrap_or(Key::Unknown);
    // skip till last key
    while let Ok(inp) = rx.try_recv() {
        key = inp;
    }

    match key {
        Key::Escape => Input::Esc,
        Key::Char(' ') => Input::Space,
        Key::Char('q') | Key::CtrlC => Input::Quit,
        Key::Char('w') | Key::ArrowUp => Input::Up,
        Key::Char('r') | Key::ArrowDown => Input::Down,
        Key::Char('a') | Key::ArrowLeft => Input::Left,
        Key::Char('s') | Key::ArrowRight => Input::Right,
        _ => Input::Unknown,
    }
}
