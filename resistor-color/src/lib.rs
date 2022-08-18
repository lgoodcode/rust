use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, IntEnum, Copy, Clone, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    return _color.int_value();
}

pub fn value_to_color_string(value: u32) -> String {
    return match value {
        0 => String::from("Black"),
        1 => String::from("Brown"),
        2 => String::from("Red"),
        3 => String::from("Orange"),
        4 => String::from("Yellow"),
        5 => String::from("Green"),
        6 => String::from("Blue"),
        7 => String::from("Violet"),
        8 => String::from("Grey"),
        9 => String::from("White"),
        _ => String::from("value out of range"),
    };
}

pub fn colors() -> Vec<ResistorColor> {
    return all::<ResistorColor>().collect::<Vec<_>>();
}
