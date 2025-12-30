use colored::{ColoredString, Colorize};

#[derive(Clone, Copy, Debug)]
pub enum ColorSupport {
    Enabled,
    Disabled,
}

fn apply<F>(text: &str, support: ColorSupport, f: F) -> ColoredString
where
    F: FnOnce(&str) -> ColoredString,
{
    match support {
        ColorSupport::Enabled => f(text),
        ColorSupport::Disabled => text.normal(),
    }
}

pub fn success(text: &str, support: ColorSupport) -> ColoredString {
    /* Neon Green */
    apply(text, support, |t| t.bright_green().bold())
}

pub fn info(text: &str, support: ColorSupport) -> ColoredString {
    /* Cyan Info */
    apply(text, support, |t| t.bright_cyan())
}

pub fn failure(text: &str, support: ColorSupport) -> ColoredString {
    /* Neon Red */
    apply(text, support, |t| t.bright_red().bold())
}

/* Big Headers */
pub fn header(text: &str, support: ColorSupport) -> ColoredString {
    apply(text, support, |t| t.bright_magenta().bold())
}

/* Accent for Labels, Option numbers.. */
pub fn accent(text: &str, support: ColorSupport) -> ColoredString {
    apply(text, support, |t| t.bright_cyan().bold())
}

/* for Less Important Text */
pub fn subtle(text: &str, support: ColorSupport) -> ColoredString {
    apply(text, support, |t| t.dimmed())
}
