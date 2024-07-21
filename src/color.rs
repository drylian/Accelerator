pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
}

pub fn color(color: Color, text: &str) -> String {
    let colors_ansi = match color {
        Color::Red => "\x1b[31m",
        Color::Green => "\x1b[32m",
        Color::Yellow => "\x1b[33m",
        Color::Blue => "\x1b[34m",
    };

    let reset = "\x1b[0m";

    format!("{}{}{}", colors_ansi, text, reset)
}