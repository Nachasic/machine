use raylib::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParseIntError {
    kind: IntErrorKind,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum IntErrorKind {
    Empty,
    InvalidDigit,
    Overflow,
    Underflow,
    InvalidLength
}

#[allow(dead_code)]
const fn to_digit_ascii (ascii: u8) -> Result<usize, ParseIntError> {
    let decimal_digit = ascii.wrapping_sub(b'0');

    if decimal_digit > 9 {
        let out = (ascii | 32).wrapping_sub(b'a') as usize;

        if out > 6 {
            Err(ParseIntError { kind: IntErrorKind::InvalidDigit })
        } else {
            Ok(out + 10)
        }
    } else {
        let decimal_digit = decimal_digit as usize;
        if decimal_digit > 16 {
            Err(ParseIntError { kind: IntErrorKind::InvalidDigit })
        } else {
            Ok(decimal_digit)
        }
    }
}

#[allow(dead_code)]
const fn parse_color (color: &str) -> Result<i32, ParseIntError> {
    let bytes = color.as_bytes();

    if bytes.len() != 6 {
        return Err( ParseIntError { kind: IntErrorKind::InvalidLength } )
    }

    let mut mul: i32= 16;
    let mut index = bytes.len() - 1;
    let mut output: i32 = {
        match to_digit_ascii(bytes[index]) {
            Ok(num) => num as i32,
            Err(e) => return Err(e)
        }
    };

    index -= 1;

    {
        let byte = bytes[index];
        let digit = match to_digit_ascii(byte) {
            Ok(num) => num,
            Err(e) => return Err(e)
        };
        let next_output = output.wrapping_add(digit as i32  * mul);

        if output > next_output {
            return Err(
                ParseIntError { kind: IntErrorKind::Overflow }
            );
        }

        mul *= 16;
        output = next_output;
        index -= 1;
    }

    {
        let byte = bytes[index];
        let digit =  match to_digit_ascii(byte) {
            Ok(num) => num,
            Err(e) => return Err(e)
        };
        let next_output = output.wrapping_add(digit as i32 * mul);

        if output > next_output {
            return Err(
                ParseIntError { kind: IntErrorKind::Overflow }
            );
        }

        mul *= 16;
        output = next_output;
        index -= 1;
    }

    {
        let byte = bytes[index];
        let digit =  match to_digit_ascii(byte) {
            Ok(num) => num,
            Err(e) => return Err(e)
        };
        let next_output = output.wrapping_add(digit as i32 * mul);

        if output > next_output {
            return Err(
                ParseIntError { kind: IntErrorKind::Overflow }
            );
        }

        mul *= 16;
        output = next_output;
        index -= 1;
    }

    {
        let byte = bytes[index];
        let digit =  match to_digit_ascii(byte) {
            Ok(num) => num,
            Err(e) => return Err(e)
        };
        let next_output = output.wrapping_add(digit as i32 * mul);

        if output > next_output {
            return Err(
                ParseIntError { kind: IntErrorKind::Overflow }
            );
        }

        mul *= 16;
        output = next_output;
        index -= 1;
    }

    {
        let byte = bytes[index];
        let digit =  match to_digit_ascii(byte) {
            Ok(num) => num,
            Err(e) => return Err(e)
        };
        let next_output = output.wrapping_add(digit as i32 * mul);

        if output > next_output {
            return Err(
                ParseIntError { kind: IntErrorKind::Overflow }
            );
        }

        mul *= 16;
        output = next_output;
    }

    Ok(output)
}

#[allow(unused_macros)]
macro_rules! color {
    (# $color_hex:expr) => {{
        const COLOR: i32 = match parse_color(stringify!($color_hex)) {
            Ok(num) => num,
            Err(e) => 0
        };
        let b = COLOR % 0x100;
        let g = (COLOR - b) / 0x100 % 0x100;
        let r = (COLOR - g) / 0x10000;

        Color {
            r: r as u8,
            g: g as u8,
            b: b as  u8,
            a: 255
        }
    }}
}

pub struct MachineColors {}
impl MachineColors {
    pub const WHITE: Color = color!(#FFFFFF);
}

#[test]
fn parse_ascii () {
    let num = to_digit_ascii(b'A').unwrap();
    assert_eq!(num, 10)
}

#[test]
fn parse_color_const () {
    let expected_color = i32::from_str_radix("FFFFFF", 16).unwrap();
    const COLOR: i32 = match parse_color("FFFFFF") {
        Ok(val) => val as i32,
        Err(e) => 0
    };

    assert_eq!(COLOR, expected_color);
}

#[test]
fn get_color () {
    let color_white = color!(#FFFFFF);
    let color_black = color!(#000000);

    assert_eq!(color_black, Color::BLACK);
    assert_eq!(color_white, Color::WHITE);
}
