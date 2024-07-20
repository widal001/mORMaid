use std::fmt;

// ==================================================================
// Enums
// ==================================================================
#[derive(Debug, PartialEq)]
pub enum RequirementType {
    Default,
    Functional,
    Interface,
    Performance,
    Physical,
    DesignConstraint,
}

impl fmt::Display for RequirementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            RequirementType::Default => "requirement",
            RequirementType::Functional => "functionalRequirement",
            RequirementType::Interface => "interfaceRequirement",
            RequirementType::Performance => "performanceRequirement",
            RequirementType::Physical => "physicalRequirement",
            RequirementType::DesignConstraint => "designConstraint",
        };
        write!(f, "{type_str}")
    }
}

#[derive(Debug, PartialEq)]
pub enum Risk {
    Low,
    Medium,
    High,
}

impl fmt::Display for Risk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let risk_str = match self {
            Risk::High => "High",
            Risk::Medium => "Medium",
            Risk::Low => "Low",
        };
        write!(f, "{risk_str}")
    }
}

#[derive(Debug, PartialEq)]
pub enum VerifyMethod {
    Analysis,
    Inspection,
    Test,
    Demo,
}

impl fmt::Display for VerifyMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self {
            VerifyMethod::Analysis => "Analysis",
            VerifyMethod::Inspection => "Inspection",
            VerifyMethod::Test => "Test",
            VerifyMethod::Demo => "Demonstration",
        };
        write!(f, "{method_str}")
    }
}

// ==================================================================
// Requirement struct and implementation
// ==================================================================

pub struct Requirement {
    pub kind: RequirementType,
    pub name: String,
    pub id: String,
    pub text: Option<String>,
    pub risk: Option<Risk>,
    pub verify_method: Option<VerifyMethod>,
}

impl Requirement {
    pub fn new(kind: RequirementType, name: &str, id: &str) -> Self {
        Requirement {
            kind,
            name: name.to_string(),
            id: id.to_string(),
            text: None,
            risk: None,
            verify_method: None,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn with_risk(mut self, risk: Risk) -> Self {
        self.risk = Some(risk);
        self
    }

    pub fn with_verify_method(mut self, method: VerifyMethod) -> Self {
        self.verify_method = Some(method);
        self
    }
}

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // format type and name with an open bracket
        let mut out_str = format!("{} {} {{", self.kind, self.name);
        // format the id value
        out_str += &format!("\n    id: {}", self.id);
        // format the risk value (if populated) on a new indented line
        if let Some(risk) = &self.risk {
            out_str += &format!("\n    risk: {risk}");
        }
        // format the text value (if populated) on a new indented line
        if let Some(text) = self.text.as_deref() {
            out_str += &format!("\n    text: \"{text}\"");
        }
        // format the verify method (if populated) on a new indented line
        if let Some(method) = &self.verify_method {
            out_str += &format!("\n    verifymethod: {method}");
        }
        // append a final closing bracket on its own line
        out_str += "\n}";
        write!(f, "{out_str}")
    }
}

// ==================================================================
// Element tests
// ==================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const ID: &str = "1.1.1";
    const NAME: &str = "milestone";
    const KIND: RequirementType = RequirementType::Default;

    #[test]
    fn create_requirement_without_optional_fields() {
        // act
        let got = Requirement::new(KIND, NAME, ID);
        // assert
        assert_eq!(got.name, NAME);
        assert_eq!(got.kind, KIND);
        assert_eq!(got.id, ID);
    }

    #[test]
    fn create_specific_kind_of_requirement() {
        // act
        let got = Requirement::new(KIND, NAME, ID);
        // assert
        assert_eq!(got.name, NAME);
        assert_eq!(got.kind, KIND);
        assert_eq!(got.id, ID);
    }

    #[test]
    fn create_requirement_with_risk() {
        // arrange
        const RISK_WANTED: Risk = Risk::High;
        // act
        let got = Requirement::new(KIND, NAME, ID).with_risk(RISK_WANTED);
        // assert
        assert_eq!(got.risk, Some(RISK_WANTED));
    }

    #[test]
    fn create_requirement_with_text() {
        // arrange
        const TEXT_WANTED: &str = "Foo bar";
        // act
        let got = Requirement::new(KIND, NAME, ID).with_text(TEXT_WANTED);
        // assert
        assert_eq!(got.text, Some(TEXT_WANTED.to_string()));
    }

    #[test]
    fn create_requirement_with_verify_method() {
        // arrange
        const METHOD_WANTED: VerifyMethod = VerifyMethod::Analysis;
        // act
        let got = Requirement::new(KIND, NAME, ID).with_verify_method(METHOD_WANTED);
        // assert
        assert_eq!(got.verify_method, Some(METHOD_WANTED));
    }

    #[test]
    fn display_element_with_required_fields_only() {
        // arrange
        let wanted = concat!("requirement milestone {\n", "    id: 1.1.1\n", "}",);
        // act
        let got = Requirement::new(KIND, NAME, ID).to_string();
        // assert
        assert_eq!(got, wanted);
    }

    #[test]
    fn display_element_with_all_fields() {
        // arrange
        let wanted = concat!(
            "functionalRequirement Foo {\n",
            "    id: Bar\n",
            "    risk: Low\n",
            "    text: \"Foo bar\"\n",
            "    verifymethod: Demonstration\n",
            "}",
        );
        // act
        let got = Requirement::new(RequirementType::Functional, "Foo", "Bar")
            .with_risk(Risk::Low)
            .with_text("Foo bar")
            .with_verify_method(VerifyMethod::Demo)
            .to_string();
        // assert
        assert_eq!(got, wanted);
    }
}
