use std::fmt;

#[must_use]
pub struct Element {
    pub name: String,
    pub kind: String,
    pub docref: Option<String>,
}

impl Element {
    pub fn new(name: &str, kind: &str) -> Self {
        Element {
            name: name.to_string(),
            kind: kind.to_string(),
            docref: None,
        }
    }

    pub fn with_docref(mut self, docref: &str) -> Self {
        self.docref = Some(docref.to_string());
        self
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // format element name name with an open bracket on its own line
        let mut out_str = format!("element {} {{\n", self.name);
        // format the element type
        out_str += &format!("    type: \"{}\"\n", self.kind);
        // format the docref if it's populated
        if let Some(docref) = self.docref.as_deref() {
            out_str += &format!("    docref: {docref}\n");
        }
        // append a final closing bracket on its own line
        out_str += "}";
        write!(f, "{out_str}")
    }
}

// ==================================================================
// Element tests
// ==================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const NAME: &str = "milestone";
    const KIND: &str = "product brief";
    const DOCREF: &str = "https://github.com/widal001/mORMaid/issues/8";

    #[test]
    fn create_element_without_docref() {
        // act
        let element = Element::new(NAME, KIND);
        // assert
        assert_eq!(element.name, NAME);
        assert_eq!(element.kind, KIND);
        assert_eq!(element.docref, None);
    }

    #[test]
    fn create_element_with_docref() {
        // act
        let element = Element::new(NAME, KIND).with_docref(DOCREF);
        // assert
        assert_eq!(element.name, NAME);
        assert_eq!(element.kind, KIND);
        assert_eq!(element.docref, Some(DOCREF.to_string()));
    }

    #[test]
    fn display_element_without_docref() {
        // arrange
        let wanted = concat!(
            "element milestone {\n",
            "    type: \"product brief\"\n",
            "}",
        );
        // act
        let got = Element::new(NAME, KIND).to_string();
        // assert
        assert_eq!(got, wanted);
    }

    #[test]
    fn display_element_with_docref() {
        // arrange
        let wanted = concat!(
            "element milestone {\n",
            "    type: \"product brief\"\n",
            "    docref: https://github.com/widal001/mORMaid/issues/8\n",
            "}",
        );
        // act
        let got = Element::new(NAME, KIND).with_docref(DOCREF).to_string();
        // assert
        assert_eq!(got, wanted);
    }
}
