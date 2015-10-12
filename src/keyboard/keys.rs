#[derive(Debug, Clone, Copy)]
pub enum Key {
    LButton = 0x1,
    RButton = 0x2,
    MButton = 0x4,

    Back = 0x8,
    Tab = 0x9,
    Enter = 0xd,
    Shift = 0x10,
    Ctrl = 0x11,
    Alt = 0x12,
    Pause = 0x13,
    Caps = 0x14,
    Esc = 0x1b,
    Space = 0x20,
    PgUp = 0x21,
    PgDn = 0x22,
    End = 0x23,
    Home = 0x24,
    Left = 0x25,
    Up = 0x26,
    Right = 0x27,
    Down = 0x28,

    PrntScrn = 0x2c,
    Insert = 0x2d,
    Delete = 0x2e,

    Key0 = 0x30,
    Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,

    A = 0x41,
    B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    Num0 = 0x60,
    Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,

    NumAdd = 0x6b,
    NumSubtract = 0x6d,
    NumDecimal = 0x6e,
    NumDivide = 0x6f,

    F1 = 0x70,
    F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    Numlock = 0x90,
    Scroll = 0x91,

    LShift = 0xa0,
    RShift = 0xa1,
    LCtrl = 0xa2,
    RCtrl = 0xa3,
    LAlt = 0xa4,
    RAlt = 0xa5,

    SemiColon = 0xba,
    Plus = 0xbb,
    Comma = 0xbc,
    Minus = 0xbd,
    Period = 0xbe,
    Slash = 0xbf,
    Accent = 0xc0,

    LBracket = 0xdb,
    BackSlash = 0xdc,
    RBracket = 0xdd,
    Quote = 0xde,
}

impl Key {

}