#![allow(dead_code)]
#![no_std] // use core instead of std



// TODO: use these tables and tips:
// https://sourceforge.net/p/oszur11/code/ci/master/tree/Chapter_06_Shell/04_Makepp/arch/i386/arch/devices/i8042.c



#[derive(Debug, Copy, Clone)]
pub struct KeyboardModifiers {
    control: bool,
    alt: bool, 
    shift: bool,
    caps_lock: bool,
    num_lock: bool,
}

impl KeyboardModifiers {
    pub fn new() -> KeyboardModifiers {
        KeyboardModifiers {
            control: false,
            alt: false, 
            shift: false,
            caps_lock: false,
            num_lock: false,
        }
    }
}


pub static KEY_RELEASED_OFFSET: u8 = 128;




/// convenience function for obtaining the ascii value for a raw scancode under the given modifiers
pub fn scancode_to_ascii(modifiers: &KeyboardModifiers, scan_code: u8) -> Option<u8> {
	// FIXME: this whole let if stmt can likely be replaced by an "and_then" flow
    if let Some(keycode) = Keycode::from_scancode(scan_code) {
        keycode.to_ascii(modifiers)
    }
    else {
        None
    }
}




#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keycode {
    OverflowError = 0,
    Escape,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equals,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LeftBracket,
    RightBracket,
    Enter,
    Control,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Quote,
    Backtick,
    LeftShift,
    Backslash,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Period,
    Slash,
    RightShift,
    PadMultiply, // Also PrintScreen
    Alt,
    Space,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    NumLock,
    ScrollLock,
    Home, // Also Pad7
    Up, // Also Pad8
    PageUp, // Also Pad9
    PadMinus,
    Left, // Also Pad4
    Pad5,
    Right, // Also Pad6
    PadPlus,
    End, // Also Pad1
    Down, // Also Pad2
    PageDown, // Also Pad3
    Insert, // Also Pad0
    Delete, // Also PadDecimal
    Unknown1,
    Unknown2,
    NonUsBackslash,
    F11,
    F12,
    Pause,
    Unknown3,
    LeftGui,
    RightGui,
    Menu,
} 




impl Keycode {

    pub fn from_scancode(scancode: u8)  -> Option<Keycode> {
        match scancode {
            0 => Some(Keycode::OverflowError),
            1 => Some(Keycode::Escape),
            2 => Some(Keycode::Num1),
            3 => Some(Keycode::Num2),
            4 => Some(Keycode::Num3),
            5 => Some(Keycode::Num4),
            6 => Some(Keycode::Num5),
            7 => Some(Keycode::Num6),
            8 => Some(Keycode::Num7),
            9 => Some(Keycode::Num8),
            10 => Some(Keycode::Num9),
            11 => Some(Keycode::Num0),
            12 => Some(Keycode::Minus),
            13 => Some(Keycode::Equals),
            14 => Some(Keycode::Backspace),
            15 => Some(Keycode::Tab),
            16 => Some(Keycode::Q),
            17 => Some(Keycode::W),
            18 => Some(Keycode::E),
            19 => Some(Keycode::R),
            20 => Some(Keycode::T),
            21 => Some(Keycode::Y),
            22 => Some(Keycode::U),
            23 => Some(Keycode::I),
            24 => Some(Keycode::O),
            25 => Some(Keycode::P),
            26 => Some(Keycode::LeftBracket),
            27 => Some(Keycode::RightBracket),
            28 => Some(Keycode::Enter),
            29 => Some(Keycode::Control),
            30 => Some(Keycode::A),
            31 => Some(Keycode::S),
            32 => Some(Keycode::D),
            33 => Some(Keycode::F),
            34 => Some(Keycode::G),
            35 => Some(Keycode::H),
            36 => Some(Keycode::J),
            37 => Some(Keycode::K),
            38 => Some(Keycode::L),
            39 => Some(Keycode::Semicolon),
            40 => Some(Keycode::Quote),
            41 => Some(Keycode::Backtick),
            42 => Some(Keycode::LeftShift),
            43 => Some(Keycode::Backslash),
            44 => Some(Keycode::Z),
            45 => Some(Keycode::X),
            46 => Some(Keycode::C),
            47 => Some(Keycode::V),
            48 => Some(Keycode::B),
            49 => Some(Keycode::N),
            50 => Some(Keycode::M),
            51 => Some(Keycode::Comma),
            52 => Some(Keycode::Period),
            53 => Some(Keycode::Slash),
            54 => Some(Keycode::RightShift),
            55 => Some(Keycode::PadMultiply), // Also PrintScreen
            56 => Some(Keycode::Alt),
            57 => Some(Keycode::Space),
            58 => Some(Keycode::CapsLock),
            59 => Some(Keycode::F1),
            60 => Some(Keycode::F2),
            61 => Some(Keycode::F3),
            62 => Some(Keycode::F4),
            63 => Some(Keycode::F5),
            64 => Some(Keycode::F6),
            65 => Some(Keycode::F7),
            66 => Some(Keycode::F8),
            67 => Some(Keycode::F9),
            68 => Some(Keycode::F10),
            69 => Some(Keycode::NumLock),
            70 => Some(Keycode::ScrollLock),
            71 => Some(Keycode::Home), // Also Pad7
            72 => Some(Keycode::Up), // Also Pad8
            73 => Some(Keycode::PageUp), // Also Pad9
            74 => Some(Keycode::PadMinus),
            75 => Some(Keycode::Left), // Also Pad4
            76 => Some(Keycode::Pad5),
            77 => Some(Keycode::Right), // Also Pad6
            78 => Some(Keycode::PadPlus),
            79 => Some(Keycode::End), // Also Pad1
            80 => Some(Keycode::Down), // Also Pad2
            81 => Some(Keycode::PageDown), // Also Pad3
            82 => Some(Keycode::Insert), // Also Pad0
            83 => Some(Keycode::Delete), // Also PadDecimal
            84 => Some(Keycode::Unknown1),
            85 => Some(Keycode::Unknown2),
            86 => Some(Keycode::NonUsBackslash),
            87 => Some(Keycode::F11),
            88 => Some(Keycode::F12),
            89 => Some(Keycode::Pause),
            90 => Some(Keycode::Unknown3),
            91 => Some(Keycode::LeftGui),
            92 => Some(Keycode::RightGui),
            93 => Some(Keycode::Menu),

            _ => None,
        }
    }



