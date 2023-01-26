use std::fmt;
use crossterm::style::Stylize;


enum ByteType {
    NULL,
    PrintableAscii,
    WhitespaceAscii,
    OtherAscii,
    NonAscii,
}

impl ByteType {
    fn get_type(data: u8) -> ByteType {
        if data == 0 {
            ByteType::NULL
        } else if data.is_ascii_graphic() {
            ByteType::PrintableAscii
        } else if data.is_ascii_whitespace() {
            ByteType::WhitespaceAscii
        } else if data.is_ascii() {
            ByteType::OtherAscii
        } else {
            ByteType::NonAscii
        }
    }

    fn colour_byte(&self, string: String) -> String {
        match self {
            Self::NULL => {string.black().to_string()}
            Self::PrintableAscii => {string.cyan().to_string()}
            Self::WhitespaceAscii => {string.white().to_string()}
            Self::OtherAscii => {string.green().to_string()}
            Self::NonAscii => {string.red().to_string()}
        }
    }
}

pub struct Byte {
    decimal: u8,
    hex: [char; 2],
    char: char,
    byte_type: ByteType,
}

#[derive(Debug)]
enum ByteError {
    NibbleParseError(u8),
}

type Result<A> = std::result::Result<A, ByteError>;

fn u8_to_hex(byte: u8) -> [char; 2] {
    let nibble_upper = nibble_to_hex(byte >> 4).unwrap();
    let nibble_lower = nibble_to_hex(byte & 0x0f).unwrap();
    [nibble_upper, nibble_lower]
}

fn nibble_to_hex(nibble: u8) -> Result<char> {
    match nibble {
        0 => Ok('0'),
        1 => Ok('1'),
        2 => Ok('2'),
        3 => Ok('3'),
        4 => Ok('4'),
        5 => Ok('5'),
        6 => Ok('6'),
        7 => Ok('7'),
        8 => Ok('8'),
        9 => Ok('9'),
        10 => Ok('A'),
        11 => Ok('B'),
        12 => Ok('C'),
        13 => Ok('D'),
        14 => Ok('E'),
        15 => Ok('F'),
        _ => Err(ByteError::NibbleParseError(nibble)),
    }
}

fn byte_to_char(byte: u8, byte_type: &ByteType) -> char {
    match byte_type {
        ByteType::NULL => '0',
        ByteType::OtherAscii => '•',
        ByteType::WhitespaceAscii => '_',
        ByteType::PrintableAscii => byte as char,
        ByteType::NonAscii => '×',
    }
}

impl Byte {
    pub fn new(data: u8) -> Byte {
        let byte_type = ByteType::get_type(data);
        Byte {
            decimal: data,
            hex: u8_to_hex(data),
            char: byte_to_char(data, &byte_type),
            byte_type,
        }
    }

    pub fn char(&self) -> String {
        self.byte_type.colour_byte(self.char.to_string())
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self.byte_type.colour_byte(format!("{}{}", self.hex[0], self.hex[1]));
        write!(f, "{}", string)
    }
}
