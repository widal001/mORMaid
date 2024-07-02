pub trait Indent {
    fn indent(&self, size: usize) -> String;
}

impl Indent for str {
    fn indent(&self, size: usize) -> String {
        let indentation = " ".repeat(size);
        self.lines()
            .map(|line| format!("{}{}", indentation, line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Indent for String {
    fn indent(&self, size: usize) -> String {
        self.as_str().indent(size)
    }
}
