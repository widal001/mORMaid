use std::fmt;

pub trait Indent {
    fn indent(&self, size: usize) -> String;
}

impl Indent for str {
    fn indent(&self, size: usize) -> String {
        let indentation = " ".repeat(size);
        self.lines()
            .map(|line| format!("{indentation}{line}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Indent for String {
    fn indent(&self, size: usize) -> String {
        self.as_str().indent(size)
    }
}

pub fn append_items<T, I>(mut curr_str: String, items: T, note: &str, indent: usize) -> String
where
    T: IntoIterator<Item = I>,
    I: fmt::Display,
{
    curr_str += &format!("\n{}%% {} start", " ".repeat(indent), note);
    for item in items {
        curr_str += &format!("\n{}", &item.to_string().indent(indent));
    }
    curr_str += &format!("\n{}%% {} end", " ".repeat(indent), note);
    curr_str
}
