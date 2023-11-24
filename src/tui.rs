use colored::*;

pub fn title(title: &str) -> ColoredString {
    format!(" {} ", title)
        .bold()
        .truecolor(42, 157, 143)
        .on_truecolor(38, 70, 83)
}

pub fn content(value: &str) -> ColoredString {
    format!(" {} ", value)
        .truecolor(42, 157, 143)
        .on_truecolor(42, 157, 143)
        .bold()
}