    // obtains the ascii value for a keycode under the given modifiers
    pub fn to_ascii(&self, modifiers: &KeyboardModifiers) -> Option<u8> {
        // handle shift key being pressed
        if modifiers.shift {
            // if shift is pressed and caps lock is on, give a regular lowercase letter
            if modifiers.caps_lock && self.is_letter() {
                return self.as_ascii();
            }
            // if shift is pressed and caps lock is not, give a regular shifted key
            else {
                return self.as_ascii_shifted()
            }
        }
        
        // just a regular caps_lock, no shift pressed 
        // (we already covered the shift && caps_lock scenario above)
        if modifiers.caps_lock {
            if self.is_letter() {
                return self.as_ascii_shifted()
            }
            else {
                return self.as_ascii()
            }
        }

        None
        
        // TODO: handle numlock
    }



    /// returns true if this keycode was a letter from A-Z
    pub fn is_letter(&self) -> bool {
        match *self {
            Keycode::Q |
            Keycode::W |
            Keycode::E |
            Keycode::R |
            Keycode::T |
            Keycode::Y |
            Keycode::U |
            Keycode::I |
            Keycode::O |
            Keycode::P |
            Keycode::A |
            Keycode::S |
            Keycode::D |
            Keycode::F |
            Keycode::G |
            Keycode::H |
            Keycode::J |
            Keycode::K |
            Keycode::L |
            Keycode::Z |
            Keycode::X |
            Keycode::C |
            Keycode::V |
            Keycode::B |
            Keycode::N |
            Keycode::M  => true,

            _ => false,
        }
    }



    /// maps a Keycode to ASCII values (u8) without any "shift" modifiers.
    fn as_ascii(&self) -> Option<u8> {
        match *self {
            Keycode::Escape => Some(27 as u8),
            Keycode::Num1 => Some('1' as u8),
            Keycode::Num2 => Some('2' as u8),
            Keycode::Num3 => Some('3' as u8),
            Keycode::Num4 => Some('4' as u8),
            Keycode::Num5 => Some('5' as u8),
            Keycode::Num6 => Some('6' as u8),
            Keycode::Num7 => Some('7' as u8),
            Keycode::Num8 => Some('8' as u8),
            Keycode::Num9 => Some('9' as u8),
            Keycode::Num0 => Some('0' as u8), 
            Keycode::Minus => Some('-' as u8),
            Keycode::Equals => Some('=' as u8),
            Keycode::Backspace => Some(8 as u8), 
            Keycode::Tab => Some(9 as u8),
            Keycode::Q => Some('q' as u8),
            Keycode::W => Some('w' as u8),
            Keycode::E => Some('e' as u8),
            Keycode::R => Some('r' as u8),
            Keycode::T => Some('t' as u8),
            Keycode::Y => Some('y' as u8),
            Keycode::U => Some('u' as u8), 
            Keycode::I => Some('i' as u8), 
            Keycode::O => Some('o' as u8),
            Keycode::P => Some('p' as u8),
            Keycode::LeftBracket => Some('[' as u8),
            Keycode::RightBracket => Some(']' as u8),
            Keycode::Enter => Some(13 as u8), 
            Keycode::A => Some('a' as u8),
            Keycode::S => Some('s' as u8),
            Keycode::D => Some('d' as u8),
            Keycode::F => Some('f' as u8),
            Keycode::G => Some('g' as u8),
            Keycode::H => Some('h' as u8),
            Keycode::J => Some('j' as u8),
            Keycode::K => Some('k' as u8),
            Keycode::L => Some('l' as u8),
            Keycode::Semicolon => Some(';' as u8),
            Keycode::Quote => Some('\'' as u8), 
            Keycode::Backtick => Some('`' as u8),
            Keycode::Backslash => Some('\\' as u8),
            Keycode::Z => Some('z' as u8),
            Keycode::X => Some('x' as u8),
            Keycode::C => Some('c' as u8),
            Keycode::V => Some('v' as u8),
            Keycode::B => Some('b' as u8),
            Keycode::N => Some('n' as u8),
            Keycode::M => Some('m' as u8),
            Keycode::Comma => Some(',' as u8),
            Keycode::Period => Some('.' as u8),
            Keycode::Slash => Some('/' as u8),
            Keycode::Space => Some(' ' as u8),
            Keycode::PadMultiply => Some('*' as u8),
            Keycode::PadMinus => Some('-' as u8),
            Keycode::PadPlus => Some('+' as u8),
            // PadSlash / PadDivide?? 

            _ => None,
        }
    }


