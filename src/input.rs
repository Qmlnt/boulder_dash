use console::{Key, Term};
use std::{sync::mpsc, thread};

pub enum Input {
    Esc,
    Quit,
    Reload,
    Unknown,

    Up,
    Down,
    Left,
    Right,
    Space,
    DelayDown,
    DelayUp,
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
    rx.try_recv().map_or(Input::Unknown, |key| match key {
        Key::Escape => Input::Esc,
        Key::Char(' ') => Input::Space,
        Key::Char('p') => Input::Reload,
        Key::Char(',') => Input::DelayDown,
        Key::Char('.') => Input::DelayUp,
        Key::Char('q') | Key::CtrlC => Input::Quit,
        Key::Char('w') | Key::ArrowUp => Input::Up,
        Key::Char('r') | Key::ArrowDown => Input::Down,
        Key::Char('a') | Key::ArrowLeft => Input::Left,
        Key::Char('s') | Key::ArrowRight => Input::Right,
        _ => Input::Unknown,
    })
}
