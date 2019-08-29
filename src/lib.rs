//! Example:
//!
//! ```
//! let conf = nbconf::Conf::parse_str("
//!     [Section 1]
//!     hello = world
//!
//!     [Section 2]
//!     nice to = meet you").expect("failed to parse config");
//!
//! assert_eq!(conf.sections[0].name, "Section 1");
//! assert_eq!(conf.sections[0].entries[0].key, "hello");
//! assert_eq!(conf.sections[0].entries[0].value, "world");
//!
//! assert_eq!(conf.sections[1].name, "Section 2");
//! assert_eq!(conf.sections[1].entries[0].key, "nice to");
//! assert_eq!(conf.sections[1].entries[0].value, "meet you");
//! ```

/// The specific type of parse error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseErrorKind {
    /// An entry was found prior to any section being declared.
    EntryOutsideOfSection,
    /// A section was declared, but the closing bracket is missing.
    MissingClosingBracket,
    /// An entry is missing an equals (`=`).
    MissingEquals,
}

/// Error produced from [`Conf::parse_str`].
///
/// [`Conf::parse_str`]: struct.Conf.html#method.parse_str
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError {
    /// Line where the error occurred (starting from 1).
    pub line: usize,
    /// Type of error.
    pub kind: ParseErrorKind,
}

impl ParseError {
    /// Create a new [`ParseError`].
    ///
    /// [`ParseError`]: struct.ParseError.html
    pub fn new(line: usize, kind: ParseErrorKind) -> ParseError {
        ParseError { line, kind }
    }
}

/// A single entry within the section.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    /// Name of the entry.
    pub key: String,
    /// Value of the entry.
    pub value: String,
}

impl Entry {
    pub fn new(key: &str, value: &str) -> Entry {
        Entry {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} = {}", self.key, self.value)
    }
}

/// A named section within the config.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Section {
    /// Name of the section.
    pub name: String,
    /// Entries within the section.
    pub entries: Vec<Entry>,
}

impl Section {
    pub fn new(name: &str) -> Section {
        Section {
            name: name.to_string(),
            entries: Vec::new(),
        }
    }

    pub fn new_with_entries(name: &str, entries: Vec<Entry>) -> Section {
        Section {
            name: name.to_string(),
            entries,
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.iter().find(|e| e.key == key).map(|e| e.value.as_str())
    }

    pub fn to_string(&self) -> String {
        let mut result = format!("[{}]", self.name);
        for entry in self.entries.iter() {
            result += "\n";
            result += &entry.to_string();
        }
        result += "\n";
        result
    }
}

/// A collection of config sections.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Conf {
    pub sections: Vec<Section>,
}

impl Conf {
    /// Create an empty config.
    pub fn new() -> Conf {
        Conf {
            sections: Vec::new(),
        }
    }

    /// Create a pre-populated config.
    pub fn from_sections(sections: Vec<Section>) -> Conf {
        Conf { sections }
    }

    /// Parse a string into a config.
    pub fn parse_str(s: &str) -> Result<Conf, ParseError> {
        let mut conf = Conf::new();
        let mut line_no = 0;
        for line in s.lines() {
            line_no += 1;
            let line = line.trim();
            if line.starts_with('[') {
                if line.ends_with(']') {
                    let name = &line[1..line.len() - 1];
                    conf.sections.push(Section::new(name));
                } else {
                    return Err(ParseError::new(
                        line_no,
                        ParseErrorKind::MissingClosingBracket,
                    ));
                }
            } else if line.len() != 0 {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    if let Some(section) = conf.sections.last_mut() {
                        section.entries.push(Entry {
                            key: parts[0].trim().to_string(),
                            value: parts[1].trim().to_string(),
                        });
                    } else {
                        return Err(ParseError::new(
                            line_no,
                            ParseErrorKind::EntryOutsideOfSection,
                        ));
                    }
                } else {
                    return Err(ParseError::new(line_no, ParseErrorKind::MissingEquals));
                }
            }
        }
        Ok(conf)
    }

    /// Serialize the config as a string.
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        let mut is_first = true;
        for section in self.sections.iter() {
            if is_first {
                is_first = false;
            } else {
                output += "\n";
            }
            output += &section.to_string();
        }
        output
    }

    /// Append a section to the config.
    pub fn add_section(&mut self, name: &str, entries: Vec<Entry>) {
        self.sections.push(Section { name: name.to_string(), entries });
    }

    /// Get all the sections' names.
    pub fn section_names(&self) -> Vec<&str> {
        self.sections.iter().map(|section| section.name.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section() {
        let mut expected = Conf::new();
        expected.add_section("mySection", Vec::new());
        assert_eq!(Conf::parse_str("[mySection]"), Ok(expected));
    }

    #[test]
    fn test_entry() {
        let mut expected = Conf::new();
        expected.add_section("mySection", vec![
            Entry {
                key: "a".to_string(),
                value: "b".to_string(),
            }
        ]);
        assert_eq!(Conf::parse_str("[mySection]\na = b"), Ok(expected));
    }

    #[test]
    fn test_to_string() {
        let mut conf = Conf::new();
        conf.add_section("sec1", vec![
            Entry {
                key: "a".to_string(),
                value: "b".to_string(),
            }
        ]);
        conf.add_section("sec2", vec![
            Entry {
                key: "c".to_string(),
                value: "d".to_string(),
            }
        ]);
        assert_eq!(conf.to_string(), "[sec1]\na = b\n\n[sec2]\nc = d\n");
    }

    #[test]
    fn test_section_get() {
        let conf = Conf::from_sections(vec![
            Section::new_with_entries("mySection", vec![
                Entry::new("a", "b"),
                Entry::new("x", "y"),
            ])
        ]);
        assert_eq!(conf.sections[0].get("x"), Some("y"));
    }

    #[test]
    fn test_conf_section_names() {
        let conf = Conf::from_sections(vec![
            Section::new("sec1"),
            Section::new("sec2"),
        ]);
        assert_eq!(conf.section_names(), vec!["sec1", "sec2"]);
    }

    #[test]
    fn test_missing_closing_bracket() {
        assert_eq!(
            Conf::parse_str("[mySection"),
            Err(ParseError::new(1, ParseErrorKind::MissingClosingBracket))
        );
    }

    #[test]
    fn test_missing_equals() {
        assert_eq!(
            Conf::parse_str("[mySection]\nmyKey"),
            Err(ParseError::new(2, ParseErrorKind::MissingEquals))
        );
    }

    #[test]
    fn test_entry_outside_of_section() {
        assert_eq!(
            Conf::parse_str("a = b"),
            Err(ParseError::new(1, ParseErrorKind::EntryOutsideOfSection))
        );
    }
}
