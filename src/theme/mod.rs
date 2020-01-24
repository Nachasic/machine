use raylib::prelude::*;

#[allow(unused_macros)]
macro_rules! color {
    (# $color_hex:expr) => {{
        let color = i32::from_str_radix(stringify!($color_hex), 16).unwrap();
        let b = color % 0x100;
        let g = (color - b) / 0x100 % 0x100;
        let r = (color - g) / 0x10000;

        Color {
            r: r as u8,
            g: g as u8,
            b: b as  u8,
            a: 255
        }
    }};
}

// pub struct MachineColors {}

// impl MachineColors {
//     pub const WHITE: Color = color!(#FFFFFF);
// }

#[test]
fn get_color () {
    let color_white = color!(#FFFFFF);
    let color_black = color!(#000000);

    assert_eq!(color_black, Color::BLACK);
    assert_eq!(color_white, Color::WHITE);
}

#[test]
fn color_breakdown_str () {
    let color = "FAFBFC";
    let mut tokens: Vec<&str> = color.split("").collect();
    let b = &tokens
        .split_off(5).concat();
    let g = &tokens
        .split_off(3).concat();
    let r = &tokens
        .split_off(1).concat();

    assert_eq!(r, "FA");
    assert_eq!(g, "FB");
    assert_eq!(b, "FC");
}
