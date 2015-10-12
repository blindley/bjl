extern crate user32;

mod keys;
pub use self::keys::Key;

mod textreader;
pub use self::textreader::TextReader;

use std;

/// Returns true if the specified key is currently pressed. Does not block.
///
/// # Examples
///
/// ```
/// use bjl::keyboard::{key_is_down, Key};
///
/// if key_is_down(Key::Enter) {
///     println!("Let go of the enter key!");
/// }
/// ```
pub fn key_is_down(key: Key) -> bool {
    platform_key_is_down(key)
}

#[cfg(target_os = "windows")]
fn platform_key_is_down(key: Key) -> bool {
    const MASK: i16 = -0x8000;
    unsafe {
        user32::GetAsyncKeyState(key as i32) & MASK != 0
    }
}

#[cfg(target_os = "linux")]
fn platform_key_is_down(key: Key) -> bool {
    unimplemented!();
}