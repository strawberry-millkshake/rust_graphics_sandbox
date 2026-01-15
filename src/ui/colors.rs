#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn _new(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }
}

pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};
pub const _BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 1,
};
pub const _RED: Color = Color {
    r: 1,
    g: 0,
    b: 0,
    a: 1,
};

pub const _GREEN: Color = Color {
    r: 0,
    g: 1,
    b: 0,
    a: 0,
};
pub const _BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 1,
    a: 0,
};