    /// same as as_ascii, but adds the effect of the "shift" modifier key. 
    /// If a Keycode's ascii value doesn't change when shifted,
    /// then it defaults to it's non-shifted value as returned by as_ascii().
    fn as_ascii_shifted(&self) -> Option<u8> {
        match *self {
            Keycode::Num1 => Some('!' as u8),
            Keycode::Num2 => Some('@' as u8),
            Keycode::Num3 => Some('#' as u8),
            Keycode::Num4 => Some('$' as u8),
            Keycode::Num5 => Some('%' as u8),
            Keycode::Num6 => Some('^' as u8),
            Keycode::Num7 => Some('&' as u8),
            Keycode::Num8 => Some('*' as u8),
            Keycode::Num9 => Some('(' as u8),
            Keycode::Num0 => Some(')' as u8), 
            Keycode::Minus => Some('_' as u8),
            Keycode::Equals => Some('+' as u8),
            Keycode::Q => Some('Q' as u8),
            Keycode::W => Some('W' as u8),
            Keycode::E => Some('E' as u8),
            Keycode::R => Some('R' as u8),
            Keycode::T => Some('T' as u8),
            Keycode::Y => Some('Y' as u8),
            Keycode::U => Some('U' as u8), 
            Keycode::I => Some('I' as u8), 
            Keycode::O => Some('O' as u8),
            Keycode::P => Some('P' as u8),
            Keycode::LeftBracket => Some('{' as u8),
            Keycode::RightBracket => Some('}' as u8),
            Keycode::A => Some('A' as u8),
            Keycode::S => Some('S' as u8),
            Keycode::D => Some('D' as u8),
            Keycode::F => Some('F' as u8),
            Keycode::G => Some('G' as u8),
            Keycode::H => Some('H' as u8),
            Keycode::J => Some('J' as u8),
            Keycode::K => Some('K' as u8),
            Keycode::L => Some('L' as u8),
            Keycode::Semicolon => Some(':' as u8),
            Keycode::Quote => Some('"' as u8), 
            Keycode::Backtick => Some('~' as u8),
            Keycode::Backslash => Some('|' as u8),
            Keycode::Z => Some('Z' as u8),
            Keycode::X => Some('X' as u8),
            Keycode::C => Some('C' as u8),
            Keycode::V => Some('V' as u8),
            Keycode::B => Some('B' as u8),
            Keycode::N => Some('N' as u8),
            Keycode::M => Some('M' as u8),
            Keycode::Comma => Some('<' as u8),
            Keycode::Period => Some('>' as u8),
            Keycode::Slash => Some('?' as u8),
            
            _ => self.as_ascii(),
        }
    }
}




// // I cant get TryFrom to work with core library
// use try_from::Err;

// #[derive(Debug)]
// pub struct TryFromKeycodeError { 
//     scan_code: u8,
// }

// impl Err for TryFromKeycodeError {
//     fn description(&self) -> &str {
//         "out of range integral type conversion attempted"
//     }
// }

// impl fmt::Display for TryFromKeycodeError {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         fmt.write_str(self.description())
//     }
// }

// impl TryFrom<u8> for Keycode {
//     type Err = TryFromKeycodeError;
//     fn try_from(original: u8) -> Result<Keycode, TryFromKeycodeError> {
//         let kc = get_keycode(original);
//         match kc {
//             Some(x) => Ok(x),
//             fail => Err(TryFromKeycodeError{ scan_code: original }),
//         }
//     }
// }