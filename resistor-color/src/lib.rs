use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Sequence, IntEnum)]
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

pub fn color_to_value(_color: ResistorColor) -> usize {
    return _color.int_value();
}

pub fn value_to_color_string(value: usize) -> String {
    // convert the value {} into a string representation of color
    let colors_list = colors();

    if value >= colors_list.len() - 1 {
        return String::from("value out of range");
    }

    format!("{:?}", colors_list[value])
}

pub fn colors() -> Vec<ResistorColor> {
    // return a list of all the colors ordered by resistance"
    all::<ResistorColor>().collect::<Vec<ResistorColor>>()
}
