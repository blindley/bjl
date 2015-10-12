use std;
use super::Key;

/// Collects text input in real time from the keyboard.
///
/// # Examples
/// ```
/// let mut reader = bjl::keyboard::TextReader::new();
/// loop {
///     reader.tick();
///     if let Some(c) = reader.text().as_bytes().last() {
///         if *c == b'\n' { break; }
///     }
/// #   break;
/// }
/// println!("{}", reader.text());
/// ```
pub struct TextReader {
    text: String,
    keystate: [i32;256],
}

impl TextReader {
    /// Constructs a new TextReader with empty text.
    pub fn new() -> TextReader {
        TextReader {
            text: String::new(),
            keystate: [0;256],
        }
    }

    /// Returns the stored text.
    pub fn text<'a>(&'a self) -> &'a str {
        &self.text
    }

    /// Checks keyboard state and updates stored text.
    ///
    /// This function should be called repeatedly in a loop
    /// in order to keep polling the keyboard and updating
    /// the stored text.
    pub fn tick(&mut self) {
        for i in 0..256 {
            let key = unsafe { std::mem::transmute(i as u8) };
            if super::key_is_down(key) {
                self.keystate[i] += 1;
                if self.keystate[i] == 1 {
                    self.key_pressed(key);
                }
            } else {
                if self.keystate[i] != 0 {
                    self.keystate[i] = 0;
                    self.key_released(key);
                }
            }
        }
    }

    fn key_pressed(&mut self, key: Key) {
        use super::Key::*;
        match key {
            A | B | C | D | E | F | G | H | I | J |
            K | L | M | N | O | P | Q | R | S | T |
            U | V | W | X | Y | Z
                => { self.letter_pressed(key); }
            Key0 | Key1 | Key2 | Key3 | Key4 | Key5 | Key6 | Key7 | Key8 | Key9
                => { self.number_pressed(key); }
            Space => { self.text.push(' '); }
            Enter => { self.text.push('\n'); }
            Back => { self.text.pop(); }
            _ => (),
        }
    }

    fn key_released(&mut self, key: Key) {

    }

    fn letter_pressed(&mut self, key: Key) {
        let c =
            if super::key_is_down(Key::Shift) {
                key as u8 as char
            } else {
                ((key as u8) + 32) as char
            };
        self.text.push(c);
    }

    fn number_pressed(&mut self, key: Key) {
        use super::Key::*;
        let c =
            if super::key_is_down(Key::Shift) {
                match key {
                    Key1 => '!', Key2 => '@', Key3 => '#',
                    Key4 => '$', Key5 => '%', Key6 => '^',
                    Key7 => '&', Key8 => '*', Key9 => '(',
                    Key0 => ')',
                    _ => panic!("not a number key")
                }
            } else {
                key as u8 as char
            };
        self.text.push(c);
    }
}
